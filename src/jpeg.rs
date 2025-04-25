use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::fmt::Display;
use thiserror::Error;
use tracing::{debug, error, info, trace};

pub struct Jpeg {
    segments: Vec<Segment>,
}

pub struct Segment {
    buffer: Bytes,
}

pub fn fix_jpeg_height(buffer: Bytes) -> Result<Option<Bytes>, ParseError> {
    let jpeg = Jpeg::from_bytes(buffer)?;
    trace!("{}", jpeg);
    if let Some(height) = jpeg.get_height_from_dnl() {
        info!("Use jpeg height from DNL segment: {height}");
        let jpeg = jpeg.with_height(height);
        Ok(Some(jpeg.into()))
    } else {
        info!("No DNL segment found.");
        Ok(None)
    }
}

#[derive(Debug, Error)]
#[error("Failed to parse: {message}")]
pub struct ParseError {
    message: String,
}

impl From<String> for ParseError {
    fn from(message: String) -> Self {
        Self { message }
    }
}

#[allow(unused)]
impl Jpeg {
    const MARKER_START: u8 = 0xff;

    // Start of Frame
    const SOF0: u8 = 0xc0;
    // Define Huffman Table
    const DHT: u8 = 0xc4;
    // {Start,End} of Image
    const SOI: u8 = 0xd8;
    const EOI: u8 = 0xd9;
    // Start of Scan
    const SOS: u8 = 0xda;
    // Define Quantization Table
    const DQT: u8 = 0xdb;
    // Define Number of Lines
    const DNL: u8 = 0xdc;
    // Define Restart Interval
    const DRI: u8 = 0xdd;
    // Application Segments
    const APP0: u8 = 0xe0;
}

impl Jpeg {
    pub fn from_bytes(mut buffer: Bytes) -> Result<Self, ParseError> {
        let mut segments = Vec::new();
        while buffer.has_remaining() {
            let segment = split_segment(&mut buffer)?;
            segments.push(segment);
        }
        Ok(Self { segments })
    }

    pub fn get_height_from_dnl(&self) -> Option<u16> {
        if let Some(dnl) = self.segments.iter().find(|s| s.marker() == Jpeg::DNL) {
            let mut buf = dnl.buffer.clone();
            buf.advance(4);
            Some(buf.get_u16())
        } else {
            None
        }
    }

    pub fn with_height(self, height: u16) -> Jpeg {
        let segments = self
            .segments
            .into_iter()
            .map(|s| {
                if s.marker() == Jpeg::SOF0 {
                    s.with_height(height)
                } else {
                    s
                }
            })
            .collect();
        Jpeg { segments }
    }

    pub fn segments(&self) -> &Vec<Segment> {
        &self.segments
    }
}

impl From<Jpeg> for Bytes {
    fn from(value: Jpeg) -> Self {
        let size = value.segments.iter().map(|s| s.len()).sum();
        let mut buffer = BytesMut::with_capacity(size);
        for segment in value.segments {
            buffer.put(segment.buffer);
        }
        buffer.into()
    }
}

impl Display for Jpeg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "---------------------------------")?;
        writeln!(
            f,
            "| {: <3} | {: <6} | {: <12} |",
            "i", "marker", "total length"
        )?;
        writeln!(f, "---------------------------------")?;

        for (i, segment) in self.segments().iter().enumerate() {
            // marker printed in HEX
            let marker = format!("{:X}", segment.marker());
            let len = segment.len();
            writeln!(f, "| {i: <3} | {marker: <6} | {len: <12} |")?;
        }
        Ok(())
    }
}

impl Segment {
    fn new(buffer: Bytes) -> Self {
        Self { buffer }
    }

    pub fn marker(&self) -> u8 {
        self.buffer[1]
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    fn with_height(self, height: u16) -> Self {
        assert!(self.marker() == Jpeg::SOF0);
        let mut before = self.buffer;
        let mut b = BytesMut::with_capacity(before.remaining());
        for _ in 0..5 {
            b.put_u8(before.get_u8());
        }
        b.put_u16(height);
        before.advance(2);
        while before.has_remaining() {
            b.put_u8(before.get_u8());
        }
        Segment::new(b.into())
    }
}

fn split_segment(buffer: &mut Bytes) -> Result<Segment, String> {
    if buffer.remaining() < 2 {
        return Err("buffer < 2 bytes".to_owned());
    }
    if buffer[0] != Jpeg::MARKER_START {
        return Err("segment not starting with 0xff".to_owned());
    }
    let marker = buffer[1];
    let len = match marker {
        Jpeg::SOI | Jpeg::EOI => 2usize,
        Jpeg::SOS => scan_for_sos_end(buffer.clone()),
        _ => {
            if buffer.remaining() < 4 {
                return Err("buffer < 4 bytes".to_owned());
            }
            let mut b = buffer.clone();
            b.advance(2);
            let size: usize = b.get_u16().into();
            2usize + size
        }
    };
    debug!("Segment {marker:x}: len = {len}");
    if buffer.remaining() < len {
        return Err(format!(
            "buffer smaller than size: {} < {}",
            buffer.remaining(),
            len
        ));
    }
    Ok(Segment::new(buffer.split_to(len)))
}

fn scan_for_sos_end(mut buffer: Bytes) -> usize {
    let mut count: usize = 2;
    buffer.advance(2);
    let mut possible_marker = false;
    loop {
        if buffer.remaining() == 0 {
            return count;
        }
        let byte = buffer.get_u8();
        if possible_marker {
            if is_marker(byte) {
                count -= 1;
                break;
            } else {
                possible_marker = false;
            }
        }
        if byte == Jpeg::MARKER_START {
            possible_marker = true;
        }
        count += 1;
    }
    count
}

fn is_marker(byte: u8) -> bool {
    match byte {
        0x00 => false,
        b if (0xD0..=0xD7).contains(&b) => false,
        _ => true,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn load_image(path: &str) -> Bytes {
        let buffer = std::fs::read(path).unwrap();
        buffer.into()
    }

    #[test]
    fn split_segment_when_invalid_buffer_then_error() {
        let mut buffer = Bytes::new();
        assert_eq!(
            Some("buffer < 2 bytes".to_owned()),
            split_segment(&mut buffer).err()
        );

        buffer = vec![0x00, 0x00].into();
        assert_eq!(
            Some("segment not starting with 0xff".to_owned()),
            split_segment(&mut buffer).err()
        );
    }

    #[test]
    fn split_segment_when_soi_then_create_segment() {
        let mut buffer = vec![Jpeg::MARKER_START, Jpeg::SOI].into();
        let segment = split_segment(&mut buffer).unwrap();
        assert_eq!(buffer.remaining(), 0);
        assert_eq!(segment.len(), 2);
        assert_eq!(segment.marker(), Jpeg::SOI);
    }

    #[test]
    fn split_segment_when_eoi_then_create_segment() {
        let mut buffer = vec![Jpeg::MARKER_START, Jpeg::EOI].into();
        let segment = split_segment(&mut buffer).unwrap();
        assert_eq!(buffer.remaining(), 0);
        assert_eq!(segment.len(), 2);
        assert_eq!(segment.marker(), Jpeg::EOI);
    }

    const DNL_TEST_FILE: &str = "doc/testdata/scan_from_adf_with_dnl_header.jpeg";

    #[test]
    fn load_file_with_dnl_header() {
        let buffer = load_image(DNL_TEST_FILE);
        let jpeg = Jpeg::from_bytes(buffer).unwrap();
        assert_eq!(jpeg.segments.len(), 10);
        assert_eq!(jpeg.get_height_from_dnl(), Some(3490));
    }

    #[test]
    fn read_segments_of_jpeg_file() {
        let mut buffer = load_image(DNL_TEST_FILE);
        let s1 = split_segment(&mut buffer).unwrap();
        let s2 = split_segment(&mut buffer).unwrap();
        let s3 = split_segment(&mut buffer).unwrap();
        let s4 = split_segment(&mut buffer).unwrap();
        let s5 = split_segment(&mut buffer).unwrap();
        let s6 = split_segment(&mut buffer).unwrap();
        let s7 = split_segment(&mut buffer).unwrap();
        let s8 = split_segment(&mut buffer).unwrap();
        let s9 = split_segment(&mut buffer).unwrap();
        let s10 = split_segment(&mut buffer).unwrap();
        let s11 = split_segment(&mut buffer);
        assert_eq!(s1.marker(), Jpeg::SOI);
        assert_eq!(s2.marker(), Jpeg::APP0);
        assert_eq!(s2.len(), 18);
        assert_eq!(s3.marker(), Jpeg::DQT);
        assert_eq!(s3.len(), 69);
        assert_eq!(s4.marker(), Jpeg::DQT);
        assert_eq!(s4.len(), 69);
        assert_eq!(s5.marker(), Jpeg::SOF0);
        assert_eq!(s5.len(), 19);
        assert_eq!(s6.marker(), Jpeg::DHT);
        assert_eq!(s6.len(), 420);
        assert_eq!(s7.marker(), Jpeg::DRI);
        assert_eq!(s7.len(), 6);
        assert_eq!(s8.marker(), Jpeg::SOS);
        assert_eq!(s8.len(), 220950);
        assert_eq!(s9.marker(), Jpeg::DNL);
        assert_eq!(s9.len(), 6);
        assert_eq!(s10.marker(), Jpeg::EOI);
        assert_eq!(s10.len(), 2);
        assert!(s11.is_err());
    }
}

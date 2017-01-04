extern crate xml;

use self::xml::{EmitterConfig, EventWriter};
use self::xml::writer::events::XmlEvent;
use self::xml::writer::Result;
use self::xml::name::Name;
use self::xml::namespace::Namespace;

use std::io::Write;
use std::borrow::Cow;

const XML_NAMESPACE: &'static str = "http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19";
const PREFIX: &'static str = "scan";

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputSource {
    Platen,
    Adf,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Format {
    Jpeg,
    Pdf,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColorSpace {
    Color,
    Gray,
}

pub struct ScanJob {
    input_source: InputSource,
    high_resolution: bool,
    format: Format,
    color_space: ColorSpace,
}

impl ScanJob {
    pub fn new(input_source: InputSource, high_resolution: bool,
               format: Format, color_space: ColorSpace) -> ScanJob {
        ScanJob {
            input_source: input_source,
            high_resolution: high_resolution,
            format: format,
            color_space: color_space,
        }
    }

    pub fn write_xml<W: Write>(&self, sink: W) -> Result<()> {
        fn enter_elem<W: Write>(w: &mut EventWriter<W>, name: &str) -> Result<()> {
            let mut namespace = Namespace::empty();
            namespace.put(PREFIX, XML_NAMESPACE);
            let empty_attrs = Vec::new();
            w.write(
                XmlEvent::StartElement {
                    name: Name::qualified(name, XML_NAMESPACE, Option::from(PREFIX)),
                    attributes: Cow::Borrowed(&empty_attrs),
                    namespace: Cow::Borrowed(&namespace),
                }
            )
        }
        fn exit_elem<W: Write>(w: &mut EventWriter<W>) -> Result<()> {
            w.write(
                XmlEvent::EndElement {
                    name: Option::None,
                }
            )
        }
        fn write_value<W: Write>(w: &mut EventWriter<W>, name: &str, val: &str) -> Result<()> {
            try!(enter_elem(w, name));
            try!(w.write(val));
            exit_elem(w)
        }

        let config = EmitterConfig::new().write_document_declaration(true).perform_indent(true);
        let mut writer = config.create_writer(sink);
        try!(enter_elem(&mut writer, "ScanJob"));
        let resolution = if self.high_resolution { "600" } else { "300" };
        try!(write_value(&mut writer, "XResolution", resolution));
        try!(write_value(&mut writer, "YResolution", resolution));
        try!(write_value(&mut writer, "XStart", "0"));
        try!(write_value(&mut writer, "YStart", "0"));
        try!(write_value(&mut writer, "Width", "2480"));
        try!(write_value(&mut writer, "Height", "3508"));
        let format = match self.format {
            Format::Jpeg => "Jpeg",
            Format::Pdf => "Pdf",
        };
        try!(write_value(&mut writer, "Format", format));
        try!(write_value(&mut writer, "CompressionQFactor", "25"));
        let color = match self.color_space {
            ColorSpace::Color => "Color",
            ColorSpace::Gray => "Gray",
        };
        try!(write_value(&mut writer, "ColorSpace", color));
        try!(write_value(&mut writer, "BitDepth", "8"));
        let source = match self.input_source {
            InputSource::Platen => "Platen",
            InputSource::Adf => "Adf",
        };
        try!(write_value(&mut writer, "InputSource", source));
        if self.input_source == InputSource::Adf {
            try!(enter_elem(&mut writer, "AdfOptions"));
            try!(exit_elem(&mut writer));
        }
        try!(write_value(&mut writer, "GrayRendering", "NTSC"));

        try!(enter_elem(&mut writer, "ToneMap"));
        try!(write_value(&mut writer, "Gamma", "1000"));
        try!(write_value(&mut writer, "Brightness", "1000"));
        try!(write_value(&mut writer, "Contrast", "1000"));
        try!(write_value(&mut writer, "Highlite", "179"));
        try!(write_value(&mut writer, "Shadow", "25"));
        try!(exit_elem(&mut writer));

        let content_type = match self.format {
            Format::Jpeg => "Photo",
            Format::Pdf => "Document",
        };
        try!(write_value(&mut writer, "ContentType", content_type));
        exit_elem(&mut writer)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test() {
        let job = ScanJob::new(InputSource::Platen, true, Format::Pdf, ColorSpace::Color);
        let mut target: Vec<u8> = Vec::new();
        job.write_xml(&mut target).unwrap();
        let result = String::from_utf8(target).unwrap();
        println!("{}", result);
    }
}

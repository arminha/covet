extern crate xml;

use self::xml::{EmitterConfig, EventWriter};
use self::xml::name::Name;
use self::xml::namespace::Namespace;
use self::xml::writer::events::XmlEvent;
use self::xml::writer::Result;

use std::borrow::Cow;
use std::io::Write;

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
    resolution: u32,
    format: Format,
    color_space: ColorSpace,
}

impl ScanJob {
    pub fn new(input_source: InputSource, resolution: u32,
               format: Format, color_space: ColorSpace) -> ScanJob {
        ScanJob {
            input_source: input_source,
            resolution: resolution,
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
        let resolution = self.resolution.to_string();
        try!(write_value(&mut writer, "XResolution", &resolution));
        try!(write_value(&mut writer, "YResolution", &resolution));
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

    const JPEG_GLASS_LOW: &'static str = "<?xml version=\"1.0\" encoding=\"utf-8\"?>\
        \n<scan:ScanJob xmlns:scan=\"http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19\">\
        \n  <scan:XResolution>300</scan:XResolution>\
        \n  <scan:YResolution>300</scan:YResolution>\
        \n  <scan:XStart>0</scan:XStart>\
        \n  <scan:YStart>0</scan:YStart>\
        \n  <scan:Width>2480</scan:Width>\
        \n  <scan:Height>3508</scan:Height>\
        \n  <scan:Format>Jpeg</scan:Format>\
        \n  <scan:CompressionQFactor>25</scan:CompressionQFactor>\
        \n  <scan:ColorSpace>Color</scan:ColorSpace>\
        \n  <scan:BitDepth>8</scan:BitDepth>\
        \n  <scan:InputSource>Platen</scan:InputSource>\
        \n  <scan:GrayRendering>NTSC</scan:GrayRendering>\
        \n  <scan:ToneMap>\
        \n    <scan:Gamma>1000</scan:Gamma>\
        \n    <scan:Brightness>1000</scan:Brightness>\
        \n    <scan:Contrast>1000</scan:Contrast>\
        \n    <scan:Highlite>179</scan:Highlite>\
        \n    <scan:Shadow>25</scan:Shadow>\
        \n  </scan:ToneMap>\
        \n  <scan:ContentType>Photo</scan:ContentType>\
        \n</scan:ScanJob>";

    const PDF_ADF_HIGH: &'static str = "<?xml version=\"1.0\" encoding=\"utf-8\"?>\
        \n<scan:ScanJob xmlns:scan=\"http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19\">\
        \n  <scan:XResolution>600</scan:XResolution>\
        \n  <scan:YResolution>600</scan:YResolution>\
        \n  <scan:XStart>0</scan:XStart>\
        \n  <scan:YStart>0</scan:YStart>\
        \n  <scan:Width>2480</scan:Width>\
        \n  <scan:Height>3508</scan:Height>\
        \n  <scan:Format>Pdf</scan:Format>\
        \n  <scan:CompressionQFactor>25</scan:CompressionQFactor>\
        \n  <scan:ColorSpace>Gray</scan:ColorSpace>\
        \n  <scan:BitDepth>8</scan:BitDepth>\
        \n  <scan:InputSource>Adf</scan:InputSource>\
        \n  <scan:AdfOptions />\
        \n  <scan:GrayRendering>NTSC</scan:GrayRendering>\
        \n  <scan:ToneMap>\
        \n    <scan:Gamma>1000</scan:Gamma>\
        \n    <scan:Brightness>1000</scan:Brightness>\
        \n    <scan:Contrast>1000</scan:Contrast>\
        \n    <scan:Highlite>179</scan:Highlite>\
        \n    <scan:Shadow>25</scan:Shadow>\
        \n  </scan:ToneMap>\
        \n  <scan:ContentType>Document</scan:ContentType>\
        \n</scan:ScanJob>";

    fn write_to_string(job: ScanJob) -> String {
        let mut target: Vec<u8> = Vec::new();
        job.write_xml(&mut target).unwrap();
        String::from_utf8(target).unwrap()
    }

    #[test]
    fn scan_job_write_xml_jpeg() {
        let job = ScanJob::new(InputSource::Platen, 300, Format::Jpeg, ColorSpace::Color);
        assert_eq!(JPEG_GLASS_LOW, write_to_string(job));
    }

    #[test]
    fn scan_job_write_xml_pdf() {
        let job = ScanJob::new(InputSource::Adf, 600, Format::Pdf, ColorSpace::Gray);
        assert_eq!(PDF_ADF_HIGH, write_to_string(job));
    }
}

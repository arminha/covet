use serde::Deserialize;
use xml::name::Name;
use xml::namespace::Namespace;
use xml::writer::Result;
use xml::writer::events::XmlEvent;
use xml::{EmitterConfig, EventWriter};

use std::borrow::Cow;
use std::io::Write;

const XML_NAMESPACE: &str = "http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19";
const PREFIX: &str = "scan";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputSource {
    Platen,
    Adf,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    Jpeg,
    Pdf,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ColorSpace {
    Color,
    Gray,
}

#[derive(Debug)]
pub struct ScanJob {
    pub input_source: InputSource,
    resolution: u32,
    quality: u32,
    pub format: Format,
    color_space: ColorSpace,
}

impl ScanJob {
    pub fn new(
        input_source: InputSource,
        resolution: u32,
        quality: u32,
        format: Format,
        color_space: ColorSpace,
    ) -> ScanJob {
        ScanJob {
            input_source,
            resolution,
            quality,
            format,
            color_space,
        }
    }

    pub fn write_xml<W: Write>(&self, sink: W) -> Result<()> {
        let config = EmitterConfig::new()
            .write_document_declaration(true)
            .perform_indent(true);
        let mut writer = XmlWriter::new(config.create_writer(sink));
        writer.enter_elem("ScanJob")?;
        let resolution = self.resolution.to_string();
        writer.write_value("XResolution", &resolution)?;
        writer.write_value("YResolution", &resolution)?;
        writer.write_value("XStart", "0")?;
        writer.write_value("YStart", "0")?;
        writer.write_value("Width", "2480")?;
        writer.write_value("Height", "3508")?;
        let format = match self.format {
            Format::Jpeg => "Jpeg",
            Format::Pdf => "Pdf",
        };
        writer.write_value("Format", format)?;
        writer.write_value("CompressionQFactor", &self.quality.to_string())?;
        let color = match self.color_space {
            ColorSpace::Color => "Color",
            ColorSpace::Gray => "Gray",
        };
        writer.write_value("ColorSpace", color)?;
        writer.write_value("BitDepth", "8")?;
        let source = match self.input_source {
            InputSource::Platen => "Platen",
            InputSource::Adf => "Adf",
        };
        writer.write_value("InputSource", source)?;
        if self.input_source == InputSource::Adf {
            writer.empty_elem("AdfOptions")?;
        }
        writer.write_value("GrayRendering", "NTSC")?;

        writer.with_elem("ToneMap", |w| {
            w.write_value("Gamma", "1000")?;
            w.write_value("Brightness", "1000")?;
            w.write_value("Contrast", "1000")?;
            w.write_value("Highlite", "179")?;
            w.write_value("Shadow", "25")
        })?;

        let content_type = match self.format {
            Format::Jpeg => "Photo",
            Format::Pdf => "Document",
        };
        writer.write_value("ContentType", content_type)?;
        writer.exit_elem()
    }
}

struct XmlWriter<W: Write> {
    inner: EventWriter<W>,
}

impl<W: Write> XmlWriter<W> {
    fn new(inner: EventWriter<W>) -> Self {
        Self { inner }
    }

    fn enter_elem(&mut self, name: &str) -> Result<()> {
        let mut namespace = Namespace::empty();
        namespace.put(PREFIX, XML_NAMESPACE);
        let empty_attrs = Vec::new();
        self.inner.write(XmlEvent::StartElement {
            name: Name::qualified(name, XML_NAMESPACE, Option::from(PREFIX)),
            attributes: Cow::Borrowed(&empty_attrs),
            namespace: Cow::Borrowed(&namespace),
        })
    }

    fn exit_elem(&mut self) -> Result<()> {
        self.inner
            .write(XmlEvent::EndElement { name: Option::None })
    }

    fn write_value(&mut self, name: &str, val: &str) -> Result<()> {
        self.enter_elem(name)?;
        self.inner.write(val)?;
        self.exit_elem()
    }

    fn empty_elem(&mut self, name: &str) -> Result<()> {
        self.enter_elem(name)?;
        self.exit_elem()
    }

    fn with_elem(&mut self, name: &str, closure: impl Fn(&mut Self) -> Result<()>) -> Result<()> {
        self.enter_elem(name)?;
        closure(self)?;
        self.exit_elem()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const JPEG_GLASS_LOW: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<scan:ScanJob xmlns:scan="http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19">
  <scan:XResolution>300</scan:XResolution>
  <scan:YResolution>300</scan:YResolution>
  <scan:XStart>0</scan:XStart>
  <scan:YStart>0</scan:YStart>
  <scan:Width>2480</scan:Width>
  <scan:Height>3508</scan:Height>
  <scan:Format>Jpeg</scan:Format>
  <scan:CompressionQFactor>25</scan:CompressionQFactor>
  <scan:ColorSpace>Color</scan:ColorSpace>
  <scan:BitDepth>8</scan:BitDepth>
  <scan:InputSource>Platen</scan:InputSource>
  <scan:GrayRendering>NTSC</scan:GrayRendering>
  <scan:ToneMap>
    <scan:Gamma>1000</scan:Gamma>
    <scan:Brightness>1000</scan:Brightness>
    <scan:Contrast>1000</scan:Contrast>
    <scan:Highlite>179</scan:Highlite>
    <scan:Shadow>25</scan:Shadow>
  </scan:ToneMap>
  <scan:ContentType>Photo</scan:ContentType>
</scan:ScanJob>"#;

    const PDF_ADF_HIGH: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<scan:ScanJob xmlns:scan="http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19">
  <scan:XResolution>600</scan:XResolution>
  <scan:YResolution>600</scan:YResolution>
  <scan:XStart>0</scan:XStart>
  <scan:YStart>0</scan:YStart>
  <scan:Width>2480</scan:Width>
  <scan:Height>3508</scan:Height>
  <scan:Format>Pdf</scan:Format>
  <scan:CompressionQFactor>1</scan:CompressionQFactor>
  <scan:ColorSpace>Gray</scan:ColorSpace>
  <scan:BitDepth>8</scan:BitDepth>
  <scan:InputSource>Adf</scan:InputSource>
  <scan:AdfOptions />
  <scan:GrayRendering>NTSC</scan:GrayRendering>
  <scan:ToneMap>
    <scan:Gamma>1000</scan:Gamma>
    <scan:Brightness>1000</scan:Brightness>
    <scan:Contrast>1000</scan:Contrast>
    <scan:Highlite>179</scan:Highlite>
    <scan:Shadow>25</scan:Shadow>
  </scan:ToneMap>
  <scan:ContentType>Document</scan:ContentType>
</scan:ScanJob>"#;

    fn write_to_string(job: ScanJob) -> String {
        let mut target: Vec<u8> = Vec::new();
        job.write_xml(&mut target).unwrap();
        String::from_utf8(target).unwrap()
    }

    #[test]
    fn scan_job_write_xml_jpeg() {
        let job = ScanJob::new(
            InputSource::Platen,
            300,
            25,
            Format::Jpeg,
            ColorSpace::Color,
        );
        assert_eq!(JPEG_GLASS_LOW, write_to_string(job));
    }

    #[test]
    fn scan_job_write_xml_pdf() {
        let job = ScanJob::new(InputSource::Adf, 600, 1, Format::Pdf, ColorSpace::Gray);
        assert_eq!(PDF_ADF_HIGH, write_to_string(job));
    }
}

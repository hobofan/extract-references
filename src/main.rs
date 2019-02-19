/// Raw text extracted from a PDF.
#[derive(Debug, Clone)]
pub struct RawText {
    pub inner: String,
}

impl RawText {
    pub fn from_path<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let content = pdf_extract::extract_text(path)?;

        Ok(Self { inner: content })
    }

    /// Try to locate the reference section by detecting something that looks like the header
    /// to a reference section.
    pub fn reference_section_by_header(&self) -> Option<&str> {
        let pattern = "\nReferences\n";
        let start_index = self.inner.find(pattern)?;

        Some(&self.inner[start_index + pattern.len()..])
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let in_path = &args[1];
    let content = RawText::from_path(in_path).unwrap();
    println!("content: {:?}", content);
    println!(
        "reference_section_by_header: {}",
        content.reference_section_by_header().unwrap()
    );
}

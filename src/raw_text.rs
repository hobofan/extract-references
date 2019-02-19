use regex::Regex;

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

        Some(&self.inner[start_index + pattern.len() + 1..])
    }

    /// Try to infer the pattern with which the start of a reference item can be detected.
    pub fn reference_item_pattern(&self) -> Option<Regex> {
        let reference_section = self.reference_section_by_header()?;

        if "[1]" == &reference_section[0..3] {
            return Some(Regex::new(r"\[(?P<id>[0-9]+)\]\s+").unwrap());
        }
        return None;
    }

    pub fn references(&self) -> Option<Vec<Reference>> {
        let reference_section = self.reference_section_by_header()?;
        let reference_item_pattern = self.reference_item_pattern()?;

        let splits = reference_item_pattern.split(reference_section).skip(1);
        let id_captures = reference_item_pattern.captures_iter(reference_section);
        Some(
            splits
                .zip(id_captures)
                .filter_map(|(split, id_capture)| {
                    Some(Reference {
                        reference_id: id_capture.name("id")?.as_str().to_owned(),
                        inner: split.trim().to_owned(),
                    })
                })
                .filter(|n| !n.inner.is_empty())
                .collect::<Vec<_>>(),
        )
    }
}

#[derive(Debug, Clone)]
pub struct Reference {
    pub reference_id: String,
    pub inner: String,
}

#[cfg(test)]
mod tests {
    use super::RawText;

    #[test]
    fn reference_section_by_header_fixture_1() {
        let raw_text = RawText::from_path("./tests/external_data/swj120_2.pdf").unwrap();
        assert_eq!(
            raw_text.reference_section_by_header().unwrap(),
            include_str!("../tests/fixtures/pdf_1_reference_section")
        );
    }

    #[test]
    fn reference_item_pattern_fixture_1() {
        let raw_text = RawText::from_path("./tests/external_data/swj120_2.pdf").unwrap();
        assert!(raw_text.reference_item_pattern().is_some(),);
    }

    #[test]
    fn references_fixture_1() {
        let raw_text = RawText::from_path("./tests/external_data/swj120_2.pdf").unwrap();
        assert_eq!(raw_text.references().unwrap().len(), 55);
        assert_eq!(
            raw_text.references().unwrap().last().unwrap().reference_id,
            "55"
        );
    }
}

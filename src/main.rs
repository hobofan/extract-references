mod raw_text;

use raw_text::RawText;

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

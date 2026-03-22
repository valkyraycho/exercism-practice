use unicode_segmentation::UnicodeSegmentation;
pub fn reverse(input: &str) -> String {
    // todo!("Write a function to reverse {input}");
    input.graphemes(true).rev().collect()
}

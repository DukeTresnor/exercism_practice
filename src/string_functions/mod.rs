// string_functions mod.rs

use unicode_segmentation::UnicodeSegmentation;


pub fn reverse(input: &str) -> String {
    //unimplemented!("Write a function to reverse {input}");
    //format!("{:?}", input.chars().rev().collect())
    //let mut change_input: &mut str = input;
    //reverse_grapheme_clusters_in_place(change_input);
    //change_input.to_string()
    let reverse_input = input.graphemes(true).rev().collect();
    reverse_input
}

// string_functions mod.rs

use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashSet;

pub fn reverse(input: &str) -> String {
    //unimplemented!("Write a function to reverse {input}");
    //format!("{:?}", input.chars().rev().collect())
    //let mut change_input: &mut str = input;
    //reverse_grapheme_clusters_in_place(change_input);
    //change_input.to_string()
    let reverse_input = input.graphemes(true).rev().collect();
    reverse_input
}



// -- lifetime problems -- //

/* 
'a means lifetime a
anagrams_for take a generic input, specifically one lifetime called 'a
'a is a generic lifetime parameter, like how <T> is usually used for a generic type
*/

// Here this function takes in 2 references, and returns a HashSet of string references
// The code doesn't know the lifetime of the output reference, since it doesn't necessarilly know which
//   reference the output is trying to refer to (word or possible_anagrams)
// Shouldn't word and possible_anagrams have specified lifetimes a? ie word: &'a str ?? Maybe this isn't
//   necessary because anagrams_for specifies a lifetime of a
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // params: word -- string slice (reference)
    // params: possible_anagrams -- reference to a list of string slices (reference to a list of references)
    // returns: hash_output -- A HashSet of string slices wth a generic lifetime a ??
    //unimplemented!(
    //    "For the '{word}' word find anagrams among the following words: {possible_anagrams:?}"
    //);
    // let hash_output = HashSet<&'a str>
    // let sorted word = word.sort_unstable
    // for anagram in reference possible_anagrams (.iter??)
    //   if sorted word == anagram.sort_unstable
    //     push anagram into hash_output --> .insert(anagram) <-- anagram should be a reference with lifetime a
    // hash_output

    // input.chars().rev().collect())

    let mut hash_output = HashSet::<&'a str>::new();


    // wrap this set of 3 commands into a function
    let lower_word = word.to_lowercase();
    let mut lower_word_vec: Vec<_> = lower_word.chars().collect();
    let base_word = lower_word;
    let base_word_vec: Vec<_> = base_word.chars().collect();
    lower_word_vec.sort_unstable();


    // and if anagram isn't original word


    for anagram in possible_anagrams.iter() {
        let lower_anagram = anagram.to_lowercase();
        let mut lower_anagram_vec: Vec<_> = lower_anagram.chars().collect();

        let base_anagram = lower_anagram;
        let base_anagram_vec: Vec<_> = base_anagram.chars().collect();

        lower_anagram_vec.sort_unstable();

        println!("word_vec: {:?}", lower_word_vec);
        println!("anagram_vec: {:?}", lower_anagram_vec);


        if lower_word_vec == lower_anagram_vec && base_word_vec != base_anagram_vec { // issue is here
            hash_output.insert(anagram);
        }
    }

    hash_output

}

// -- lifetime problems -- //
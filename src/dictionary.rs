#[path = "utils.rs"]
mod utils;

extern crate serde_wasm_bindgen;

use std::collections::{HashMap};

use serde::ser::{Serialize, Serializer, SerializeStruct};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Default)]
pub struct Dictionary {
    name: String,
    tag: String,
}

#[wasm_bindgen]
impl Dictionary {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Dictionary {
        utils::set_panic_hook();
        Dictionary {
            name: String::new(),
            tag: String::new(),
        }
    }

    pub fn get_articles_data(&self) -> JsValue {
        // Dummy data: TODO: remove
        // Move to json file or something
        let programing_names: Vec<String> = vec![
            "Generating all subsets using basic Combinatorial Patterns",
            "Uninformed search in Artificial Intelligence",
            "Encoding logic in Artificial Intelligence",
            "Encoding logic in Artificial Intelligence using Theorem Proving",
            "Lexicographic permutation generation",
        ].into_iter().map(|i| i.to_string()).collect();

        let pe_names: Vec<String> = vec![
            "Is it?", "0x1: A Godless world or not?", "What am I?",
            "0x2: Unknown", "0x2: Not Not Single", "The Real",
            "Could a meme make mind?", "F**k red & blue. I want the green pill",
            "The Prison", "The Hidden World", "Love and The Universe",
            "The City", "The Universe Within", "Save Yourselves",
            "The RTC", "Unexplored light", "The Hunt",
        ].into_iter().map(|i| i.to_string()).collect();

        let programing_tags: Vec<String> = vec![
            "1,Combinatorics,Programming,Artificial Intelligence",
            "2,Search,Programming,Artificial Intelligence",
            "3,Logic,Knowledge,Programming,Artificial Intelligence",
            "4,Logic,Knowledge,Programming,Artificial Intelligence",
            "14,Combinatorics,Algorithms,Python,Permutations,Lexicographic",
        ].into_iter().map(|i| i.to_string()).collect();

        let pe_tags: Vec<String> = vec![
            "5,Poetry,Consciousness,Mind,Artificial Intelligence",
            "21,God,Worlds,FreeWill,Essay,Consciousness",
            "18,Consciousness,Poetry,Unconsciousness",
            "19,Consciousness,Essay,God,FreeWill,Simulation",
            "20,Consciousness,Essay,God,FreeWill,Simulation",
            "6,Poetry,Consciousness,Imagination",
            "7,Consciousness,Poetry,Artificial Intelligence,Memes,Richard Dawkins",
            "8,Poetry,Love,Consciousness",
            "9,Poetry,Consciousness,Escape,Mind",
            "10,Poetry,Consciousness,Imagination",
            "11,Love,Poetry,Consciousness,Imagination",
            "12,Artificial Intelligence,Dreams,Mind,Consciousness,Imagination",
            "13,Imagination,Consciousness,Poetry",
            "15,Poetry,Consciousness,Imagination",
            "16,Poetry,Imagination,Consciousness,Unknown",
            "17,Poetry,Otherworlds,Minds,Consciousness,TheoryOfMind",
            "22,Poetry,God,Humanity,FreeWill,Supernatural",
        ].into_iter().map(|i| i.to_string()).collect();

        let programing_dictionaries: Vec<Dictionary> = programing_names.into_iter().zip(programing_tags)
            .map(|(name, tag)| Dictionary { name, tag }).collect();

        let pe_dictionaries: Vec<Dictionary> = pe_names.into_iter().zip(pe_tags)
            .map(|(name, tag)| Dictionary { name, tag }).collect();

        let hash_map = HashMap::from([
            ("programing_ai", programing_dictionaries),
            ("poetry_and_essays", pe_dictionaries),
        ]);
        serde_wasm_bindgen::to_value(&hash_map).unwrap()
    }
}

impl Serialize for Dictionary {
    // Implemnet serialization without using #[derive(Serialize)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Dictionary", 2)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("tag", &self.tag)?;
        state.end()
    }
}

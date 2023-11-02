mod utils;

extern crate serde;
extern crate serde_json;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use wasm_bindgen::JsValue;


// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
#[derive(Default)]
pub struct Main {
    name: String,
    route: String,
}

#[wasm_bindgen]
#[derive(Default)]
pub struct Dictionary {
    name: String,
    tag: String,
}

#[wasm_bindgen]
impl Main {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_route(&self) -> String {
        self.route.clone()
    }

    pub fn set_route(&mut self, route: &str) {
        self.route = route.to_string();
    }

    #[wasm_bindgen(constructor)]
    pub fn new() -> Main {
        utils::set_panic_hook();
        let route = web_sys::window().unwrap().location().pathname().unwrap();
        log!("Current route: {}", route);
        Main { 
            name: "Brian Mboya".to_string(),
            route: route
        }
    }

    pub fn handle_route(&self) -> JsValue {
        // Handling routes: return the specific route using clicks
        let route: &str = &self.get_route();
        let dict_instance = Dictionary::new();
        log!("Current route: {}", route);
        match route {
            "/" => dict_instance.get_tags(),
            "/about" => dict_instance.get_about(),
            "/projects" => dict_instance.get_projects(),
            "/article" => dict_instance.get_article(),
            _ => "Page not found".to_string().into(),
        }
    }
}


#[wasm_bindgen]
impl Dictionary {
    // Constructor
    #[wasm_bindgen(constructor)]
    pub fn new() -> Dictionary {
        utils::set_panic_hook();
        Dictionary {
            name: String::new(),
            tag: String::new(),
        }
    }

    // TODO: Add database functionality
    pub fn get_article(&self) -> JsValue {
        let json_data = serde_json::to_string(&vec![Dictionary {
            name: "Article page not implemented yet! Stay tuned.".to_string(),
            tag: "None".to_string()
        }]).unwrap();
        JsValue::from_str(&json_data)
    }

    // TODO: Add database functionality
    pub fn get_projects(&self) -> JsValue {
        let json_data = serde_json::to_string(&vec![Dictionary {
            name: "Project page not implemented yet! Stay tuned.".to_string(),
            tag: "None".to_string()
        }]).unwrap();
        JsValue::from_str(&json_data)
    }

    // TODO: Add database functionality
    pub fn get_about(&self) -> JsValue {
        let json_data = serde_json::to_string(&vec![Dictionary {
            name: "About page not implemented yet! Stay tuned.".to_string(),
            tag: "None".to_string()
        }]).unwrap();
        JsValue::from_str(&json_data)
    }

    // Function to retrieve dictionary entriess from Database
    // TODO: Add database functionality
    pub fn get_tags(&self) -> JsValue {
        // For example
        // let entries = database::query("SELECT key, value FROM dictionaries");
        
        // Dummy data: TODO: remove
        let names: Vec<String> = vec![
            "Generating all subsets using basic Combinatorial Patterns",
            "Uninformed search in Artificial Intelligence",
            "Encoding logic in Artificial Intelligence",
            "Encoding logic in AI using Theorem Proving",
            "Could a meme make mind?",
            "Is it?",
            "The Real",
            "F**k red & blue. I want the green pill",
            "The Prison"
        ].into_iter().map(|i| i.to_string()).collect();
        let tags: Vec<String> = vec![
            "Programming,ArtificialIntelligence",
            "Programming,ArtificialIntelligence",
            "Programming,ArtificialIntelligence",
            "Programming,ArtificialIntelligence",
            "Poetry,Consciousness",
            "Poetry,Consciousness,BrainScience",
            "Poetry,Consciousness",
            "Poetry,Love",
            "Poetry,Consciousness,Escape",
        ].into_iter().map(|i| i.to_string()).collect();
        let dictionaries: Vec<Dictionary> = names
            .into_iter()
            .zip(tags)
            .map(|(name, tag)| Dictionary { name, tag })
            .collect();

        let json_data = serde_json::to_string(&dictionaries).unwrap();
        log!("{}", json_data);
        JsValue::from_str(&json_data)
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

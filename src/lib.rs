mod utils;

extern crate serde_json;
extern crate web_sys;

use std::io::{Error, ErrorKind};
use std::collections::BTreeMap;
use std::fmt;

use wasm_bindgen::prelude::*;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use wasm_bindgen::JsValue;
use lopdf::{Document};
use rayon::iter::{IntoParallelIterator, ParallelIterator};


// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
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

#[derive(Debug)]
pub struct PdfText {
    text: BTreeMap<u32, Vec<String>>,
    errors: Vec<String>
}

impl fmt::Display for PdfText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

fn get_pdf_text(pdf_data: &[u8]) -> Result<PdfText, Error> {
    let mut pdf_text: PdfText = PdfText {
        text: BTreeMap::new(),
        errors: Vec::new()
    };
    let mut doc = Document::load_mem(&pdf_data).map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
    let pages: Vec<Result<(u32, Vec<String>), Error>> = doc
        .get_pages()
        .into_par_iter()
        .map(
            |(page_num, page_id): (u32, (u32, u16))| -> Result<(u32, Vec<String>), Error> { 
                let text = doc.extract_text(&[page_num]).map_err(|e| {
                    Error::new(ErrorKind::Other, e.to_string())
                })?;
                Ok((page_num, text.split('\n').map(|s| s.trim_end().to_string()).collect::<Vec<String>>()))
            }
        )
        .collect();

    for page in pages {
        match page {
            Ok((page_num, line)) => {
                pdf_text.text.insert(page_num, line);
            },
            Err(e) => {
                pdf_text.errors.push(e.to_string());
            }
        }
    }
    Ok(pdf_text)
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
        let mut route = web_sys::window().unwrap().location().pathname().unwrap();
        if route.len() != 1 && route.as_bytes()[route.len() - 1] as char == '/' {
            route = route[0..route.len() - 1].to_string();
        }
        Main { 
            name: "Brian Mboya".to_string(),
            route: route
        }
    }

    pub fn handle_route(&self, _id: u8, pdf_data: &[u8]) -> JsValue {
        // Handling routes: return the specific route using clicks
        let mut route: &str = &self.get_route();
        if route.len() != 1 && route.as_bytes()[route.len() - 1] as char == '/' {
            route = &route[0..route.len() - 1];
        }
        let dict_instance = Dictionary::new();
        let mapped_route = format!("/articles/{_id}");
        log!("Current route: {}", route);
        log!("Mapped current route: {}", mapped_route);
        match route {
            "/" => dict_instance.get_tags(),
            "/about" => dict_instance.get_about(),
            "/projects" => dict_instance.get_projects(),
            "/view_cv" => self.get_pdf_data(pdf_data),
            mapped_route if mapped_route == route => String::new().into(),
            _ => "Page not found".to_string().into(),
        }
    }

    pub fn get_pdf_data(&self, pdf_data: &[u8]) -> JsValue {
        // let text = get_pdf_text(pdf_data);
        // log!("Text here: {text:?}");
        let json_data = serde_json::to_string(&vec![Dictionary {
            name: "Download page not implemented yet! Stay tuned.".to_string(),
            tag: "None".to_string()
        }]).unwrap();   
        JsValue::from_str(&json_data)
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
            "PE",
            "Is it?",
            "The Real",
            "Could a meme make mind?",
            "F**k red & blue. I want the green pill",
            "The Prison",
            "The Hidden World",
            "Love and The Universe",
            "The City",
            "The Universe Within",
        ].into_iter().map(|i| i.to_string()).collect();
        let tags: Vec<String> = vec![
            "1,Programming,AI",
            "2,Programming,AI",
            "3,Programming,AI",
            "4,Programming,AI",
            "0,PE",
            "5,Poetry,Consciousness,Mind,AI",
            "6,Poetry,Consciousness,Imagination",
            "7,Consciousness,Poetry,AI,Memes,Richard Dawkins",
            "8,Poetry,Love,Consciousness",
            "9,Poetry,Consciousness,Escape,Mind",
            "10,Poetry,Imagination",
            "11,Love,Poetry,Imagination",
            "12,AI,Dreams,Mind,Imagination",
            "13,Imagination,Poetry",
        ].into_iter().map(|i| i.to_string()).collect();
        let dictionaries: Vec<Dictionary> = names
            .into_iter()
            .zip(tags)
            .map(|(name, tag)| Dictionary { name, tag })
            .collect();

        let json_data = serde_json::to_string(&dictionaries).unwrap();
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

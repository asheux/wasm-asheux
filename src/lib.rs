mod utils;
mod dictionary;
mod crawler;

extern crate web_sys;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
#[derive(Default)]
pub struct Main {
    name: String,
    route: String,
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
        let _ = crawler::Crawler::new();
        if route.len() != 1 && route.as_bytes()[route.len() - 1] as char == '/' {
            route = route[0..route.len() - 1].to_string();
        }
        Main { 
            name: "Brian Mboya".to_string(),
            route: route
        }
    }

    pub fn handle_route(&self, _id: u8) -> JsValue {
        // Handling routes: return the specific route using clicks
        let mut route: &str = &self.get_route();
        if route.len() != 1 && route.as_bytes()[route.len() - 1] as char == '/' {
            route = &route[0..route.len() - 1];
        }
        let dict_instance = dictionary::Dictionary::new();

        match route {
            "/" => dict_instance.get_tags(),
            "/about" => "About page".to_string().into(),
            "/projects" => "Project Crawl".to_string().into(),
            "/view_cv" => "PDF Data".to_string().into(),
            mapped_route if mapped_route == route => String::new().into(),
            _ => "Page not found".to_string().into(),
        }
    }
}

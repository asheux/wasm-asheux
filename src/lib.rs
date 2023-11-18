mod utils;

extern crate serde_json;
extern crate web_sys;
extern crate urlparse;

use std::collections::{VecDeque};

use wasm_bindgen::prelude::*;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use wasm_bindgen::JsValue;
use urlparse::urlparse;
use web_sys::{RequestInit, RequestMode, Request, window, Response};
// use regex::bytes::Regex;


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

#[wasm_bindgen]
#[derive(Debug)]
pub struct Crawler {
    roots: Vec<String>,
    q: VecDeque<String>,
    root_domains: Vec<String>,
}

#[wasm_bindgen]
impl Crawler {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Crawler {
        utils::set_panic_hook();
        let roots = vec![];
        let q = VecDeque::new();
        Crawler {
            roots: roots.clone(),
            q: q,
            root_domains: roots.clone()
        }
    }

    pub fn init_roots(&mut self) {
        // parse urls 
        let mut root_domains: Vec<String> = vec![];
        // let re = Regex::new(r"\A[\d\.]*\Z").unwrap(); // Match IP address: 192.168.0.1
        for root in &self.roots {
            // fix url
            let _root: String;
            if !root.contains("://") {
                _root = format!("https://{}", &root);
            } else {
                _root = root.to_owned();
            } 
            // Url parse
            let url = urlparse(_root.clone());
            let netloc_list: Vec<_> = url.netloc.split(":").collect();
            let netloc_list_len = netloc_list.len();
            let mut host: String = "".to_string();

            if netloc_list_len > 1 {
                host = netloc_list[0].to_string();
            } else if netloc_list_len == 1 {
                host = netloc_list[0].to_string();
            }
            if host == "" {
                continue
            }
            if !host.starts_with("https://") {
                host = "https://".to_string() + &host;
            }
            root_domains.push(host);
        }
        self.root_domains = root_domains;
    }

    pub fn add_url_to_queue(&mut self) {
        for rdomain in &self.root_domains {
            self.q.push_back(rdomain.clone());
        }
    }

    pub fn crawl(&mut self) {
        self.add_url_to_queue(); // Add to queue

        while let Some(url) = self.q.pop_front() {
            self.fetch(url);
        }
        // for _ in 0..10 { // Workers
        //     // process queue items forever
        //     thread::spawn(move || {
        //         while let Some(url) = self.q.pop_front() {
        //             self.fetch(url);
        //         }
        //     });
        // }
    }

    pub fn fetch(&mut self, url: String) {
        let mut init = RequestInit::new();
        init.method("GET").mode(RequestMode::Cors);

        let request = Request::new_with_str_and_init(&url, &init).unwrap();
        let window = window().unwrap();

        let mut tries = 0;
        let mut promise_response = None;

        while tries < 4 {
            promise_response = Some(window.fetch_with_request(&request));

            if tries > 1 {
                log!("try {tries:?} for {url:?} success");
            }
            tries += 1;
        }
        let fetch_future = wasm_bindgen_futures::JsFuture::from(promise_response.unwrap());

        wasm_bindgen_futures::spawn_local(async move {
            match fetch_future.await {
                Ok(response_value) => {
                    let response: Response = response_value.dyn_into().unwrap();
                    if response.ok() {
                        let body_promise = response.text();
                        log!("RESPONSE: {body_promise:?}");
                    }
                }
                Err(err) => {
                    web_sys::console::error_1(&format!("Error making request: {:?}", err).into());
                }
            }
        });
    }

    pub fn set_roots(&mut self, roots_str: &str) {
        // Set the user url inputs to the crawler
        let roots = roots_str.split(",").map(|s| s.into());
        self.roots = roots.collect::<Vec<_>>();
    }
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

    pub fn handle_route(&self, _id: u8) -> JsValue {
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
            "/view_cv" => "PDF Data".to_string().into(),
            mapped_route if mapped_route == route => String::new().into(),
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
            "Lexicographic permutation generation",
            "PE",
            "Is it?",
            "0x1: A Godless world or not?",
            "What am I?",
            "0x2: Unknown",
            "0x2: Not Not Single",
            "The Real",
            "Could a meme make mind?",
            "F**k red & blue. I want the green pill",
            "The Prison",
            "The Hidden World",
            "Love and The Universe",
            "The City",
            "The Universe Within",
            "Save Yourselves",
            "The RTC",
            "Unexplored light",
        ].into_iter().map(|i| i.to_string()).collect();
        let tags: Vec<String> = vec![
            "1,Programming,AI",
            "2,Programming,AI",
            "3,Programming,AI",
            "4,Programming,AI",
            "14,Combinatorics,Algorithms,Python,Permutations,Lexicographic",
            "0,PE",
            "5,Poetry,Consciousness,Mind,AI",
            "21,God,Worlds,FreeWill,Essay,Consciousness",
            "18,Consciousness,Poetry,Unconsciousness",
            "19,Consciousness,Essay,God,FreeWill,Simulation",
            "20,Consciousness,Essay,God,FreeWill,Simulation",
            "6,Poetry,Consciousness,Imagination",
            "7,Consciousness,Poetry,AI,Memes,Richard Dawkins",
            "8,Poetry,Love,Consciousness",
            "9,Poetry,Consciousness,Escape,Mind",
            "10,Poetry,Consciousness,Imagination",
            "11,Love,Poetry,Consciousness,Imagination",
            "12,AI,Dreams,Mind,Consciousness,Imagination",
            "13,Imagination,Consciousness,Poetry",
            "15,Poetry,Consciousness,Imagination",
            "16,Poetry,Imagination,Consciousness,Unknown",
            "17,Poetry,Otherworlds,Minds,Consciousness,TheoryOfMind",
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

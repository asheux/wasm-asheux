#[path = "utils.rs"]
mod utils;

extern crate serde_wasm_bindgen;
extern crate urlparse;

use std::collections::{VecDeque, HashSet, HashMap};

use wasm_bindgen::prelude::*;
use urlparse::urlparse;
use urlparse::urlunparse;
use regex::Regex;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Crawler {
    roots: Vec<String>,
    q: VecDeque<String>,
    root_domains: HashSet<String>,
    seen_urls: HashSet<String>,
    errors: HashSet<String>,
}

#[wasm_bindgen]
impl Crawler {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Crawler {
        utils::set_panic_hook();
        let roots = vec![];
        let default = HashSet::new();
        let queue = VecDeque::new();
        Crawler {
            roots: roots,
            q: queue,
            root_domains: default.clone(),
            seen_urls: default.clone(),
            errors: default.clone()
        }
    }

    pub fn init_roots(&mut self) {
        // parse urls 
        let mut root_domains: HashSet<String> = HashSet::new();
        let re = Regex::new(r"^[A-Za-z0-9]([A-Za-z0-9-]{0,61}[A-Za-z0-9])?(\.[A-Za-z]{2,})+$").unwrap();
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
            // ('host:port') --> ['host', 'port']. We assume host and port are valid
            let netloc_list: Vec<_> = url.netloc.split(":").collect();
            let netloc_list_len = netloc_list.len();
            let mut host: String = "".to_string();

            if netloc_list_len > 1 {
                host = netloc_list[0].to_string();
            } else if netloc_list_len == 1 {
                host = netloc_list[0].to_string();
            }

            if host.is_empty() {
                continue
            }
            let is_match = re.is_match(&host);
            if is_match {
                if !host.starts_with("https://") {
                    host = "https://".to_string() + &host;
                }
                root_domains.insert(host.to_lowercase());
            }
            
        }
        self.root_domains = root_domains;
    }

    pub fn add_url_to_queue(&mut self) {
        for rdomain in &self.root_domains {
            self.q.push_back(rdomain.clone());
        }
    }

    pub async fn crawl(&mut self, limit: u8) -> JsValue {
        self.add_url_to_queue(); // Add to queue

        while let Some(url) = self.q.pop_front() {
            if !self.seen_urls.contains(&url) {
                self.fetch(url).await;
            }
            if self.seen_urls.len() >= limit as usize {
                break;
            }
        }
        let hash_set_roots: HashSet<String> = self.roots.clone().into_iter().collect();
        let hash_set_queue: HashSet<String> = self.q.clone().into_iter().collect();
        let union: HashSet<String> = hash_set_queue.difference(&self.seen_urls).cloned().collect();
        let queued: HashSet<String> = hash_set_queue.difference(&hash_set_roots).cloned().collect();
        let data = HashMap::from([
            ("roots", hash_set_roots),
            ("root_domains", self.root_domains.clone()),
            ("queue", queued),
            ("seen", self.seen_urls.clone()),
            ("result", union),
            ("errors", self.errors.clone())
        ]);
        serde_wasm_bindgen::to_value::<HashMap<&str, HashSet<String>>>(&data).unwrap()
    }

    pub async fn fetch(&mut self, url: String) {
        let proxified = format!("https://api.asheux.com/crawl?url={}", url);
        let mut tries = 0;
        let mut response = None;

        while tries < 4 {
            response = Some(reqwest::get(proxified.clone()).await);

            if response.is_some() {
                log!("try {tries:?} for {url:?} success");
                break;
            }
            tries += 1;
        }
        match response.unwrap() {
            Ok(res) => {
                if res.status() == 200 {
                    let body = res.text().await;
                    let links = self.parse_links(&url, &body.unwrap());
                    let urls = serde_wasm_bindgen::from_value::<Vec<String>>(links);
                    match urls {
                        Ok(r) => {
                            for u in r {
                                self.q.push_back(u);
                            }
                            self.seen_urls.insert(url);
                            self.errors.clear();
                        }
                        Err(_) => {
                            self.errors.insert(
                                "Error making request. Check if domain is valid/network and try again!".to_string()
                            );
                        }
                    }
                }
            }
            Err(_) => {
                self.errors.insert(
                    "Error making request. Check if domain is valid/network and try again!".to_string()
                );
            }
        };
    }

    pub fn parse_links(&mut self, url: &str, result: &str) -> JsValue {
        let re = Regex::new(r#"(?i)href=[\"']([^\s\"'<>]+)"#).unwrap();
        let matches: Vec<_> = re.captures_iter(&result).collect();
        let links: Vec<String> = matches
            .into_iter()
            .filter_map(|caps| caps.get(1).map(|m| m.as_str().to_string()))
            .collect();

        let mut found_links: HashSet<String> = HashSet::new();
        for link in links {
            if link.contains("css") || link.contains("ico") {
                continue
            }

            let new_link = self.urljoin(url, &link);
            found_links.insert(new_link);
        }
        serde_wasm_bindgen::to_value(&found_links).unwrap()
    }

    pub fn urljoin(&self, base: &str, url: &str) -> String {
        let uses_relative: Vec<String> = vec![
            "", "ftp", "http", "gopher", "nntp", "imap",
            "wais", "file", "https", "shttp", "mms",
            "prospero", "rtsp", "rtsps", "rtspu", "sftp",
            "svn", "svn+ssh", "ws", "wss"
        ].into_iter().map(|s| s.to_string()).collect();
        let uses_netloc: Vec<String> = vec![
            "", "ftp", "http", "gopher", "nntp", "telnet",
            "imap", "wais", "file", "mms", "https", "shttp",
            "snews", "prospero", "rtsp", "rtsps", "rtspu", "rsync",
            "svn", "svn+ssh", "sftp", "nfs", "git", "git+ssh",
            "ws", "wss", "itms-services"
        ].into_iter().map(|s| s.to_string()).collect();

        if base.is_empty() {
            return url.to_string();
        }
        if url.is_empty() {
            return base.to_string();
        }
        let bparsed = urlparse(base);
        let uparsed = urlparse(url); 
        if !uses_relative.contains(&uparsed.scheme) {
            return url.to_string();
        }
        let mut netloc = uparsed.netloc.clone();
        let mut path = uparsed.path.clone();

        if uses_netloc.contains(&bparsed.scheme) {
            if !netloc.is_empty() {
                let mut _link = bparsed.scheme.clone() + "://" + &netloc + &path;
                return urlunparse(urlparse(_link)); // Returns url string
            }
            netloc = bparsed.netloc.clone(); 
        } 
        if path.is_empty() {
            path = bparsed.path.clone();
            let newlink = bparsed.scheme.clone() + "://" + &netloc + &path;
            return urlunparse(urlparse(newlink));
        } 
        let nlink = bparsed.scheme.clone() + "://" + &netloc + &path;
        return nlink.to_string();
    }

    pub fn reset(&mut self) {
        // Reset things
        self.q.clear();
        self.seen_urls.clear();
        self.roots.clear();
        self.root_domains.clear();
    }

    pub fn set_roots(&mut self, mut roots_str: &str) {
        // Set the user url inputs to the crawler
        if !roots_str.is_empty() {
            // clear queue
            self.reset();
            if roots_str == "reset".to_string() {
                roots_str = "";
            }
        }
        let roots = roots_str.split(",").map(|s| s.into());
        self.roots = roots.collect::<Vec<_>>();
    }
}

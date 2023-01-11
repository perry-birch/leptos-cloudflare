use core::fmt::{ Debug, Formatter, Result };

use worker::*;

pub struct FileDetails {
    // Full key to lookup file from KV Store
    pub key: String,
    // File path up to extension
    // * Not including Wasm-Pack hash
    pub root: String,
    // File Extension
    pub ext: Option<String>,
    // Cloudflare KV Store Hash
    pub kv_hash: Option<String>,
    // Wasm-Pack Appended Hash
    pub wp_hash: Option<String>,
}

impl FileDetails {
    pub fn new(key: String, root: String, ext: Option<String>, kv_hash: Option<String>, wp_hash: Option<String>) -> Self {
        FileDetails { key, root, ext, kv_hash, wp_hash }
    }

    pub fn file_path(&self) -> String {
        match &self.ext {
            None => self.root.to_owned(),
            Some(ext) => format!("{}.{}", self.root, ext),
        }
    }

    pub fn content_type(&self) -> &str {
        match &self.ext {
            None => get_content_type(""),
            Some(ext) => get_content_type(ext),
        }
    }
}

impl From<&worker::kv::Key> for FileDetails {
    fn from(value: &worker::kv::Key) -> Self {
        extract_file_details(value)
    }
}

impl Debug for FileDetails {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("FileDetails")
            .field("key", &self.key)
            .field("path", &self.file_path())
            .finish()
    }
}

fn extract_file_details(key: &worker::kv::Key) -> FileDetails {
    // Extract the file name and ext
    let (file_name_hashed, ext) = match key.name.rsplit_once('.') {
        None => {
            console_warn!("Unable to extract ext or kv hash from file [{}]", key.name);
            return FileDetails::new(key.name.to_owned(), key.name.to_owned(), None, None, None);
        },
        Some(v) => v,
    };
    let (kv_file_name, kv_hash) = match file_name_hashed.rsplit_once('.') {
        None => {
            console_warn!("Unable to extract kv hash from file [{}]", key.name);
            return FileDetails::new(key.name.to_owned(), file_name_hashed.to_owned(), Some(ext.to_owned()), None, None);
        }
        Some(v) => v,
    };
    let (file_name, wp_hash) = match kv_file_name.rsplit_once('-') {
        None => {
            return FileDetails::new(key.name.to_owned(), kv_file_name.to_owned(), Some(ext.to_owned()), Some(kv_hash.to_owned()), None);
        }
        Some(v) => v,
    };
    FileDetails::new(key.name.to_owned(), file_name.to_owned(), Some(ext.to_owned()), Some(kv_hash.to_owned()), Some(wp_hash.to_owned()))
}

fn get_content_type(ext: &str) -> &'static str {
    match ext {
        "html" => "text/html",
        "css" => "text/css",
        "js" => "text/javascript",
        "json" => "application/json",
        "png" => "image/png",
        "jpg" => "image/jpeg",
        "jpeg" => "image/jpeg",
        "ico" => "image/x-icon",
        "wasm" => "application/wasm",
        _ => "text/plain",
    }
}
use serde::{Serialize, Deserialize};
use crate::users;
use std::env;
use std::fs;

static DOCX_DIR : &str = "/data/docx/"; 

/* Generate path to the directory of all OWNERS' docx */
pub fn generate_all_docx_path(owner : &String) -> String {
    let cwd = env::current_dir().unwrap();

    let mut docx_path : String = cwd.display().to_string() + DOCX_DIR;
    docx_path.push_str(owner);
    return docx_path;
}

/* Generate the path to a document owned by OWNER */
pub fn generate_docx_path(owner : &String, file_name : &String) -> String {
    let mut docx_path = generate_all_docx_path(owner);
    docx_path += "/";
    docx_path.push_str(file_name);
    println!("docx path: {}", docx_path);
    return docx_path;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mailbox {
    pub owner : String,
    pub access : Vec<users::User>,
    pub file_name : String,
    /* This is where we would handle security for the file names
     * Should be a layer of abstraction so that the user does not
     * ever know about the actual path to the file, but everyone
     * hass knowledge of the mailbox. Usually security would come 
     * in here, and if you have a key that can be used to open this
     * mailbox, then access of the file would be granted, otherwise not.
     * Instead for the sake of not wanting to implement an entire hashing scheme
     * w/ my bootleg database, we will make the request take in more information
     */
}


impl Mailbox {
    pub fn new(owner : &String, file_name : &String) -> Self {
        let docx : Doc = Doc::new();
        let mb : Self = Self {
            owner : owner.to_owned(),
            access : Vec::new(),
            file_name : (*file_name.clone()).to_string(),
        };

        save_docx(&mb, &docx);
        return mb;
    }
}

// Document
#[derive(Serialize, Deserialize, Debug)]
pub struct Doc {
    // Add a lock for when we have multiple writers 
    pub contents : String,    // Contents of the document
}

impl Doc {
    pub fn new() -> Self {
        return Self {
            contents : "".to_string(),
        }
    }
}

pub fn save_docx(mail_box : &Mailbox, docx: &Doc) {
    let serialized_docx = serde_json::to_string(&docx).unwrap();
    fs::write(generate_docx_path(&mail_box.owner, &mail_box.file_name), serialized_docx).unwrap();
}

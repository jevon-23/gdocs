use std::collections::HashSet;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub username : String,
    pub id : u8,
    pub logged_in : bool,
    pub files : HashSet<String>,
    // Cookie? 
}

impl User  {
    pub fn new(username : String, id : u8) -> Self {
        return Self {
            username : username,
            id : id,
            logged_in : false,
            files : HashSet::new(),
        };
    }

    #[allow(dead_code)]
    pub fn get_vars(user : &Self) {
        println!("username: {}", user.username);
        println!("id: {}", user.id);
        println!("logged in: {}", user.logged_in);
    }

}

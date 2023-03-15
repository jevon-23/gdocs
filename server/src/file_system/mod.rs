use crate::users;
use crate::Input;
pub mod docx;
use std::env;
use std::fs;
use crate::utils;

static USERS_PATH : &str = "/data/users/users.json";
static MAILBOX_PATH : &str = "/data/mailboxes/";

/*******************************/
/* Path generators for queries */
/*******************************/


/* Generate the path to all the users */
fn generate_user_path() -> String {
    let check_path = env::current_dir();
    /* Could use .unwrap() but wanted to understand Result<> */
    let path = match check_path {
        Ok(path) => path,
        Err(e) => panic!("Could not find the users file {}", e),
    };

    // cwd/data/users.json
    let users_path : String =
        path.display().to_string() + USERS_PATH;
    return users_path;
}

/* Generate the path for all of this users' mailboxes */
pub fn generate_all_mailboxes_path(owner : &String) -> String {
    let cwd = env::current_dir().unwrap();
    let mut mb_path : String =
        cwd.display().to_string() + MAILBOX_PATH;
    mb_path.push_str(owner);
    return mb_path;
}

/* Generate the path to a mailbox */
pub fn generate_mailbox_path(owner : &String,
                             file_name : &String) -> String {
    // cwd/data/mailboxes/user/file_name
    let mut mb_path : String =
        generate_all_mailboxes_path(owner);
    mb_path.push_str("/");
    mb_path.push_str(file_name);
    return mb_path;
}
/*****************/
/* User data i/o */
/*****************/

/* Get all users out of database */
pub fn get_users() -> Vec<users::User> {
    let user_path : String = generate_user_path();

    /* Read in the file user path */
    let serialized = fs::read_to_string(user_path);
    let serialized_users : String = match serialized {
        Ok(ser) => ser,
        _ => return Vec::new(),
    };

    /* Deserialize the users list */
    let users : Vec<users::User> = serde_json::from_str(
        &serialized_users).unwrap();

    return users;
}

/* Get a specific user */
pub fn get_user(users : &Vec<users::User>, username : &String) 
    -> Option<users::User> {
        /* Get the users with this username */
        for u in users {
            if u.username == *username {
                return Some(u.clone());
            }
        }
        return None;
    }

/* Basically, swap this user with its new instance */
pub fn update_user(all_users : &mut Vec<users::User>, user : &users::User) {

    /* See if user is stored in database */
    let mut user_index : i8 = -1;
    for i in 0..all_users.len() as i8 {
        if all_users[i as usize].username == user.username {
            user_index = i;
            break;
        }
    }
    /* See if we could update this uesr  */
    if user_index == -1 {
        println!("Could not update the user {}\n", user.username);
        return;
    }

    /* Remove the old instance of the user */
    all_users.remove(user_index as usize);

    /* Upadate database with new instance */
    all_users.insert(user.id as usize, (*user).clone());
    save_users(all_users);
}

/* Save users to the databse */
pub fn save_users(users: &Vec<users::User>) {
    let serialized_users = serde_json::to_string(&users).unwrap();
    fs::write(generate_user_path(), serialized_users).unwrap();
}

/***************/
/* Mailbox i/o */
/***************/

/* Save the mailbox to a file */
pub fn save_mailbox(mb : &docx::Mailbox) {
    /* Save the mailbox */
    let serialized_mb = serde_json::to_string(&mb).unwrap();
        fs::write(
            generate_mailbox_path(&mb.owner, &mb.file_name),
            serialized_mb)
        .unwrap();
}

/* Get the mailbox for FILENAME owned by OWNER, on behalf of USER */
fn get_mailbox(user : &users::User, owner : &str, file_name : &str) 
    -> Option<docx::Mailbox> {
        println!("user {} trying to access {}/{}",
                 user.username, owner, file_name);

        /* Get the path to the mailbox for this file */
        let mb_path : String = generate_mailbox_path(
            &owner.to_string(), 
            &file_name.to_string());

        /* Read in the file mbpath  & deserialize */
        let serialized_mb = fs::read_to_string(mb_path).unwrap();
        let mb : docx::Mailbox =
            serde_json::from_str(&serialized_mb).unwrap();

        /* Return mb if we have access to file, otherwise None */
        if mb.owner == user.username {
            return Some(mb);
        }

        if !docx::Mailbox::has_access(&mb, &user.username) {
            println!("User: {} does not have access 2 file: {}",
                     user.username, file_name);
            return None;
        }

        return Some(mb);
    }


/* Get the file FILENAME owned by OWNER, on behalf of USER */
fn get_file(user : &users::User, owner : &str, file_name : &str)
    -> Option<docx::Doc> {

        /* Get the mailbox for this file */
        let mb : docx::Mailbox =
            match get_mailbox(user, owner, file_name) {
                Some(mb) => mb,
                None => {
                    println!("Could not access mailbox: {}/{}",
                        owner, file_name);
                    return None;
                }
            };

        /* Generate the path to the document */
        let doc_path : String = docx::generate_docx_path(
            &mb.owner.to_string(),
            &mb.file_name.to_string());

        /* Read in the file mbpath  & deserialize */
        let serialized_doc = fs::read_to_string(doc_path).unwrap();
        let doc : docx::Doc= serde_json::from_str(
            &serialized_doc).unwrap();
        return Some(doc);
    }

/*********************/
/* File Manipulation */
/*********************/

/* Save the mailbox to a file */
pub fn save_docx(docx_path : String, docx : &docx::Doc) {
    /* Save the mailbox */

    /* TODO: Will need to handle synchroniztion here */
    let serialized_docx = serde_json::to_string(&docx).unwrap();
    fs::write(docx_path, serialized_docx).unwrap();
}

/* Generate a new docx file */
pub fn new_file(inp :Input) {
    /* TODO: First, we need to check to see if the file 
     * exists already, but for rn and for the sake of time 
     * we will take the overwriteon rename */

    /* Save the new file name to the users' namespace */
    let mut all_users : Vec<users::User> = get_users();
    let mut user : users::User =
        match get_user(&all_users, &inp.user) {
            Some(u) => u,
            None => { 
                let response : String =
                    format!("Could not find user: {}", inp.user);
                utils::send_error_response(inp.output_stream,
                                           response);
                return;
            }
        };

    /* Save the mailbox to the database */
    let mut mb : docx::Mailbox =
        docx::Mailbox::new(&inp.user, &inp.params[0]);
    mb.access.insert((user.username).clone());
    save_mailbox(&mb);

    /* Update the user to have the new file in their access list */
    user.files.insert(format!("{}/{}",
                      &user.username, mb.file_name));
    update_user(&mut all_users, &user);

    /* Send the response back to client */
    let response : String =
        format!("created a new file for {} named {}",
                inp.user, inp.params[0]);
    utils::send_response(inp.output_stream, response);
}

/* Return the document that is trying to be accessed */
pub fn read_db_file(usr : &str, own : &str, file : &str)
    -> Option<docx::Doc> {
    let username : String = usr.to_string();
    let owner : String = own.to_string();
    let file_name : String = file.to_string();

    /* Get the mailbox for this file */
    let all_users : Vec<users::User> = get_users();
    let user : users::User =
        match get_user(&all_users, &username) {
            Some(u) => u,
            None => {
                println!("Could not find user: {}", username);
                return None;
            }
        };

    /* Check to see if the file is in users files || 
     * the files that we have access to */
    let has_access : Vec<String>= user.clone().files.into_iter()
        .filter(|f| *f == format!("{}/{}", &owner, &file_name))
        .collect();
    if has_access.len() != 1 {
        println!("User does not have access to file");
        return None;
    }

    return get_file(&user, &owner, &file_name);
}

/* Read route; read the file and return the contents back to the 
 * user */
pub fn read_file(inp : Input) {
    let owner : String = inp.params[0].clone();
    let file_name : String = inp.params[1].clone();
    let docx : docx::Doc = match read_db_file(
        &inp.user, &owner, &file_name) {
            Some(doc) => doc,
            None => {
                let response : String =
                    format!("Could not access: {}",file_name);
                utils::send_error_response(
                    inp.output_stream, response);
                return;
        }
    };
    
    let mut response : String =
        format!("{} read {} owned by {}",
                inp.user, file_name, owner);
    response.push_str("\n");
    response.push_str(&docx.contents);
    utils::send_response(inp.output_stream, response);
}

/* Update route => update the contents of the file */
pub fn update_file(inp : Input) {
    /* Get the mailbox for this file */
    let all_users : Vec<users::User> = get_users();
    let user : users::User =
        match get_user(&all_users, &inp.user) {
            Some(u) => u,
            None => {
                let response : String =
                    format!("Could not find user: {}", inp.user);
                utils::send_error_response(
                    inp.output_stream, response);
                return;
            },
        };

    let owner : String = inp.params[0].clone();
    let file_name : String = inp.params[1].clone();

    let mut docx : docx::Doc = match read_db_file(
        &inp.user,
        &owner, &file_name) {
            Some(doc) => doc,
            None => {
                let response : String = 
                    format!("Could nto get documetnt ");
                utils::send_error_response(
                    inp.output_stream, response);
                return;
            }
        };

    /* Update the contents of the file =>
     * For now, simply append on to the end of the file
     */
    docx.contents += inp.body.as_str();
    save_docx(
        docx::generate_docx_path(
            &owner,
            &file_name),
            &docx);
    /* Send the response back to client */
    let response : String =
        format!("{} updated {} owned by {}",
                user.username, file_name, owner);
    utils::send_response(inp.output_stream, response);
}

/* Invite desired user to manipulate file */
pub fn invite(inp : Input) {
    /* Get the mailbox for this file */
    let mut all_users : Vec<users::User> = get_users();
    let owner : users::User =
        match get_user(&all_users, &inp.user) {
            Some(u) => u,
            None => {
                let response =
                    format!("Could not find user: {}", inp.user);
                utils::send_error_response(
                    inp.output_stream, response);
                return;
            }
        };

    let invitee : String = inp.params[0].clone(); // user being inv
    let file_name : String = inp.params[1].clone();

    /* Get the mailbox for this file, and append new user to 
     * list of people with access */
    let mut mb : docx::Mailbox =  match 
        get_mailbox(&owner, &owner.username, &file_name) {
            Some(mb) => mb,
            None => {
                let response : String =
                    format!("Could not access mailbox: {}/{}",
                            owner.username, file_name);
                utils::send_error_response(
                    inp.output_stream, response);
                return;
            }
        };

    if owner.username != mb.owner {
        let response : String =
            format!("{} does not own the file: {}",
                    owner.username, file_name);
        utils::send_error_response(inp.output_stream, response);
        return;
    }

    mb.access.insert(invitee.clone());

    /* Save the mailbox to db */
    save_mailbox(&mb);

    /* Update the new users' access to contain this file */
    let mut user2 : users::User =
        match get_user(&all_users, &invitee) {
            Some(u) => u,
            None => {
                let response : String =
                    format!("Could not find user: {}", invitee);
                utils::send_error_response(
                    inp.output_stream, response);
                return;
            },
        };
    user2.files.insert(format!("{}/{}", &owner.username, file_name));

    /* Save new user back to db */
    update_user(&mut all_users, &user2);

    /* Send response back */
    let body : String = format!("{} invited {} to edit {}", 
                                owner.username, user2.username,
                                file_name);
    utils::send_response(inp.output_stream, body);
}


use crate::Input;
use crate::users;
use crate::file_system;
use std::fs;
use crate::utils;

pub fn handle_login(inp : Input) {
    println!("Attempting to log user in\n");

    /* Get the username out of the path */
    let username : String = inp.user;

    let mut all_users : Vec<users::User> = file_system::get_users();

    /* See if user is stored in database */
    let mut user_index : i8 = -1;
    for i in 0..all_users.len() as i8 {
        if all_users[i as usize].username == username {
            user_index = i;
            break;
        }
    }

    /* User that is being logged in */
    let mut user : users::User;

    if user_index == -1 {
        /* First time user */
        let mut id : u8 = 0;
        if all_users.len() != 0 {
            id = all_users[all_users.len()-1].id + 1;
        }
        user = users::User::new(username, id);

        /* Create their namespace for mailboxes & docx */
        let mb_path : String = file_system::
            generate_all_mailboxes_path(&user.username);
        let docx_path : String = file_system::docx::
            generate_all_docx_path(&user.username);

        println!("Inside of 1st user, mb: {}, docs: {}", mb_path, docx_path);
        fs::create_dir_all(mb_path).unwrap();
        fs::create_dir_all(docx_path).unwrap();

    } else {
        user = all_users.remove(user_index as usize);
    }

    /* Log user in */
    user.logged_in = true;

    /* Write response back to the user */
    let response  : String = format!("user: {} logged in", user.username);

    /* Upadate database */
    all_users.insert(user.id as usize, user);
    file_system::save_users(&all_users);

    /* Send response */
    utils::send_response(inp.output_stream, response);
}

pub fn handle_logout(inp : Input) {
    println!("Attempting to log user out\n");
    /* Get the username out of the path */
    let username : String = inp.user;

    let mut all_users : Vec<users::User> = file_system::get_users();

    /* See if user is stored in database */
    let mut user_index : i8 = -1;
    for i in 0..all_users.len() as i8 {
        if all_users[i as usize].username == username {
            user_index = i;
            break;
        }
    }

    /* User that is being logged in */
    if user_index == -1 {
        println!("User does not exist\n");
        return;
    }
    let mut user: users::User = all_users.remove(user_index as usize);
    user.logged_in = false;

    /* Create response */
    let response : String = format!("user: {} logged out", user.username);

    /* Save the users */
    all_users.insert(user.id as usize, user);
    file_system::save_users(&all_users);

    /* Send response back to client */
    utils::send_response(inp.output_stream, response);
}

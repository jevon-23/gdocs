extern crate reqwest; // 0.9.18
use std::io::Read;
use std::process::Command;

static LOGIN: &str = "http://localhost:8477/login/user";
static LOGOUT: &str = "http://localhost:8477/logout/user";
static NEW_FILE: &str = "http://localhost:8477/new/user/test";
static UPDATE_FILE: &str = "http://localhost:8477/update/user/user/test";
static READ_FILE: &str = "http://localhost:8477/read/user/user/test";
static INVITE: &str = "http://localhost:8477/invite/user/user2/test";
static REVOKE: &str = "http://localhost:8477/revoke/user/user2/test";

/* Send a post request with a body */
fn make_post_raw(req : &str, contents : &str) -> String {
    let client = reqwest::Client::new();
    let mut res = client.post(req)
        .body(contents.to_string())
        .send()
        .unwrap() ;
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);
    return body;
}

fn make_request_raw(req : &str) -> String {
    let mut res = reqwest::get(req).unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);
    return body;
}

#[allow(dead_code)]
fn make_request(url : &str, req : &str, params : &str) {
    let mut u : String = url.to_string();
    u.push_str(req);
    u.push_str(params);
    make_request_raw(&u);
}

/* Request format: url/action/user/... */
fn main()  {
    /* TODO: implement client side action, but testing 4 now */
    println!("Make a request to the server");
}

/**********************************************************/
/* Testing Below                                          */
/* NOTE: Tests work individually as of rn, need to update */
/**********************************************************/
#[test]
fn test_login_logout() {
    /* Login */
    let mut body : String = make_request_raw(LOGIN);
    assert_eq!(body,"user: user logged in");
    Command::new("sleep").arg("2").spawn().unwrap();
    /* Logout */
    body = make_request_raw(LOGOUT);
    Command::new("sleep").arg("2").spawn().unwrap();
    assert_eq!(body,"user: user logged out");
}

#[test]
fn test_new_file() {
    /* Login */
    let mut body : String = make_request_raw(LOGIN);
    Command::new("sleep").arg("2").spawn().unwrap();
    assert_eq!(body,"user: user logged in");
    /* Create new file */
    body = make_request_raw(NEW_FILE);
    Command::new("sleep").arg("2").spawn().unwrap();
    assert_eq!(body,"created a new file for user named test");
    /* Logout */
    body = make_request_raw(LOGOUT);
    Command::new("sleep").arg("2").spawn().unwrap();
    assert_eq!(body,"user: user logged out");
}

#[test]
fn test_update_file() {
    /* Login */
    let mut body : String = make_request_raw(LOGIN);
    Command::new("sleep").arg("2").spawn().unwrap();
    assert_eq!(body,"user: user logged in");
    /* Create new empty file */
    body = make_request_raw(NEW_FILE);
    Command::new("sleep").arg("2").spawn().unwrap();
    assert_eq!(body,"created a new file for user named test");
    /* Update file */
    body = make_post_raw(UPDATE_FILE, "hello world");
    Command::new("sleep").arg("2").spawn().unwrap();
    assert_eq!(body,"user updated test owned by user");
    /* Logout */
    body = make_request_raw(LOGOUT);
    Command::new("sleep").arg("2").spawn().unwrap();
    assert_eq!(body,"user: user logged out");

}

#[test]
fn test_read_file() {
    /* Login */
    let mut body : String = make_request_raw(LOGIN);
    Command::new("sleep").arg("2").spawn().unwrap();
    assert_eq!(body,"user: user logged in");
    /* Create new empty file */
    body = make_request_raw(NEW_FILE);
    Command::new("sleep").arg("2").spawn().unwrap();
    assert_eq!(body,"created a new file for user named test");
    /* Update file */
    body = make_post_raw(UPDATE_FILE, "hello world");
    Command::new("sleep").arg("2").spawn().unwrap();
    assert_eq!(body,"user updated test owned by user");
    /* Read file */
    body = make_request_raw(READ_FILE);
    Command::new("sleep").arg("2").spawn().unwrap();
    assert_eq!(body,"user read test owned by user\nhello world");
    /* Logout */
    body = make_request_raw(LOGOUT);
    Command::new("sleep").arg("2").spawn().unwrap();
    assert_eq!(body,"user: user logged out");
}

#[test]
fn test_invite() {
    /* Login as user 1*/
    let mut body : String = make_request_raw(LOGIN);
    Command::new("sleep").arg("2").spawn().unwrap();
    assert_eq!(body,"user: user logged in");
    /* Create new empty file */
    body = make_request_raw(NEW_FILE);
    Command::new("sleep").arg("2").spawn().unwrap();
    assert_eq!(body,"created a new file for user named test");
    /* Login as user 2 */
    body = make_request_raw(
        "http://localhost:8477/login/user2");
    Command::new("sleep").arg("2").spawn().unwrap();
    assert_eq!(body,"user: user2 logged in");
    /* Logout as user 2 */
    body = make_request_raw(
        "http://localhost:8477/logout/user2");
    Command::new("sleep").arg("2").spawn().unwrap();
    assert_eq!(body,"user: user2 logged out");
    /* Invite user 2 to edit this file */
    body = make_request_raw(INVITE);
    Command::new("sleep").arg("2").spawn().unwrap();
    assert_eq!(body,"user invited user2 to edit test");
    /* Logout */
    body = make_request_raw(LOGOUT);
    Command::new("sleep").arg("2").spawn().unwrap();
    assert_eq!(body,"user: user logged out");

}

#[test]
fn test_multi_user() {
    /* Login as user 1*/
    let mut body : String = make_request_raw(LOGIN);
    Command::new("sleep").arg("3").spawn().unwrap();
    assert_eq!(body,"user: user logged in");
    /* Create new empty file */
    body = make_request_raw(NEW_FILE);
    Command::new("sleep").arg("3").spawn().unwrap();
    assert_eq!(body,"created a new file for user named test");
    /* Login as user 2 */
    body = make_request_raw(
        "http://localhost:8477/login/user2");
    Command::new("sleep").arg("3").spawn().unwrap();
    assert_eq!(body,"user: user2 logged in");
    /* Invite user 2 to edit this file */
    body = make_request_raw(INVITE);
    Command::new("sleep").arg("3").spawn().unwrap();
    assert_eq!(body,"user invited user2 to edit test");
    /* Write to the file as user1 */
    body = make_post_raw(UPDATE_FILE, "hello world");
    Command::new("sleep").arg("3").spawn().unwrap();
    assert_eq!(body,"user updated test owned by user");
    /* Read file as user 1*/
    body = make_request_raw(READ_FILE);
    Command::new("sleep").arg("3").spawn().unwrap();
    assert_eq!(body,"user read test owned by user\nhello world");
    /* Read file as user 2 */
    body = make_request_raw(
        "http://localhost:8477/read/user2/user/test",
        );
    Command::new("sleep").arg("3").spawn().unwrap();
    assert_eq!(body,
               "user2 read test owned by user\nhello world");
    /* Write to the file as user 2 */
    body = make_post_raw(
        "http://localhost:8477/update/user2/user/test",
        "hello world from user 2");
    Command::new("sleep").arg("3").spawn().unwrap();
    assert_eq!(body,"user2 updated test owned by user");
    /* Read file as user 1*/
    body = make_request_raw(READ_FILE);
    Command::new("sleep").arg("3").spawn().unwrap();
    assert_eq!(body,
               "user read test owned by user\nhello worldhello world from user 2");
    /* Read file as user 2 */
    body = make_request_raw(
        "http://localhost:8477/read/user2/user/test",
        );
    Command::new("sleep").arg("3").spawn().unwrap();
    assert_eq!(body,
               "user2 read test owned by user\nhello worldhello world from user 2");
    /* Logout as user 2 */
    body = make_request_raw(
        "http://localhost:8477/logout/user2");
    Command::new("sleep").arg("3").spawn().unwrap();
    assert_eq!(body,"user: user2 logged out");
    /* Logout */
    body = make_request_raw(LOGOUT);
    Command::new("sleep").arg("3").spawn().unwrap();
    assert_eq!(body,"user: user logged out");
}

#[test]
fn test_revoked_user() {
    /* Login as user 1*/
    let mut body : String = make_request_raw(LOGIN);
    Command::new("sleep").arg("3").spawn().unwrap();
    assert_eq!(body,"user: user logged in");
    /* Create new empty file */
    body = make_request_raw(NEW_FILE);
    Command::new("sleep").arg("3").spawn().unwrap();
    assert_eq!(body,"created a new file for user named test");
    /* Login as user 2 */
    body = make_request_raw(
        "http://localhost:8477/login/user2");
    Command::new("sleep").arg("3").spawn().unwrap();
    assert_eq!(body,"user: user2 logged in");
    /* Invite user 2 to edit this file */
    body = make_request_raw(INVITE);
    Command::new("sleep").arg("3").spawn().unwrap();
    assert_eq!(body,"user invited user2 to edit test");
    /* Write to the file as user1 */
    body = make_post_raw(UPDATE_FILE, "hello world");
    Command::new("sleep").arg("3").spawn().unwrap();
    assert_eq!(body,"user updated test owned by user");
    /* Read file as user 1*/
    body = make_request_raw(READ_FILE);
    Command::new("sleep").arg("3").spawn().unwrap();
    assert_eq!(body,"user read test owned by user\nhello world");
    /* Read file as user 2 */
    body = make_request_raw(
        "http://localhost:8477/read/user2/user/test",
        );
    Command::new("sleep").arg("3").spawn().unwrap();
    assert_eq!(body,
               "user2 read test owned by user\nhello world");
    /* Write to the file as user 2 */
    body = make_post_raw(
        "http://localhost:8477/update/user2/user/test",
        "hello world from user 2");
    Command::new("sleep").arg("3").spawn().unwrap();
    assert_eq!(body,"user2 updated test owned by user");
    /* Read file as user 1*/
    body = make_request_raw(READ_FILE);
    Command::new("sleep").arg("3").spawn().unwrap();
    assert_eq!(body,
               "user read test owned by user\nhello worldhello world from user 2");
    /* Read file as user 2 */
    body = make_request_raw(
        "http://localhost:8477/read/user2/user/test",
        );
    Command::new("sleep").arg("3").spawn().unwrap();
    assert_eq!(body,
               "user2 read test owned by user\nhello worldhello world from user 2");

    /* Revoke access to file from user2 */
    body = make_request_raw(REVOKE);
    Command::new("sleep").arg("3").spawn().unwrap();
    assert_eq!(body, "user revoked user2 from test");
    /* Attempt to read file as user 2 */
    body = make_request_raw(
        "http://localhost:8477/read/user2/user/test",
        );
    Command::new("sleep").arg("3").spawn().unwrap();
    assert_eq!(body, "Could not access: test");

    /* Attempt to write to file as user 2 */
    body = make_post_raw(
        "http://localhost:8477/update/user2/user/test",
        "Failed write");
    Command::new("sleep").arg("3").spawn().unwrap();
    assert_eq!(body,"Could not get document");
    /* Logout as user 2 */
    body = make_request_raw(
        "http://localhost:8477/logout/user2");
    Command::new("sleep").arg("3").spawn().unwrap();
    assert_eq!(body,"user: user2 logged out");
    /* Logout */
    body = make_request_raw(LOGOUT);
    Command::new("sleep").arg("3").spawn().unwrap();
    assert_eq!(body,"user: user logged out");
}


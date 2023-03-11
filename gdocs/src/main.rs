extern crate reqwest; // 0.9.18
use std::io::Read;
use std::process::Command;

static LOGIN: &str = "http://localhost:8477/login/user";
static LOGOUT: &str = "http://localhost:8477/logout/user";
static NEW_FILE: &str = "http://localhost:8477/new/user/test";
// static BROKE: &str = "http://localhost:8477/login";
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

    body = make_request_raw(NEW_FILE);
    Command::new("sleep").arg("2").spawn().unwrap();
    assert_eq!(body,"created a new file for user named test");

    body = make_request_raw(LOGOUT);
    Command::new("sleep").arg("2").spawn().unwrap();
    assert_eq!(body,"user: user logged out");
}




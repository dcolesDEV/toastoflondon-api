#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::fs::File;
use std::io::Read;

use rand::thread_rng;
use rand::seq::SliceRandom;

fn fetch_names() -> Result<Vec<String>, String> {
    let mut file = File::open("names.json").unwrap();
    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let json = serde_json::from_str(&data);

    match json {
        Ok(json) => Ok(json),
        Err(_) => return Err(String::from("Error processing names"))
    }
}

fn choose_random_name(names: Vec<String>) -> Result<String, String> {
    let mut rng = thread_rng();

    if names.len() == 0 {
        return Err(String::from("No names available to be randomly selected from"))
    }

    let random_name = names.choose(&mut rng);

    match random_name {
        Some(random_name) => Ok(String::from(random_name)),
        None => return Err(String::from("Random name was unable to be selected"))
    }
}

#[get("/")]
fn get() -> Result<String, String> {
    let names = fetch_names();

    match names {
        Ok(names) => {
            let result = choose_random_name(names);
        
            match result {
                Ok(name) => Ok(name),
                Err(error) => Err(error)
            }
        },
        Err(error) => return Err(error)
    }

}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![get])
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn test_fetch_names() {
        let names: Result<Vec<String>, String> = fetch_names();

        match names {
            Ok(names) => assert_eq!(names[4], "Kikini Bamalam"),
            Err(_) => assert!(false)
        }
    }

    #[test]
    fn test_choose_random_name() {
        let names: Result<Vec<String>, String> = fetch_names();

        match names {
            Ok(names) => {
                let result: Result<String, String> = choose_random_name(names.to_owned());
                
                match result {
                    Ok(result) => assert_eq!(names.contains(&result), true),
                    Err(_) => assert!(false)
                }
            },
            Err(_) => assert!(false)
        }
    }

    #[test]
    fn test_choose_random_name_with_no_names() {
        let names: Vec<String> = vec![];

        let result: Result<String, String> = choose_random_name(names.to_owned());
        
        match result {
            Ok(_) => assert!(false),
            Err(error) => assert_eq!(error, "No names available to be randomly selected from")
        }
    }

    #[test]
    fn test_get() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();

        let names: Result<Vec<String>, String> = fetch_names();

        match names {
            Ok(names) => {
                let result: Option<String> = response.body_string();
        
                assert_eq!(response.status(), Status::Ok);
                
                match result {
                    Some(result) => assert_eq!(names.contains(&result), true),
                    None => assert!(false)
                }
            },
            Err(_) => assert!(false)
        }
    }
}
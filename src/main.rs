#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::fs::File;
use std::io::Read;

use rand::seq::SliceRandom;
use rand::thread_rng;

fn fetch_names(file_name: String) -> Result<Vec<String>, String> {
    let file = File::open(file_name);

    match file {
        Ok(mut opened_file) => {
            let mut data = String::new();

            opened_file.read_to_string(&mut data).unwrap();

            let json = serde_json::from_str(&data);

            match json {
                Ok(json) => Ok(json),
                Err(_) => Err(String::from("Error reading from file")),
            }
        }
        Err(_) => Err(String::from("File provided does not exist")),
    }
}

fn choose_random_name(names: Vec<String>) -> Result<String, String> {
    let mut rng = thread_rng();
    let random_name = names.choose(&mut rng);

    match random_name {
        Some(random_name) => Ok(String::from(random_name)),
        None => Err(String::from(
            "No names available to be randomly selected from",
        )),
    }
}

#[get("/")]
fn get() -> Result<String, String> {
    let names = fetch_names(String::from("names.json"));

    match names {
        Ok(names) => {
            let result = choose_random_name(names);

            match result {
                Ok(name) => Ok(name),
                Err(error) => Err(error),
            }
        }
        Err(error) => Err(error),
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
    use rocket::http::Status;
    use rocket::local::Client;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn convert_file_path(file_path: PathBuf) -> String {
        file_path.into_os_string().into_string().unwrap()
    }

    #[test]
    fn test_fetch_names() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test-names.json");
        let mut file = File::create(&file_path).unwrap();

        write!(file, "[\"Kikini Bamalam\", \"Sal Commotion\"]").unwrap();

        let names = fetch_names(convert_file_path(file_path)).unwrap();

        assert_eq!(names.len(), 2)
    }

    #[test]
    #[should_panic(expected = "Error reading from file")]
    fn test_fetch_names_with_invalid_file_content() {
        fetch_names(String::from("Cargo.toml")).unwrap();
    }

    #[test]
    #[should_panic(expected = "File provided does not exist")]
    fn test_fetch_names_with_nonexistent_file() {
        fetch_names(String::from("nonexistent.file")).unwrap();
    }

    #[test]
    fn test_choose_random_name() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test-names.json");
        let mut file = File::create(&file_path).unwrap();

        write!(file, "[\"Sookie Houseboat\", \"Basil Watchfair\"]").unwrap();

        let names = fetch_names(convert_file_path(file_path)).unwrap();
        let result = choose_random_name(names.to_owned()).unwrap();

        assert_eq!(names.contains(&result), true)
    }

    #[test]
    #[should_panic(expected = "No names available to be randomly selected from")]
    fn test_choose_random_name_with_no_names() {
        let names: Vec<String> = vec![];

        choose_random_name(names).unwrap();
    }

    #[test]
    fn test_get() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();

        let names = fetch_names(String::from("names.json")).unwrap();
        let result = response.body_string().unwrap();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(names.contains(&result), true)
    }
}

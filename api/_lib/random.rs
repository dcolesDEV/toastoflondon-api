use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Name {
    name: String,
}

pub fn get_random_name(names: Vec<String>) -> Result<Name, String> {
    let mut rng = thread_rng();
    let random_name = names.choose(&mut rng);

    match random_name {
        Some(random_name) => Ok(Name {
            name: random_name.to_owned(),
        }),
        None => Err(String::from("No names to select from.")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random_name_gets_random_name() {
        let names: Vec<String> = vec![
            String::from("Sal Commotion"),
            String::from("Kikini Bamalam"),
        ];
        let data = get_random_name(names.clone()).unwrap();

        assert_eq!(names.contains(&data.name), true)
    }

    #[test]
    #[should_panic(expected = "No names to select from.")]
    fn test_get_random_name_with_no_names() {
        let names: Vec<String> = vec![];

        get_random_name(names).unwrap();
    }
}

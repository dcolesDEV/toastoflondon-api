use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn get_random_name(names: Vec<String>) -> Result<String, String> {
    let mut rng = thread_rng();
    let random_name = names.choose(&mut rng);

    match random_name {
        Some(random_name) => Ok(String::from(random_name)),
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
        let name = get_random_name(names.clone()).unwrap();

        assert_eq!(names.contains(&name), true)
    }

    #[test]
    #[should_panic(expected = "No names to select from.")]
    fn test_get_random_name_with_no_names() {
        let names: Vec<String> = vec![];

        get_random_name(names).unwrap();
    }
}

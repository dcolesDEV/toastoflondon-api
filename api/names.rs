use serde_json::json;

pub fn get_names() -> Vec<String> {
    let names: serde_json::Value = json!([
        "Iqbal Achieve",
        "Martin Aynuss",
        "Forrest Ash",
        "Francis Bacon",
        "Kikini Bamalam",
        "Danny Bear",
        "Howard Blackcap",
        "Cliff Bonanza",
        "Charles Bonus",
        "Cocker Boo",
        "Kika Bright",
        "Derek Buildings",
        "Howard Bugowitz",
        "Greta Cargo",
        "Duncan Clench",
        "Scott Chestnut",
        "Sal Commotion",
        "Portia de Coogan",
        "Royce Coolidge",
        "Colin Corknorth",
        "Ray Dooney",
        "Clem Fandango",
        "Kate Fear",
        "Norris Flipjack",
        "Milly Floss",
        "Dinky Frinkbuster",
        "Beezus Fuffoon",
        "Jemima Gina",
        "Max Gland",
        "Jon Hamm",
        "Chloe Hammersnag",
        "Sheila Hancock",
        "Acker Herron",
        "Roy Highknock",
        "Brooke Hooberman",
        "Pooky Hook",
        "Sookie Houseboat",
        "Ed Howzerblack",
        "Axel Jacklin",
        "Basil Jet",
        "Bentley Kenneth",
        "Daz Klondyke",
        "Stanley Kubrick",
        "Kit Leggings",
        "Una Length",
        "Varrity Map",
        "Hamilton Meathouse",
        "Sam Mendips",
        "Clancy Moped",
        "Larry Muggins",
        "Wendy Nook",
        "Jackie Paper",
        "Maggie Philbin",
        "Betty Pimples",
        "Parker Pipe",
        "Jane Plough",
        "Peggy Plywood",
        "Sterling Porrich",
        "Linda Praise",
        "Cliff Promise",
        "Bill Purchase",
        "Ray Purchase",
        "Heathcote Pursuit",
        "Jill Quear",
        "Giuseppe Race",
        "Susan Random",
        "Strawberry Rathbone",
        "Carol Sass",
        "Ricky Seasack",
        "Colin Skittles",
        "Nan Slack",
        "Jenny Spasm",
        "Ken Suggestion",
        "Nick Swivney",
        "Kay Tightneck",
        "Blair Toast",
        "Steven Toast",
        "Penny Traitor",
        "Patrick Treble",
        "Vic Titball",
        "Basil Watchfair",
        "Dick Weerdly",
        "Peanut Whistle",
        "Yvonne Wryly",
        "Lorna Wynde"
    ]);

    let buf = serde_json::to_vec(&names).expect("Could not read names json");

    serde_json::from_slice::<Vec<String>>(&buf).expect("Could not parse names into specified type")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_names_gets_all_names() {
        let names = get_names();

        assert_eq!(names.len(), 85)
    }
}

use std::collections::HashMap;

struct Md5Checker {
    map: HashMap<String, u8>,
}

impl Md5Checker {
    pub fn new() -> Md5Checker {
        let map: HashMap<String, u8> = HashMap::from([
            (String::from("0"), 4),
            (String::from("1"), 3),
            (String::from("2"), 2),
            (String::from("3"), 2),
            (String::from("4"), 1),
            (String::from("5"), 1),
            (String::from("6"), 1),
            (String::from("7"), 1),
            (String::from("8"), 0),
            (String::from("9"), 0),
            (String::from("A"), 0),
            (String::from("B"), 0),
            (String::from("C"), 0),
            (String::from("D"), 0),
            (String::from("E"), 0),
            (String::from("F"), 0),
        ]);
        Md5Checker {
            map
        }
    }

    fn get_bits_to_zero(&self, key: String) -> u8 {
        *self.map.get(&key).unwrap()
    }
}
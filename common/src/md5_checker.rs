use std::collections::HashMap;

pub struct Md5Checker {
    map: HashMap<String, u32>,
}

impl Md5Checker {
    pub fn new() -> Md5Checker {
        let map: HashMap<String, u32> = HashMap::from([
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

    pub fn get_bits_to_zero(&self, key: String) -> u32 {
        *self.map.get(&key).unwrap()
    }
}

#[test]
fn test_get_bits_to_zero(){
    let checker : Md5Checker = Md5Checker::new();
    let result1 : u32 = checker.get_bits_to_zero('F'.to_string());
    let result2 : u32 = checker.get_bits_to_zero('0'.to_string());

    assert_eq!(result1, 0);
    assert_eq!(result2, 4)
}
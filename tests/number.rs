use serde_derive::{Serialize, Deserialize};
use serde_pkl;

#[derive(Serialize, Deserialize)]
struct T000 {
    myint: u8,
}

#[test]
fn parse_000() {
    let pkl = "myint = 31";

    let data: T000 = serde_pkl::from_str(pkl).unwrap();

    assert_eq!(data.myint, 31);
}
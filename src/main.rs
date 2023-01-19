use ssz_rs::List;

const COUNT: usize = 8;

fn main() {
    let bytes = vec![
        0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 0u8,
    ];
    let _expected: List<u8, COUNT> = match bytes.try_into() {
        Ok(value) => value,
        Err(err) => {
            match err {
                //ssz_rs::list::Error => print!("too short: {}", err),
                //ssz_rs::Error::List(e) => print!("too short: {}", err)
            }
            print!("{}", err);
            panic!("{}",err)
        },
    };
}

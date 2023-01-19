use ssz_rs::{Error, List};

const COUNT: usize = 8;

fn main() {
    let bytes = vec![
        0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 0u8,
    ];
    let _expected: List<u8, COUNT> = match bytes.try_into() {
        Ok(value) => value,
        Err(err) => {
            match err {
                Error::List(e) => match e {
                    ssz_rs::list::Error::IncorrectLength { .. } => {}
                }
            }
            panic!("{}",err)
        },
    };
}

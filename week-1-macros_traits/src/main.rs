#[derive(Debug)]
struct Swap {
    qty_1: u32,
    qty_2: u32,
}

trait Serialize {
    fn serialize(&self) -> Vec<u8>;
}

trait Deserialize {
    fn deserialize(v: &[u8]) -> Result<Swap, String>;
}

impl Serialize for Swap {
    fn serialize(&self) -> Vec<u8> {
        let mut v = Vec::new();

        v.extend_from_slice(&self.qty_1.to_le_bytes());
        v.extend_from_slice(&self.qty_2.to_le_bytes());
        v
    }
}
impl Deserialize for Swap {
    fn deserialize(data: &[u8]) -> Result<Swap, String> {
        if data.len() < 8 {
            return Err("Not enough bytes".to_string());
        }

        let qty_1 = u32::from_le_bytes(data[0..4].try_into().unwrap());
        let qty_2 = u32::from_le_bytes(data[4..8].try_into().unwrap());

        return Ok(Swap { qty_1, qty_2 });
    }
}
fn main() {
    let s = Swap { qty_1: 1, qty_2: 2 };

    let s1 = s.serialize();
    println!("{:?}", s1);

    let s2 = Swap::deserialize(&s1).unwrap();
    println!("{:?}", s2);
}

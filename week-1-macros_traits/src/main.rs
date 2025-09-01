struct Swap {
    qty_1: u32,
    qty_2: u32,
}

trait Serialize {
    fn serialize(&self) -> Vec<u8>;
}

trait Deserialize {
    fn deserialize(&self) -> Self;
}

impl Serialize for Swap {
    fn serialize(&self) -> Vec<u8> {
        let mut v = Vec::new();
        v.extend_from_slice(&self.qty_1.to_le_bytes());
        v
    }
}

fn main() {}

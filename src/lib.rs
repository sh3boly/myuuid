use pyo3::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;

#[pyclass]
struct MyUuid([u8; 16]);
#[pymethods]
impl MyUuid {
    #[new]
    fn new_v4() -> Self {
        let mut rng = rand::rng();
        let mut bytes = [0u8; 16];
        rng.fill(&mut bytes);
        bytes[6] = (bytes[6] & 0x0F) | 0x40;
        bytes[8] = (bytes[8] & 0x3F) | 0x80;
        MyUuid(bytes)
    }
    #[staticmethod]
    fn new_v7() -> Self {
        let now = SystemTime::now();
        let mut rng = rand::rng();
        let mut bytes = [0u8; 16];
        
        let timestamp_bytes = now.duration_since(UNIX_EPOCH).unwrap().as_millis().to_be_bytes();
        rng.fill(&mut bytes);
        bytes[0..6].copy_from_slice(&timestamp_bytes[10..16]);
        bytes[6] = (bytes[6] & 0x0F) | 0x70;
        bytes[8] = (bytes[8] & 0x3F) | 0x80;
        MyUuid(bytes)
    }
    fn to_string(&self) -> String {
        let b = &self.0;
        format!(
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            b[0], b[1], b[2], b[3],
            b[4], b[5],
            b[6], b[7],
            b[8], b[9],
            b[10], b[11], b[12], b[13], b[14], b[15]
        )
    }
}

#[pymodule]
fn uuid_rust(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MyUuid>()?;
    Ok(())
}


#[cfg(test)]
mod tests {

    use super::*;
    use uuid::{Uuid, Version, Variant};

    #[test]
    fn test_uuid_v4_is_valid(){
        let my_uuid = MyUuid::new_v4();
        let s = my_uuid.to_string();

        let parsed = Uuid::parse_str(&s).unwrap();

        assert_eq!(parsed.get_version(), Some(Version::Random));
        assert_eq!(parsed.get_variant(), Variant::RFC4122);
    }
    #[test]
    fn test_uuid_v7_is_valid(){
        let my_uuid = MyUuid::new_v7();
        let s = my_uuid.to_string();

        let parsed = Uuid::parse_str(&s).unwrap();

        assert_eq!(parsed.get_version(), Some(Version::SortRand));
        assert_eq!(parsed.get_variant(), Variant::RFC4122);
    }
}
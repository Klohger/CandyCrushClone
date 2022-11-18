use serialization::AsBytes;

mod serialization;
fn main() {
    let str = "Hello world!";
    let serialized = str.to_bytes()?;
    let deserialized = &*str::from_bytes(&serialized)?.0;
    assert_eq!(str, deserialized);

    let (u8, u16, u32, u64): (u8, u16, u32, u64) = (5, 2573, 2155232, 24121412);
    
    let serialized = u8.to_bytes()?;
    let deserialized = *u8::from_bytes(&serialized)?.0;
    assert_eq!(u8, deserialized);

    let serialized = u16.to_bytes()?;
    let deserialized = *u16::from_bytes(&serialized)?.0;
    assert_eq!(u16, deserialized);

    let serialized = u32.to_bytes()?;
    let deserialized = *u32::from_bytes(&serialized)?.0;
    assert_eq!(u32, deserialized);



}

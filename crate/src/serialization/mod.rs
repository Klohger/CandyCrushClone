



use std::mem::size_of;

use cgmath::{SquareMatrix, num_traits::Float, Vector4, Matrix4};



/*
pub struct SerializationManager {
    deserialisation_map: HashMap<String, fn(String) -> Object>,
}
impl SerializationManager {
    pub fn deserialize<P: AsRef<Path>>(&self, path: P) -> io::Result<Vec<Object>> {
        match File::open(path) {
            Ok(file) => {

                let mut buf = BufReader::new(file);

                let count = buf.read_u64()?;
                let vec = Vec::new();
                for i in 0..count {
                    let nameLength = buf.read_u64()?;
                    let name = String::with_capacity(nameLength);
                    buf.read_exact(name)?;

                    if self.deserialisation_map.contains_key(&name) {

                        let mut data = Vec::new();
                        buf.read_to_end(&mut data)?;

                        vec.push((self.deserialisation_map[name])(data));
                    }
                }
                return Ok(vec);
            }
            Err(err) => return Err(err),
        }
    }
    pub fn serialize<P: AsRef<Path>>(
        theStuff: Vec<Object>,
        path: P,
    ) -> io::Result<()> {
        match File::create(path) {
            Ok(mut file) => {
                for serializable in theStuff {
                    match file.write_all(serializable.serialize()) {
                        Err(err) => return Err(err),
                        _ => (),
                    }
                }
            }
            Err(err) => return Err(err),
        }
        return Ok(());
    }
}
 */
/*
#[macro_export] macro_rules! serialize {
    ($object:ident, $( $x:ident ),+) => {
        fn serialize(&self) -> Vec<u8>{
            let vec = Vec::new();
            $(
                vec.append(&mut self.$x.to_bytes()?);
            )*
            return vec;
        }
    };
}
*/


pub trait AsBytes {
    fn to_bytes(&self) -> Result<Vec<u8>, &str>;
    fn from_bytes(data : &[u8]) -> Result<(Box<Self>, u32), &str>;
}
/*
macro_rules! impl_as_bytes_num {
    ($( $x:ident, $y:block ),*) => {
        $(
            impl AsBytes for $x {
                fn to_bytes(&self) -> Vec<u8> {
                    return self.to_ne_bytes().to_vec();
                }
                fn from_bytes(data : Vec<u8>) -> Option<Box<Self>> $y
                fn dyn_from_bytes(&self, data: Vec<u8>) -> Option<Box<Self>> $y
            }
        )*
    };
}

macro_rules! impl_as_bytes {
    ($( $x:ident, $to:stmt, $from:stmt),*) => {
        $(
            impl AsBytes for $x {
                fn to_bytes(&self) -> Vec<Self> $to
                fn from_bytes(data : Vec<u8>) -> Option<Box<Self>> $from
                fn dyn_from_bytes(&self, data : Vec<u8>) -> Option<Box<Self>> $from
            }
        )*
    };
}
*/
impl AsBytes for u8 {
    fn to_bytes(&self) -> Result<Vec<u8>, &str> {
        return Ok(self.to_be_bytes().to_vec());
    }

    fn from_bytes(data : &[u8]) -> Result<(Box<Self>,u32),&str> {
        if data.len() < size_of::<Self>() {
            return Err("length of buffer smaller than length of type!");
        } else {
            return Ok((Box::new(data[0]), size_of::<Self>() as u32));
        }
    }
}
impl AsBytes for i8 {
    fn to_bytes(&self) -> Result<Vec<u8>, &str> {
        return Ok(self.to_be_bytes().to_vec());
    }

    fn from_bytes(data : &[u8]) -> Result<(Box<Self>,u32),&str> {
        if data.len() < size_of::<Self>() {
            return Err("length of buffer smaller than length of type!");
        } else {
            return Ok((Box::new(Self::from_be_bytes([data[0]])), size_of::<Self>() as u32));
        }
    }
}
impl AsBytes for u16 {
    fn to_bytes(&self) -> Result<Vec<u8>, &str> {
        return Ok(self.to_be_bytes().to_vec());
    }

    fn from_bytes(data : &[u8]) -> Result<(Box<Self>,u32),&str> {
        if data.len() < size_of::<Self>() {
            return Err("length of buffer smaller than length of type!");
        } else {
            return Ok((Box::new(Self::from_be_bytes([data[0], data[1]])), size_of::<Self>() as u32));
        }
    }
}
impl AsBytes for i16 {
    fn to_bytes(&self) -> Result<Vec<u8>, &str> {
        return Ok(self.to_be_bytes().to_vec());
    }

    fn from_bytes(data : &[u8]) -> Result<(Box<Self>,u32),&str> {
        if data.len() < size_of::<Self>() {
            return Err("length of buffer smaller than length of type!");
        } else {
            return Ok((Box::new(Self::from_be_bytes([data[0], data[1]])), size_of::<Self>() as u32));
        }
    }
}
impl AsBytes for u32 {
    fn to_bytes(&self) -> Result<Vec<u8>, &str> {
        return Ok(self.to_be_bytes().to_vec());
    }

    fn from_bytes(data : &[u8]) -> Result<(Box<Self>,u32),&str> {
        if data.len() < size_of::<Self>() {
            return Err("length of buffer smaller than length of type!");
        } else {
            return Ok((Box::new(Self::from_be_bytes([data[0], data[1],data[2],data[3]])), size_of::<Self>() as u32));
        }
    }
}
impl AsBytes for i32 {
    fn to_bytes(&self) -> Result<Vec<u8>, &str> {
        return Ok(self.to_be_bytes().to_vec());
    }

    fn from_bytes(data : &[u8]) -> Result<(Box<Self>,u32),&str> {
        if data.len() < size_of::<Self>() {
            return Err("length of buffer smaller than length of type!");
        } else {
            return Ok((Box::new(Self::from_be_bytes([data[0], data[1],data[2],data[3]])), size_of::<Self>() as u32));
        }
    }
}
impl AsBytes for f32 {
    fn to_bytes(&self) -> Result<Vec<u8>, &str> {
        return Ok(self.to_be_bytes().to_vec());
    }

    fn from_bytes(data : &[u8]) -> Result<(Box<Self>,u32),&str> {
        if data.len() < size_of::<Self>() {
            return Err("length of buffer smaller than length of type!");
        } else {
            return Ok((Box::new(Self::from_be_bytes([data[0], data[1],data[2],data[3]])), size_of::<Self>() as u32));
        }
    }
}

impl AsBytes for str {
    fn to_bytes(&self) -> Result<Vec<u8>, &str> {
        if self.len() > u32::MAX as usize {
            return Err("string larger than u32::MAX");
        }
        let bytes = self.as_bytes();
        let length = (bytes.len() as u32).to_be_bytes();

        let mut result = Vec::new();
        result.append(&mut length.to_vec());
        result.append(&mut bytes.to_vec());
        return Ok(result);
    }

    fn from_bytes(data : &[u8]) -> Result<(Box<str>, u32), &str> {
        if data.len() < size_of::<u32>() {
            return Err("To small buffer sorry :)");
        } else {
            let length = *u32::from_bytes(data)?.0;
            if ((data.len() as u32) - size_of::<u32>() as u32) < length {
                return Err("To small buffer sorry :)");
            } else {
                let rest = &data[size_of::<u32>()..size_of::<u32>() + (length as usize)];
                return Ok((std::str::from_utf8(rest)?.into(), (size_of::<u32>() + rest.len()) as u32));
            }
        }
    }
}

impl AsBytes for String {
    fn to_bytes(&self) -> Result<Vec<u8>, &str> {
        if self.len() > u32::MAX as usize {
            return Err("string larger than u32::MAX");
        }
        let bytes = self.as_bytes();
        let length = (bytes.len() as u32).to_be_bytes();

        let mut result = Vec::new();
        result.append(&mut length.to_vec());
        result.append(&mut bytes.to_vec());
        return Ok(result);
    }

    fn from_bytes(data : &[u8]) -> Result<(Box<String>, u32), &str> {
        if data.len() < size_of::<u32>() {
            return Err("To small buffer sorry :)");
        } else {
            let length = *u32::from_bytes(data)?.0;
            if ((data.len() as u32) - size_of::<u32>() as u32) < length {
                return Err("To small buffer sorry :)");
            } else {
                let rest = &data[size_of::<u32>()..size_of::<u32>() + (length as usize)];
                return Ok((Box::new(String::from_utf8(rest.to_vec())?), (size_of::<u32>() + rest.len()) as u32));
            }
        }
    }
}





impl AsBytes for Matrix4<f32> {
    fn to_bytes(&self) -> Result<Vec<u8>, &str> {
        let vec = Vec::new();
        let swag = Into::<[[f32;4];4]>::into(*self);
        
        for arr in swag  {
            vec.append(&mut arr[0].to_bytes()?);
            vec.append(&mut arr[1].to_bytes()?);
            vec.append(&mut arr[2].to_bytes()?);
            vec.append(&mut arr[3].to_bytes()?);
        }
        return Ok(vec);
    }
    
    fn from_bytes(data : &[u8]) -> Result<(Box<Self>, u32), &str> {
        let mut mat = Matrix4::identity();
        let bytesRead = 0;
        let position = 0;
// #region
        let rest = &data[position..data.len()];
        let result = f32::from_bytes(rest)?;
        mat.x.x = *result.0;
        let bytesRead = result.1;
        let position = position + result.1 as usize;
        
        let rest = &data[position..data.len()];
        let result = f32::from_bytes(rest)?;
        mat.x.y = *result.0;
        let bytesRead = result.1;
        let position = position + result.1 as usize;
        
        let rest = &data[position..data.len()];
        let result = f32::from_bytes(rest)?;
        mat.x.z = *result.0;
        let bytesRead = result.1;
        let position = position + result.1 as usize;
        
        let rest = &data[position..data.len()];
        let result = f32::from_bytes(rest)?;
        mat.x.w = *result.0;
        let bytesRead = result.1;
        let position = position + result.1 as usize;

        let rest = &data[position..data.len()];
        let result = f32::from_bytes(rest)?;
        mat.y.z = *result.0;
        let bytesRead = result.1;
        let position = position + result.1 as usize;

        let rest = &data[position..data.len()];
        let result = f32::from_bytes(rest)?;
        mat.y.w = *result.0;
        let bytesRead = result.1;
        let position = position + result.1 as usize;

        let rest = &data[position..data.len()];
        let result = f32::from_bytes(rest)?;
        mat.z.x = *result.0;
        let bytesRead = result.1;
        let position = position + result.1 as usize;

        let rest = &data[position..data.len()];
        let result = f32::from_bytes(rest)?;
        mat.z.y = *result.0;
        let bytesRead = result.1;
        let position = position + result.1 as usize;

        let rest = &data[position..data.len()];
        let result = f32::from_bytes(rest)?;
        mat.z.z = *result.0;
        let bytesRead = result.1;
        let position = position + result.1 as usize;

        let rest = &data[position..data.len()];
        let result = f32::from_bytes(rest)?;
        mat.z.w = *result.0;
        let bytesRead = result.1;
        let position = position + result.1 as usize;

        let rest = &data[position..data.len()];
        let result = f32::from_bytes(rest)?;
        mat.w.x = *result.0;
        let bytesRead = result.1;
        let position = position + result.1 as usize;

        let rest = &data[position..data.len()];
        let result = f32::from_bytes(rest)?;
        mat.w.y = *result.0;
        let bytesRead = result.1;
        let position = position + result.1 as usize;

        let rest = &data[position..data.len()];
        let result = f32::from_bytes(rest)?;
        mat.w.z = *result.0;
        let bytesRead = result.1;
        let position = position + result.1 as usize;

        let rest = &data[position..data.len()];
        let result = f32::from_bytes(rest)?;
        mat.w.w = *result.0;
        let bytesRead = result.1;
        let position = position + result.1 as usize;

// #endregion
        Ok((Box::new(mat), bytesRead));
    }
}
/*
impl<T : AsBytes> AsBytes for Vec<T> {
    fn to_bytes(&self) -> Result<Vec<u8>, &str> {
        if self.len() > u32::MAX as usize {
            return Err("vector larger than u32::MAX");
        }
        let mut vec = Vec::new();
        let count = (self.len() as u32).to_be_bytes();
        vec.append(&mut count.to_vec());
        for t in self {
            let mut bytes = t.to_bytes()?;
            vec.append(&mut bytes);
        }
        return Ok(vec);
    }
    
    fn from_bytes(data : &[u8]) -> Result<Box<Self>, &str> {
        let mut vec = Vec::new();
        let position : u64 = 0;
        
        loop {
            if (data.len() - 1) as u64 == position {
                return Ok(Box::new(vec));
            }
            
            if ((data.len() - 1) as u64) < position + 8 {
                return None;
            }
            let length = *u64::from_bytes(&data[position..(position + 8)])?;
            position += 8;
            if (data.len() as u64) < position + length {
                return None;
            }
            

            
        }
    }
    
}

impl<T : AsBytes, const N : usize> AsBytes for [T;N] {
    fn to_bytes(&self) -> Result<Vec<u8>, &str> {
        let mut vec = Vec::new();
        for t in self {
            let mut bytes = t.to_bytes();
            let mut length = bytes.len().to_bytes();
            vec.append(&mut length);
            vec.append(&mut bytes);
        }
        return Ok(vec);
    }
}

*/
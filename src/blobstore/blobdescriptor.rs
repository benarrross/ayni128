// #![allow(unused)]
// use std::io::prelude::*;


// pub struct BlobDescriptor <'a> {
//     pub position : u64,
//     pub size : u32,   // NYI make this a u32 when we add bytes for next_empty?
//     pub unwritten_bytes : Option<&'a [u8]>,
// }


// impl<'a> BlobDescriptor<'a> {
//     pub fn default() -> BlobDescriptor<'a> {
//         BlobDescriptor {
//             position : 0,
//             size: 0,
//             unwritten_bytes : Option::None,
//         }
//     }

//     pub fn serialize<Writer: Write>(&self, backing_store : & mut Writer) {
//         backing_store.write_all(&self.position.to_le_bytes());
//         backing_store.write_all(&self.size.to_le_bytes());
//     }
// }



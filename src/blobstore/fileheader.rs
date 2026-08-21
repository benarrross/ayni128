#![allow(unused)]
use std::{f32::consts::E, io::prelude::*, num::NonZeroU64};
use crate::blobid::*;
use super::stream::*;


#[repr(C)]
pub struct FileHeader {
    magic_number: u64,  // 0xb10bf11e
    file_format_version: u32,
    header_size: u32,
    pub root_blob_id: BlobId,
    extra0 : u64,
    extra1 : u128,
    extra2 : u128,
}


impl FileHeader {
    pub fn new(root_blob_id: BlobId) -> Self {
        FileHeader {
                magic_number: 0xb10bf11e,
                file_format_version: 1,
                header_size: std::mem::size_of::<FileHeader>() as u32,
                root_blob_id: root_blob_id,
                extra0 : 0,
                extra1 : 0,
                extra2 : 0,
        }
    }

    pub fn default() -> Self {
        Self::new(BlobId::new(NonZeroU64::MAX))
    }

    pub fn read(backing_store : &mut dyn Stream) -> FileHeader {

        let magic_number = backing_store.read_u64();
        let file_format_version = backing_store.read_u32();
        let header_size = backing_store.read_u32();
        let root_blob_id = backing_store.read_u64();
        let extra0 = backing_store.read_u64();
        let extra1 = backing_store.read_u128();
        let extra2 = backing_store.read_u128();

        FileHeader {
            magic_number: magic_number,
            file_format_version: file_format_version,
            header_size: header_size,
            root_blob_id: BlobId::new(std::num::NonZeroU64::new(root_blob_id).unwrap()),
            extra0: extra0,
            extra1: extra1,
            extra2: extra2,
        }
    }

    pub fn serialize(&self, backing_store : &mut dyn Stream) {
        backing_store.write_all(&self.magic_number.to_le_bytes());
        backing_store.write_all(&self.file_format_version.to_le_bytes());
        backing_store.write_all(&self.header_size.to_le_bytes());
        backing_store.write_all(&self.root_blob_id.to_le_bytes());
        backing_store.write_all(&self.extra0.to_le_bytes());
        backing_store.write_all(&self.extra1.to_le_bytes());
        backing_store.write_all(&self.extra2.to_le_bytes());
    }
}

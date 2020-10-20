pub(crate) mod builder;
mod iterator;

use crate::checksum;
use crate::opt::Options;
use crate::Error;
use crate::Result;
use bytes::{Buf, Bytes};
use prost::Message;
use proto::meta::{BlockOffset, Checksum, TableIndex};
use std::fs;
use std::io::{Read, Seek, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::Mutex;

// TODO: use a mmap library instead of handling I/O on our own
enum MmapFile {
    File {
        name: PathBuf,
        // TODO: remove this mutex and allow multi-thread read
        file: Mutex<fs::File>,
    },
    Memory {
        data: Bytes,
    },
}

impl MmapFile {
    pub fn is_in_memory(&self) -> bool {
        match self {
            Self::File { .. } => false,
            Self::Memory { .. } => true,
        }
    }
}

pub struct TableInner {
    file: MmapFile,
    table_size: usize,
    smallest: Bytes,
    biggest: Bytes,
    id: u64,
    checksum: Bytes,
    estimated_size: u32,
    index: TableIndex,
    index_start: usize,
    index_len: usize,
    opt: Options,
}

pub struct Table {
    inner: Arc<TableInner>,
}

/*
impl Drop for TableInner {
    fn drop(&mut self) {
        let f = match self.file.take() {
            Some(f) => f,
            None => return,
        };
        f.file.set_len(0).unwrap();
        drop(f.file);
        fs::remove_file(&f.path).unwrap();
    }
}
*/

#[derive(Default)]
pub struct Block {
    offset: usize,
    data: Bytes,
    checksum: Bytes,
    entries_index_start: usize,
    entry_offsets: Vec<u32>,
    checksum_len: usize,
}

impl TableInner {
    pub fn create(path: &Path, data: Bytes, opt: Options) -> Result<TableInner> {
        unimplemented!()
    }

    pub fn open(path: &Path, opt: Options) -> Result<TableInner> {
        unimplemented!()
    }

    pub fn open_in_memory(data: Bytes, id: u64, opt: Options) -> Result<TableInner> {
        unimplemented!()
    }

    fn init_biggest_and_smallest(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn init_index(&mut self) -> Result<&BlockOffset> {
        unimplemented!()
    }

    fn key_splits(&mut self, n: usize, _prefix: Bytes) -> Vec<String> {
        unimplemented!()
    }

    fn fetch_index(&self) -> &TableIndex {
        return &self.index;
        // TODO: encryption
    }

    fn offsets_length(&self) -> usize {
        self.fetch_index().offsets.len()
    }

    fn offsets(&self, idx: usize) -> Option<&BlockOffset> {
        self.fetch_index().offsets.get(idx)
    }

    fn block(&self, idx: usize, _use_cache: bool) -> Result<Arc<Block>> {
        unimplemented!()
    }

    fn index_key(&self) -> u64 {
        self.id
    }

    pub fn key_count(&self) -> u32 {
        self.fetch_index().key_count
    }

    pub fn index_size(&self) -> usize {
        self.index_len
    }

    pub fn bloom_filter_size(&self) -> usize {
        self.fetch_index().bloom_filter.len()
    }

    pub fn size(&self) -> u64 {
        self.table_size as u64
    }

    pub fn smallest(&self) -> &Bytes {
        &self.smallest
    }

    pub fn biggest(&self) -> &Bytes {
        &self.biggest
    }

    pub fn filename(&self) -> String {
        match &self.file {
            MmapFile::Memory { .. } => "<memtable>".to_string(),
            MmapFile::File { name, .. } => name.to_string_lossy().into_owned(),
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn does_not_have(_hash: u32) -> bool {
        false
        // TODO: add bloom filter
    }

    fn read_bloom_filter(&self) {
        unimplemented!()
    }

    pub(crate) fn read_table_index(&self) -> Result<TableIndex> {
        unimplemented!()
    }

    fn verify_checksum(&self) -> Result<()> {
        unimplemented!()
    }

    fn read(&self, offset: usize, size: usize) -> Result<Bytes> {
        self.bytes(offset, size)
    }

    fn bytes(&self, offset: usize, size: usize) -> Result<Bytes> {
        unimplemented!()
    }

    fn is_in_memory(&self) -> bool {
        self.file.is_in_memory()
    }

    fn max_version(&self) -> u64 {
        unimplemented!()
    }
}

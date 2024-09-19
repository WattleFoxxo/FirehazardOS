use spin::Mutex;
use alloc::vec::Vec;
use crate::fs::blockdevice::{ BlockDevice };

use fatfs::{ FileSystem, FsOptions };

use alloc::sync::Arc;

// #[derive(Debug, Clone, Copy)]
pub static GLOBAL_FILESYSTEM: Arc<FileSystem<BlockDevice>> = Arc::new(FileSystem::new(BlockDevice::new(), FsOptions::new()).unwrap());

pub fn init() {
    // &FILESYSTEMS.push(FileSystem::new(BlockDevice::new(), FsOptions::new()).unwrap());
}
use crate::drivers::disk::ata;
use block_device::{ BlockDevice };

use alloc::vec::Vec;
use crate::println;

#[derive(Clone, Copy, Debug)]
pub struct BlockDeviceATA();

unsafe impl Sync for BlockDeviceATA {

}

unsafe impl Send for BlockDeviceATA {

}

impl BlockDeviceATA {
    pub fn new() -> Self {
        Self()
    }
}

impl BlockDevice for BlockDeviceATA {
    type Error = usize;

    fn read(&self, buf: &mut [u8], address: usize, number_of_blocks: usize) -> Result<(), Self::Error> {

        if (number_of_blocks == 0) {
            return Ok(());
        }

        if (buf.len() < 512) {
            println!("read {} {}", buf.len(), number_of_blocks);
        }

        let mut buffer: Vec<u8> = Vec::new();

        buffer.resize(512 * number_of_blocks, 0);

        let mut ind = 0;

        for i in address / 512..address / 512 + number_of_blocks {
            ata::read_sector(i as u64, &mut buffer.as_mut_slice()[512 * ind..512 * (ind + 1)]);

            ind += 1;
        }

        buf.copy_from_slice(&buffer[..buf.len()]);

        Ok(())
    }

    fn write(&self, buf: &[u8], address: usize, number_of_blocks: usize) -> Result<(), Self::Error> {

        assert!(buf.len() >= 5);

        for i in address / 512..address / 512 + number_of_blocks {
            ata::write_sector(i as u64, &buf[0..512]);
        }

        Ok(())
    }
}
use crate::drivers::disk::ata;

use fatfs::{ Write, Read, Seek, SeekFrom, IoBase };

use core::cmp;

pub struct BlockDevice {
    position: u64,
}

use crate::{ print, println };

impl BlockDevice {
    pub fn new() -> Self {
        Self {
            position: 0,
        }
    }

    fn read_sector(&self, address: u64, buf: &mut [u8]) {
        ata::read_sector(address as u64, buf);
    }

    fn write_sector(&self, address: u64, buf: &[u8]) {
        ata::write_sector(address as u64, buf);
    }

    fn current_sector(&self) -> u64 {
        self.position / 512 as u64
    }

    fn sector_offset(&self) -> usize {
        (self.position % 512 as u64) as usize
    }
}

impl IoBase for BlockDevice {
    type Error = ();
}

impl Read for BlockDevice {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let mut total_read = 0;

        while (total_read < buf.len()) {
            let sector = self.current_sector();
            let offset = self.sector_offset();
            let remaining_in_sector = 512 - offset;
            let to_read = (buf.len() - total_read).min(remaining_in_sector);

            let mut sector_buffer: [u8; 512] = [0u8; 512];

            self.read_sector(sector, &mut sector_buffer);

            buf[total_read..total_read + to_read].copy_from_slice(&sector_buffer[offset..offset + to_read]);

            self.position += to_read as u64;
            total_read += to_read;
        }

        Ok(total_read)
    }
}

impl Write for BlockDevice {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let mut total_written = 0;

        while (total_written < buf.len()) {
            let sector = self.current_sector();
            let offset = self.sector_offset();
            let remaining_in_sector = 512 - offset;
            let to_write = (buf.len() - total_written).min(remaining_in_sector);

            let mut sector_buffer: [u8; 512] = [0u8; 512];

            if (offset != 0 || to_write != 512) {
                self.read_sector(sector, &mut sector_buffer);
            }

            sector_buffer[offset..offset + to_write].copy_from_slice(&buf[total_written..total_written + to_write]);

            self.write_sector(sector, &sector_buffer);

            self.position += to_write as u64;
            total_written += to_write;
        }

        Ok(total_written)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl Seek for BlockDevice {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, Self::Error> {
        match pos {
            SeekFrom::Start(offset) => {
                self.position = offset;
            }

            SeekFrom::End(offset) => {
                todo!("Seek from end");
            }

            SeekFrom::Current(offset) => {
                if offset < 0 {
                    let new_position = self.position.saturating_sub(offset.unsigned_abs() as u64);
                    self.position = cmp::max(new_position, 0);
                } else {
                    self.position = self.position.saturating_add(offset as u64);
                }
            }
        }

        Ok(self.position)
    }
}

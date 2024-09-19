use crate::println;
use lazy_static::lazy_static;
use x86_64::instructions::port::{Port, PortReadOnly, PortWriteOnly};
use alloc::string::{String, ToString};
use spin::Mutex;
use alloc::vec::Vec;
use bit_field::BitField;

pub static ATADISKS: Mutex<Vec<AtaDevice>> = Mutex::new(Vec::new());

#[derive(Debug)]
pub struct AtaDeviceInfo {
    pub name: String,
    pub serial_number: String,
    pub size: u64,
}

#[derive(Debug)]
pub struct AtaDevice {
    data_port: Port<u16>,
    error_port: PortReadOnly<u8>,
    features_port: PortWriteOnly<u8>,
    sector_count_port: Port<u8>,
    lba_low_port: Port<u8>,
    lba_mid_port: Port<u8>,
    lba_high_port: Port<u8>,
    device_port: Port<u8>,
    status_port: PortReadOnly<u8>,
    command_port: PortWriteOnly<u8>,
    control_port: PortWriteOnly<u8>,
    alt_status_port: PortReadOnly<u8>,
}

impl AtaDevice {
    pub fn new(base: u16) -> Self {
        AtaDevice {
            data_port: Port::new(base),
            error_port: PortReadOnly::new(base + 1),
            features_port: PortWriteOnly::new(base + 1),
            sector_count_port: Port::new(base + 2),
            lba_low_port: Port::new(base + 3),
            lba_mid_port: Port::new(base + 4),
            lba_high_port: Port::new(base + 5),
            device_port: Port::new(base + 6),
            status_port: PortReadOnly::new(base + 7),
            command_port: PortWriteOnly::new(base + 7),
            control_port: PortWriteOnly::new(base + 0xC),
            alt_status_port: PortReadOnly::new(base + 0xC),
        }
    }

    pub fn identify_device(&mut self) -> AtaDeviceInfo {

        self.wait_busy();

        unsafe {
            self.device_port.write(0xA0);

            self.command_port.write(0xEC);
        }

        self.wait_busy();

        let mut buf = [0u8; 512];
        unsafe {
            for i in 0..256 {
                let short: u16 = self.data_port.read();
                buf[i * 2] = short.to_be_bytes()[0];
                buf[i * 2 + 1] = short.to_be_bytes()[1];
            }
        }

        let name = String::from_utf8_lossy(&buf[54..94]).to_string();
        let serial_number = String::from_utf8_lossy(&buf[20..40]).to_string();
        let sectors = u32::from_be_bytes(buf[120..124].try_into().unwrap()).rotate_left(16);

        if (sectors == 0) {
            return AtaDeviceInfo {
                name: String::from("No Device"),
                serial_number: String::from(""),
                size: 0,
            };
        }

        AtaDeviceInfo {
            name,
            serial_number,
            size: (<u32 as Into<u64>>::into(sectors) * 512),
        }
    }

    pub fn read_sector(&mut self, lba: u64, buf: &mut [u8]) -> bool {
        self.wait_busy();

        unsafe {
            self.device_port.write(0xE0 | ((lba >> 24) as u8 & 0x0F));
            self.sector_count_port.write(0x01); // Amout of sectors

            self.lba_low_port.write(lba as u8);
            self.lba_mid_port.write((lba >> 8) as u8);
            self.lba_high_port.write((lba >> 16) as u8);
            self.command_port.write(0x20); // Read sector command
        }

        self.wait_busy();

        // unsafe {
        //     for i in (0..256).step_by(2) {
        //         let data = self.data_port.read();
        //         buf[i] = data.get_bits(0..8) as u8;
        //         buf[i + 1] = data.get_bits(8..16) as u8;
        //     }
        // }

        unsafe {
            for i in 0..256 {
                let data = self.data_port.read();
                buf[i * 2] = data as u8;
                buf[i * 2 + 1] = (data >> 8) as u8;
            }
        }
        
        true
    }

    pub fn write_sector(&mut self, lba: u64, buf: &[u8]) -> bool {
        self.wait_busy();

        unsafe {
            self.device_port.write(0xE0 | ((lba >> 24) as u8 & 0x0F));
            self.sector_count_port.write(0x01); // Amout of sectors

            self.lba_low_port.write(lba as u8);
            self.lba_mid_port.write((lba >> 8) as u8);
            self.lba_high_port.write((lba >> 16) as u8);

            self.command_port.write(0x30); // Write sector command
        }

        
        self.wait_busy();
        
        // unsafe {
        //     for i in 0..256 {
        //         let short = u16::from_be_bytes([buf[i * 2], buf[i * 2 + 1]]);
        //         self.data_port.write(short);
        //     }
        // }

        unsafe {
            for i in 0..256 {
                let mut data = 0 as u16;
                data.set_bits(0..8, buf[i * 2] as u16);
                data.set_bits(8..16, buf[i * 2 + 1] as u16);
                
                self.data_port.write(data);
            }
        }
        
        true
    }

    fn wait_busy(&mut self) {
        while unsafe{self.status_port.read()} & 0x80 != 0 {}
    }
}

pub fn init() {
    let mut disks = ATADISKS.lock();
    disks.push(AtaDevice::new(0x170));
}

pub fn read_sector(lba: u64, buf: &mut [u8]) -> bool {
    let mut disks = ATADISKS.lock();
    disks[0].read_sector(lba, buf)
}

pub fn write_sector(lba: u64, buf: &[u8]) -> bool {
    let mut disks = ATADISKS.lock();
    disks[0].write_sector(lba, buf)
}

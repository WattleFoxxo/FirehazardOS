#![no_std]
#![no_main]

extern crate alloc;

use fhos::println;
use fhos::task::{ executor::Executor, keyboard, Task };
use bootloader::{ entry_point, BootInfo };
use core::panic::PanicInfo;
use fhos::drivers::disk::ata;

use fhos::fs::{ 
    filesystem::{ self, GLOBAL_FILESYSTEM },
    blockdevice::{ BlockDevice }
};

use fatfs::{ FileSystem, FsOptions, Read };

use crate::ata::AtaDevice;

use log::{ Record, Level, Metadata, trace };

struct SimpleLogger;


impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use fhos::allocator;
    use fhos::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;
    use fhos::wasm;
    use alloc::string::{String, ToString};

    // use fat32::volume::Volume;
    use alloc::vec::Vec;
    use log::{SetLoggerError, LevelFilter};

    fhos::init();
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Trace));

    println!("TRACE -> FHOS INIT");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    println!("TRACE -> MEMORY INIT");

    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    println!("TRACE -> ALLOC INIT");

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("shits fucked :/");
    println!("TRACE -> HEAP INIT");
    
    ata::init();
    println!("TRACE -> ATA INIT");

    filesystem::init();
    println!("TRACE -> FILESYSTEM INIT");

    let volume = BlockDevice::new();

    let mut fs = &GLOBAL_FILESYSTEM;
    let root_dir = fs.root_dir();
    
    let mut welcome_file = root_dir.open_file("sys/welcome.txt").unwrap();

    let mut buffer = Vec::new();
    let mut data = [0u8; 512];
    loop {
        match welcome_file.read(&mut data) {
            Ok(0) => break,
            Ok(n) => buffer.extend_from_slice(&data[..n]),
            Err(_) => break, 
        }
    }

    println!("/sys/welcome.txt: {}", String::from_utf8(buffer).unwrap());

    let mut executor = Executor::new();

    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.spawn(Task::new(wasm::run_file("bin/test.was")));

    executor.run();

    fhos::hlt_loop();
}

async fn async_numb() -> u32 {
    2
}

async fn hello_world_task() {
    let res = async_numb().await;
    println!("Hello, World! {}", res);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    fhos::hlt_loop();
}


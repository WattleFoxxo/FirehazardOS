#![no_std]
#![no_main]

extern crate alloc;

use fhos::println;
use fhos::task::{executor::Executor, keyboard, Task};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use fhos::drivers::disk::ata;
use fhos::fs::block_device::{ BlockDeviceATA };
use fat32;

use crate::ata::AtaDevice;

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use fhos::allocator;
    use fhos::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;
    use fhos::wasm;
    use alloc::string::{String, ToString};

    use fat32::volume::Volume;
    use alloc::vec::Vec;

    fhos::init();

    println!("FHOS 0.0.0");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("shits fucked :/");
    
    ata::init();

    let disk = BlockDeviceATA::new();
    let vol = Volume::new(disk);
    let mut root = vol.root_dir();

    // root.create_file("test.txt").unwrap();
    // open file
    let mut file = root.open_file("test.txt").unwrap();
    
    let mut filebuf: [u8; 13] = [42; 13];

    let wah = file.read(&mut filebuf).unwrap();

    println!("{}", wah);

    println!("{:?}", filebuf);
    
    // write buffer to file
    // file.write(&[80; 1234]).unwrap();

    println!("Ready!");

    fhos::hlt_loop();

    // let mut executor = Executor::new();
    // executor.spawn(Task::new(hello_world_task()));
    // executor.spawn(Task::new(keyboard::print_keypresses()));
    // executor.spawn(Task::new(wasm::init()));
    // executor.run();
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


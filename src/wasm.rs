use wasmi::*;
use crate::println;
use alloc::vec::Vec;

use fatfs::{FileSystem, File, FsOptions, Read};
use crate::fs::filesystem::{ GLOBAL_FILESYSTEM };

pub async fn run_file( wasm_file_name: &str ) {
    let engine = Engine::default();

    let mut fs = &GLOBAL_FILESYSTEM;

    println!("Loading WASM Binary: {}", wasm_file_name);

    let root_dir = fs.root_dir();
    let mut wasm_file = root_dir.open_file(wasm_file_name).unwrap();

    let mut wasm = Vec::new();
    let mut data = [0u8; 512];
    loop {
        match wasm_file.read(&mut data) {
            Ok(0) => break,
            Ok(n) => wasm.extend_from_slice(&data[..n]),
            Err(_) => break, 
        }
    }

    let module = Module::new(&engine, &wasm[..]).unwrap();

    type HostState = u32;
    let mut store = Store::new(&engine, 42);
    let host_hello = Func::wrap(&mut store, |caller: Caller<'_, HostState>, param: i32| {
        println!("Got {param} from WebAssembly");
        println!("My host state is: {}", caller.data());
    });

    let mut linker = <Linker<HostState>>::new(&engine);

    linker.define("host", "hello", host_hello);
    
    let instance = linker.instantiate(&mut store, &module).expect("REASON").start(&mut store);
    let hello = instance.expect("REASON").get_typed_func::<(), ()>(&store, "hello");

    hello.expect("REASON").call(&mut store, ());
}
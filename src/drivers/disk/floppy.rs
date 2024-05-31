use crate::println;
use lazy_static::lazy_static;
use x86_64::instructions::port::{Port, PortReadOnly, PortWriteOnly};
use alloc::string::{String, ToString};
use spin::Mutex;
use alloc::vec::Vec;

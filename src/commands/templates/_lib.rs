#![no_std]

use gstd::{debug, msg};

#[no_mangle]
pub unsafe extern "C" fn handle() {
    debug!("handle()");
    msg::reply(b"Hello world!", 0, 0);
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    debug!("init()");
}

#![no_std]

use gstd::msg;

#[gstd_async::main]
async fn main() {
    msg::reply(b"Hello world!", 0, 0);
}

#[no_mangle]
pub unsafe extern "C" fn init() {}

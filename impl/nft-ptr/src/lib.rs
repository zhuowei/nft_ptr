#![feature(once_cell)]

use nft_ptr_lib::{make_nft_ptr_lib, NftPtrLib, NftPtrLibTransport};
use std::ffi::CStr;
use std::lazy::SyncLazy;
use std::sync::Mutex;

static RUNTIME: SyncLazy<tokio::runtime::Runtime> =
    SyncLazy::new(|| tokio::runtime::Runtime::new().unwrap());

// https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
static NFTPTRLIB: SyncLazy<Mutex<NftPtrLib<NftPtrLibTransport>>> = SyncLazy::new(|| {
    // TODO(zhuowei): find a real place for this, haha
    env_logger::init();
    let mut lib = RUNTIME.block_on(make_nft_ptr_lib());
    RUNTIME.block_on(lib.initialize());
    Mutex::new(lib)
});

#[no_mangle]
pub extern "C" fn WdbNftPtrInitialize(
    owner_address: u64,
    caller_pc: u64,
    ptr_object_type: *const i8,
) {
    let ptr_object_type_str = unsafe { CStr::from_ptr(ptr_object_type) }.to_str().unwrap();
    RUNTIME.block_on(NFTPTRLIB.lock().unwrap().ptr_initialize(
        owner_address,
        caller_pc,
        ptr_object_type_str,
    ));
}

#[no_mangle]
pub extern "C" fn WdbNftPtrMoveToken(
    owner_address: u64,
    previous_owner_address: u64,
    value: u64,
    caller_pc: u64,
    object_type: *const i8,
) {
    let object_type_str = unsafe { CStr::from_ptr(object_type) }.to_str().unwrap();
    RUNTIME.block_on(NFTPTRLIB.lock().unwrap().move_token(
        owner_address,
        previous_owner_address,
        value,
        caller_pc,
        object_type_str,
    ));
}

#[no_mangle]
pub extern "C" fn WdbNftPtrDestroy(owner_address: u64) {
    RUNTIME.block_on(NFTPTRLIB.lock().unwrap().ptr_destroy(owner_address));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

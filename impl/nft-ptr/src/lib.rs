#[no_mangle]
pub extern "C" fn WdbNftPtrInitialize(
    owner_address: u64,
    caller_pc: u64,
    ptr_object_type: *const u8,
) {
}

#[no_mangle]
pub extern "C" fn WdbNftPtrMoveToken(
    owner_address: u64,
    previous_owner_address: u64,
    value: u64,
    caller_pc: u64,
    object_type: *const u8,
) {
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

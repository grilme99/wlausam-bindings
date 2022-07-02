#[no_mangle]
fn __heap_alloc_string(capacity: usize) -> *mut u8 {
    let mut str = String::with_capacity(capacity);
    let ptr = str.as_mut_ptr();
    std::mem::forget(str);
    ptr
}

#[no_mangle]
fn __drop_box_dyn_fn(_ptr: Box<dyn Fn(String, bool) -> String>) {}

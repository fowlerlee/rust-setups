#[allow(dead_code)]
fn unsafe_raw_pointer() -> i32 {
    let mut value = 42;
    let raw_ptr = &mut value as *mut i32; // Getting a raw mutable pointer

    unsafe {
        *raw_ptr += 1; // Modifying data through a raw pointer
    }

    println!("Value after increment: {}", value);
    return value;
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(unsafe_raw_pointer(), 42);
    }
}

#[cfg(target_family = "unix")]
#[link(name = "c")]
extern "C" {
    fn write(fd: u32, buf: *const u8, count: usize) -> i32;
}

fn syscall(message: String) -> std::io::Result<()> {
    let msg_ptr = message.as_ptr();
    let len = message.len();
    let res = unsafe { write(1, msg_ptr, len) };
    if res == -1 {
        return Err(std::io::Error::last_os_error());
    }
    Ok(())
}

fn main() {
    let message = "Hey from libc\n".to_string();
    syscall(message).unwrap()
}

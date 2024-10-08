use std::arch::asm;

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
#[inline(never)]
fn syscall(message: String) {
    let msg_ptr = message.as_ptr();
    let len = message.len();

    unsafe {
        asm!(
            "mov x16, 4",
            "mov x0, 1",
            "svc 0",
            in("x1") msg_ptr,
            in("x2") len,
            out("x16") _,
            out("x0") _,
            lateout("x1") _,
            lateout("x2") _
        );
    }
}

fn main() {
    let message = "Check out my raw syscall!\n".to_string();
    syscall(message);
}

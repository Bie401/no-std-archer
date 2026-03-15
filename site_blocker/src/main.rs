#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    syscall_exit(1);
}

// Change from _start to main
// Use i32 return type to satisfy the C caller
#[unsafe(no_mangle)]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    syscall_write(b"Hello from MIPS main!\n");
    0
}

fn syscall_write(msg: &[u8]) {
    unsafe {
        core::arch::asm!(
            ".set noat",
            "li $2, 4004",   // syscall number for write
            "li $4, 1",      // stdout
            "move $5, {0}",  // buffer
            "move $6, {1}",  // length
            "syscall",
            ".set at",
            in(reg) msg.as_ptr(),
            in(reg) msg.len(),
            out("$2") _, out("$4") _, out("$5") _, out("$6") _,
        );
    }
}

fn syscall_exit(code: i32) -> ! {
    unsafe {
        core::arch::asm!(
            "li $2, 4001",   // v0 = exit
            "move $4, {0}",  // a0 = code
            "syscall",
            in(reg) code,
            options(noreturn)
        );
    }
}
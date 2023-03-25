//! crt0

#![no_std]
#![feature(linkage)]

use core::arch::global_asm;


#[cfg(target_arch = "x86")]
global_asm!(
    "
    .globl _start
    .type _start, @function
_start:
    sub esp, 8

    mov DWORD PTR [esp], 0x00001F80
    # ldmxcsr [esp]
    mov WORD PTR [esp], 0x037F
    fldcw [esp]

    add esp, 8

    push esp
    call dlibc_start
    .size _start, . - _start
"
);

#[cfg(target_arch = "x86_64")]
global_asm!(
    "
    .globl _start
    .type _start, @function
_start:
    mov rdi, rsp
    and rsp, 0xFFFFFFFFFFFFFFF0

    sub rsp, 8

    mov DWORD PTR [rsp], 0x00001F80
    ldmxcsr [rsp]
    mov WORD PTR [rsp], 0x037F
    fldcw [rsp]

    add rsp, 8

    call dlibc_start
    .size _start, . - _start
"
);

#[linkage = "weak"]
#[no_mangle]
extern "C" fn dlibc_panic(pi: &::core::panic::PanicInfo) -> ! {
    loop {}
}

#[panic_handler]
#[linkage = "weak"]
#[no_mangle]
pub unsafe extern "C" fn rust_begin_unwind(pi: &::core::panic::PanicInfo) -> ! {
    dlibc_panic(pi)
}
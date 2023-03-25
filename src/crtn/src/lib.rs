// ! crti
#![no_std]
#![feature(linkage)]

use core::arch::global_asm;

#[cfg(target_arch = "x86")]
global_asm!(
    r#"
    .section .init
        // This happens after crti.o and gcc has inserted code
        // Pop the stack frame
        pop ebp
        ret

    .section .fini
        // This happens after crti.o and gcc has inserted code
        // Pop the stack frame
        pop ebp
        ret
"#
);

// https://wiki.osdev.org/Creating_a_C_Library#crtbegin.o.2C_crtend.o.2C_crti.o.2C_and_crtn.o
#[cfg(target_arch = "x86_64")]
global_asm!(
    r#"
    .section .init
        // This happens after crti.o and gcc has inserted code
        // Pop the stack frame
        pop rbp
        ret

    .section .fini
        // This happens after crti.o and gcc has inserted code
        // Pop the stack frame
        pop rbp
        ret
"#
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
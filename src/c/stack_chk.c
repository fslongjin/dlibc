#include <stdint.h>

uintptr_t __stack_chk_guard = 0xd048c37519fcadfe;

void abort(void) __attribute__((noreturn));

__attribute__((noreturn))
void __stack_chk_fail(void) {
    abort();
}
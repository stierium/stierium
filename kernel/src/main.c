#include "bootstub.h"
#include "gfx/fb.h"
#include "mm/pmm.h"

#ifdef ARCH_X86_64
#include "arch/x86_64/cpu/cpu.h"
#include "arch/x86_64/devs/serial.h"
#elif ARCH_AARCH64
#include "arch/aarch64/cpu/cpu.h"
#endif

bootctx_t *bootctx;

void kmain(bootctx_t *ctx)
{
    bootctx = ctx;
    serial_init();
    pmm_init();
       
    hcf();
}



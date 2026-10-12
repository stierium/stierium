#include "bootstub.h"
#include "fb.h"

#ifdef ARCH_X86_64
#include "arch/x86_64/cpu/cpu.h"
#include "arch/x86_64/devs/serial.h"
#elif ARCH_AARCH64
#include "arch/aarch64/cpu/cpu.h"
#endif

void fb_draw_rect(fb_t *fb, size_t x, size_t y, size_t width, size_t height, color_t color)
{
    color_t *line = fb->addr + x + y * (fb->pitch / 4);
    for (size_t i = 0; i < height; i++)
    {
        for (size_t j = 0; j < width; j++)
        {
            line[j] = color;
        }
        line += fb->pitch / 4;
    }
}

void kmain(bootctx_t *ctx)
{
    (void)ctx;
    
    fb_t *fb = &ctx->fbs.fbs[0];
    fb_draw_rect(fb, 50, 50, 25, 25, 0xFF0000);
    srprintf("%x\n", ctx->fbs.fbs);
    srputs("Hello, World!");
    hcf();
}



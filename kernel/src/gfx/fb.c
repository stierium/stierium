#include "bootstub.h"
#include "gfx/fb.h"

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



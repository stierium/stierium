#include <stddef.h>
#include <stdint.h>

#include "bootstub.h"
#include "fb.h"

static bootctx_t bootctx;
static void load_bootctx(void);

extern void kinit(bootctx_t *ctx);

#define LIMINE_API_REVISION 4
#include "limine.h"

__attribute__((used, section(".limine_requests")))
static volatile uint64_t limine_base_revision[] = LIMINE_BASE_REVISION(LIMINE_API_REVISION);

__attribute__((used, section(".limine_requests_start")))
static volatile uint64_t limine_requests_start_marker[] = LIMINE_REQUESTS_START_MARKER;

__attribute__((used, section(".limine_requests_end")))
static volatile uint64_t limine_requests_end_marker[] = LIMINE_REQUESTS_END_MARKER;

__attribute__((used, section(".limine_requests")))
static volatile struct limine_framebuffer_request framebuffer_request = {
    .id = LIMINE_FRAMEBUFFER_REQUEST_ID,
    .revision = LIMINE_API_REVISION
};

__attribute__((used, section(".limine_requests"))) 
static volatile struct limine_hhdm_request hhdm_request = {
    .id = LIMINE_HHDM_REQUEST_ID,
    .revision = LIMINE_API_REVISION
};

__attribute__((used, section(".limine_requests")))
static volatile struct limine_memmap_request memmap_request = {
    .id = LIMINE_MEMMAP_REQUEST_ID,
    .revision = LIMINE_API_REVISION
};

static void *mem_reserve(size_t size)
{
    for (size_t i = 0; i < memmap_request.response->entry_count; i++)
    {
        struct limine_memmap_entry *entry = memmap_request.response->entries[i];
        if (entry->type == LIMINE_MEMMAP_USABLE && entry->length >= size)
        {
            entry->length -= size;
            return (void*)(entry->base + entry->length + hhdm_request.response->offset);
        }
    }
    return NULL;
}

static void from_limine_fb(fb_t *fb, size_t idx)
{
    struct limine_framebuffer *lfb = framebuffer_request.response->framebuffers[idx];
    fb->addr = lfb->address;
    fb->width = lfb->width;
    fb->height = lfb->height;
    fb->bpp = lfb->bpp;
    fb->pitch = lfb->pitch;
    fb->memory_model = lfb->memory_model;
    fb->blue_mask_shift = lfb->blue_mask_shift;
    fb->red_mask_shift = lfb->red_mask_shift;
    fb->green_mask_shift = lfb->green_mask_shift;
    fb->red_mask_size = lfb->red_mask_size;
    fb->blue_mask_size = lfb->blue_mask_size;
    fb->green_mask_size = lfb->green_mask_size;
}

static void fbs_create(fbs_t *fbs)
{
    fbs->num_fbs = framebuffer_request.response->framebuffer_count;
    void *ptr = mem_reserve(sizeof(fb_t) * fbs->num_fbs);
    fbs->fbs = ptr;
    for (size_t i = 0; i < fbs->num_fbs; i++)
    {   
        from_limine_fb(&fbs->fbs[i], i);
    }
}

static int from_limine_memmap_type(int type) 
{
    switch (type) 
    {
        case LIMINE_MEMMAP_USABLE: return MEMMAP_USABLE;
        case LIMINE_MEMMAP_RESERVED: return MEMMAP_RESERVED;
        case LIMINE_MEMMAP_RESERVED_MAPPED: return MEMMAP_RESERVED_MAPPED;
        case LIMINE_MEMMAP_ACPI_RECLAIMABLE: return MEMMAP_ACPI_RECLAIMABLE;
        case LIMINE_MEMMAP_EXECUTABLE_AND_MODULES: return MEMMAP_EXECUTABLE_AND_MODULES;
        case LIMINE_MEMMAP_ACPI_NVS: return MEMMAP_ACPI_NVS;
        case LIMINE_MEMMAP_BAD_MEMORY: return MEMMAP_BAD_MEMORY;
        case LIMINE_MEMMAP_BOOTLOADER_RECLAIMABLE: return MEMMAP_BOOTLOADER_RECLAIMABLE;
        case LIMINE_MEMMAP_FRAMEBUFFER: return MEMMAP_FRAMEBUFFER;
        default: return MEMMAP_UNKNOWN;
    }
}

static void memmap_create(memmap_t *memmap)
{
    size_t num_entries = memmap_request.response->entry_count;
    size_t entries_size = sizeof(memmap_entry_t) * num_entries;
    void *entries = mem_reserve(entries_size);

    memmap->entries = entries;
    memmap->num_entries = num_entries;
    memmap->num_usable_pages = 0;
    for (size_t i = 0; i < num_entries; i++)
    {
        struct limine_memmap_entry *old = memmap_request.response->entries[i];
        memmap_entry_t *new = &memmap->entries[i];
        if (old->length == 0)
        {
            continue;
        }
        
        new->type = from_limine_memmap_type(old->type);
        new->base = old->base;
        new->len = old->length;

        if (new->type == MEMMAP_USABLE)
        {
            memmap->num_usable_pages += new->len / 4096;
        }
    }

}

static void load_bootctx(void)
{
    fbs_create(&bootctx.fbs);
    memmap_create(&bootctx.memmap);

    bootctx.hhdm_offset = hhdm_request.response->offset;
}

void kstart(void) 
{
    load_bootctx();
    kinit(&bootctx);   
}

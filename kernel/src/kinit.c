#include "bootstub.h"
#include "mm/pmm.h"

extern void kmain(bootctx_t *ctx);

static void *alloc(bootctx_t *bootctx, size_t size)
{
    for (size_t i = 0; i < bootctx->memmap.num_entries; i++)
    {
        memmap_entry_t *entry = &bootctx->memmap.entries[i];
        if (entry->type == MEMMAP_USABLE && entry->len >= size)
        {
            entry->len -= size;
            return (void*)(entry->base + entry->len + bootctx->hhdm_offset);
        }
    }
    return NULL;
}


void kinit(bootctx_t *ctx)
{
    ctx->pages = alloc(ctx, sizeof(pginfo_t) * ctx->memmap.num_usable_pages);
        
    size_t idx = 0;
    for (size_t i = 0; i < ctx->memmap.num_entries; i++)
    {
        memmap_entry_t *entry = &ctx->memmap.entries[i];
        if (entry->type == MEMMAP_USABLE)
        {
            size_t basepfn = entry->base / 4096;
            size_t pages = entry->len / 4096;
            for (size_t page = 0; page < pages; page++)
            {
                ctx->pages[idx++].pfn = basepfn + page;
            }
        }
    }

    ctx->num_pages = idx + 1;

    kmain(ctx);
}

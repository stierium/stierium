#include "arch/x86_64/devs/serial.h"
#include "bootstub.h"
#include "kernel.h"
#include <string.h>
#include "mm/pmm.h"

static size_t max_page = 0;
static size_t max_pfn = 0;
static pginfo_t *pfndb = NULL;

// zone_t *zones;

void pmm_init()
{
    for (size_t i = 0; i < bootctx->memmap.num_entries; i++)
    {
        memmap_entry_t *entry = &bootctx->memmap.entries[i];
        if (entry->type != MEMMAP_USABLE)
            continue;
        srprintf("base: %x; len: %d; type: %d;\n", entry->base, entry->len / 4096, entry->type);
    

    }
}


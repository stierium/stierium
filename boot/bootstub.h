#pragma once

#include "fb.h"
#include "libds/include/list.h"
enum
{
    MEMMAP_RESERVED,
    MEMMAP_ACPI_RECLAIMABLE,
    MEMMAP_EXECUTABLE_AND_MODULES,
    MEMMAP_ACPI_NVS,
    MEMMAP_BAD_MEMORY,
    MEMMAP_BOOTLOADER_RECLAIMABLE,
    MEMMAP_FRAMEBUFFER,
    MEMMAP_USABLE,
    MEMMAP_UNKNOWN,
    MEMMAP_RESERVED_MAPPED,
    MEMMAP_ENTRY_TYPE_COUNT,
};

typedef struct
{
    uintptr_t base;
    size_t len;
    int type;
} memmap_entry_t;

typedef struct {
    memmap_entry_t *entries;
    size_t num_entries;

    size_t num_usable_pages;
} memmap_t;

#if 0
#define MAX_ORDER 10

typedef struct {
    list_t freelist[MAX_ORDER];
} zone_t;
#endif

typedef struct {
    list_t list;
    size_t refcount;
    long flags;
    size_t pfn;
} pginfo_t;


typedef struct {
    pginfo_t *pages;
    size_t num_pages;

    memmap_t memmap;
    fbs_t fbs;
    size_t hhdm_offset;
} bootctx_t;


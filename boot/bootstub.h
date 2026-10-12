#pragma once

#include "fb.h"

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
    uintptr_t   base;
    size_t      len;
    int         type;
} memmap_entry_t;

typedef struct {
    memmap_entry_t *entries;
    size_t num_entries;
} memmap_t;

typedef struct {
    memmap_t memmap;
    fbs_t fbs;
} bootctx_t;


#pragma once

#include "bootstub.h"

#define PAGE_SIZE 4096

#define paddr_to_vaddr(paddr) (((char*)(paddr)) + ((bootctx)->hhdm_offset))

extern bootctx_t *bootctx;

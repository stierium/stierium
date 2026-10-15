#pragma once

#define align_up(address, alignment)    (((address) + (alignment - 1)) & ~((alignment) - 1))
#define align_down(address, alignment)  ((address) & ~((alignment) - 1))


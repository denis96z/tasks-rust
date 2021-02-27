#ifndef RALLOC_RALLOC_H
#define RALLOC_RALLOC_H

#include <stdint.h>
#include <stdlib.h>

typedef enum
{
    ERR_NONE        = 0,
    ERR_ALLOC       = 1,
    ERR_NO_URAND    = 2,
    ERR_URAND_OPEN  = 3,
    ERR_URAND_READ  = 4,
    ERR_URAND_CLOSE = 5,
}
error_t;

uint8_t *
ralloc(size_t n, error_t *err);

void
rfree(uint8_t *x);

#endif

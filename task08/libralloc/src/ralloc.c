#include "defos.h"
#include "ralloc.h"

#include <assert.h>

#ifdef __LINUX__
#include <fcntl.h>
#include <unistd.h>
#endif

uint8_t *
ralloc(size_t n, error_t *err)
{
    assert(n && err);

#ifdef __LINUX__
    uint8_t *b = malloc(n);
    if (!b)
    {
        *err = ERR_ALLOC;
        return NULL;
    }

    int fd = open("/dev/urandom", 0);
    if (fd == -1)
    {
        free(b);

        *err = ERR_URAND_OPEN;
        return NULL;
    }
    if (read(fd, b, n) == -1)
    {
        free(b);

        *err = ERR_URAND_READ;
        return NULL;
    }
    if (close(fd) == -1)
    {
        free(b);

        *err = ERR_URAND_CLOSE;
        return NULL;
    }

    *err = ERR_NONE;
    return b;
#endif

    *err = ERR_NO_URAND;
    return NULL;
}

void
rfree(uint8_t *x)
{
    free(x);
}

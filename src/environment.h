//
// Created by P4o1o on 14/05/2024.
//

#ifndef SSCRIPT_ENVIRONMENT_H
#define SSCRIPT_ENVIRONMENT_H

#include <stdint.h>
#include <stddef.h>
#include "math.h"

struct EnvElem{
    char *key;
    char *value;
    struct EnvElem *next;
};

struct Environment{
    struct EnvElem **content;
    size_t capacity;
};

uint64_t SipHash_2_4(uint64_t keytop, uint64_t keybottom, const char *message, size_t len);


#endif //SSCRIPT_ENVIRONMENT_H

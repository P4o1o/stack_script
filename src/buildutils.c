//
// Created by P4o1o on 09/05/2024.
//
#include <stdlib.h>
#include <string.h>
#include "buildutils.h"

// start with k = 1, i = 0 and t starts from 1 to len included, size(t) = size(a) + 1 (len = size(a))
size_t reorder_array_bst(const char* a[], char* t[], const size_t len, size_t k, size_t i) {
    if (k <= len) {
        i = reorder_array_bst(a, t, len, 2 * k, i);
        t[k] = malloc(strlen(a[i]) + 1);
        strcpy(t[k], a[i]);
        i += 1;
        i = reorder_array_bst(a, t, len, 2 * k + 1, i);
    }
    return i;
}

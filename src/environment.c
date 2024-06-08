//
// Created by P4o1o on 08/05/2024.
//

#include "environment.h"
#include "math.h"

#define SIPROUND \
        v0 += v1; \
        v1 = (v1 << 13) | (v1 >> 51); \
        v1 ^= v0; \
        v0 = (v0 << 32) | (v0 >> 32); \
        v2 += v3; \
        v3 = (v3 << 16) | (v3 >> 48);     \
        v3 ^= v2; \
        v2 += v1; \
        v1 = (v1 << 17) | (v0 >> 47); \
        v1 ^= v2; \
        v2 = (v2 << 32) | (v2 >> 32); \
        v0 += v3; \
        v3 = (v3 << 21) | (v0 >> 43); \
        v3 ^= v0;

uint64_t SipHash_2_4(const uint64_t keytop, const uint64_t keybottom, const char *message, const size_t len){
    uint64_t v0 = keybottom ^ 0x736f6d6570736575ULL;
    uint64_t v1 = keytop ^ 0x646f72616e646f6dULL;
    uint64_t v2 = keybottom ^ 0x6c7967656e657261ULL;
    uint64_t v3 = keytop ^ 0x7465646279746573ULL;
    size_t lenlast = len & 7;
    char* end = message + (len - lenlast);
    while (message != end) {
        uint64_t mess_i = ((uint64_t)message[7] << 56)
            | ((uint64_t)message[6] << 48)
            | ((uint64_t)message[5] << 40)
            | ((uint64_t)message[4] << 32)
            | ((uint64_t)message[3] << 24)
            | ((uint64_t)message[2] << 16)
            | ((uint64_t)message[1] << 8)
            | ((uint64_t)message[0]);
        v3 ^= mess_i;
        SIPROUND
        SIPROUND
        v0 ^= mess_i;
        message = message + 8;
    }
    uint64_t mess_last = ((uint64_t)(len & 255)) << 56;
    switch (lenlast) {
    case 7:
        mess_last |= ((uint64_t)message[6] << 48);
    case 6:
        mess_last |= ((uint64_t)message[5] << 40);
    case 5:
        mess_last |= ((uint64_t)message[4] << 32);
    case 4:
        mess_last |= ((uint64_t)message[3] << 24);
    case 3:
        mess_last |= ((uint64_t)message[2] << 16);
    case 2:
        mess_last |= ((uint64_t)message[1] << 8);
    case 1:
        mess_last |= ((uint64_t)message[0]);
        break;
    case 0:
        break;
    }
    v3 ^= mess_last;
    SIPROUND
    SIPROUND
    v0 ^= mess_last;
    // Finalization
    v2 ^= 0xff;
    SIPROUND
    SIPROUND
    SIPROUND
    SIPROUND
    return v0 ^ v1 ^ v2 ^ v3;
}
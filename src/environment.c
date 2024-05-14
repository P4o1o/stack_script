//
// Created by P4o1o on 08/05/2024.
//

#include "environment.h"
#include "math.h"

#define SIPROUND \
    do{ \
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
              v3 ^= v0; \
    }while(0);

uint64_t SipHash_2_4(const uint64_t keytop, const uint64_t keybottom, const char *message, const size_t len){
    uint64_t v0 = keybottom ^ 0x736f6d6570736575ULL;
    uint64_t v1 = keytop ^ 0x646f72616e646f6dULL;
    uint64_t v2 = keybottom ^ 0x6c7967656e657261ULL;
    uint64_t v3 = keytop ^ 0x7465646279746573ULL;
    size_t wordsize = (size_t) ceil(((double)len + 1.0)/ 8.0);
    for(int i = 0; i < wordsize - 1; i++){
        uint64_t mess_i = ((uint64_t)message[i] << 56)
                          | ((uint64_t)message[i + 1] << 48)
                          | ((uint64_t)message[i + 2] << 40)
                          | ((uint64_t)message[i + 3] << 32)
                          | ((uint64_t)message[i + 4] << 24)
                          | ((uint64_t)message[i + 5] << 16)
                          | ((uint64_t)message[i + 6] << 8)
                          | ((uint64_t)message[i + 7]);
        v3 ^= mess_i;
        SIPROUND
        SIPROUND
        v0 ^= mess_i;
    }
    uint64_t mess_last = ((uint64_t) (len % 256)) << 56;
    for(int index = 0; index < len % 8; index++) {
        mess_last |= ((uint64_t)message[index] << (index * 8));
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
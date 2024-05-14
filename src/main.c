#include <stdio.h>
#include "interpreter.h"

int main() {
    struct ProgramState state = init_PrgState(256, 256);
    struct ExceptionHandler *try_buf = malloc(sizeof(struct ExceptionHandler));
    TRY(try_buf){
        execute(&state, "[size swap(size 1 -) [+] loop(size 2 >) swap /] define(mean)", try_buf);
        execute(&state, "[[swap(size 1 -) 1 + dup] [< swap swap(size 1 -) swap] swap3 quote compose swap quote swap2 compose compose loop swap(size 1 -) drop] define(fill)", try_buf);
        for(int i = 0; i < 10; i++){
            execute(&state, "0 10 30 fill mean print clear", try_buf);
        }
    }CATCHALL{
        printf("Exception number %d\n", try_buf->exit_value);
    }
    free(try_buf);
    free_PrgState(&state);
    return 0;
}

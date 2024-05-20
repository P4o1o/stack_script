#include <stdio.h>
#include "interpreter.h"
#include "buildutils.h"
#define BUFFERSIZE 256

int main() {
    struct ProgramState state = init_PrgState(256, 256);
    struct ExceptionHandler* try_buf = malloc(sizeof(struct ExceptionHandler));
    char bufferin[BUFFERSIZE];
    while(1){
        printf(">");
        fflush(stdout);
        TRY(try_buf) {
            if (fgets(bufferin, BUFFERSIZE, stdin) >= 0) {
                execute(&state, bufferin, try_buf);
            }
        }CATCH(try_buf, ProgramExit) {
            break;
        }CATCHALL{
            printf("Exception number %d\n", try_buf->exit_value);
        }
    }
    free(try_buf);
    free_PrgState(&state);
    return 0;
}

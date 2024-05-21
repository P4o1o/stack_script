#include <stdio.h>
#include "interpreter.h"
#include "buildutils.h"
#define BUFFERSIZE 256

#define MIN(a, b) ((a) < (b) ? (a) : (b))

int print_usage() {
    printf("\nUsage: sscript [-options] [File to load before start]\tfields are optional\n\n" \
        "doucumentation available at https://p4o1o.github.io/stack_script/\n" \
        "options:\n" \
        "- v\t print the last element of the stack after every input.\n" \
        "- v<size>\t print the last <size> element of the stack after every input.\n" \
        "- h\t print this message.\n\n");
    return -1;
}

int main(int argc, char *argv[]) {
    struct ProgramState state = init_PrgState(256, 256);
    struct ExceptionHandler* try_buf = malloc(sizeof(struct ExceptionHandler));
    if (try_buf == NULL)
        return -1;
    size_t size = 0;
    char* file_toload;
    if (argc > 1) {
        if (argv[1][0] == '-') {
            size_t i = 1;
            while (argv[1][i] != '\0') {
                if (argv[1][i] == 'v') {
                    i++;
                    if ('0' <= argv[1][i] && argv[1][i] <= '9') {
                        size = 0;
                        while ('0' <= argv[1][i] && argv[1][i] <= '9') {
                            size = size * 10 + ((size_t)argv[1][i] - '0');
                            i++;
                        }
                    }
                    else {
                        size = 1;
                    }
                }
                else if (argv[1][i] == 'h') {
                    print_usage();
                    return 0;
                }
                else {
                    print_usage();
                    return 1;
                }
            }

            if (argc == 3) {
                file_toload = argv[2];
            }
            else if (argc == 2) {
                goto StartShell;
            }
            else {
                print_usage();
                return 1;
            }
        }
        else {
            if (argc > 2) {
                print_usage();
                return 1;
            }
            file_toload = argv[1];
        }
        TRY(try_buf) {
            brop_load(&state, file_toload, strlen(argv[1]), try_buf);
        }CATCHALL{
            printf("Exception number %d while loading file %s\n", try_buf->exit_value, argv[1]);
            return;
        }
    }
StartShell:
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
        size_t elem_to_print = MIN(size, state.stack.next);
        print_stack(&state, size);
    }
    free(try_buf);
    free_PrgState(&state);
    return 0;
}

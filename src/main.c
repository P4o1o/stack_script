#include <stdio.h>
#include "interpreter.h"
#include "buildutils.h"
#define BUFFERSIZE 256

#define MIN(a, b) ((a) < (b) ? (a) : (b))

void print_usage() {
    printf("\nUsage:\n\tsscript [-options] [File to load before the shell starts]\n" \
        "\targs are optionals:\n\n" \
        "doucumentation available at https://p4o1o.github.io/stack_script/\n\n" \
        "options must be in this format: -v, -sv2m -sv, ... (the order doesen't matter)\n" \
        "options available:\n" \
        "\t-v\t print the last element of the stack after every input.\n" \
        "\t-v<size>\t print the last <size> element of the stack after every input.\n" \
        "\t-h\t print this message.\n" \
        "\t-m load the math library before the shell starts\n" \
        "\t-s load the stack operations library before the shell starts\n\n"
    );
}

void load_file(struct ProgramState* state, char* filepath) {
    struct ExceptionHandler* try_buf = malloc(sizeof(struct ExceptionHandler));
    TRY(try_buf) {
        brop_load(state, filepath, strlen(filepath), try_buf);
        free(try_buf);
        free_PrgState(state);
    }CATCHALL{
        printf("Exception number %d while loading file %s\n", try_buf->exit_value, filepath);
        free(try_buf);
        free_PrgState(state);
        exit(try_buf->exit_value);
    }
}


int main(int argc, char *argv[]) {
    struct ProgramState state = init_PrgState(256, 256);
    struct ExceptionHandler* try_buf = malloc(sizeof(struct ExceptionHandler));
    if (try_buf == NULL)
        return -1;
    size_t size = 0;
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
                else if (argv[1][i] == 'm') {
                    load_file(&state, "math.sksp");
                }
                else if (argv[1][i] == 's') {
                    load_file(&state, "stackop.sksp");
                }
                else {
                    print_usage();
                    return 1;
                }
            }
            if (argc >  2) {
                load_file(&state, argv[2]);
            }
        }
        else {
            if (argc > 2) {
                print_usage();
                return -1;
            }
            load_file(&state, argv[1]);
        }
    }
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

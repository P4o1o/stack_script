//
// Created by P4o1o on 06/05/2024.
//

#ifndef SSCRIPT_PROGRAMSTATE_H
#define SSCRIPT_PROGRAMSTATE_H
#include <stdlib.h>
#include "stack.h"
#include "environment.h"
#include <setjmp.h>

struct ExceptionHandler{
    jmp_buf buffer;
    uint32_t exit_value;
    char *not_exec;
};

#define TRY(EXCHANDLER) if ((EXCHANDLER->exit_value = setjmp(EXCHANDLER->buffer)) == 0)
#define CATCH(EXCHANDLER, EXCNUM) else if (EXCHANDLER->exit_value == EXCNUM)
#define CATCHALL else
#define RAISE(EXCHANDLER, EXCNUM) longjmp(EXCHANDLER->buffer, EXCNUM)

#define ProgramOk 0
#define ParenthesisError 1
#define InvalidChar 2
#define InvalidInstruction 3
#define StackUnderflow 4
#define ValueError 5
#define InvalidOperands 6
#define ProgramPanic 7
#define ProgramExit 8
#define IOError 9
#define FileNotFound 10
#define FileNotCreatable 11

void print_Exception(struct ExceptionHandler* exc);

struct ProgramState{
    struct Stack stack;
    struct Environment env;
};

typedef void (*operations)(struct ProgramState*, struct ExceptionHandler*);

typedef void (*br_operations)(struct ProgramState*, char*, size_t, struct ExceptionHandler*);

typedef void (*num_operations)(struct ProgramState*, size_t, struct ExceptionHandler*);

struct ProgramState init_PrgState(size_t stack_capacity, size_t env_capacity);
void free_PrgState(struct ProgramState *inter);

#endif //SSCRIPT_PROGRAMSTATE_H

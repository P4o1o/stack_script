//
// Created by P4o1o on 06/05/2024.
//

#ifndef SSCRIPT_PROGRAMSTATE_H
#define SSCRIPT_PROGRAMSTATE_H
#include <stdlib.h>
#include "stack.h"
#include "environment.h"
#include <setjmp.h>

struct OpenMemMap{
    char *openmem;
    struct OpenMemMap *next;
};

struct ExceptionHandler{
    jmp_buf buffer;
    uint32_t exit_value;
    char **not_exec;
    size_t bt_size;
    size_t bt_capacity;
    struct OpenMemMap **openmemmap;
};
#define OM_VEC_CAPACITY 32
#define BT_VEC_CAPACITY 32

#define TRY(EXCHANDLER) if ((EXCHANDLER->exit_value = setjmp(EXCHANDLER->buffer)) == 0)
#define CATCH(EXCHANDLER, EXCNUM) else if (EXCHANDLER->exit_value == EXCNUM)
#define CATCHALL else
#define RAISE(EXCHANDLER, EXCNUM) longjmp(EXCHANDLER->buffer, EXCNUM)

#define ProgramOk 0
#define ProgramExit 1
#define InvalidChar 2
#define InvalidInstruction 3
#define StackUnderflow 4
#define ValueError 5
#define InvalidOperands 6
#define ProgramPanic 7
#define IOError 8
#define FileNotFound 9
#define FileNotCreatable 10
#define RoundParenthesisError 11
#define SquaredParenthesisError 12
#define StringQuotingError 13

void print_Exception(struct ExceptionHandler* exc);

struct ProgramState{
    struct Stack stack;
    struct Environment env;
};

typedef void (*operations)(struct ProgramState*, struct ExceptionHandler*);

typedef void (*br_operations)(struct ProgramState*, char*, size_t, struct ExceptionHandler*);

typedef void (*num_operations)(struct ProgramState*, size_t, struct ExceptionHandler*);

struct ProgramState init_PrgState(size_t stack_capacity, size_t env_capacity);
void reload_Exceptionhandler(struct ExceptionHandler *try_buf);
void free_PrgState(struct ProgramState *inter);

struct ExceptionHandler *init_ExceptionHandler();
void free_ExceptionHandler(struct ExceptionHandler *exh);

#endif //SSCRIPT_PROGRAMSTATE_H

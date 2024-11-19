//
// Created by P4o1o on 14/05/2024.
//

#ifndef STACK_OP_H
#define STACK_OP_H
#include "programstate.h"

void op_none(struct ProgramState *state, struct ExceptionHandler *jbuff);

void op_dup(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_swap(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_roll(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_top(struct ProgramState* state, struct ExceptionHandler* jbuff);

void op_drop(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_clear(struct ProgramState* state, struct ExceptionHandler* jbuff);

void op_quote(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_compose(struct ProgramState* state, struct ExceptionHandler* jbuff);

void op_exit(struct ProgramState *state, struct ExceptionHandler *jbuff);

void op_print(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_printall(struct ProgramState *state, struct ExceptionHandler *jbuff);

void op_nop(struct ProgramState* state, struct ExceptionHandler* jbuff);

void numop_dup(struct ProgramState *state, size_t num, struct ExceptionHandler *jbuff);
void numop_swap(struct ProgramState *state, size_t num, struct ExceptionHandler *jbuff);
void numop_dig(struct ProgramState* state, size_t num, struct ExceptionHandler *jbuff);

#endif
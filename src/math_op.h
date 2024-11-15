//
// Created by P4o1o on 14/05/2024.
//

#ifndef MATH_OP_H
#define MATH_OP_H
#include "programstate.h"

void op_size(struct ProgramState* state, struct ExceptionHandler* jbuff);

void op_int(struct ProgramState* state, struct ExceptionHandler* jbuff);

void op_sum(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_mul(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_sub(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_div(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_mod(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_pow(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_sqrt(struct ProgramState* state, struct ExceptionHandler* jbuff);

#endif
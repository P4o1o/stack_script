//
// Created by P4o1o on 14/05/2024.
//

#ifndef MATH_OP_H
#define MATH_OP_H
#include "programstate.h"
#include <math.h>

void op_size(struct ProgramState* state, struct ExceptionHandler* jbuff);

void op_int(struct ProgramState* state, struct ExceptionHandler* jbuff);

void op_sum(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_mul(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_sub(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_div(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_mod(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_pow(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_sqrt(struct ProgramState* state, struct ExceptionHandler* jbuff);

void op_sin(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_cos(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_tan(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_arccos(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_arcsin(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_arctan(struct ProgramState* state, struct ExceptionHandler* jbuff);

void op_sinh(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_cosh(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_tanh(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_arcsinh(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_arccosh(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_arctanh(struct ProgramState* state, struct ExceptionHandler* jbuff);

#endif
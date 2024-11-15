//
// Created by P4o1o on 14/05/2024.
//

#ifndef BOOL_OP_H
#define BOOL_OP_H
#include "programstate.h"

void op_true(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_false(struct ProgramState* state, struct ExceptionHandler* jbuff);

void op_empty(struct ProgramState* state, struct ExceptionHandler* jbuff);

void op_not(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_and(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_or(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_xor(struct ProgramState *state, struct ExceptionHandler *jbuff);

void op_equal(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_notequal(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_greather(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_greathereq(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_lower(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_lowereq(struct ProgramState* state, struct ExceptionHandler* jbuff);

#endif
//
// Created by P4o1o on 14/05/2024.
//

#ifndef TYPES_OP_H
#define TYPES_OP_H
#include "programstate.h"

void op_INSTR(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_INT(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_FLOAT(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_STR(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_BOOL(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_TYPE(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_NONE(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_STACK(struct ProgramState *state, struct ExceptionHandler *jbuff);

void op_type(struct ProgramState *state, struct ExceptionHandler *jbuff); // NON-DESTRUCTIVE


#endif
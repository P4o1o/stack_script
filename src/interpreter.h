//
// Created by P4o1o on 14/05/2024.
//

#ifndef SSCRIPT_INTERPRETER_H
#define SSCRIPT_INTERPRETER_H
#include "programstate.h"
#include "stack.h"
#include "environment.h"



extern const char *BOOL[];
extern const char *INSTRUCTIONS[];
extern const char *BRACKETS_INSTR[];


struct OperationElem {
	char* key;
	operations op;
	struct OperationElem* next;
};

struct BrOperationElem {
	char* key;
	br_operations op;
	struct BrOperationElem* next;
};

struct Builtins {
	num_operations num_op[3];
	struct OperationElem** op_map;
	struct BrOperationElem** brop_map;
};



void op_equal(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_notequal(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_greather(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_greathereq(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_lower(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_lowereq(struct ProgramState* state, struct ExceptionHandler* jbuff);

void op_dup(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_swap(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_drop(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_size(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_empty(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_clear(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_apply(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_quote(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_roll(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_dip(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_compose(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_top(struct ProgramState* state, struct ExceptionHandler* jbuff);

void op_int(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_try(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_print(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_printall(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_exit(struct ProgramState *state, struct ExceptionHandler *jbuff);

void op_sum(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_mul(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_sub(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_div(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_mod(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_pow(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_sqrt(struct ProgramState* state, struct ExceptionHandler* jbuff);


void op_not(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_and(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_or(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_xor(struct ProgramState *state, struct ExceptionHandler *jbuff);

void op_if(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_loop(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_nop(struct ProgramState* state, struct ExceptionHandler* jbuff);



void numop_dup(struct ProgramState *state, size_t num, struct ExceptionHandler *jbuff);
void numop_swap(struct ProgramState *state, size_t num, struct ExceptionHandler *jbuff);
void numop_dig(struct ProgramState* state, size_t num, struct ExceptionHandler *jbuff);



void brop_if(struct ProgramState *state, char *cond, size_t condlen, struct ExceptionHandler *jbuff);
void brop_loop(struct ProgramState *state, char *cond, size_t condlen, struct ExceptionHandler *jbuff);
void brop_swap(struct ProgramState *state, char *comand, size_t clen, struct ExceptionHandler *jbuff);
void brop_dup(struct ProgramState *state, char *comand, size_t clen, struct ExceptionHandler *jbuff);
void brop_save(struct ProgramState *state, char *filename, size_t fnlen, struct ExceptionHandler *jbuff);
void brop_load(struct ProgramState *state, char *filename, size_t fnlen, struct ExceptionHandler *jbuff);
void brop_isdef(struct ProgramState *state, char *funcname, size_t fnlen, struct ExceptionHandler *jbuff);
void brop_define(struct ProgramState *state, char *funcname, size_t fnlen, struct ExceptionHandler *jbuff);
void brop_delete(struct ProgramState *state, char *funcname, size_t fnlen, struct ExceptionHandler *jbuff);



void execute_instr(struct ProgramState *state, char *instr, size_t instrlen, struct ExceptionHandler *jbuff);
void parse_script(struct ProgramState *state, char *comands, size_t clen, struct ExceptionHandler *jbuff);
void execute(struct ProgramState *state, char *comands, struct ExceptionHandler *jbuff);
void print_stack(struct ProgramState* state, size_t num_elem);

#endif //SSCRIPT_INTERPRETER_H

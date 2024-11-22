//
// Created by P4o1o on 14/05/2024.
//

#ifndef SSCRIPT_INTERPRETER_H
#define SSCRIPT_INTERPRETER_H
#include "programstate.h"
#include "math_op.h"
#include "bool_op.h"
#include "types_op.h"
#include "stack_op.h"
#include <omp.h>

extern char *INSTRUCTIONS[];
extern char *BRACKETS_INSTR[];


struct OperationElem {
	char* key;
	operations op;
	struct OperationElem* next;
	size_t key_len;
};

struct BrOperationElem {
	char* key;
	br_operations brop;
	struct BrOperationElem* next;
	size_t key_len;
};

struct NumOperationElem {
	char* key;
	num_operations numop;
	struct NumOperationElem* next;
	size_t key_len;
};

struct Builtins {
	struct OperationElem** op_map;
	struct BrOperationElem** brop_map;
	struct NumOperationElem** numop_map;
};

enum TokenType{
    InstrToken,
    StringToken,
    BrInstrToken,
    StackToken,
    GenericToken,
	NumericToken,
};

struct Token{
    char *instr;
    enum TokenType type;
    size_t instrlen;
    size_t indx;
};

extern struct Builtins builtins;
int init_builtins();
void free_builtins();

void op_stack(struct ProgramState *state, struct ExceptionHandler *jbuff);

void brop_save(struct ProgramState *state, char *filename, size_t fnlen, struct ExceptionHandler *jbuff);
void brop_load(struct ProgramState *state, char *filename, size_t fnlen, struct ExceptionHandler *jbuff);

void op_apply(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_dip(struct ProgramState* state, struct ExceptionHandler* jbuff);

void op_compress(struct ProgramState* state, struct ExceptionHandler* jbuff);

void op_push(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_pop(struct ProgramState* state, struct ExceptionHandler* jbuff);
void op_inject(struct ProgramState* state, struct ExceptionHandler* jbuff);

void op_try(struct ProgramState* state, struct ExceptionHandler* jbuff);

void op_if(struct ProgramState *state, struct ExceptionHandler *jbuff);
void op_loop(struct ProgramState *state, struct ExceptionHandler *jbuff);

void op_split(struct ProgramState* state, struct ExceptionHandler* jbuff);

void numop_inject(struct ProgramState *state, size_t num, struct ExceptionHandler *jbuff);
void numop_pinject(struct ProgramState *state, size_t num, struct ExceptionHandler *jbuff);

void brop_dig(struct ProgramState* state, char* number, size_t numberlen, struct ExceptionHandler* jbuff);
void brop_swap(struct ProgramState *state, char *comand, size_t clen, struct ExceptionHandler *jbuff);
void brop_dup(struct ProgramState *state, char *comand, size_t clen, struct ExceptionHandler *jbuff);

void brop_if(struct ProgramState *state, char *cond, size_t condlen, struct ExceptionHandler *jbuff);
void brop_loop(struct ProgramState *state, char *cond, size_t condlen, struct ExceptionHandler *jbuff);
void brop_times(struct ProgramState* state, char* number, size_t numberlen, struct ExceptionHandler* jbuff);

void brop_split(struct ProgramState *state, char *comand, size_t clen, struct ExceptionHandler *jbuff);
void brop_compose(struct ProgramState *state, char *comand, size_t clen, struct ExceptionHandler *jbuff);

void brop_isdef(struct ProgramState *state, char *funcname, size_t fnlen, struct ExceptionHandler *jbuff);
void brop_define(struct ProgramState *state, char *funcname, size_t fnlen, struct ExceptionHandler *jbuff);
void brop_delete(struct ProgramState *state, char *funcname, size_t fnlen, struct ExceptionHandler *jbuff);

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void execute_instr(struct ProgramState *state, struct Token *token, struct ExceptionHandler *jbuff);
void parse_script(struct ProgramState *state, char *comands, size_t clen, struct ExceptionHandler *jbuff);
void execute(struct ProgramState *state, char *comands, struct ExceptionHandler *jbuff);
void print_stack(struct ProgramState* state, size_t num_elem);

#endif //SSCRIPT_INTERPRETER_H

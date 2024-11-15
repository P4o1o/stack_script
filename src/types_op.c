#include "types_op.h"


void op_type(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if (state->stack->next == 0)
        RAISE(jbuff, StackUnderflow);
    struct StackElem elem;
    elem.type = Type;
    elem.val.ival = state->stack->content[state->stack->next - 1].type;
    push_Stack(state->stack, elem, jbuff);
}

void op_INSTR(struct ProgramState *state, struct ExceptionHandler *jbuff){
    struct StackElem elem;
    elem.type = Type;
    elem.val.ival = Instruction;
    push_Stack(state->stack, elem, jbuff);
}

void op_INT(struct ProgramState *state, struct ExceptionHandler *jbuff){
    struct StackElem elem;
    elem.type = Type;
    elem.val.ival = Integer;
    push_Stack(state->stack, elem, jbuff);
}

void op_FLOAT(struct ProgramState *state, struct ExceptionHandler *jbuff){
    struct StackElem elem;
    elem.type = Type;
    elem.val.ival = Floating;
    push_Stack(state->stack, elem, jbuff);
}

void op_STR(struct ProgramState *state, struct ExceptionHandler *jbuff){
    struct StackElem elem;
    elem.type = Type;
    elem.val.ival = String;
    push_Stack(state->stack, elem, jbuff);
}

void op_BOOL(struct ProgramState *state, struct ExceptionHandler *jbuff){
    struct StackElem elem;
    elem.type = Type;
    elem.val.ival = Boolean;
    push_Stack(state->stack, elem, jbuff);
}

void op_TYPE(struct ProgramState *state, struct ExceptionHandler *jbuff){
    struct StackElem elem;
    elem.type = Type;
    elem.val.ival = Type;
    push_Stack(state->stack, elem, jbuff);
}

void op_NONE(struct ProgramState *state, struct ExceptionHandler *jbuff){
    struct StackElem elem;
    elem.type = Type;
    elem.val.ival = None;
    push_Stack(state->stack, elem, jbuff);
}

void op_STACK(struct ProgramState *state, struct ExceptionHandler *jbuff){
    struct StackElem elem;
    elem.type = Type;
    elem.val.ival = InnerStack;
    push_Stack(state->stack, elem, jbuff);
}

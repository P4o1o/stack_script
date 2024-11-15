#include "stack_op.h"

void copy_Stack(struct Stack *dest, struct Stack *src, struct ExceptionHandler *jbuff){
    dest->capacity = src->capacity;
    dest->next = src->next;
    dest->content = malloc(sizeof(struct StackElem) * src->capacity);
    if(dest->content == NULL)
        RAISE(jbuff, ProgramPanic);
    for(size_t i = 0; i < src->next; i++){
        dest->content[i].type = src->content[i].type;
        switch(src->content[i].type){
            case String:
            case Instruction:
                dest->content[i].val.instr = malloc(strlen(src->content[i].val.instr) + 1);
                if (dest->content[i].val.instr == NULL)
                    RAISE(jbuff, ProgramPanic);
                memcpy(dest->content[i].val.instr, src->content[i].val.instr, strlen(src->content[i].val.instr) + 1);
                break;
            case Type:
            case Boolean:
            case Integer:
                dest->content[i].val.ival == src->content[i].val.ival;
                break;
            case Floating:
                dest->content[i].val.fval == src->content[i].val.fval;
            case None:
                dest->content[i].val.ival == 0;
                break;
            case InnerStack:
                copy_Stack(dest->content[i].val.stack, src->content[i].val.stack, jbuff);
                break;
            default:
                UNREACHABLE;
        }
    }
}

void op_dup(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack->next == 0)
        RAISE(jbuff, StackUnderflow);
    struct StackElem copy;
    copy.type = state->stack->content[state->stack->next - 1].type;
    if (copy.type == Instruction || copy.type == String) {
        size_t srclen = strlen(state->stack->content[state->stack->next - 1].val.instr) + 1;
        copy.val.instr = malloc(srclen);
        if (copy.val.instr == NULL)
            RAISE(jbuff, ProgramPanic);
        memcpy(copy.val.instr, state->stack->content[state->stack->next - 1].val.instr, srclen);
    }else if(copy.type == InnerStack){
        copy.val.stack = malloc(sizeof(struct Stack));
        if(copy.val.stack == NULL)
            RAISE(jbuff, ProgramPanic);
        copy_Stack(copy.val.stack, state->stack->content[state->stack->next - 1].val.stack, jbuff);
    }else {
        copy.val = state->stack->content[state->stack->next - 1].val;
    }
    push_Stack(state->stack, copy, jbuff);
}

void op_top(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack->next == 0)
        RAISE(jbuff, StackUnderflow);
    struct StackElem copy;
    copy.type = state->stack->content[0].type;
    copy.val = state->stack->content[0].val;
    push_Stack(state->stack, copy, jbuff);
}

void op_swap(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack->next < 2)
        RAISE(jbuff, StackUnderflow);
    size_t index1 = state->stack->next - 1;
    size_t index2 = index1 - 1;
    struct StackElem temp;
    temp = state->stack->content[index1];
    state->stack->content[index1] = state->stack->content[index2];
    state->stack->content[index2] = temp;
}

void op_drop(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack->next == 0)
        RAISE(jbuff, StackUnderflow);
    state->stack->next -= 1;
    if(state->stack->content[state->stack->next].type == Instruction || state->stack->content[state->stack->next].type == String)
        free(state->stack->content[state->stack->next].val.instr);
    else if(state->stack->content[state->stack->next].type == InnerStack){
        free_Stack(state->stack->content[state->stack->next].val.stack);
    }
}

void op_clear(struct ProgramState *state, struct ExceptionHandler *jbuff){
    for(size_t i = 0; i < state->stack->next; i++){
        if(state->stack->content[i].type == Instruction || state->stack->content[i].type == String)
            free(state->stack->content[i].val.instr);
        else if(state->stack->content[i].type == InnerStack)
            free_Stack(state->stack->content[i].val.stack);
    }
    state->stack->next = 0;
}


void op_roll(struct ProgramState* state, struct ExceptionHandler* jbuff){
    if (state->stack->next == 0) {
        return;
    }
    struct StackElem temp = state->stack->content[state->stack->next - 1];
    for (size_t i = state->stack->next - 1; i > 0 ; i--) {
        state->stack->content[i] = state->stack->content[i - 1];
    }
    state->stack->content[0] = temp;
}

void numop_dig(struct ProgramState* state, size_t num, struct ExceptionHandler* jbuff){
    if (state->stack->next <= num) {
        RAISE(jbuff, StackUnderflow);
    }
    size_t index = state->stack->next - 1;
    size_t indextar = state->stack->next - 1 - num;
    struct StackElem temp = state->stack->content[indextar];
    for (size_t i = indextar; i < index; i++) {
        state->stack->content[i] = state->stack->content[i + 1];
    }
    state->stack->content[index] = temp;
}

void numop_dup(struct ProgramState *state, size_t num, struct ExceptionHandler *jbuff){
    if(num >= state->stack->next)
        RAISE(jbuff, StackUnderflow);
    struct StackElem copy;
    size_t index = state->stack->next - 1 - num;
    copy.type = state->stack->content[index].type;
    if (copy.type == Instruction || copy.type == String) {
        size_t srclen = strlen(state->stack->content[index].val.instr) + 1;
        copy.val.instr = malloc(srclen);
        if (copy.val.instr == NULL)
            RAISE(jbuff, ProgramPanic);
        memcpy(copy.val.instr, state->stack->content[index].val.instr, srclen);
    }else if(copy.type == InnerStack){
        copy.val.stack = malloc(sizeof(struct Stack));
        if(copy.val.stack == NULL)
            RAISE(jbuff, ProgramPanic);
        copy_Stack(copy.val.stack, state->stack->content[state->stack->next - 1].val.stack, jbuff);
    }else {
        copy.val = state->stack->content[index].val;
    }
    push_Stack(state->stack, copy, jbuff);
}

void numop_swap(struct ProgramState *state, size_t num, struct ExceptionHandler *jbuff){
    if(num >= state->stack->next)
        RAISE(jbuff, StackUnderflow);
    size_t index1 = state->stack->next - 1;
    size_t index2 = index1 - num;
    struct StackElem temp;
    temp = state->stack->content[index1];
    state->stack->content[index1] = state->stack->content[index2];
    state->stack->content[index2] = temp;
}

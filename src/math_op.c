#include "math_op.h"

void op_size(struct ProgramState *state, struct ExceptionHandler *jbuff){
    struct StackElem siz;
    siz.type = Integer;
    siz.val.ival = (int64_t)state->stack->next;
    push_Stack(state->stack, siz, jbuff);
}

void op_int(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack->next == 0)
        RAISE(jbuff, StackUnderflow);
    size_t resindex = state->stack->next - 1;
    if(state->stack->content[resindex].type == Floating){
        int64_t temp = (int64_t) state->stack->content[resindex].val.fval;
        state->stack->content[resindex].type = Integer;
        state->stack->content[resindex].val.ival = temp;
    }else if(state->stack->content[resindex].type != Integer){
        RAISE(jbuff, InvalidOperands);
    }
}

void op_sum(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack->next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack->next -= 1;
    size_t resindex = state->stack->next - 1;
    if(state->stack->content[resindex].type == Floating){
        if(state->stack->content[state->stack->next].type == Floating){
            state->stack->content[resindex].val.fval = state->stack->content[resindex].val.fval + state->stack->content[state->stack->next].val.fval;
        }else if(state->stack->content[state->stack->next].type == Integer) {
            state->stack->content[resindex].val.fval = state->stack->content[resindex].val.fval + (double) state->stack->content[state->stack->next].val.ival;
        }else{
            state->stack->next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else if(state->stack->content[resindex].type == Integer){
        if(state->stack->content[state->stack->next].type == Floating){
            state->stack->content[resindex].type = Floating;
            state->stack->content[resindex].val.fval = (double) state->stack->content[resindex].val.ival + state->stack->content[state->stack->next].val.fval;
        }else if(state->stack->content[state->stack->next].type == Integer) {
            state->stack->content[resindex].val.ival = state->stack->content[resindex].val.ival + state->stack->content[state->stack->next].val.ival;
        }else{
            state->stack->next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else{
        state->stack->next += 1;
        RAISE(jbuff, InvalidOperands);
    }
}

void op_sub(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack->next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack->next -= 1;
    size_t resindex = state->stack->next - 1;
    if(state->stack->content[resindex].type == Floating){
        if(state->stack->content[state->stack->next].type == Floating){
            state->stack->content[resindex].val.fval = state->stack->content[resindex].val.fval - state->stack->content[state->stack->next].val.fval;
        }else if(state->stack->content[state->stack->next].type == Integer) {
            state->stack->content[resindex].val.fval = state->stack->content[resindex].val.fval - (double) state->stack->content[state->stack->next].val.ival;
        }else{
            state->stack->next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else if(state->stack->content[resindex].type == Integer){
        if(state->stack->content[state->stack->next].type == Floating){
            state->stack->content[resindex].type = Floating;
            state->stack->content[resindex].val.fval = (double) state->stack->content[resindex].val.ival - state->stack->content[state->stack->next].val.fval;
        }else if(state->stack->content[state->stack->next].type == Integer) {
            state->stack->content[resindex].val.ival = state->stack->content[resindex].val.ival - state->stack->content[state->stack->next].val.ival;
        }else{
            state->stack->next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else{
        state->stack->next += 1;
        RAISE(jbuff, InvalidOperands);
    }
}

void op_mul(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack->next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack->next -= 1;
    size_t resindex = state->stack->next - 1;
    if(state->stack->content[resindex].type == Floating){
        if(state->stack->content[state->stack->next].type == Floating){
            state->stack->content[resindex].val.fval = state->stack->content[resindex].val.fval * state->stack->content[state->stack->next].val.fval;
        }else if(state->stack->content[state->stack->next].type == Integer) {
            state->stack->content[resindex].val.fval = state->stack->content[resindex].val.fval * (double) state->stack->content[state->stack->next].val.ival;
        }else{
            state->stack->next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else if(state->stack->content[resindex].type == Integer) {
        if(state->stack->content[state->stack->next].type == Floating) {
            state->stack->content[resindex].type = Floating;
            state->stack->content[resindex].val.fval =
                    (double) state->stack->content[resindex].val.ival * state->stack->content[state->stack->next].val.fval;
        }else if(state->stack->content[state->stack->next].type == Integer) {
            state->stack->content[resindex].val.ival =
                    state->stack->content[resindex].val.ival * state->stack->content[state->stack->next].val.ival;

        } else {
            state->stack->next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else{
        state->stack->next += 1;
        RAISE(jbuff, InvalidOperands);
    }
}

void op_sqrt(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack->next == 0)
        RAISE(jbuff, StackUnderflow);
    size_t resindex = state->stack->next - 1;
    if(state->stack->content[resindex].type == Floating){
        if(state->stack->content[resindex].val.fval == 0){
            RAISE(jbuff, ValueError);
        }else {
            state->stack->content[resindex].val.fval = sqrt(state->stack->content[resindex].val.fval);
        }
    }else if(state->stack->content[resindex].type == Integer){
        if(state->stack->content[resindex].val.ival == 0){
            RAISE(jbuff, ValueError);
        }else {
            state->stack->content[resindex].type = Floating;
            state->stack->content[resindex].val.fval = sqrt((double) state->stack->content[resindex].val.ival);
        }
    }else{
        RAISE(jbuff, InvalidOperands);
    }
}

void op_pow(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack->next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack->next -= 1;
    size_t resindex = state->stack->next - 1;
    if(state->stack->content[resindex].type == Floating){
        if(state->stack->content[state->stack->next].type == Floating){
            state->stack->content[resindex].val.fval = pow(state->stack->content[resindex].val.fval, state->stack->content[state->stack->next].val.fval);
        }else if(state->stack->content[state->stack->next].type == Integer) {
            state->stack->content[resindex].val.fval = pow(state->stack->content[resindex].val.fval, (double) state->stack->content[state->stack->next].val.ival);
        }else{
            state->stack->next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else if(state->stack->content[resindex].type == Integer) {
        if(state->stack->content[state->stack->next].type == Floating) {
            state->stack->content[resindex].type = Floating;
            state->stack->content[resindex].val.fval =
                    pow((double) state->stack->content[resindex].val.ival, state->stack->content[state->stack->next].val.fval);
        }else if(state->stack->content[state->stack->next].type == Integer) {
            state->stack->content[resindex].type = Floating;
            state->stack->content[resindex].val.fval =
                    pow((double) state->stack->content[resindex].val.ival, (double) state->stack->content[state->stack->next].val.ival);

        } else {
            state->stack->next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else{
        state->stack->next += 1;
        RAISE(jbuff, InvalidOperands);
    }
}

void op_div(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack->next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack->next -= 1;
    size_t resindex = state->stack->next - 1;
    if(state->stack->content[state->stack->next].type == Floating){
        if(state->stack->content[state->stack->next].val.fval == 0){
            state->stack->next += 1;
            RAISE(jbuff, ValueError);
        }else if(state->stack->content[resindex].type == Floating){
            state->stack->content[resindex].val.fval = state->stack->content[resindex].val.fval / state->stack->content[state->stack->next].val.fval;
        }else if(state->stack->content[resindex].type == Integer){
            state->stack->content[resindex].type = Floating;
            state->stack->content[resindex].val.fval = (double) state->stack->content[resindex].val.ival / state->stack->content[state->stack->next].val.fval;
        }else{
            state->stack->next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else if(state->stack->content[state->stack->next].type == Integer){
        if(state->stack->content[state->stack->next].val.ival == 0){
            state->stack->next += 1;
            RAISE(jbuff, ValueError);
        }else if(state->stack->content[resindex].type == Floating){
            state->stack->content[resindex].val.fval = state->stack->content[resindex].val.fval / (double) state->stack->content[state->stack->next].val.ival;
        }else if(state->stack->content[resindex].type == Integer){
            state->stack->content[resindex].type = Floating;
            state->stack->content[resindex].val.fval = (double) state->stack->content[resindex].val.ival / (double) state->stack->content[state->stack->next].val.ival;
        }else{
            state->stack->next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else{
        state->stack->next += 1;
        RAISE(jbuff, InvalidOperands);
    }
}

void op_mod(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack->next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack->next -= 1;
    size_t resindex = state->stack->next - 1;
    if(state->stack->content[resindex].type != Integer || state->stack->content[state->stack->next].type != Integer) {
        state->stack->next += 1;
        RAISE(jbuff, InvalidOperands);
    }
    if(state->stack->content[state->stack->next].val.ival == 0) {
        state->stack->next += 1;
        RAISE(jbuff, ValueError);
    }
    state->stack->content[resindex].val.ival =
                state->stack->content[resindex].val.ival % state->stack->content[state->stack->next].val.ival;
}


void op_sin(struct ProgramState* state, struct ExceptionHandler* jbuff){
    if(state->stack->next < 1)
        RAISE(jbuff, StackUnderflow);
    size_t indx = state->stack->next - 1;
    if(state->stack->content[indx].type == Floating){
        state->stack->content[indx].val.fval = sin(state->stack->content[indx].val.fval);
    }else if(state->stack->content[indx].type == Integer){
        state->stack->content[indx].val.fval = sin((double) state->stack->content[indx].val.ival);
        state->stack->content[indx].type = Floating;
    }else{
        RAISE(jbuff, InvalidOperands);
    }
}

void op_cos(struct ProgramState* state, struct ExceptionHandler* jbuff){
    if(state->stack->next < 1)
        RAISE(jbuff, StackUnderflow);
    size_t indx = state->stack->next - 1;
    if(state->stack->content[indx].type == Floating){
        state->stack->content[indx].val.fval = cos(state->stack->content[indx].val.fval);
    }else if(state->stack->content[indx].type == Integer){
        state->stack->content[indx].val.fval = cos((double) state->stack->content[indx].val.ival);
        state->stack->content[indx].type = Floating;
    }else{
        RAISE(jbuff, InvalidOperands);
    }
}

void op_tan(struct ProgramState* state, struct ExceptionHandler* jbuff){
    if(state->stack->next < 1)
        RAISE(jbuff, StackUnderflow);
    size_t indx = state->stack->next - 1;
    if(state->stack->content[indx].type == Floating){
        state->stack->content[indx].val.fval = tan(state->stack->content[indx].val.fval);
    }else if(state->stack->content[indx].type == Integer){
        state->stack->content[indx].val.fval = tan((double) state->stack->content[indx].val.ival);
        state->stack->content[indx].type = Floating;
    }else{
        RAISE(jbuff, InvalidOperands);
    }
}

void op_arccos(struct ProgramState* state, struct ExceptionHandler* jbuff){
    if(state->stack->next < 1)
        RAISE(jbuff, StackUnderflow);
    size_t indx = state->stack->next - 1;
    if(state->stack->content[indx].type == Floating){
        state->stack->content[indx].val.fval = acos(state->stack->content[indx].val.fval);
    }else if(state->stack->content[indx].type == Integer){
        state->stack->content[indx].val.fval = acos((double) state->stack->content[indx].val.ival);
        state->stack->content[indx].type = Floating;
    }else{
        RAISE(jbuff, InvalidOperands);
    }
}

void op_arcsin(struct ProgramState* state, struct ExceptionHandler* jbuff){
    if(state->stack->next < 1)
        RAISE(jbuff, StackUnderflow);
    size_t indx = state->stack->next - 1;
    if(state->stack->content[indx].type == Floating){
        state->stack->content[indx].val.fval = asin(state->stack->content[indx].val.fval);
    }else if(state->stack->content[indx].type == Integer){
        state->stack->content[indx].val.fval = asin((double) state->stack->content[indx].val.ival);
        state->stack->content[indx].type = Floating;
    }else{
        RAISE(jbuff, InvalidOperands);
    }
}

void op_arctan(struct ProgramState* state, struct ExceptionHandler* jbuff){
    if(state->stack->next < 1)
        RAISE(jbuff, StackUnderflow);
    size_t indx = state->stack->next - 1;
    if(state->stack->content[indx].type == Floating){
        state->stack->content[indx].val.fval = atan(state->stack->content[indx].val.fval);
    }else if(state->stack->content[indx].type == Integer){
        state->stack->content[indx].val.fval = atan((double) state->stack->content[indx].val.ival);
        state->stack->content[indx].type = Floating;
    }else{
        RAISE(jbuff, InvalidOperands);
    }
}

void op_sinh(struct ProgramState* state, struct ExceptionHandler* jbuff){
    if(state->stack->next < 1)
        RAISE(jbuff, StackUnderflow);
    size_t indx = state->stack->next - 1;
    if(state->stack->content[indx].type == Floating){
        state->stack->content[indx].val.fval = sinh(state->stack->content[indx].val.fval);
    }else if(state->stack->content[indx].type == Integer){
        state->stack->content[indx].val.fval = sinh((double) state->stack->content[indx].val.ival);
        state->stack->content[indx].type = Floating;
    }else{
        RAISE(jbuff, InvalidOperands);
    }
}

void op_cosh(struct ProgramState* state, struct ExceptionHandler* jbuff){
    if(state->stack->next < 1)
        RAISE(jbuff, StackUnderflow);
    size_t indx = state->stack->next - 1;
    if(state->stack->content[indx].type == Floating){
        state->stack->content[indx].val.fval = cosh(state->stack->content[indx].val.fval);
    }else if(state->stack->content[indx].type == Integer){
        state->stack->content[indx].val.fval = cosh((double) state->stack->content[indx].val.ival);
        state->stack->content[indx].type = Floating;
    }else{
        RAISE(jbuff, InvalidOperands);
    }
}

void op_tanh(struct ProgramState* state, struct ExceptionHandler* jbuff){
    if(state->stack->next < 1)
        RAISE(jbuff, StackUnderflow);
    size_t indx = state->stack->next - 1;
    if(state->stack->content[indx].type == Floating){
        state->stack->content[indx].val.fval = tanh(state->stack->content[indx].val.fval);
    }else if(state->stack->content[indx].type == Integer){
        state->stack->content[indx].val.fval = tanh((double) state->stack->content[indx].val.ival);
        state->stack->content[indx].type = Floating;
    }else{
        RAISE(jbuff, InvalidOperands);
    }
}

void op_arcsinh(struct ProgramState* state, struct ExceptionHandler* jbuff){
    if(state->stack->next < 1)
        RAISE(jbuff, StackUnderflow);
    size_t indx = state->stack->next - 1;
    if(state->stack->content[indx].type == Floating){
        state->stack->content[indx].val.fval = asinh(state->stack->content[indx].val.fval);
    }else if(state->stack->content[indx].type == Integer){
        state->stack->content[indx].val.fval = asinh((double) state->stack->content[indx].val.ival);
        state->stack->content[indx].type = Floating;
    }else{
        RAISE(jbuff, InvalidOperands);
    }
}

void op_arccosh(struct ProgramState* state, struct ExceptionHandler* jbuff){
    if(state->stack->next < 1)
        RAISE(jbuff, StackUnderflow);
    size_t indx = state->stack->next - 1;
    if(state->stack->content[indx].type == Floating){
        state->stack->content[indx].val.fval = acosh(state->stack->content[indx].val.fval);
    }else if(state->stack->content[indx].type == Integer){
        state->stack->content[indx].val.fval = acosh((double) state->stack->content[indx].val.ival);
        state->stack->content[indx].type = Floating;
    }else{
        RAISE(jbuff, InvalidOperands);
    }
}

void op_arctanh(struct ProgramState* state, struct ExceptionHandler* jbuff){
    if(state->stack->next < 1)
        RAISE(jbuff, StackUnderflow);
    size_t indx = state->stack->next - 1;
    if(state->stack->content[indx].type == Floating){
        state->stack->content[indx].val.fval = atanh(state->stack->content[indx].val.fval);
    }else if(state->stack->content[indx].type == Integer){
        state->stack->content[indx].val.fval = atanh((double) state->stack->content[indx].val.ival);
        state->stack->content[indx].type = Floating;
    }else{
        RAISE(jbuff, InvalidOperands);
    }
}
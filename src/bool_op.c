#include "bool_op.h"

void op_true(struct ProgramState* state, struct ExceptionHandler* jbuff) {
    struct StackElem elem;
    elem.type = Boolean;
    elem.val.ival = 1;
    push_Stack(state->stack, elem, jbuff);
}

void op_false(struct ProgramState* state, struct ExceptionHandler* jbuff) {
    struct StackElem elem;
    elem.type = Boolean;
    elem.val.ival = 0;
    push_Stack(state->stack, elem, jbuff);
}

void op_empty(struct ProgramState *state, struct ExceptionHandler *jbuff){
    struct StackElem siz;
    siz.type = Boolean;
    siz.val.ival = (state->stack->next == 0);
    push_Stack(state->stack, siz, jbuff);
}


static inline int equal_Stack(struct Stack *s1, struct Stack *s2){
    if(s1->next == s2->next){
            for(size_t i = 0; i < s1->next; i++){
                if(s1->content[i].type == s2->content[i].type){
                    unsigned equals;
                        switch(s1->content[i].type){
                            case String:
                            case Instruction:
                                equals = (strcmp(s1->content[i].val.instr, s2->content[i].val.instr) == 0);
                                break;
                            case Type:
                            case Boolean:
                            case Integer:
                                equals = (s1->content[i].val.ival == s2->content[i].val.ival);
                                break;
                            case Floating:
                                equals = (s1->content[i].val.fval == s2->content[i].val.fval);
                            case None:
                                equals = 1;
                                break;
                            case InnerStack:
                                equals = equal_Stack(s1->content[i].val.stack, s2->content[i].val.stack);
                                break;
                            default:
                                UNREACHABLE;
                        }
                    if(!equals)
                        return 0;
                }else{
                    return 0;
                }
            }
            return 1;
        }
        return 0;
}

void op_equal(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack->next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack->next -= 1;
    size_t resindex = state->stack->next - 1;
    struct StackElem result;
    result.type = Boolean;
    result.val.ival = 0;
    switch (state->stack->content[state->stack->next].type)
    {
    case String:
        if(state->stack->content[resindex].type == String) {
            result.val.ival = (strcmp(state->stack->content[state->stack->next].val.instr, state->stack->content[resindex].val.instr) == 0);
            free(state->stack->content[state->stack->next].val.instr);
        }
        break;
    case Instruction:
        if(state->stack->content[resindex].type == Instruction) {
            result.val.ival = (strcmp(state->stack->content[state->stack->next].val.instr, state->stack->content[resindex].val.instr) == 0);
            free(state->stack->content[state->stack->next].val.instr);
        }
        break;
    case Integer:
        if(state->stack->content[resindex].type == Integer){
            result.val.ival = (state->stack->content[state->stack->next].val.ival == state->stack->content[resindex].val.ival);
        } else if(state->stack->content[resindex].type == Floating){
            result.val.ival = ((double) state->stack->content[state->stack->next].val.ival == state->stack->content[resindex].val.fval);
        }
        break;
    case Floating:
        if(state->stack->content[resindex].type == Integer){
            result.val.ival = (state->stack->content[state->stack->next].val.fval == (double) state->stack->content[resindex].val.ival);
        } else if(state->stack->content[resindex].type == Floating){
            result.val.ival = (state->stack->content[state->stack->next].val.fval == state->stack->content[resindex].val.fval);
        }
        break;
    case Type:
    case None:
    case Boolean:
        if(state->stack->content[state->stack->next].type == state->stack->content[resindex].type){
            result.val.ival = (state->stack->content[state->stack->next].val.ival == state->stack->content[resindex].val.ival);
        }
        break;
    case InnerStack:
        if(state->stack->content[resindex].type == InnerStack){
            result.val.ival = equal_Stack(state->stack->content[state->stack->next].val.stack, state->stack->content[resindex].val.stack);
        }
        break;
    default:
        UNREACHABLE;
    }
    if(state->stack->content[resindex].type == Instruction || state->stack->content[resindex].type == String){
        free(state->stack->content[resindex].val.instr);
    }else if(state->stack->content[resindex].type == InnerStack || state->stack->content[state->stack->next].type == InnerStack){
        state->stack->next += 1;
        push_Stack(state->stack, result, jbuff);
        return;
    }
    state->stack->content[resindex] = result;
}

void op_notequal(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack->next < 2){
        RAISE(jbuff, StackUnderflow);
    }
    state->stack->next -= 1;
    size_t resindex = state->stack->next - 1;
    struct StackElem result;
    result.type = Boolean;
    result.val.ival = 1;
    switch (state->stack->content[state->stack->next].type)
    {
    case String:
        if(state->stack->content[resindex].type == String) {
            result.val.ival = (strcmp(state->stack->content[state->stack->next].val.instr, state->stack->content[resindex].val.instr) != 0);
            free(state->stack->content[state->stack->next].val.instr);
        }
        break;
    case Instruction:
        if(state->stack->content[resindex].type == Instruction) {
           result.val.ival = (strcmp(state->stack->content[state->stack->next].val.instr, state->stack->content[resindex].val.instr) != 0);
            free(state->stack->content[state->stack->next].val.instr);
        }
        break;
    case Integer:
        if(state->stack->content[resindex].type == Integer){
            result.val.ival = (state->stack->content[state->stack->next].val.ival != state->stack->content[resindex].val.ival);
        } else if(state->stack->content[resindex].type == Floating){
            result.val.ival = (((double) state->stack->content[state->stack->next].val.ival) != state->stack->content[resindex].val.fval);
        }
        break;
    case Floating:
        if(state->stack->content[resindex].type == Integer){
            result.val.ival = (state->stack->content[state->stack->next].val.fval != ((double) state->stack->content[resindex].val.ival));
        } else if(state->stack->content[resindex].type == Floating){
            result.val.ival = (state->stack->content[state->stack->next].val.fval != state->stack->content[resindex].val.fval);
        }
        break;
    case Type:
    case None:
    case Boolean:
        if(state->stack->content[state->stack->next].type == state->stack->content[resindex].type){
            result.val.ival = (state->stack->content[state->stack->next].val.ival != state->stack->content[resindex].val.ival);
        }
        break;
    case InnerStack:
        if(state->stack->content[resindex].type == InnerStack){
            result.val.ival = ! equal_Stack(state->stack->content[state->stack->next].val.stack, state->stack->content[resindex].val.stack);
        }
        break;
    default:
        UNREACHABLE;
    }
    if(state->stack->content[resindex].type == Instruction || state->stack->content[resindex].type == String){
        free(state->stack->content[resindex].val.instr);
    }else if(state->stack->content[resindex].type == InnerStack || state->stack->content[state->stack->next].type == InnerStack){
        state->stack->next += 1;
        push_Stack(state->stack, result, jbuff);
        return;
    }
    state->stack->content[resindex] = result;
}

void op_greather(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack->next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack->next -= 1;
    size_t resindex = state->stack->next - 1;
    struct StackElem result;
    result.type = Boolean;
    if(state->stack->content[resindex].type == Integer){
        if(state->stack->content[state->stack->next].type == Floating){
            result.val.ival = ((double) state->stack->content[resindex].val.ival) > state->stack->content[state->stack->next].val.fval;
        }else if (state->stack->content[state->stack->next].type == Integer){
            result.val.ival = state->stack->content[resindex].val.ival > state->stack->content[state->stack->next].val.ival;
        }else{
            state->stack->next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else if(state->stack->content[resindex].type == Floating){
        if(state->stack->content[state->stack->next].type == Floating){
            result.val.ival = state->stack->content[resindex].val.fval > state->stack->content[state->stack->next].val.fval;
        }else if (state->stack->content[state->stack->next].type == Integer){
            result.val.ival = state->stack->content[resindex].val.fval > ((double) state->stack->content[state->stack->next].val.ival);
        }else{
            state->stack->next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else{
        state->stack->next += 1;
        RAISE(jbuff, InvalidOperands);
    }
    state->stack->content[resindex] = result;
}

void op_greathereq(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack->next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack->next -= 1;
    size_t resindex = state->stack->next - 1;
    struct StackElem result;
    result.type = Boolean;
    if(state->stack->content[resindex].type == Integer){
        if(state->stack->content[state->stack->next].type == Floating){
            result.val.ival = ((double) state->stack->content[resindex].val.ival) >= state->stack->content[state->stack->next].val.fval;
        }else if (state->stack->content[state->stack->next].type == Integer){
            result.val.ival = state->stack->content[resindex].val.ival >= state->stack->content[state->stack->next].val.ival;
        }else{
            state->stack->next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else if(state->stack->content[resindex].type == Floating){
        if(state->stack->content[state->stack->next].type == Floating){
            result.val.ival = state->stack->content[resindex].val.fval >= state->stack->content[state->stack->next].val.fval;
        }else if (state->stack->content[state->stack->next].type == Integer){
            result.val.ival = state->stack->content[resindex].val.fval >= ((double) state->stack->content[state->stack->next].val.ival);
        }else{
            state->stack->next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else{
        state->stack->next += 1;
        RAISE(jbuff, InvalidOperands);
    }
    state->stack->content[resindex] = result;
}

void op_lower(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack->next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack->next -= 1;
    size_t resindex = state->stack->next - 1;
    struct StackElem result;
    result.type = Boolean;
    if(state->stack->content[resindex].type == Integer){
        if(state->stack->content[state->stack->next].type == Floating){
            result.val.ival = ((double) state->stack->content[resindex].val.ival) < state->stack->content[state->stack->next].val.fval;
        }else if (state->stack->content[state->stack->next].type == Integer){
            result.val.ival = state->stack->content[resindex].val.ival < state->stack->content[state->stack->next].val.ival;
        }else{
            state->stack->next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else if(state->stack->content[resindex].type == Floating){
        if(state->stack->content[state->stack->next].type == Floating){
            result.val.ival = state->stack->content[resindex].val.fval < state->stack->content[state->stack->next].val.fval;
        }else if (state->stack->content[state->stack->next].type == Integer){
            result.val.ival = state->stack->content[resindex].val.fval < ((double) state->stack->content[state->stack->next].val.ival);
        }else{
            state->stack->next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else{
        state->stack->next += 1;
        RAISE(jbuff, InvalidOperands);
    }
    state->stack->content[resindex] = result;
}

void op_lowereq(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack->next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack->next -= 1;
    size_t resindex = state->stack->next - 1;
    struct StackElem result;
    result.type = Boolean;
    if(state->stack->content[resindex].type == Integer){
        if(state->stack->content[state->stack->next].type == Floating){
            result.val.ival = ((double) state->stack->content[resindex].val.ival) <= state->stack->content[state->stack->next].val.fval;
        }else if (state->stack->content[state->stack->next].type == Integer){
            result.val.ival = state->stack->content[resindex].val.ival <= state->stack->content[state->stack->next].val.ival;
        }else{
            state->stack->next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else if(state->stack->content[resindex].type == Floating){
        if(state->stack->content[state->stack->next].type == Floating){
            result.val.ival = state->stack->content[resindex].val.fval <= state->stack->content[state->stack->next].val.fval;
        }else if (state->stack->content[state->stack->next].type == Integer){
            result.val.ival = state->stack->content[resindex].val.fval <= ((double) state->stack->content[state->stack->next].val.ival);
        }else{
            state->stack->next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else{
        state->stack->next += 1;
        RAISE(jbuff, InvalidOperands);
    }
    state->stack->content[resindex] = result;
}

void op_and(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack->next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack->next -= 1;
    size_t resindex = state->stack->next - 1;
    if(state->stack->content[resindex].type != Boolean || state->stack->content[state->stack->next].type != Boolean){
        state->stack->next += 1;
        RAISE(jbuff, InvalidOperands);
    }
    state->stack->content[resindex].val.ival = state->stack->content[resindex].val.ival & state->stack->content[state->stack->next].val.ival;
}

void op_or(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack->next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack->next -= 1;
    size_t resindex = state->stack->next - 1;
    if(state->stack->content[resindex].type != Boolean || state->stack->content[state->stack->next].type != Boolean){
        state->stack->next += 1;
        RAISE(jbuff, InvalidOperands);
    }
    state->stack->content[resindex].val.ival = state->stack->content[resindex].val.ival | state->stack->content[state->stack->next].val.ival;
}

void op_xor(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack->next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack->next -= 1;
    size_t resindex = state->stack->next - 1;
    if(state->stack->content[resindex].type != Boolean || state->stack->content[state->stack->next].type != Boolean){
        state->stack->next += 1;
        RAISE(jbuff, InvalidOperands);
    }
    state->stack->content[resindex].val.ival = state->stack->content[resindex].val.ival ^ state->stack->content[state->stack->next].val.ival;
}

void op_not(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack->next == 0)
        RAISE(jbuff, StackUnderflow);
    size_t resindex = state->stack->next - 1;
    if(state->stack->content[resindex].type != Boolean){
        RAISE(jbuff, InvalidOperands);
    }
    state->stack->content[resindex].val.ival = (! state->stack->content[resindex].val.ival);
}
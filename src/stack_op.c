#include "stack_op.h"

static inline void print_InnerStack(struct Stack *stack){
    printf("{ ");
    for(size_t i = 0; i< stack->next; i++){
        switch (stack->content[i].type){
            case Instruction:
                printf("[ %s ] ", stack->content[i].val.instr);
                break;
            case String:
                printf("\"%s\" ", stack->content[i].val.instr);
                break;
            case Integer:
                printf("%ld ", stack->content[i].val.ival);
                break;
            case Floating:
                printf("%lf ", stack->content[i].val.fval);
                break;
            case Boolean:
                printf("%s ", BOOL[stack->content[i].val.ival]);
                break;
            case None:
                printf("none\n");
                break;
            case Type:
                printf("%s ", TYPES[stack->content[i].val.ival]);
                break;
            case InnerStack:
                print_InnerStack(stack->content[i].val.stack);
                break;
            default:
                UNREACHABLE;
            }
        }
    printf("} ");
}

static inline void print_single(struct Stack *stack, size_t num){
    switch (stack->content[stack->next - num].type)
    {
    case Instruction:
        printf("[ %s ]\n", stack->content[stack->next - num].val.instr);
        break;
    case String:
        printf("\"%s\"\n", stack->content[stack->next - num].val.instr);
        break;
    case Integer:
        printf("%ld\n", stack->content[stack->next - num].val.ival);
        break;
    case Floating:
        printf("%lf\n", stack->content[stack->next - num].val.fval);
        break;
    case Boolean:
        printf("%s\n", BOOL[stack->content[stack->next - num].val.ival]);
        break;
    case None:
        printf("none\n");
        break;
    case Type:
        printf("%s\n", TYPES[stack->content[stack->next - num].val.ival]);
        break;
    case InnerStack:
        print_InnerStack(stack->content[stack->next - num].val.stack);
        printf("\n");
        break;
    default:
        UNREACHABLE;
    }
}

void print_stack(struct ProgramState* state, size_t num_elem) {
    for (size_t i = num_elem; i > 0; i--) {
        print_single(state->stack, i);
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


void op_quote(struct ProgramState *state, struct ExceptionHandler *jbuff) {
    if (state->stack->next == 0)
        RAISE(jbuff, StackUnderflow);
    size_t resindex = state->stack->next - 1;
    size_t finallen;
    char *resstr;
    char buffer[1];
    int result;
    switch (state->stack->content[resindex].type)
    {
    case String:
        finallen = strlen(state->stack->content[resindex].val.instr) + 5;
        resstr = malloc(finallen);
        if (resstr == NULL) {
            RAISE(jbuff, ProgramPanic);
        } else {
            strcpy(resstr + 2, state->stack->content[resindex].val.instr);
            resstr[0] = '[';
            resstr[1] = '"';
            resstr[finallen - 3] = '"';
            resstr[finallen - 2] = ']';
            resstr[finallen - 1] = '\0';
            free(state->stack->content[resindex].val.instr);
            state->stack->content[resindex].val.instr = resstr;
        }
        break;
    case Instruction:
        finallen = strlen(state->stack->content[resindex].val.instr) + 3;
        resstr = malloc(finallen);
        if (resstr == NULL) {
            RAISE(jbuff, ProgramPanic);
        } else {
            strcpy(resstr + 1, state->stack->content[resindex].val.instr);
            resstr[0] = '[';
            resstr[finallen - 2] = ']';
            resstr[finallen - 1] = '\0';
            free(state->stack->content[resindex].val.instr);
            state->stack->content[resindex].val.instr = resstr;
        }
        break;
    case Integer:
        finallen = (int) log10((double)state->stack->content[resindex].val.ival + 1) + 1 + (state->stack->content[resindex].val.ival < 0);
        resstr = malloc(finallen + 3);
        if (resstr == NULL) {
            RAISE(jbuff, ProgramPanic);
        } else {
            snprintf(resstr, finallen + 3, "[%ld]", state->stack->content[resindex].val.ival);
            state->stack->content[resindex].val.instr = resstr;
            state->stack->content[resindex].type = Instruction;
        }
        break;
    case Floating:
        result = snprintf(buffer, 1, "%lf", state->stack->content[resindex].val.fval);
        if (result < 1) {
            RAISE(jbuff, ProgramPanic);
        } else {
            char *resstr = malloc(result + 3);
            if (resstr == NULL) {
                RAISE(jbuff, ProgramPanic);
            } else {
                snprintf(resstr, result + 3, "[%lf]", state->stack->content[resindex].val.fval);
                state->stack->content[resindex].val.instr = resstr;
                state->stack->content[resindex].type = Instruction;
            }
        }
        break;
    case Boolean:
        result = state->stack->content[resindex].val.ival;
        state->stack->content[resindex].val.instr = malloc(8 - result);
        if (state->stack->content[resindex].val.instr == NULL) {
            RAISE(jbuff, ProgramPanic);
        } else {
            state->stack->content[resindex].val.instr[0] = '[';
            strncpy(state->stack->content[resindex].val.instr + 1, BOOL[result], 5 - result);
            state->stack->content[resindex].val.instr[6 - result] = ']';
            state->stack->content[resindex].val.instr[7 - result] = '\0';
            state->stack->content[resindex].type = Instruction;
        }
        break;
    case None:
        state->stack->content[resindex].val.instr = malloc(7);
        if (state->stack->content[resindex].val.instr == NULL) {
            RAISE(jbuff, ProgramPanic);
        } else {
            state->stack->content[resindex].val.instr[0] = '[';
            strncpy(state->stack->content[resindex].val.instr + 1, NONE, 4);
            state->stack->content[resindex].val.instr[6] = ']';
            state->stack->content[resindex].val.instr[7] = '\0';
            state->stack->content[resindex].type = Instruction;
        }
        break;
    case Type:
        result = state->stack->content[resindex].val.ival;
        state->stack->content[resindex].val.instr = malloc(TYPES_LEN[result] + 3);
        if (state->stack->content[resindex].val.instr == NULL) {
            RAISE(jbuff, ProgramPanic);
        } else {
            state->stack->content[resindex].val.instr[0] = '[';
            strncpy(state->stack->content[resindex].val.instr + 1, TYPES[result], TYPES_LEN[result]);
            state->stack->content[resindex].val.instr[TYPES_LEN[result] + 1] = ']';
            state->stack->content[resindex].val.instr[TYPES_LEN[result] + 2] = '\0';
            state->stack->content[resindex].type = Instruction;
        }
    break;
    case InnerStack:
        RAISE(jbuff, InvalidOperands);
        break;
    default:
        UNREACHABLE;
    }
}

void op_compose(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack->next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack->next -= 1;
    if((state->stack->content[state->stack->next].type == Instruction && state->stack->content[state->stack->next - 1].type == Instruction)
        || (state->stack->content[state->stack->next].type == String && state->stack->content[state->stack->next - 1].type == String)){
        size_t lensecond = strlen(state->stack->content[state->stack->next].val.instr);
        size_t lenfirst =  strlen(state->stack->content[state->stack->next - 1].val.instr);
        char *composte = realloc(state->stack->content[state->stack->next - 1].val.instr, lensecond + lenfirst + 2);
        if(composte == NULL){
            RAISE(jbuff, ProgramPanic);
        }
        state->stack->content[state->stack->next - 1].val.instr = composte;
        state->stack->content[state->stack->next - 1].val.instr[lenfirst] = ' ';
        strcpy(state->stack->content[state->stack->next - 1].val.instr + lenfirst + 1, state->stack->content[state->stack->next].val.instr);
        free(state->stack->content[state->stack->next].val.instr);
        state->stack->content[state->stack->next - 1].val.instr[lensecond + lenfirst + 1] = '\0';
    }else{
        state->stack->next += 1;
        RAISE(jbuff, InvalidOperands);
    }
}

void op_nop(struct ProgramState *state, struct ExceptionHandler *jbuff){
    return;
}

void op_exit(struct ProgramState *state, struct ExceptionHandler *jbuff){
    RAISE(jbuff, ProgramExit);
}

void op_print(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack->next != 0)
        print_single(state->stack, 1);
}

void op_printall(struct ProgramState *state, struct ExceptionHandler *jbuff){
    for(size_t i = state->stack->next; i > 0; i--){
        print_single(state->stack, i);
    }
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

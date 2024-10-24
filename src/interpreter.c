//
// Created by P4o1o on 14/05/2024.
//
#include "interpreter.h"
#include <string.h>
#include <math.h>
#include <stdlib.h>
#include <errno.h>

const char *BOOL[] = {
        "true",
        "false"
};

const char *NUMBERED_INSTR[] = {
        "dup", "swap", "dig"
};

#define BRACKETS_SIZE 12
const char *BRACKETS_INSTR[] = {
        "load","if","save","compose",
        "delete","isdef","loop","split",
        "swap","define","dup", "times", "dig"
};

const char BR_CLOSE = ')';

#define INSTR_SIZE 40
const char* INSTRUCTIONS[] = {
        "int", "clear", "quote", "<=", "dup",
        "or", "swap", "+", "and", "dip",
        "exit", "nop", "print", "size", "try",
        "%", "/", ">", "apply", "compose",
        "drop", "empty", "if", "loop", "not",
        "pow", "printall", "roll", "sqrt", "top",
        "xor", "!=", "*", "-", "<",
        "==", ">=", "true", "false", "split"
};

const operations INSTR_OP[] ={
        op_int, op_clear, op_quote, op_lowereq, op_dup,
        op_or, op_swap, op_sum, op_and, op_dip,
        op_exit, op_nop, op_print, op_size, op_try,
        op_mod, op_div, op_greather, op_apply, op_compose,
        op_drop, op_empty, op_if, op_loop, op_not,
        op_pow, op_printall, op_roll, op_sqrt, op_top,
        op_xor, op_notequal, op_mul, op_sub, op_lower,
        op_equal, op_greathereq, op_true, op_false, op_split
};

br_operations BR_INSTR_OP[] ={
        brop_load, brop_if, brop_save, brop_compose,
        brop_delete, brop_isdef, brop_loop, brop_split,
        brop_swap, brop_define, brop_dup, brop_times, brop_dig
};

num_operations NUM_INSTR_OP[] = {
        numop_dup, numop_swap, numop_dig
};

#define IS_INDENT(comand) ((int) ((comand) == ' ' || (comand) == '\n' || (comand) == '\t' || (comand) == '\r' || (comand) == '\0'))

struct Builtins builtins;

#define HASHKEY_OP0 0x734ad7e3439432a3ULL
#define HASHKEY_OP1 0x54dc762ab02dc4deULL
#define HASHKEY_BROP0 0x734ad7e3439432a3ULL
#define HASHKEY_BROP1 0x54dc762ab02dc4deULL
int init_builtins() {
    builtins.op_map = malloc(OP_MAP_SIZE * sizeof(struct OperationElem*));
    if (builtins.op_map == NULL)
        return 0;
    for (size_t i = 0; i < OP_MAP_SIZE; i++) {
        builtins.op_map[i] = NULL;
    }
    builtins.brop_map = malloc(BROP_MAP_SIZE * sizeof(struct BrOperationElem*));
    if (builtins.brop_map == NULL)
        return 0;
    for (size_t i = 0; i < BROP_MAP_SIZE; i++) {
        builtins.brop_map[i] = NULL;
    }
    for (size_t i = 0; i < INSTR_SIZE; i++) {
        uint64_t index = SipHash_2_4(HASHKEY_OP0, HASHKEY_OP1, INSTRUCTIONS[i], strlen(INSTRUCTIONS[i])) & (OP_MAP_SIZE - 1);
        struct OperationElem* elem = builtins.op_map[index];
        while (elem != NULL) {
            if (strcmp(INSTRUCTIONS[i], elem->key) == 0) {
                return 0;
            }
            elem = elem->next;
        }
        elem = malloc(sizeof(struct OperationElem));
        if (elem == NULL)
            return 0;
        elem->key = INSTRUCTIONS[i];
        elem->op = INSTR_OP[i];
        elem->next = builtins.op_map[index];
        builtins.op_map[index] = elem;
    }
    for (size_t i = 0; i < BRACKETS_SIZE; i++) {
        uint64_t index = SipHash_2_4(HASHKEY_BROP0, HASHKEY_BROP1, BRACKETS_INSTR[i], strlen(BRACKETS_INSTR[i])) & (BROP_MAP_SIZE - 1);
        struct BrOperationElem* elem = builtins.brop_map[index];
        while (elem != NULL) {
            if (strcmp(BRACKETS_INSTR[i], elem->key) == 0) {
                return 0;
            }
            elem = elem->next;
        }
        elem = malloc(sizeof(struct BrOperationElem));
        if (elem == NULL)
            return 0;
        elem->key = BRACKETS_INSTR[i];
        elem->brop = BR_INSTR_OP[i];
        elem->next = builtins.brop_map[index];
        builtins.brop_map[index] = elem;
    }
    return 1;
}



void free_builtins() {
    for (size_t i = 0; i < OP_MAP_SIZE; i++) {
        struct OperationElem* elem = builtins.op_map[i];
        while (elem != NULL) {
            struct OperationElem* temp = elem->next;
            free(elem);
            elem = temp;
        }
    }
    free(builtins.op_map);
    for (size_t i = 0; i < BROP_MAP_SIZE; i++) {
        struct BrOperationElem* elem = builtins.brop_map[i];
        while (elem != NULL) {
            struct BrOperationElem* temp = elem->next;
            free(elem);
            elem = temp;
        }
    }
    free(builtins.brop_map);
}

//---------------------------------------------------------------------------------------------------------------------------------------------------------

static inline void push_Stack(struct Stack *stack, const struct StackElem val, struct ExceptionHandler *jbuff){
    if(stack->next + 1 == stack->capacity){
        stack->capacity = stack->capacity << 1;
        struct StackElem *newmem = realloc(stack->content, stack->capacity);
        if(newmem == NULL){
            if(val.type == Instruction)
                free(val.val.instr);
            RAISE(jbuff, ProgramPanic);
        }
        stack->content = newmem;
    }
    stack->content[stack->next] = val;
    stack->next += 1;
}

#define HASHKEY0 0x734bc7ed439782a3ULL
#define HASHKEY1 0x542f7629b02ac4deULL

static inline int set_Environment(struct Environment* env, char* key, size_t keylen, char* val, struct ExceptionHandler* jbuff) {
    size_t index = (size_t)(SipHash_2_4(HASHKEY0, HASHKEY1, key, keylen) % env->capacity);
    struct EnvElem* elem = env->content[index];
    while (elem != NULL) {
        if (keylen == strlen(elem->key) && strncmp(key, elem->key, keylen) == 0) {
            free(elem->value);
            elem->value = val;
            return 1;
        }
        elem = elem->next;
    }
    elem = malloc(sizeof(struct EnvElem));
    if (elem == NULL)
        RAISE(jbuff, ProgramPanic);
    elem->key = malloc(keylen + 1);
    if (elem->key == NULL)
        RAISE(jbuff, ProgramPanic);
    strncpy(elem->key, key, keylen);
    elem->key[keylen] = '\0';
    elem->value = val;
    elem->next = env->content[index];
    env->content[index] = elem;
    return 0;
}

static inline int get_Environment(struct Environment* env, const char* key, size_t keylen, char** out) {
    size_t index = (size_t)(SipHash_2_4(HASHKEY0, HASHKEY1, key, keylen) % env->capacity);
    struct EnvElem* elem = env->content[index];
    while (elem != NULL) {
        if (keylen == strlen(elem->key) && strncmp(key, elem->key, keylen) == 0) {
            *out = elem->value;
            return 1;
        }
        elem = elem->next;
    }
    return 0;
}

static inline int remove_Environment(struct Environment* env, const char* key, size_t keylen) {
    size_t index = (size_t)(SipHash_2_4(HASHKEY0, HASHKEY1, key, keylen) % env->capacity);
    struct EnvElem** elem_ptr = &env->content[index];
    struct EnvElem* elem = *elem_ptr;
    while (elem != NULL) {
        if (keylen == strlen(elem->key) && strncmp(key, elem->key, keylen) == 0) {
            free(elem->key);
            free(elem->value);
            *elem_ptr = elem->next;
            free(elem);
            return 1;
        }
        elem_ptr = &elem->next;
        elem = *elem_ptr;
    }
    return 0;
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
    case BoolTrue:
        printf("%s\n", BOOL[0]);
        break;
    case BoolFalse:
        printf("%s\n", BOOL[1]);
        break;
    default:
        UNREACHABLE;
    }
}

inline void add_backtrace(struct ExceptionHandler *jbuff){
    jbuff->bt_size += 1;
    if(jbuff->bt_size >= jbuff->bt_capacity){
        jbuff->bt_capacity *= 2;
        jbuff->not_exec = realloc(jbuff->not_exec, sizeof(char *) * jbuff->bt_capacity);
    }
}

inline void remove_backtrace(struct ExceptionHandler *jbuff){
    jbuff->bt_size -= 1;
}

inline void add_memory(struct ExceptionHandler *jbuff, char *mem){
    size_t index = ((size_t) mem) % OM_VEC_CAPACITY;
    struct OpenMemMap *elem = malloc(sizeof(struct OpenMemMap));
    if (elem == NULL)
        RAISE(jbuff, ProgramPanic);
    elem->openmem = mem;
    elem->next = jbuff->openmemmap[index];
    jbuff->openmemmap[index] = elem;
}

inline int remove_memory(struct ExceptionHandler *jbuff, char *mem){
    size_t index = ((size_t) mem) % OM_VEC_CAPACITY;
    struct OpenMemMap** elem_ptr = &jbuff->openmemmap[index];
    struct OpenMemMap* elem = *elem_ptr;
    while (elem != NULL) {
        if (mem == elem->openmem) {
            free(mem);
            *elem_ptr = elem->next;
            free(elem);
            return 1;
        }
        elem_ptr = &elem->next;
        elem = *elem_ptr;
    }
    return 0;
}
//------------------------------------------------------------------------------------------------------

inline void execute_instr(struct ProgramState *state, char *instr, size_t instrlen, struct ExceptionHandler *jbuff){
    jbuff->not_exec[jbuff->bt_size - 1] = instr;
    if(instr[0] == '[' && instr[instrlen - 1] == ']'){
        struct StackElem elem;
        elem.type = Instruction;
        size_t qexpsize = instrlen - 1;
        elem.val.instr = malloc(qexpsize);
        if(elem.val.instr == NULL)
            RAISE(jbuff, ProgramPanic);
        strncpy(elem.val.instr, instr + 1, qexpsize - 1);
        elem.val.instr[qexpsize - 1] = '\0';
        push_Stack(&state->stack, elem, jbuff);
        return;
    }else if(instr[0] == '"' && instr[instrlen - 1] == '"'){
        struct StackElem elem;
        elem.type = String;
        size_t qexpsize = instrlen - 1;
        elem.val.instr = malloc(qexpsize);
        if(elem.val.instr == NULL)
            RAISE(jbuff, ProgramPanic);
        strncpy(elem.val.instr, instr + 1, qexpsize - 1);
        elem.val.instr[qexpsize - 1] = '\0';
        push_Stack(&state->stack, elem, jbuff);
        return;
    }
    char *endptr;
    int64_t intval = strtol(instr, &endptr, 10);
    if(endptr == (instr + instrlen) && errno != ERANGE){
        struct StackElem elem;
        elem.type = Integer;
        elem.val.ival = intval;
        push_Stack(&state->stack, elem, jbuff);
        return;
    }
    endptr = NULL;
    double floatval = strtod(instr, &endptr);
    if(endptr == (instr + instrlen) && errno != ERANGE){
        struct StackElem elem;
        elem.type = Floating;
        elem.val.fval = floatval;
        push_Stack(&state->stack, elem, jbuff);
        return;
    }
    if(instr[instrlen - 1] == BR_CLOSE){
        size_t br_index;
        for(size_t i = 0; i < instrlen; i++)
            if (instr[i] == '(') {
                br_index = i;
                goto found_open_br;
            }
        RAISE(jbuff, InvalidInstruction);
        size_t index;
found_open_br:
        index = (size_t)(SipHash_2_4(HASHKEY_BROP0, HASHKEY_BROP1, instr, br_index) & (BROP_MAP_SIZE - 1));
        struct BrOperationElem* elem = builtins.brop_map[index];
        while (elem != NULL) {
            if (strncmp(instr, elem->key, br_index) == 0 && br_index == strlen(elem->key)) {
                elem->brop(state, instr + (br_index + 1), instrlen - br_index - 2, jbuff);
                return;
            }
            elem = elem->next;
        }
    }else {
        size_t index = (size_t)(SipHash_2_4(HASHKEY_OP0, HASHKEY_OP1, instr, instrlen) & (OP_MAP_SIZE - 1));
        struct OperationElem* elem = builtins.op_map[index];
        while (elem != NULL) {
            if (strncmp(instr, elem->key, instrlen) == 0 && instrlen == strlen(elem->key)) {
                elem->op(state, jbuff);
                return;
            }
            elem = elem->next;
        }
        for (short i = 0; i < 3; i++) {
            size_t nilen = strlen(NUMBERED_INSTR[i]);
            if (strncmp(NUMBERED_INSTR[i], instr, nilen) == 0) {
                size_t number = strtol(instr + nilen, &endptr, 10);
                if (endptr == (instr + instrlen) && errno != ERANGE) {
                    NUM_INSTR_OP[i](state, number, jbuff);
                    return;
                }
            }
        }
        char** funct = malloc(sizeof(char*));
        if (funct == NULL)
            RAISE(jbuff, ProgramPanic);
        if (get_Environment(&state->env, instr, instrlen, funct) == 1) {
            char* text = *funct;
            free(funct);
            add_backtrace(jbuff);
            parse_script(state, text, strlen(text), jbuff);
            remove_backtrace(jbuff);
            return;
        }else{
            free(funct);
        }
    }
    RAISE(jbuff, InvalidInstruction);
}

void parse_script(struct ProgramState *state, char *comands, size_t clen, struct ExceptionHandler *jbuff){
    size_t start = 0;
    size_t quote = 0;
    size_t round_br = 0;
    short string = 0;
    for(size_t i = 0; i < clen; i++){
        if(comands[i] == '['){
            quote += 1;
        }else if(comands[i] == ']'){
            if(quote == 0)
                RAISE(jbuff, SquaredParenthesisError);
            quote -= 1;
            if(quote == 0 && round_br == 0 && string == 0){
                execute_instr(state, comands + start, i + 1 - start, jbuff);
                start = i + 1;
            }
        }else if(comands[i] == '('){
            round_br += 1;
        }else if(comands[i] == ')'){
            if(round_br == 0)
                RAISE(jbuff, RoundParenthesisError);
            round_br -= 1;
            if(quote == 0 && round_br == 0 && string == 0){
                execute_instr(state, comands + start, i + 1 - start, jbuff);
                start = i + 1;
            }
        }else if(comands[i] == '"'){
            string += 1;
            string %= 2;
            if(quote == 0 && round_br == 0 && string == 0){
                execute_instr(state, comands + start, i + 1 - start, jbuff);
                start = i + 1;
            }
        }else if(comands[i] == ' ' || comands[i] == '\n' || comands[i] == '\t' || comands[i] == '\r' || comands[i] == '\0'){
            if(quote == 0 && round_br == 0 && string == 0){
                if(i - start > 0) {
                    execute_instr(state, comands + start, i - start, jbuff);
                }
                start = i + 1;
            }
        }
    }
    if(quote != 0){
        RAISE(jbuff, SquaredParenthesisError);
    }else if(round_br != 0){
        RAISE(jbuff, RoundParenthesisError);
    }else if(quote != 0){
        RAISE(jbuff, StringQuotingError);
    }else{
        if(start != clen){
            execute_instr(state, comands + start, clen - start, jbuff);
        }
    }
}

void execute(struct ProgramState *state, char *comands, struct ExceptionHandler *jbuff){
    parse_script(state, comands, strlen(comands), jbuff);
}

void print_stack(struct ProgramState* state, size_t num_elem) {
    for (size_t i = num_elem; i > 0; i--) {
        print_single(&state->stack, i);
    }
}



//------------------------------------------------------------------------------------------------------

void op_split(struct ProgramState* state, struct ExceptionHandler* jbuff){
    if (state->stack.next == 0)
        RAISE(jbuff, StackUnderflow);
    state->stack.next -= 1;
    if (state->stack.content[state->stack.next].type == Instruction) {
        size_t quote = 0;
        size_t start = 0;
        size_t round_br = 0;
        short string = 0;
        size_t i = 0;
        char *original = state->stack.content[state->stack.next].val.instr;
        for(; original[i] != '\0'; i++){
            if(original[i] == '['){
            quote += 1;
            }else if(original[i] == ']'){
                quote -= 1;
                if(quote == 0 && round_br == 0 && string == 0){
                    struct StackElem elem;
                    elem.type = Instruction;
                    elem.val.instr = malloc(i + 2 - start);
                    if (elem.val.instr == NULL)
                        RAISE(jbuff, ProgramPanic);
                    strncpy(elem.val.instr, original + start, i + 1 - start);
                    elem.val.instr[i + 1 - start] = '\0';
                    push_Stack(&state->stack, elem, jbuff);
                    start = i + 1;
                }
            }else if(original[i] == '('){
                round_br += 1;
            }else if(original[i] == ')'){
                round_br -= 1;
                if(quote == 0 && round_br == 0 && string == 0){
                    struct StackElem elem;
                    elem.type = Instruction;
                    elem.val.instr = malloc(i + 2 - start);
                    if (elem.val.instr == NULL)
                        RAISE(jbuff, ProgramPanic);
                    strncpy(elem.val.instr, original + start, i + 1 - start);
                    elem.val.instr[i + 1 - start] = '\0';
                    push_Stack(&state->stack, elem, jbuff);
                    start = i + 1;
                }
            }else if(original[i] == '"'){
                string += 1;
                string %= 2;
                if(quote == 0 && round_br == 0 && string == 0){
                    struct StackElem elem;
                    elem.type = Instruction;
                    elem.val.instr = malloc(i + 2 - start);
                    if (elem.val.instr == NULL)
                        RAISE(jbuff, ProgramPanic);
                    strncpy(elem.val.instr, original + start, i + 1 - start);
                    elem.val.instr[i + 1 - start] = '\0';
                    push_Stack(&state->stack, elem, jbuff);
                    start = i + 1;
                }
            }else if(IS_INDENT(original[i])){
                if(quote == 0 && round_br == 0 && string == 0){
                    if(i - start > 0) {
                        struct StackElem elem;
                        elem.type = Instruction;
                        elem.val.instr = malloc(i + 1 - start);
                        if (elem.val.instr == NULL)
                            RAISE(jbuff, ProgramPanic);
                        strncpy(elem.val.instr, original + start, i - start);
                        elem.val.instr[i - start] = '\0';
                        push_Stack(&state->stack, elem, jbuff);
                    }
                    start = i + 1;
                }
            }
        }
        if(i) {
            struct StackElem elem;
            elem.type = Instruction;
            elem.val.instr = malloc(i + 1 - start);
            if (elem.val.instr == NULL)
                RAISE(jbuff, ProgramPanic);
            strncpy(elem.val.instr, original + start, i - start);
            elem.val.instr[i - start] = '\0';
            push_Stack(&state->stack, elem, jbuff);
        }
        free(original);
    }else if(state->stack.content[state->stack.next].type == String){
        char *original = state->stack.content[state->stack.next].val.instr;
        char *token = strtok(original, " ");
        do{
            struct StackElem elem;
            elem.type = String;
            size_t tokenlen = strlen(token);
            elem.val.instr = malloc(tokenlen + 1);
             if (elem.val.instr == NULL)
                RAISE(jbuff, ProgramPanic);
            strncpy(elem.val.instr, token, tokenlen);
            elem.val.instr[tokenlen] = '\0';
            push_Stack(&state->stack, elem, jbuff);
        }while((token = strtok(NULL, " ")) != NULL);
        free(original);
    }else{
        state->stack.next += 1;
        RAISE(jbuff, InvalidOperands);
    }
}

void brop_split(struct ProgramState *state, char *comand, size_t clen, struct ExceptionHandler *jbuff){
    add_backtrace(jbuff);
    parse_script(state, comand, clen, jbuff);
    remove_backtrace(jbuff);
    if(state->stack.next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack.next -= 1;
    struct StackElem delimiter = state->stack.content[state->stack.next];
    state->stack.next -= 1;
    struct StackElem string = state->stack.content[state->stack.next];
    if(delimiter.type == String && string.type == String) {
        char *token = strtok(string.val.instr, " ");
        do{
            struct StackElem elem;
            elem.type = String;
            size_t tokenlen = strlen(token);
            elem.val.instr = malloc(tokenlen + 1);
             if (elem.val.instr == NULL)
                RAISE(jbuff, ProgramPanic);
            strncpy(elem.val.instr, token, tokenlen);
            elem.val.instr[tokenlen] = '\0';
            push_Stack(&state->stack, elem, jbuff);
        }while((token = strtok(NULL, delimiter.val.instr)) != NULL);
        free(string.val.instr);
        free(delimiter.val.instr);
    }else{
        state->stack.next += 1;
        RAISE(jbuff, InvalidOperands);
    }
}

void brop_compose(struct ProgramState *state, char *comand, size_t clen, struct ExceptionHandler *jbuff){
    add_backtrace(jbuff);
    parse_script(state, comand, clen, jbuff);
    remove_backtrace(jbuff);
    if(state->stack.next < 3)
        RAISE(jbuff, StackUnderflow);
     state->stack.next -= 1;
    struct StackElem delimiter = state->stack.content[state->stack.next];
    state->stack.next -= 1;
    struct StackElem second = state->stack.content[state->stack.next];
    if(state->stack.content[state->stack.next - 1].type == String && second.type == String && delimiter.type == String){
        size_t delimlen = strlen(delimiter.val.instr);
        size_t lensecond = strlen(second.val.instr);
        size_t lenfirst =  strlen(state->stack.content[state->stack.next - 1].val.instr);
        size_t totsize = lensecond + lenfirst + delimlen + 1;
        state->stack.content[state->stack.next - 1].val.instr = realloc(state->stack.content[state->stack.next - 1].val.instr, totsize);
        if(state->stack.content[state->stack.next - 1].val.instr == NULL){
            RAISE(jbuff, ProgramPanic);
        }
        strcpy(state->stack.content[state->stack.next - 1].val.instr + lenfirst, delimiter.val.instr);
        strcpy(state->stack.content[state->stack.next - 1].val.instr + delimlen + lenfirst, second.val.instr);
        state->stack.content[state->stack.next - 1].val.instr[totsize - 1] = '\0';
        free(delimiter.val.instr);
        free(second.val.instr);
    }else{
        state->stack.next += 2;
        RAISE(jbuff, InvalidOperands);
    }
}

void op_true(struct ProgramState* state, struct ExceptionHandler* jbuff) {
    struct StackElem elem;
    elem.type = BoolTrue;
    elem.val.instr = NULL;
    push_Stack(&state->stack, elem, jbuff);
}

void op_false(struct ProgramState* state, struct ExceptionHandler* jbuff) {
    struct StackElem elem;
    elem.type = BoolFalse;
    elem.val.instr = NULL;
    push_Stack(&state->stack, elem, jbuff);
}

void brop_times(struct ProgramState* state, char* number, size_t numberlen, struct ExceptionHandler* jbuff) {
    if (state->stack.next == 0)
        RAISE(jbuff, StackUnderflow);
    state->stack.next -= 1;
    if (state->stack.content[state->stack.next].type != Instruction) {
        state->stack.next += 1;
        RAISE(jbuff, InvalidOperands);
    }
    char* mem = state->stack.content[state->stack.next].val.instr;
    add_memory(jbuff, mem);
    add_backtrace(jbuff);
    parse_script(state, number, numberlen, jbuff);
    state->stack.next -= 1;
    if (state->stack.content[state->stack.next].type != Integer) {
        state->stack.next += 2;
        RAISE(jbuff, InvalidOperands);
    }
    remove_backtrace(jbuff);
    add_backtrace(jbuff);
    for (int i = 0; i < state->stack.content[state->stack.next].val.ival; i++) {
        parse_script(state, mem, strlen(mem), jbuff);

    }
    remove_backtrace(jbuff);
    remove_memory(jbuff, mem);
}

void brop_dig(struct ProgramState* state, char* number, size_t numberlen, struct ExceptionHandler* jbuff) {
    add_backtrace(jbuff);
    parse_script(state, number, numberlen, jbuff);
    state->stack.next -= 1;
    if (state->stack.content[state->stack.next].type != Integer) {
        state->stack.next += 1;
        RAISE(jbuff, InvalidOperands);
    }
    if (state->stack.next <= state->stack.content[state->stack.next].val.ival){
        state->stack.next += 1;
        RAISE(jbuff, StackUnderflow);
    }
    size_t index = state->stack.next - 1;
    size_t indextar = state->stack.next - 1 - state->stack.content[state->stack.next].val.ival;
    struct StackElem temp = state->stack.content[indextar];
    for (size_t i = indextar; i < index; i++) {
        state->stack.content[i] = state->stack.content[i + 1];
    }
    state->stack.content[index] = temp;
    remove_backtrace(jbuff);
}


void brop_isdef(struct ProgramState *state, char *funcname, size_t fnlen, struct ExceptionHandler *jbuff){
    struct StackElem elem;
    elem.val.instr = NULL;
    char **out = malloc(sizeof(char *));
    int q = get_Environment(&state->env, funcname, fnlen, out);
    free(out);
    elem.type = BoolTrue * q + (1 - q) * BoolFalse;
    push_Stack(&state->stack, elem, jbuff);
}

void brop_define(struct ProgramState *state, char *funcname, size_t fnlen, struct ExceptionHandler *jbuff){
    if(state->stack.next == 0)
        RAISE(jbuff, StackUnderflow);
    state->stack.next -= 1;
    if(state->stack.content[state->stack.next].type != Instruction) {
        state->stack.next += 1;
        RAISE(jbuff, InvalidOperands);
    }
    set_Environment(&state->env, funcname, fnlen, state->stack.content[state->stack.next].val.instr, jbuff);
}

void brop_delete(struct ProgramState *state, char *funcname, size_t fnlen, struct ExceptionHandler *jbuff){
    remove_Environment(&state->env, funcname, fnlen);
}

void brop_load(struct ProgramState *state, char *filename, size_t fnlen, struct ExceptionHandler *jbuff){
    char *path = malloc(fnlen + 1);
    if(path == NULL)
        RAISE(jbuff, ProgramPanic);
    strncpy(path, filename, fnlen);
    path[fnlen] = '\0';
    FILE *target = fopen(path, "r");
    free(path);
    if(target == NULL)
        RAISE(jbuff, FileNotFound);
    fseek(target, 0, SEEK_END);
    long flen = ftell(target);
    if(flen < 0){
        RAISE(jbuff, IOError);
    }else if(flen == 0){
        return;
    }
    char *fcontent = malloc(flen + 1);
    if (fcontent == NULL)
        RAISE(jbuff, ProgramPanic);
    rewind(target);
    size_t comandlen = fread(fcontent, 1, flen, target);
    if(comandlen == 0 || fclose(target) != 0)
        RAISE(jbuff, IOError);
    fcontent[comandlen] = '\0';
    add_memory(jbuff, fcontent);
    add_backtrace(jbuff);
    parse_script(state, fcontent, comandlen, jbuff);
    remove_backtrace(jbuff);
    remove_memory(jbuff, fcontent);
}

void brop_save(struct ProgramState *state, char *filename, size_t fnlen, struct ExceptionHandler *jbuff){
    char *path = malloc(fnlen + 1);
    if(path == NULL)
        RAISE(jbuff, ProgramPanic);
    strncpy(path, filename, fnlen);
    path[fnlen] = '\0';
    FILE *target = fopen(path, "w");
    if(target == NULL)
        RAISE(jbuff, FileNotCreatable);
    for(size_t i = 0; i < state->stack.next; i++){
        if(state->stack.content[i].type == Instruction){
            if(fprintf(target, "[%s] ", state->stack.content[i].val.instr) < 0){
                fclose(target);
                RAISE(jbuff, IOError);
            }
        }else if(state->stack.content[i].type == String){
            if(fprintf(target, "\"%s\" ", state->stack.content[i].val.instr) < 0){
                fclose(target);
                RAISE(jbuff, IOError);
            }
        }else if(state->stack.content[i].type == Integer){
            if(fprintf(target, "%ld ", state->stack.content[i].val.ival) < 0){
                fclose(target);
                RAISE(jbuff, IOError);
            }
        }else if(state->stack.content[i].type == Floating){
            if(fprintf(target, "%lf ", state->stack.content[i].val.fval) < 0){
                fclose(target);
                RAISE(jbuff, IOError);
            }
        }else if(state->stack.content[i].type == BoolTrue){
            if(fprintf(target, "%s ", BOOL[0]) < 0){
                fclose(target);
                RAISE(jbuff, IOError);
            }
        }else if(state->stack.content[i].type == BoolFalse){
            if(fprintf(target, "%s ", BOOL[1]) < 0){
                fclose(target);
                RAISE(jbuff, IOError);
            }
        }
    }
    if(fclose(target) != 0)
        RAISE(jbuff, IOError);
}



void op_roll(struct ProgramState* state, struct ExceptionHandler* jbuff){
    if (state->stack.next == 0) {
        return;
    }
    struct StackElem temp = state->stack.content[state->stack.next - 1];
    for (size_t i = state->stack.next - 1; i > 0 ; i--) {
        state->stack.content[i] = state->stack.content[i - 1];
    }
    state->stack.content[0] = temp;
}

void op_dip(struct ProgramState* state, struct ExceptionHandler* jbuff){
    if (state->stack.next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack.next -= 1;
    if (state->stack.content[state->stack.next].type != Instruction) {
        state->stack.next += 1;
        RAISE(jbuff, InvalidOperands);
    }
    char* mem = state->stack.content[state->stack.next].val.instr;
    state->stack.next -= 1;
    struct StackElem temp = state->stack.content[state->stack.next];
    add_memory(jbuff, mem);
    add_backtrace(jbuff);
    parse_script(state, mem, strlen(mem), jbuff);
    remove_memory(jbuff, mem);
    remove_backtrace(jbuff);
    push_Stack(&state->stack, temp, jbuff);
}

void numop_dig(struct ProgramState* state, size_t num, struct ExceptionHandler* jbuff){
    if (state->stack.next <= num) {
        RAISE(jbuff, StackUnderflow);
    }
    size_t index = state->stack.next - 1;
    size_t indextar = state->stack.next - 1 - num;
    struct StackElem temp = state->stack.content[indextar];
    for (size_t i = indextar; i < index; i++) {
        state->stack.content[i] = state->stack.content[i + 1];
    }
    state->stack.content[index] = temp;
}



void op_if(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack.next < 3)
        RAISE(jbuff, StackUnderflow);
    state->stack.next -= 1;
    if(state->stack.content[state->stack.next].type != Instruction){
        state->stack.next += 1;
        RAISE(jbuff, InvalidOperands);
    }
    char *memf = state->stack.content[state->stack.next].val.instr;
    state->stack.next -= 1;
    if(state->stack.content[state->stack.next].type != Instruction){
        state->stack.next += 2;
        RAISE(jbuff, InvalidOperands);
    }
    char *memt = state->stack.content[state->stack.next].val.instr;
    state->stack.next -= 1;
    if(state->stack.content[state->stack.next].type == BoolTrue){
        add_memory(jbuff, memt);
        add_memory(jbuff, memf);
        add_backtrace(jbuff);
        parse_script(state, memt, strlen(memt), jbuff);
    }else if(state->stack.content[state->stack.next].type == BoolFalse){
        add_memory(jbuff, memt);
        add_memory(jbuff, memf);
        add_backtrace(jbuff);
        parse_script(state, memf, strlen(memf), jbuff);
    }else{
        state->stack.next += 3;
        RAISE(jbuff, InvalidOperands);
    }
    remove_memory(jbuff, memf);
    remove_memory(jbuff, memt);
    remove_backtrace(jbuff);
}

void brop_if(struct ProgramState *state, char *cond, size_t condlen, struct ExceptionHandler *jbuff){
    if(state->stack.next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack.next -= 1;
    if(state->stack.content[state->stack.next].type != Instruction){
        state->stack.next += 1;
        RAISE(jbuff, InvalidOperands);
    }
    char *memf = state->stack.content[state->stack.next].val.instr;
    state->stack.next -= 1;
    if(state->stack.content[state->stack.next].type != Instruction){
        state->stack.next += 2;
        RAISE(jbuff, InvalidOperands);
    }
    char *memt = state->stack.content[state->stack.next].val.instr;
    add_memory(jbuff, memt);
    add_memory(jbuff, memf);
    add_backtrace(jbuff);
    parse_script(state, cond, condlen, jbuff);
    if(state->stack.next < 1)
        RAISE(jbuff, StackUnderflow);
    state->stack.next -= 1;
    if (state->stack.content[state->stack.next].type == BoolTrue) {
        parse_script(state, memt, strlen(memt), jbuff);
    } else if (state->stack.content[state->stack.next].type == BoolFalse){
        parse_script(state, memf, strlen(memf), jbuff);
    } else {
        state->stack.next += 1;
        RAISE(jbuff, InvalidOperands);
    }
    remove_memory(jbuff, memt);
    remove_memory(jbuff, memf);
    remove_backtrace(jbuff);
}

void op_loop(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack.next == 0)
        RAISE(jbuff, StackUnderflow);
    state->stack.next -= 1;
    if(state->stack.content[state->stack.next].type != Instruction){
        state->stack.next += 1;
        RAISE(jbuff, InvalidOperands);
    }
    char *mem = state->stack.content[state->stack.next].val.instr;
    add_memory(jbuff, mem);
    add_backtrace(jbuff);
    while (1){
        parse_script(state, mem, strlen(mem), jbuff);
        state->stack.next -= 1;
        if (state->stack.content[state->stack.next].type == BoolFalse) {
            break;
        } else if (state->stack.content[state->stack.next].type != BoolTrue) {
            state->stack.next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }
    remove_memory(jbuff, mem);
    remove_backtrace(jbuff);
}

void brop_loop(struct ProgramState *state, char *cond, size_t condlen, struct ExceptionHandler *jbuff){
    if(state->stack.next == 0)
        RAISE(jbuff, StackUnderflow);
    state->stack.next -= 1;
    if(state->stack.content[state->stack.next].type != Instruction){
        state->stack.next += 1;
        RAISE(jbuff, InvalidOperands);
    }
    char *mem = state->stack.content[state->stack.next].val.instr;
    add_memory(jbuff, mem);
    add_backtrace(jbuff);
    while (1){
        parse_script(state, cond, condlen, jbuff);
        state->stack.next -= 1;
        if(state->stack.content[state->stack.next].type == BoolFalse){
            break;
        }else if(state->stack.content[state->stack.next].type != BoolTrue){
            state->stack.next += 1;
            RAISE(jbuff, InvalidOperands);
        }
        parse_script(state, mem, strlen(mem), jbuff);
    }
    remove_memory(jbuff, mem);
    remove_backtrace(jbuff);
}

void op_greather(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack.next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack.next -= 1;
    size_t resindex = state->stack.next - 1;
    if(state->stack.content[resindex].type == Integer){
        if(state->stack.content[state->stack.next].type == Floating){
            unsigned result = ((double) state->stack.content[resindex].val.ival) > state->stack.content[state->stack.next].val.fval;
            state->stack.content[resindex].type = BoolTrue*result +(1 - result)*BoolFalse;
        }else if (state->stack.content[state->stack.next].type == Integer){
            unsigned result = state->stack.content[resindex].val.ival > state->stack.content[state->stack.next].val.ival;
            state->stack.content[resindex].type = BoolTrue*result +(1 - result)*BoolFalse;
        }else{
            state->stack.next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else if(state->stack.content[resindex].type == Floating){
        if(state->stack.content[state->stack.next].type == Floating){
            unsigned result = state->stack.content[resindex].val.fval > state->stack.content[state->stack.next].val.fval;
            state->stack.content[resindex].type = BoolTrue*result +(1 - result)*BoolFalse;
        }else if (state->stack.content[state->stack.next].type == Integer){
            unsigned result = state->stack.content[resindex].val.fval > ((double) state->stack.content[state->stack.next].val.ival);
            state->stack.content[resindex].type = BoolTrue*result +(1 - result)*BoolFalse;
        }else{
            state->stack.next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else{
        state->stack.next += 1;
        RAISE(jbuff, InvalidOperands);
    }
}

void op_greathereq(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack.next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack.next -= 1;
    size_t resindex = state->stack.next - 1;
    if(state->stack.content[resindex].type == Integer){
        if(state->stack.content[state->stack.next].type == Floating){
            unsigned result = ((double) state->stack.content[resindex].val.ival) >= state->stack.content[state->stack.next].val.fval;
            state->stack.content[resindex].type = BoolTrue*result +(1 - result)*BoolFalse;
        }else if (state->stack.content[state->stack.next].type == Integer){
            unsigned result = state->stack.content[resindex].val.ival >= state->stack.content[state->stack.next].val.ival;
            state->stack.content[resindex].type = BoolTrue*result +(1 - result)*BoolFalse;
        }else{
            state->stack.next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else if(state->stack.content[resindex].type == Floating){
        if(state->stack.content[state->stack.next].type == Floating){
            unsigned result = state->stack.content[resindex].val.fval >= state->stack.content[state->stack.next].val.fval;
            state->stack.content[resindex].type = BoolTrue*result +(1 - result)*BoolFalse;
        }else if (state->stack.content[state->stack.next].type == Integer){
            unsigned result = state->stack.content[resindex].val.fval >= ((double) state->stack.content[state->stack.next].val.ival);
            state->stack.content[resindex].type = BoolTrue*result +(1 - result)*BoolFalse;
        }else{
            state->stack.next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else{
        state->stack.next += 1;
        RAISE(jbuff, InvalidOperands);
    }
}

void op_lower(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack.next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack.next -= 1;
    size_t resindex = state->stack.next - 1;
    if(state->stack.content[resindex].type == Integer){
        if(state->stack.content[state->stack.next].type == Floating){
            unsigned result = ((double) state->stack.content[resindex].val.ival) < state->stack.content[state->stack.next].val.fval;
            state->stack.content[resindex].type = BoolTrue*result +(1 - result)*BoolFalse;
        }else if (state->stack.content[state->stack.next].type == Integer){
            unsigned result = state->stack.content[resindex].val.ival < state->stack.content[state->stack.next].val.ival;
            state->stack.content[resindex].type = BoolTrue*result +(1 - result)*BoolFalse;
        }else{
            state->stack.next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else if(state->stack.content[resindex].type == Floating){
        if(state->stack.content[state->stack.next].type == Floating){
            unsigned result = state->stack.content[resindex].val.fval < state->stack.content[state->stack.next].val.fval;
            state->stack.content[resindex].type = BoolTrue*result +(1 - result)*BoolFalse;
        }else if (state->stack.content[state->stack.next].type == Integer){
            unsigned result = state->stack.content[resindex].val.fval < ((double) state->stack.content[state->stack.next].val.ival);
            state->stack.content[resindex].type = BoolTrue*result +(1 - result)*BoolFalse;
        }else{
            state->stack.next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else{
        state->stack.next += 1;
        RAISE(jbuff, InvalidOperands);
    }
}

void op_lowereq(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack.next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack.next -= 1;
    size_t resindex = state->stack.next - 1;
    if(state->stack.content[resindex].type == Integer){
        if(state->stack.content[state->stack.next].type == Floating){
            unsigned result = ((double) state->stack.content[resindex].val.ival) <= state->stack.content[state->stack.next].val.fval;
            state->stack.content[resindex].type = BoolTrue*result +(1 - result)*BoolFalse;
        }else if (state->stack.content[state->stack.next].type == Integer){
            unsigned result = state->stack.content[resindex].val.ival <= state->stack.content[state->stack.next].val.ival;
            state->stack.content[resindex].type = BoolTrue*result +(1 - result)*BoolFalse;
        }else{
            state->stack.next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else if(state->stack.content[resindex].type == Floating){
        if(state->stack.content[state->stack.next].type == Floating){
            unsigned result = state->stack.content[resindex].val.fval <= state->stack.content[state->stack.next].val.fval;
            state->stack.content[resindex].type = BoolTrue*result +(1 - result)*BoolFalse;
        }else if (state->stack.content[state->stack.next].type == Integer){
            unsigned result = state->stack.content[resindex].val.fval <= ((double) state->stack.content[state->stack.next].val.ival);
            state->stack.content[resindex].type = BoolTrue*result +(1 - result)*BoolFalse;
        }else{
            state->stack.next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else{
        state->stack.next += 1;
        RAISE(jbuff, InvalidOperands);
    }
}

void op_and(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack.next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack.next -= 1;
    size_t resindex = state->stack.next - 1;
    if(state->stack.content[resindex].type == BoolTrue){
        if(state->stack.content[state->stack.next].type == BoolFalse){
            state->stack.content[resindex].type = BoolFalse;
        }else if(state->stack.content[state->stack.next].type != BoolTrue){
            state->stack.next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else if(state->stack.content[resindex].type == BoolFalse){
        if(state->stack.content[state->stack.next].type != BoolTrue
           && state->stack.content[state->stack.next].type != BoolFalse){
            state->stack.next += 1;
            RAISE(jbuff, InvalidOperands);
        }else{
            state->stack.content[resindex].type = BoolFalse;
        }
    }else{
        state->stack.next += 1;
        RAISE(jbuff, InvalidOperands);
    }
}

void op_or(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack.next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack.next -= 1;
    size_t resindex = state->stack.next - 1;
    if(state->stack.content[resindex].type == BoolTrue){
        if(state->stack.content[state->stack.next].type != BoolTrue
           && state->stack.content[state->stack.next].type != BoolFalse){
            state->stack.next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else if(state->stack.content[resindex].type == BoolFalse){
        if(state->stack.content[state->stack.next].type == BoolTrue){
            state->stack.content[resindex].type = BoolTrue;
        }else if(state->stack.content[state->stack.next].type != BoolFalse){
            state->stack.next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else{
        state->stack.next += 1;
        RAISE(jbuff, InvalidOperands);
    }
}

void op_xor(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack.next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack.next -= 1;
    size_t resindex = state->stack.next - 1;
    if(state->stack.content[resindex].type == BoolTrue){
        if(state->stack.content[state->stack.next].type == BoolTrue){
            state->stack.content[resindex].type = BoolFalse;
        }else if(state->stack.content[state->stack.next].type != BoolFalse){
            state->stack.next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else if(state->stack.content[resindex].type == BoolFalse){
        if(state->stack.content[state->stack.next].type == BoolTrue){
            state->stack.content[resindex].type = BoolTrue;
        }else if(state->stack.content[state->stack.next].type != BoolFalse){
            state->stack.next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else{
        state->stack.next += 1;
        RAISE(jbuff, InvalidOperands);
    }
}

void op_mod(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack.next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack.next -= 1;
    size_t resindex = state->stack.next - 1;
    if(state->stack.content[resindex].type != Integer || state->stack.content[state->stack.next].type != Integer) {
        state->stack.next += 1;
        RAISE(jbuff, InvalidOperands);
    }
    if(state->stack.content[state->stack.next].val.ival == 0) {
        state->stack.next += 1;
        RAISE(jbuff, ValueError);
    }
    state->stack.content[resindex].val.ival =
                state->stack.content[resindex].val.ival % state->stack.content[state->stack.next].val.ival;
}

void op_not(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack.next == 0)
        RAISE(jbuff, StackUnderflow);
    size_t resindex = state->stack.next - 1;
    if(state->stack.content[resindex].type == BoolTrue){
        state->stack.content[resindex].type = BoolFalse;
    }else if(state->stack.content[resindex].type == BoolFalse){
        state->stack.content[resindex].type = BoolTrue;
    }else{
        RAISE(jbuff, InvalidOperands);
    }
}

void op_int(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack.next == 0)
        RAISE(jbuff, StackUnderflow);
    size_t resindex = state->stack.next - 1;
    if(state->stack.content[resindex].type == Floating){
        int64_t temp = (int64_t) state->stack.content[resindex].val.fval;
        state->stack.content[resindex].type = Integer;
        state->stack.content[resindex].val.ival = temp;
    }else if(state->stack.content[resindex].type != Integer){
        RAISE(jbuff, InvalidOperands);
    }
}

void op_quote(struct ProgramState *state, struct ExceptionHandler *jbuff) {
    if (state->stack.next == 0)
        RAISE(jbuff, StackUnderflow);
    size_t resindex = state->stack.next - 1;
    size_t finallen;
    char *resstr;
    char buffer[1];
    int result;
    switch (state->stack.content[resindex].type)
    {
    case String:
        finallen = strlen(state->stack.content[resindex].val.instr) + 5;
        resstr = malloc(finallen);
        if (resstr == NULL) {
            RAISE(jbuff, ProgramPanic);
        } else {
            strcpy(resstr + 2, state->stack.content[resindex].val.instr);
            resstr[0] = '[';
            resstr[1] = '"';
            resstr[finallen - 3] = '"';
            resstr[finallen - 2] = ']';
            resstr[finallen - 1] = '\0';
            free(state->stack.content[resindex].val.instr);
            state->stack.content[resindex].val.instr = resstr;
        }
        break;
    case Instruction:
        finallen = strlen(state->stack.content[resindex].val.instr) + 3;
        resstr = malloc(finallen);
        if (resstr == NULL) {
            RAISE(jbuff, ProgramPanic);
        } else {
            strcpy(resstr + 1, state->stack.content[resindex].val.instr);
            resstr[0] = '[';
            resstr[finallen - 2] = ']';
            resstr[finallen - 1] = '\0';
            free(state->stack.content[resindex].val.instr);
            state->stack.content[resindex].val.instr = resstr;
        }
        break;
    case Integer:
        finallen = (int) log10((double)state->stack.content[resindex].val.ival + 1) + 1 + (state->stack.content[resindex].val.ival < 0);
        resstr = malloc(finallen + 1);
        if (resstr == NULL) {
            RAISE(jbuff, ProgramPanic);
        } else {
            snprintf(resstr, finallen + 1, "%ld", state->stack.content[resindex].val.ival);
            state->stack.content[resindex].val.instr = resstr;
            state->stack.content[resindex].type = Instruction;
        }
        break;
    case Floating:
        result = snprintf(buffer, 1, "%lf", state->stack.content[resindex].val.fval);
        if (result < 1) {
            RAISE(jbuff, ProgramPanic);
        } else {
            char *resstr = malloc(result + 1);
            if (resstr == NULL) {
                RAISE(jbuff, ProgramPanic);
            } else {
                snprintf(resstr, result + 1, "%lf", state->stack.content[resindex].val.fval);
                resstr[result] ='\0';
                state->stack.content[resindex].val.instr = resstr;
                state->stack.content[resindex].type = Instruction;
            }
        }
        break;
    case BoolFalse:
        state->stack.content[resindex].val.instr = malloc(6);
        if (state->stack.content[resindex].val.instr == NULL) {
            RAISE(jbuff, ProgramPanic);
        } else {
            strcpy(state->stack.content[resindex].val.instr, BOOL[1]);
            state->stack.content[resindex].type = Instruction;
        }
        break;
    case BoolTrue:
        state->stack.content[resindex].val.instr = malloc(5);
        if (state->stack.content[resindex].val.instr == NULL) {
            RAISE(jbuff, ProgramPanic);
        } else {
            strcpy(state->stack.content[resindex].val.instr, BOOL[0]);
            state->stack.content[resindex].type = Instruction;
        }
        break;
    
    default:
        UNREACHABLE;
    }
}

void op_try(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack.next == 0)
        RAISE(jbuff, StackUnderflow);
    state->stack.next -= 1;
    if(state->stack.content[state->stack.next].type != Instruction){
        state->stack.next += 1;
        RAISE(jbuff, InvalidOperands);
    }
    struct ExceptionHandler *try_buf = init_ExceptionHandler();
    if(try_buf == NULL) {
        state->stack.next += 1;
        RAISE(jbuff, ProgramPanic);
    }
    char *mem = state->stack.content[state->stack.next].val.instr;
    struct StackElem result;
    result.val.instr = NULL;
    TRY(try_buf){
        parse_script(state, mem, strlen(mem), try_buf);
        result.type = BoolTrue;
    }CATCHALL{
        result.type = BoolFalse;
    }
    free(mem);
    free(try_buf);
    push_Stack(&state->stack, result, jbuff);
}

void op_sum(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack.next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack.next -= 1;
    size_t resindex = state->stack.next - 1;
    if(state->stack.content[resindex].type == Floating){
        if(state->stack.content[state->stack.next].type == Floating){
            state->stack.content[resindex].val.fval = state->stack.content[resindex].val.fval + state->stack.content[state->stack.next].val.fval;
        }else if(state->stack.content[state->stack.next].type == Integer) {
            state->stack.content[resindex].val.fval = state->stack.content[resindex].val.fval + (double) state->stack.content[state->stack.next].val.ival;
        }else{
            state->stack.next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else if(state->stack.content[resindex].type == Integer){
        if(state->stack.content[state->stack.next].type == Floating){
            state->stack.content[resindex].type = Floating;
            state->stack.content[resindex].val.fval = (double) state->stack.content[resindex].val.ival + state->stack.content[state->stack.next].val.fval;
        }else if(state->stack.content[state->stack.next].type == Integer) {
            state->stack.content[resindex].val.ival = state->stack.content[resindex].val.ival + state->stack.content[state->stack.next].val.ival;
        }else{
            state->stack.next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else{
        state->stack.next += 1;
        RAISE(jbuff, InvalidOperands);
    }
}

void op_sub(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack.next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack.next -= 1;
    size_t resindex = state->stack.next - 1;
    if(state->stack.content[resindex].type == Floating){
        if(state->stack.content[state->stack.next].type == Floating){
            state->stack.content[resindex].val.fval = state->stack.content[resindex].val.fval - state->stack.content[state->stack.next].val.fval;
        }else if(state->stack.content[state->stack.next].type == Integer) {
            state->stack.content[resindex].val.fval = state->stack.content[resindex].val.fval - (double) state->stack.content[state->stack.next].val.ival;
        }else{
            state->stack.next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else if(state->stack.content[resindex].type == Integer){
        if(state->stack.content[state->stack.next].type == Floating){
            state->stack.content[resindex].type = Floating;
            state->stack.content[resindex].val.fval = (double) state->stack.content[resindex].val.ival - state->stack.content[state->stack.next].val.fval;
        }else if(state->stack.content[state->stack.next].type == Integer) {
            state->stack.content[resindex].val.ival = state->stack.content[resindex].val.ival - state->stack.content[state->stack.next].val.ival;
        }else{
            state->stack.next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else{
        state->stack.next += 1;
        RAISE(jbuff, InvalidOperands);
    }
}

void op_mul(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack.next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack.next -= 1;
    size_t resindex = state->stack.next - 1;
    if(state->stack.content[resindex].type == Floating){
        if(state->stack.content[state->stack.next].type == Floating){
            state->stack.content[resindex].val.fval = state->stack.content[resindex].val.fval * state->stack.content[state->stack.next].val.fval;
        }else if(state->stack.content[state->stack.next].type == Integer) {
            state->stack.content[resindex].val.fval = state->stack.content[resindex].val.fval * (double) state->stack.content[state->stack.next].val.ival;
        }else{
            state->stack.next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else if(state->stack.content[resindex].type == Integer) {
        if(state->stack.content[state->stack.next].type == Floating) {
            state->stack.content[resindex].type = Floating;
            state->stack.content[resindex].val.fval =
                    (double) state->stack.content[resindex].val.ival * state->stack.content[state->stack.next].val.fval;
        }else if(state->stack.content[state->stack.next].type == Integer) {
            state->stack.content[resindex].val.ival =
                    state->stack.content[resindex].val.ival * state->stack.content[state->stack.next].val.ival;

        } else {
            state->stack.next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else{
        state->stack.next += 1;
        RAISE(jbuff, InvalidOperands);
    }
}

void op_sqrt(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack.next == 0)
        RAISE(jbuff, StackUnderflow);
    size_t resindex = state->stack.next - 1;
    if(state->stack.content[resindex].type == Floating){
        if(state->stack.content[resindex].val.fval == 0){
            RAISE(jbuff, ValueError);
        }else {
            state->stack.content[resindex].val.fval = sqrt(state->stack.content[resindex].val.fval);
        }
    }else if(state->stack.content[resindex].type == Integer){
        if(state->stack.content[resindex].val.ival == 0){
            RAISE(jbuff, ValueError);
        }else {
            state->stack.content[resindex].type = Floating;
            state->stack.content[resindex].val.fval = sqrt((double) state->stack.content[resindex].val.ival);
        }
    }else{
        RAISE(jbuff, InvalidOperands);
    }
}

void op_pow(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack.next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack.next -= 1;
    size_t resindex = state->stack.next - 1;
    if(state->stack.content[resindex].type == Floating){
        if(state->stack.content[state->stack.next].type == Floating){
            state->stack.content[resindex].val.fval = pow(state->stack.content[resindex].val.fval, state->stack.content[state->stack.next].val.fval);
        }else if(state->stack.content[state->stack.next].type == Integer) {
            state->stack.content[resindex].val.fval = pow(state->stack.content[resindex].val.fval, (double) state->stack.content[state->stack.next].val.ival);
        }else{
            state->stack.next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else if(state->stack.content[resindex].type == Integer) {
        if(state->stack.content[state->stack.next].type == Floating) {
            state->stack.content[resindex].type = Floating;
            state->stack.content[resindex].val.fval =
                    pow((double) state->stack.content[resindex].val.ival, state->stack.content[state->stack.next].val.fval);
        }else if(state->stack.content[state->stack.next].type == Integer) {
            state->stack.content[resindex].type = Floating;
            state->stack.content[resindex].val.fval =
                    pow((double) state->stack.content[resindex].val.ival, (double) state->stack.content[state->stack.next].val.ival);

        } else {
            state->stack.next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else{
        state->stack.next += 1;
        RAISE(jbuff, InvalidOperands);
    }
}

void op_div(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack.next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack.next -= 1;
    size_t resindex = state->stack.next - 1;
    if(state->stack.content[state->stack.next].type == Floating){
        if(state->stack.content[state->stack.next].val.fval == 0){
            state->stack.next += 1;
            RAISE(jbuff, ValueError);
        }else if(state->stack.content[resindex].type == Floating){
            state->stack.content[resindex].val.fval = state->stack.content[resindex].val.fval / state->stack.content[state->stack.next].val.fval;
        }else if(state->stack.content[resindex].type == Integer){
            state->stack.content[resindex].type = Floating;
            state->stack.content[resindex].val.fval = (double) state->stack.content[resindex].val.ival / state->stack.content[state->stack.next].val.fval;
        }else{
            state->stack.next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else if(state->stack.content[state->stack.next].type == Integer){
        if(state->stack.content[state->stack.next].val.ival == 0){
            state->stack.next += 1;
            RAISE(jbuff, ValueError);
        }else if(state->stack.content[resindex].type == Floating){
            state->stack.content[resindex].val.fval = state->stack.content[resindex].val.fval / (double) state->stack.content[state->stack.next].val.ival;
        }else if(state->stack.content[resindex].type == Integer){
            state->stack.content[resindex].type = Floating;
            state->stack.content[resindex].val.fval = (double) state->stack.content[resindex].val.ival / (double) state->stack.content[state->stack.next].val.ival;
        }else{
            state->stack.next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }else{
        state->stack.next += 1;
        RAISE(jbuff, InvalidOperands);
    }
}

void op_compose(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack.next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack.next -= 1;
    if((state->stack.content[state->stack.next].type == Instruction && state->stack.content[state->stack.next - 1].type == Instruction)
        || (state->stack.content[state->stack.next].type == String && state->stack.content[state->stack.next - 1].type == String)){
        size_t lensecond = strlen(state->stack.content[state->stack.next].val.instr);
        size_t lenfirst =  strlen(state->stack.content[state->stack.next - 1].val.instr);
        char *composte = realloc(state->stack.content[state->stack.next - 1].val.instr, lensecond + lenfirst + 2);
        if(composte == NULL){
            RAISE(jbuff, ProgramPanic);
        }
        state->stack.content[state->stack.next - 1].val.instr = composte;
        state->stack.content[state->stack.next - 1].val.instr[lenfirst] = ' ';
        strcpy(state->stack.content[state->stack.next - 1].val.instr + lenfirst + 1, state->stack.content[state->stack.next].val.instr);
        free(state->stack.content[state->stack.next].val.instr);
        state->stack.content[state->stack.next - 1].val.instr[lensecond + lenfirst + 1] = '\0';
    }else{
        state->stack.next += 1;
        RAISE(jbuff, InvalidOperands);
    }
}

void op_apply(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack.next == 0)
        RAISE(jbuff, StackUnderflow);
    state->stack.next -= 1;
    if(state->stack.content[state->stack.next].type != Instruction){
        state->stack.next += 1;
        RAISE(jbuff, InvalidOperands);
    }
    char *mem = state->stack.content[state->stack.next].val.instr;
    add_memory(jbuff, mem);
    add_backtrace(jbuff);
    parse_script(state, mem, strlen(mem), jbuff);
    remove_memory(jbuff, mem);
    remove_backtrace(jbuff);
}

void op_equal(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack.next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack.next -= 1;
    size_t resindex = state->stack.next - 1;
    enum ElemType result;
    result = BoolFalse;
    switch (state->stack.content[state->stack.next].type)
    {
    case String:
        if(state->stack.content[resindex].type == String) {
            if (strcmp(state->stack.content[state->stack.next].val.instr, state->stack.content[resindex].val.instr) == 0)
                result = BoolTrue;
            free(state->stack.content[state->stack.next].val.instr);
        }
        break;
    case Instruction:
        if(state->stack.content[resindex].type == Instruction) {
            if (strcmp(state->stack.content[state->stack.next].val.instr, state->stack.content[resindex].val.instr) == 0)
                result = BoolTrue;
            free(state->stack.content[state->stack.next].val.instr);
        }
        break;
    case Integer:
        if(state->stack.content[resindex].type == Integer){
            if(state->stack.content[state->stack.next].val.ival == state->stack.content[resindex].val.ival)
                result = BoolTrue;
        } else if(state->stack.content[resindex].type == Floating){
            if((double) state->stack.content[state->stack.next].val.ival == state->stack.content[resindex].val.fval)
                result = BoolTrue;
        }
        break;
    case Floating:
        if(state->stack.content[resindex].type == Integer){
            if(state->stack.content[state->stack.next].val.fval == (double) state->stack.content[resindex].val.ival)
                result = BoolTrue;
        } else if(state->stack.content[resindex].type == Floating){
            if(state->stack.content[state->stack.next].val.fval == state->stack.content[resindex].val.fval)
                result = BoolTrue;
        }
        break;
    case BoolFalse:
    case BoolTrue:
        if(state->stack.content[state->stack.next].type == state->stack.content[resindex].type){
            result = BoolTrue;
        }
        break;
    default:
        UNREACHABLE;
    }
    if(state->stack.content[resindex].type == Instruction || state->stack.content[resindex].type == String){
        free(state->stack.content[resindex].val.instr);
    }
    state->stack.content[resindex].type = result;
    state->stack.content[resindex].val.instr = NULL;
}

void op_notequal(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack.next < 2){
        RAISE(jbuff, StackUnderflow);
    }
    state->stack.next -= 1;
    size_t resindex = state->stack.next - 1;
    enum ElemType result = BoolTrue;
    switch (state->stack.content[state->stack.next].type)
    {
    case String:
        if(state->stack.content[resindex].type == String) {
            if (strcmp(state->stack.content[state->stack.next].val.instr, state->stack.content[resindex].val.instr) == 0)
                result = BoolFalse;
            free(state->stack.content[state->stack.next].val.instr);
        }
        break;
    case Instruction:
        if(state->stack.content[resindex].type == Instruction) {
            if (strcmp(state->stack.content[state->stack.next].val.instr, state->stack.content[resindex].val.instr) == 0)
                result = BoolFalse;
            free(state->stack.content[state->stack.next].val.instr);
        }
        break;
    case Integer:
        if(state->stack.content[resindex].type == Integer){
            if(state->stack.content[state->stack.next].val.ival == state->stack.content[resindex].val.ival)
                result = BoolFalse;
        } else if(state->stack.content[resindex].type == Floating){
            if((double) state->stack.content[state->stack.next].val.ival == state->stack.content[resindex].val.fval)
                result = BoolFalse;
        }
        break;
    case Floating:
        if(state->stack.content[resindex].type == Integer){
            if(state->stack.content[state->stack.next].val.fval == (double) state->stack.content[resindex].val.ival)
                result = BoolFalse;
        } else if(state->stack.content[resindex].type == Floating){
            if(state->stack.content[state->stack.next].val.fval == state->stack.content[resindex].val.fval)
                result = BoolFalse;
        }
        break;
    case BoolFalse:
    case BoolTrue:
        if(state->stack.content[state->stack.next].type == state->stack.content[resindex].type){
            result = BoolFalse;
        }
        break;
    default:
        UNREACHABLE;
    }
    if(state->stack.content[resindex].type == Instruction || state->stack.content[resindex].type == String){
        free(state->stack.content[resindex].val.instr);
    }
    state->stack.content[resindex].type = result;
    state->stack.content[resindex].val.instr = NULL;
}

void op_size(struct ProgramState *state, struct ExceptionHandler *jbuff){
    struct StackElem siz;
    siz.type = Integer;
    siz.val.ival = (int64_t)state->stack.next;
    push_Stack(&state->stack, siz, jbuff);
}

void op_empty(struct ProgramState *state, struct ExceptionHandler *jbuff){
    struct StackElem siz;
    siz.val.instr = NULL;
    uint32_t q = (state->stack.next == 0);
    siz.type = BoolTrue * q + BoolFalse * (1 - q);
    push_Stack(&state->stack, siz, jbuff);
}

void op_dup(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack.next == 0)
        RAISE(jbuff, StackUnderflow);
    struct StackElem copy;
    copy.type = state->stack.content[state->stack.next - 1].type;
    if (copy.type == Instruction) {
        size_t srclen = strlen(state->stack.content[state->stack.next - 1].val.instr) + 1;
        copy.val.instr = malloc(srclen);
        if (copy.val.instr == NULL)
            RAISE(jbuff, ProgramPanic);
        memcpy(copy.val.instr, state->stack.content[state->stack.next - 1].val.instr, srclen);
    }
    else {
        copy.val = state->stack.content[state->stack.next - 1].val;
    }
    push_Stack(&state->stack, copy, jbuff);
}

void op_top(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack.next == 0)
        RAISE(jbuff, StackUnderflow);
    struct StackElem copy;
    copy.type = state->stack.content[0].type;
    copy.val = state->stack.content[0].val;
    push_Stack(&state->stack, copy, jbuff);
}

void op_swap(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack.next < 2)
        RAISE(jbuff, StackUnderflow);
    size_t index1 = state->stack.next - 1;
    size_t index2 = index1 - 1;
    struct StackElem temp;
    temp = state->stack.content[index1];
    state->stack.content[index1] = state->stack.content[index2];
    state->stack.content[index2] = temp;
}

void numop_dup(struct ProgramState *state, size_t num, struct ExceptionHandler *jbuff){
    if(num >= state->stack.next)
        RAISE(jbuff, StackUnderflow);
    struct StackElem copy;
    size_t index = state->stack.next - 1 - num;
    copy.type = state->stack.content[index].type;
    if (copy.type == Instruction || copy.type == String) {
        size_t srclen = strlen(state->stack.content[index].val.instr) + 1;
        copy.val.instr = malloc(srclen);
        if (copy.val.instr == NULL)
            RAISE(jbuff, ProgramPanic);
        memcpy(copy.val.instr, state->stack.content[index].val.instr, srclen);
    }
    else {
        copy.val = state->stack.content[index].val;
    }
    push_Stack(&state->stack, copy, jbuff);
}

void numop_swap(struct ProgramState *state, size_t num, struct ExceptionHandler *jbuff){
    if(num >= state->stack.next)
        RAISE(jbuff, StackUnderflow);
    size_t index1 = state->stack.next - 1;
    size_t index2 = index1 - num;
    struct StackElem temp;
    temp = state->stack.content[index1];
    state->stack.content[index1] = state->stack.content[index2];
    state->stack.content[index2] = temp;
}

void brop_dup(struct ProgramState *state, char *comand, size_t clen, struct ExceptionHandler *jbuff){
    parse_script(state, comand, clen, jbuff);
    if(state->stack.next < 1)
        RAISE(jbuff, StackUnderflow);
    state->stack.next -= 1;
    if(state->stack.content[state->stack.next].type == Integer) {
        if (state->stack.content[state->stack.next].val.ival >= state->stack.next)
            RAISE(jbuff, StackUnderflow);
        struct StackElem copy;
        size_t index = state->stack.next - 1 - state->stack.content[state->stack.next].val.ival;
        copy.type = state->stack.content[index].type;
        if (copy.type == Instruction || copy.type == String) {
            size_t srclen = strlen(state->stack.content[index].val.instr) + 1;
            copy.val.instr = malloc(srclen);
            memcpy(copy.val.instr, state->stack.content[index].val.instr, srclen);
        }
        else {
            copy.val = state->stack.content[index].val;
        }
        push_Stack(&state->stack, copy, jbuff);
    }else{
        state->stack.next += 1;
        RAISE(jbuff, InvalidOperands);
    }
}

void brop_swap(struct ProgramState *state, char *comand, size_t clen, struct ExceptionHandler *jbuff){
    parse_script(state, comand, clen, jbuff);
    if(state->stack.next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack.next -= 1;
    if(state->stack.content[state->stack.next].type == Integer) {
        if(state->stack.content[state->stack.next].val.ival >= state->stack.next)
            RAISE(jbuff, StackUnderflow);
        size_t index1 = state->stack.next - 1;
        size_t index2 = index1 - state->stack.content[state->stack.next].val.ival;
        struct StackElem temp;
        temp = state->stack.content[index1];
        state->stack.content[index1] = state->stack.content[index2];
        state->stack.content[index2] = temp;
    }else{
        state->stack.next += 1;
        RAISE(jbuff, InvalidOperands);
    }

}


void op_drop(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack.next == 0)
        RAISE(jbuff, StackUnderflow);
    state->stack.next -= 1;
    if(state->stack.content[state->stack.next].type == Instruction || state->stack.content[state->stack.next].type == String)
        free(state->stack.content[state->stack.next].val.instr);
}

void op_clear(struct ProgramState *state, struct ExceptionHandler *jbuff){
    for(size_t i = 0; i < state->stack.next; i++){
        if(state->stack.content[i].type == Instruction || state->stack.content[i].type == String)
            free(state->stack.content[i].val.instr);
    }
    state->stack.next = 0;
}

void op_nop(struct ProgramState *state, struct ExceptionHandler *jbuff){
}

void op_exit(struct ProgramState *state, struct ExceptionHandler *jbuff){
    RAISE(jbuff, ProgramExit);
}

void op_print(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack.next != 0)
        print_single(&state->stack, 1);
}

void op_printall(struct ProgramState *state, struct ExceptionHandler *jbuff){
    for(size_t i = state->stack.next; i > 0; i--){
        print_single(&state->stack, i);
    }
}

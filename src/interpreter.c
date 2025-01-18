//
// Created by P4o1o on 14/05/2024.
//
#include "interpreter.h"
#include <math.h>
#include <errno.h>

#define INNER_STACK_CAPACITY 256

#define NUMBERED_SIZE 5
char *NUMBERED_INSTR[] = {
        "dup", "swap", "dig", "inject", "pinject"
};
const num_operations NUM_INSTR_OP[] = {
        numop_dup, numop_swap, numop_dig, numop_inject, numop_pinject
};
#define NUMOP_MAP_SIZE 16

#define BRACKETS_SIZE 12
char *BRACKETS_INSTR[] = {
        "load","if","save","compose",
        "delete","isdef","loop","split",
        "swap","define","dup", "times", "dig"
};
const br_operations BR_INSTR_OP[] ={
        brop_load, brop_if, brop_save, brop_compose,
        brop_delete, brop_isdef, brop_loop, brop_split,
        brop_swap, brop_define, brop_dup, brop_times, brop_dig
};
#define BROP_MAP_SIZE 32

#define INSTR_SIZE 74
char* INSTRUCTIONS[] = {
        "int", "clear", "quote", "<=", "dup",
        "or", "swap", "+", "and", "dip",
        "exit", "nop", "print", "size", "try",
        "%", "/", ">", "apply", "compose",
        "drop", "empty", "if", "loop", "not",
        "pow", "printall", "roll", "sqrt", "top",
        "xor", "!=", "*", "-", "<",
        "==", ">=", "true", "false", "split",
        "stack", "push", "pop", "inject", "compress",
        "none", "type", "INSTR", "INT", "FLOAT",
        "BOOL", "STR", "TYPE", "NONE", "STACK",
        "sin", "cos", "tan", "arcsin", "arccos",
        "arctan", "sinh", "cosh", "tanh", "arcsinh",
        "arccosh", "arctanh", "exp", "--", "!",
        "gamma", "log", "log2", "log10"
};
const operations INSTR_OP[] ={
        op_int, op_clear, op_quote, op_lowereq, op_dup,
        op_or, op_swap, op_sum, op_and, op_dip,
        op_exit, op_nop, op_print, op_size, op_try,
        op_mod, op_div, op_greather, op_apply, op_compose,
        op_drop, op_empty, op_if, op_loop, op_not,
        op_pow, op_printall, op_roll, op_sqrt, op_top,
        op_xor, op_notequal, op_mul, op_sub, op_lower,
        op_equal, op_greathereq, op_true, op_false, op_split,
        op_stack, op_push, op_pop, op_inject, op_compress,
        op_none, op_type, op_INSTR, op_INT, op_FLOAT,
        op_BOOL, op_STR, op_TYPE, op_NONE, op_STACK,
        op_sin, op_cos, op_tan, op_arcsin, op_arccos,
        op_arctan, op_sinh, op_cosh, op_tanh, op_arcsinh,
        op_arccosh, op_arctanh, op_exp, op_opposite, op_factorial,
        op_gamma, op_log, op_log2, op_log10
};
#define OP_MAP_SIZE 128

#define IS_INDENT(c) ((c) == ' ' || (c) == '\t' || (c) == '\r' || (c) == '\n' || (c) == '\0')

#define RESERVED_CHAR(c) (IS_INDENT(c) || (c) == '[' || (c) == ']' || (c) == '{' || (c) == '}' || (c) == '(' || (c) == ')' || (c) == '"')

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
        elem->key_len = strlen(elem->key);
        builtins.op_map[index] = elem;
    }

    builtins.brop_map = malloc(BROP_MAP_SIZE * sizeof(struct BrOperationElem*));
    if (builtins.brop_map == NULL)
        return 0;
    for (size_t i = 0; i < BROP_MAP_SIZE; i++) {
        builtins.brop_map[i] = NULL;
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
        elem->key_len = strlen(elem->key);
        builtins.brop_map[index] = elem;
    }

    builtins.numop_map = malloc(NUMOP_MAP_SIZE * sizeof(struct NumOperationElem*));
    if (builtins.numop_map == NULL)
        return 0;
    for (size_t i = 0; i < NUMOP_MAP_SIZE; i++) {
        builtins.numop_map[i] = NULL;
    }
    for (size_t i = 0; i < NUMBERED_SIZE; i++) {
        uint64_t index = SipHash_2_4(HASHKEY_OP0, HASHKEY_OP1, NUMBERED_INSTR[i], strlen(NUMBERED_INSTR[i])) & (NUMOP_MAP_SIZE - 1);
        struct NumOperationElem* elem = builtins.numop_map[index];
        while (elem != NULL) {
            if (strcmp(NUMBERED_INSTR[i], elem->key) == 0) {
                return 0;
            }
            elem = elem->next;
        }
        elem = malloc(sizeof(struct NumOperationElem));
        if (elem == NULL)
            return 0;
        elem->key = NUMBERED_INSTR[i];
        elem->numop = NUM_INSTR_OP[i];
        elem->next = builtins.numop_map[index];
        elem->key_len = strlen(elem->key);
        builtins.numop_map[index] = elem;
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

    for (size_t i = 0; i < NUMOP_MAP_SIZE; i++) {
        struct NumOperationElem* elem = builtins.numop_map[i];
        while (elem != NULL) {
            struct NumOperationElem* temp = elem->next;
            free(elem);
            elem = temp;
        }
    }
    free(builtins.numop_map);
}

//---------------------------------------------------------------------------------------------------------------------------------------------------------


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

static inline void add_backtrace(struct ExceptionHandler *jbuff){
    jbuff->bt_size += 1;
    if(jbuff->bt_size >= jbuff->bt_capacity){
        jbuff->bt_capacity *= 2;
        jbuff->not_exec = realloc(jbuff->not_exec, sizeof(char *) * jbuff->bt_capacity);
    }
}

static inline void remove_backtrace(struct ExceptionHandler *jbuff){
    jbuff->bt_size -= 1;
}

static inline void add_memory(struct ExceptionHandler *jbuff, char *mem){
    size_t index = ((size_t) mem) % OM_VEC_CAPACITY;
    struct OpenMemMap *elem = malloc(sizeof(struct OpenMemMap));
    if (elem == NULL)
        RAISE(jbuff, ProgramPanic);
    elem->openmem = mem;
    elem->next = jbuff->openmemmap[index];
    jbuff->openmemmap[index] = elem;
}

static inline int remove_memory(struct ExceptionHandler *jbuff, char *mem){
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

static inline struct StackElem new_Stack(struct ExceptionHandler *jbuff){
    struct StackElem elem;
    elem.type = InnerStack;
    elem.val.stack = malloc(sizeof(struct Stack));
    if(elem.val.stack == NULL)
        RAISE(jbuff, ProgramPanic);
    elem.val.stack->capacity = INNER_STACK_CAPACITY;
    elem.val.stack->next = 0;
    elem.val.stack->content = malloc(sizeof(struct StackElem) * elem.val.stack->capacity);
    if(elem.val.stack->content == NULL)
        RAISE(jbuff, ProgramPanic);
    return elem;
}

//------------------------------------------------------------------------------------------------------

void execute_instr(struct ProgramState *state, struct Token *token, struct ExceptionHandler *jbuff){
    jbuff->not_exec[jbuff->bt_size - 1] = token->instr;
    struct StackElem elem;
    size_t index;
    switch (token->type){
        case StringToken:
            elem.type = String;
            elem.val.instr = malloc(token->info.stringlen + 1);
            if(elem.val.instr == NULL)
                RAISE(jbuff, ProgramPanic);
            strncpy(elem.val.instr, token->instr, token->info.stringlen);
            elem.val.instr[token->info.stringlen] = '\0';
            push_Stack(state->stack, elem, jbuff);
            break;
        
        case InstrToken:
            elem.type = Instruction;
            elem.val.instr = malloc(token->info.stringlen + 1);
            if(elem.val.instr == NULL)
                RAISE(jbuff, ProgramPanic);
            strncpy(elem.val.instr, token->instr, token->info.stringlen);
            elem.val.instr[token->info.stringlen] = '\0';
            push_Stack(state->stack, elem, jbuff);
            break;
        
        case BrInstrToken:
            index = (size_t)(SipHash_2_4(HASHKEY_BROP0, HASHKEY_BROP1, token->instr, token->info.special.val) & (BROP_MAP_SIZE - 1));
            struct BrOperationElem* bropelem = builtins.brop_map[index];
            while (bropelem != NULL) {
                if (strncmp(token->instr, bropelem->key, token->info.special.val) == 0 && token->info.special.val == bropelem->key_len) {
                    bropelem->brop(state, token->instr + token->info.special.val + 1, token->info.special.instrlen, jbuff);
                    return;
                }
                bropelem = bropelem->next;
            }
            char** funct = malloc(sizeof(char*));
            if (funct == NULL)
                RAISE(jbuff, ProgramPanic);
            if (get_Environment(state->env, token->instr, token->info.stringlen, funct) == 1) {
                char* text = *funct;
                free(funct);
                add_backtrace(jbuff);
                parse_script(state, text, strlen(text), jbuff);
                remove_backtrace(jbuff);
                return;
            }else{
                free(funct);
                RAISE(jbuff, InvalidInstruction);
            }
            break;
        
        case StackToken:
            elem = new_Stack(jbuff);
            struct ProgramState sstat;
            sstat.stack = elem.val.stack;
            sstat.env = state->env;
            add_backtrace(jbuff);
            parse_script(&sstat, token->instr, token->info.stringlen, jbuff);
            remove_backtrace(jbuff);
            push_Stack(state->stack, elem, jbuff);
            break;
        
        case IntegerToken:
            elem.type = Integer;
            elem.val.ival = token->info.integer;
            push_Stack(state->stack, elem, jbuff);
            break;

        case DecimalToken:
            elem.type = Floating;
            elem.val.fval = token->info.decimal;
            push_Stack(state->stack, elem, jbuff);
            break;
        case NumInsrtToken:
            index = (size_t)(SipHash_2_4(HASHKEY_BROP0, HASHKEY_BROP1, token->instr, token->info.special.instrlen) & (NUMOP_MAP_SIZE - 1));
            struct NumOperationElem* numopelem = builtins.numop_map[index];
            while (numopelem != NULL) {
                if (strncmp(token->instr, numopelem->key, token->info.special.instrlen) == 0 && token->info.special.instrlen == numopelem->key_len) {
                    numopelem->numop(state, token->info.special.val, jbuff);
                    return;
                }
                numopelem = numopelem->next;
            }
            RAISE(jbuff, InvalidInstruction);
            break;
        case GenericToken:
            index = (size_t)(SipHash_2_4(HASHKEY_OP0, HASHKEY_OP1, token->instr, token->info.stringlen) & (OP_MAP_SIZE - 1));
            struct OperationElem* opelem = builtins.op_map[index];
            while (opelem != NULL) {
                if (strncmp(token->instr, opelem->key, token->info.stringlen) == 0 && token->info.stringlen == opelem->key_len) { // OP
                    opelem->op(state, jbuff);
                    return;
                }
                opelem = opelem->next;
            }
            char** funct = malloc(sizeof(char*));
            if (funct == NULL)
                RAISE(jbuff, ProgramPanic);
            if (get_Environment(state->env, token->instr, token->info.stringlen, funct) == 1) {
                char* text = *funct;
                free(funct);
                add_backtrace(jbuff);
                parse_script(state, text, strlen(text), jbuff);
                remove_backtrace(jbuff);
                return;
            }else{
                free(funct);
                RAISE(jbuff, InvalidInstruction);
            }
            break;
        
        default:
            UNREACHABLE;
    }
}

struct Token stringToken(char *comand, size_t *clen, struct ExceptionHandler *jbuff){
    struct Token res;
    res.type = StringToken;
    res.instr = comand + 1;
    for(size_t i = 1; i < *clen; i++){
        if(comand[i] == '"'){
            res.info.stringlen = i - 1;
            *clen = i + 1;
            return res;
        }
    }
    RAISE(jbuff, StringQuotingError);
}

struct Token instrToken(char *comand, size_t *clen, struct ExceptionHandler *jbuff){
    struct Token res;
    res.type = InstrToken;
    res.instr = comand + 1;
    size_t count = 0;
    for(size_t i = 1; i < *clen; i++){
        if(comand[i] == ']'){
            if(count == 0){
                res.info.stringlen = i - 1;
                *clen = i + 1;
                return res;
            }
            count -= 1;
        }else if(comand[i] == '['){
            count += 1;
        }
    }
    RAISE(jbuff, SquaredParenthesisError);
}

struct Token stackToken(char *comand, size_t *clen, struct ExceptionHandler *jbuff){
    struct Token res;
    res.type = StackToken;
    res.instr = comand + 1;
    size_t count = 0;
    for(size_t i = 1; i < *clen; i++){
        if(comand[i] == '}'){
            if(count == 0){
                res.info.stringlen = i - 1;
                *clen = i + 1;
                return res;
            }
            count -= 1;
        }else if(comand[i] == '{'){
            count += 1;
        }
    }
    RAISE(jbuff, CurlyParenthesisError);
}

struct Token numericToken(char *comand, size_t *clen, struct ExceptionHandler *jbuff){
    struct Token res;
    char *endptr;
    res.instr = comand;
    uint64_t intval = strtol(comand, &endptr, 10);
    if(*endptr == '.' || *endptr == ','){
        double dblval = strtod(comand, &endptr);
        res.type = DecimalToken;
        res.info.decimal = dblval;
    }else{
        res.type = IntegerToken;
        res.info.integer = intval;
    }
    *clen = (size_t)(endptr - comand);
    return res;
}

struct Token scriptToken(char *comand, size_t *clen, struct ExceptionHandler *jbuff){
    struct Token res;
    res.type = GenericToken;
    res.instr = comand;
    for(size_t i = 0; i < *clen; i++){
        if(IS_INDENT(comand[i])){
            res.info.stringlen = i;
            *clen = i + 1;
            return res;
        }else if(comand[i] == '('){
            res.type = BrInstrToken;
            res.info.special.val = i;
            i += 1;
            while(i < *clen){
                if(comand[i] == ')'){
                    res.type = BrInstrToken;
                    res.info.special.instrlen = i - 1 - res.info.special.val;
                    *clen = i + 1;
                    return res;
                }
                i += 1;
            }
            RAISE(jbuff, RoundParenthesisError);
        }else if(comand[i] >= '0' && comand[i] <= '9'){
            char *endptr;
            res.info.stringlen = i;
            res.info.special.val = strtol(comand + i, &endptr, 10);
            *clen = (size_t)(endptr - comand);
            res.type = NumInsrtToken;
            return res;
        }
    }
    res.info.stringlen = *clen;
    return res;
}

void parse_script(struct ProgramState *state, char *comands, size_t clen, struct ExceptionHandler *jbuff){
    jbuff->not_exec[jbuff->bt_size - 1] = comands;
    struct Token token;
    size_t i = 0;
    while(i < clen){
        if(IS_INDENT(comands[i])){
            i += 1;
            continue;
        }else if(comands[i] == '"'){
            size_t start = clen - i;
            token = stringToken(comands + i, &start, jbuff);
            execute_instr(state, &token, jbuff);
            i += start;
        }else if(comands[i] == '['){
            size_t start = clen - i;
            token = instrToken(comands + i, &start, jbuff);
            execute_instr(state, &token, jbuff);
            i += start;
        }else if(comands[i] == '{'){
            size_t start = clen - i;
            token = stackToken(comands + i, &start, jbuff);
            execute_instr(state, &token, jbuff);
            i += start;
        }else if(comands[i] >= '0' && comands[i] <='9'){
            size_t start = clen - i;
            token = numericToken(comands + i, &start, jbuff);
            execute_instr(state, &token, jbuff);
            i += start;
        }else if(comands[i] == '-'){
            if(i + 1 < clen && comands[i + 1] >= '0' && comands[i + 1] <= '9'){
                size_t start = clen - i;
                token = numericToken(comands + i, &start, jbuff);
                execute_instr(state, &token, jbuff);
                i += start;
            }else{
                size_t start = clen - i;
                token = scriptToken(comands + i, &start, jbuff);
                execute_instr(state, &token, jbuff);
                i += start;
            }
        }else{
            size_t start = clen - i;
            token = scriptToken(comands + i, &start, jbuff);
            execute_instr(state, &token, jbuff);
            i += start;
        }
    }
}

void execute(struct ProgramState *state, char *comands, struct ExceptionHandler *jbuff){
    parse_script(state, comands, strlen(comands), jbuff);
}

//------------------------------------------------------------------------------------------------------

void brop_split(struct ProgramState *state, char *comand, size_t clen, struct ExceptionHandler *jbuff){
    add_backtrace(jbuff);
    parse_script(state, comand, clen, jbuff);
    remove_backtrace(jbuff);
    if(state->stack->next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack->next -= 1;
    struct StackElem delimiter = state->stack->content[state->stack->next];
    state->stack->next -= 1;
    struct StackElem string = state->stack->content[state->stack->next];
    if(delimiter.type == String && string.type == String) {
        char *token = strtok(string.val.instr, delimiter.val.instr);
        do{
            struct StackElem elem;
            elem.type = String;
            size_t tokenlen = strlen(token);
            elem.val.instr = malloc(tokenlen + 1);
             if (elem.val.instr == NULL)
                RAISE(jbuff, ProgramPanic);
            memcpy(elem.val.instr, token, tokenlen);
            elem.val.instr[tokenlen] = '\0';
            push_Stack(state->stack, elem, jbuff);
        }while((token = strtok(NULL, delimiter.val.instr)) != NULL);
        free(string.val.instr);
        free(delimiter.val.instr);
    }else{
        state->stack->next += 1;
        RAISE(jbuff, InvalidOperands);
    }
}

void op_split(struct ProgramState* state, struct ExceptionHandler* jbuff){
    if (state->stack->next == 0)
        RAISE(jbuff, StackUnderflow);
    state->stack->next -= 1;
    if (state->stack->content[state->stack->next].type == Instruction) {
        size_t quote = 0;
        size_t start = 0;
        size_t round_br = 0;
        short string = 0;
        size_t i = 0;
        char *original = state->stack->content[state->stack->next].val.instr;
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
                    push_Stack(state->stack, elem, jbuff);
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
                    push_Stack(state->stack, elem, jbuff);
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
                    push_Stack(state->stack, elem, jbuff);
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
                        push_Stack(state->stack, elem, jbuff);
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
            push_Stack(state->stack, elem, jbuff);
        }
        free(original);
    }else if(state->stack->content[state->stack->next].type == String){
        char *original = state->stack->content[state->stack->next].val.instr;
        char *token = strtok(original, " ");
        do{
            struct StackElem elem;
            elem.type = String;
            size_t tokenlen = strlen(token);
            elem.val.instr = malloc(tokenlen + 1);
             if (elem.val.instr == NULL)
                RAISE(jbuff, ProgramPanic);
            memcpy(elem.val.instr, token, tokenlen);
            elem.val.instr[tokenlen] = '\0';
            push_Stack(state->stack, elem, jbuff);
        }while((token = strtok(NULL, " ")) != NULL);
        free(original);
    }else if(state->stack->content[state->stack->next].type == InnerStack){
        struct Stack *src = state->stack->content[state->stack->next].val.stack;
        for(size_t i = 0; i < src->next; i++){
            push_Stack(state->stack, src->content[i], jbuff);
        }
        free_Stack(src);
    }else{
        state->stack->next += 1;
        RAISE(jbuff, InvalidOperands);
    }
}

void brop_compose(struct ProgramState *state, char *comand, size_t clen, struct ExceptionHandler *jbuff){
    add_backtrace(jbuff);
    parse_script(state, comand, clen, jbuff);
    remove_backtrace(jbuff);
    if(state->stack->next < 3)
        RAISE(jbuff, StackUnderflow);
     state->stack->next -= 1;
    struct StackElem delimiter = state->stack->content[state->stack->next];
    state->stack->next -= 1;
    struct StackElem second = state->stack->content[state->stack->next];
    if(state->stack->content[state->stack->next - 1].type == String && second.type == String && delimiter.type == String){
        size_t delimlen = strlen(delimiter.val.instr);
        size_t lensecond = strlen(second.val.instr);
        size_t lenfirst =  strlen(state->stack->content[state->stack->next - 1].val.instr);
        size_t totsize = lensecond + lenfirst + delimlen + 1;
        state->stack->content[state->stack->next - 1].val.instr = realloc(state->stack->content[state->stack->next - 1].val.instr, totsize);
        if(state->stack->content[state->stack->next - 1].val.instr == NULL){
            RAISE(jbuff, ProgramPanic);
        }
        strcpy(state->stack->content[state->stack->next - 1].val.instr + lenfirst, delimiter.val.instr);
        strcpy(state->stack->content[state->stack->next - 1].val.instr + delimlen + lenfirst, second.val.instr);
        state->stack->content[state->stack->next - 1].val.instr[totsize - 1] = '\0';
        free(delimiter.val.instr);
        free(second.val.instr);
    }else{
        state->stack->next += 2;
        RAISE(jbuff, InvalidOperands);
    }
}

void op_none(struct ProgramState *state, struct ExceptionHandler *jbuff){
    struct StackElem elem;
    elem.type = None;
    elem.val.ival = 0;
    push_Stack(state->stack, elem, jbuff);
}

void op_stack(struct ProgramState *state, struct ExceptionHandler *jbuff){
    push_Stack(state->stack, new_Stack(jbuff), jbuff);
}


void op_push(struct ProgramState* state, struct ExceptionHandler* jbuff){
    if (state->stack->next < 2)
        RAISE(jbuff, StackUnderflow);
    size_t stackindx = state->stack->next - 2;
    if(state->stack->content[stackindx].type != InnerStack)
        RAISE(jbuff, InvalidOperands);
    state->stack->next -= 1;
    push_Stack(state->stack->content[stackindx].val.stack, state->stack->content[state->stack->next], jbuff);
}

void op_pop(struct ProgramState* state, struct ExceptionHandler* jbuff){
    if (state->stack->next == 0)
        RAISE(jbuff, StackUnderflow);
    size_t stackindx = state->stack->next - 1;
    if(state->stack->content[stackindx].type != InnerStack)
        RAISE(jbuff, InvalidOperands);
    struct StackElem res;
    if(state->stack->content[stackindx].val.stack->next == 0){
        res.type = None;
        res.val.ival = 0;
    }else{
        state->stack->content[stackindx].val.stack->next -= 1;
        res = state->stack->content[stackindx].val.stack->content[state->stack->content[stackindx].val.stack->next];
    }
    push_Stack(state->stack, res, jbuff);
}

void op_inject(struct ProgramState* state, struct ExceptionHandler* jbuff){
    if (state->stack->next < 2)
        RAISE(jbuff, StackUnderflow);
    size_t stackindx = state->stack->next - 2;
    if(state->stack->content[stackindx].type != InnerStack)
        RAISE(jbuff, InvalidOperands);
    state->stack->next -= 1;
    if(state->stack->content[state->stack->next].type != Instruction){
        state->stack->next += 1;
        RAISE(jbuff, InvalidOperands);
    }
    struct ProgramState stat;
    stat.stack = state->stack->content[stackindx].val.stack;
    stat.env = state->env;
    char *mem = state->stack->content[state->stack->next].val.instr;
    add_memory(jbuff, mem);
    add_backtrace(jbuff);
    parse_script(&stat, mem, strlen(mem), jbuff);
    remove_backtrace(jbuff);
    remove_memory(jbuff, mem);
}

void numop_inject(struct ProgramState *state, size_t num, struct ExceptionHandler *jbuff) {
    if (state->stack->next < num + 1)
        RAISE(jbuff, StackUnderflow);
    state->stack->next -= 1;
    if(state->stack->content[state->stack->next].type != Instruction){
        state->stack->next += 1;
        RAISE(jbuff, InvalidOperands);
    }
    char *mem = state->stack->content[state->stack->next].val.instr;
    add_memory(jbuff, mem);
    add_backtrace(jbuff);
    for(size_t i = state->stack->next - num; i < state->stack->next; i++){
        if(state->stack->content[i].type != InnerStack){
            state->stack->next += 1;
            RAISE(jbuff, InvalidOperands);
        }
        struct ProgramState stat;
        stat.stack = state->stack->content[i].val.stack;
        stat.env = state->env;
        parse_script(&stat, mem, strlen(mem), jbuff);
    }
    remove_backtrace(jbuff);
    remove_memory(jbuff, mem);
}

void numop_pinject(struct ProgramState *state, size_t num, struct ExceptionHandler *jbuff) {
    if (state->stack->next < num + 1)
        RAISE(jbuff, StackUnderflow);
    state->stack->next -= 1;
    if(state->stack->content[state->stack->next].type != Instruction){
        state->stack->next += 1;
        RAISE(jbuff, InvalidOperands);
    }
    for(size_t i = state->stack->next - num; i < state->stack->next; i++){
        if(state->stack->content[i].type != InnerStack){
            state->stack->next += 1;
            RAISE(jbuff, InvalidOperands);
        }
    }
    char *mem = state->stack->content[state->stack->next].val.instr;
    add_memory(jbuff, mem);
    add_backtrace(jbuff);
    jbuff->stack_num = num;
    jbuff->inject_err = malloc(sizeof(struct ExceptionHandler *) * num);
    int error = 0;
#pragma omp parallel for schedule(dynamic)
    for(size_t i = state->stack->next - num; i < state->stack->next; i++){
        struct ProgramState stat;
        stat.stack = state->stack->content[i].val.stack;
        stat.env = state->env;
        jbuff->inject_err[i] = init_ExceptionHandler();
        if (jbuff->inject_err[i] == NULL)
            exit(-1);
        TRY(jbuff->inject_err[i]) {
            parse_script(&stat, mem, strlen(mem), jbuff->inject_err[i]);
        }CATCHALL{
            error = 1;
            continue;
        }
        free_ExceptionHandler(jbuff->inject_err[i]);
        jbuff->inject_err[i] = NULL;
    }
    if(error)
        RAISE(jbuff, InjectError);
    free(jbuff->inject_err);
    jbuff->stack_num = 0;
    remove_backtrace(jbuff);
    remove_memory(jbuff, mem);
}

void op_compress(struct ProgramState* state, struct ExceptionHandler* jbuff){
    struct StackElem res = new_Stack(jbuff);
    for(size_t i = 0; i < state->stack->next; i++){
        push_Stack(res.val.stack, state->stack->content[i], jbuff);
    }
    state->stack->next = 0;
    push_Stack(state->stack, res, jbuff);
}

void brop_times(struct ProgramState* state, char* number, size_t numberlen, struct ExceptionHandler* jbuff) {
    if (state->stack->next == 0)
        RAISE(jbuff, StackUnderflow);
    state->stack->next -= 1;
    if (state->stack->content[state->stack->next].type != Instruction) {
        state->stack->next += 1;
        RAISE(jbuff, InvalidOperands);
    }
    char* mem = state->stack->content[state->stack->next].val.instr;
    add_memory(jbuff, mem);
    add_backtrace(jbuff);
    parse_script(state, number, numberlen, jbuff);
    state->stack->next -= 1;
    if (state->stack->content[state->stack->next].type != Integer) {
        state->stack->next += 2;
        RAISE(jbuff, InvalidOperands);
    }
    remove_backtrace(jbuff);
    add_backtrace(jbuff);
    for (int i = 0; i < state->stack->content[state->stack->next].val.ival; i++) {
        parse_script(state, mem, strlen(mem), jbuff);

    }
    remove_backtrace(jbuff);
    remove_memory(jbuff, mem);
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
    for(size_t i = 0; i < state->stack->next; i++){
        switch (state->stack->content[i].type)
        {
        case Instruction:
            if(fprintf(target, "[%s] ", state->stack->content[i].val.instr) < 0){
                fclose(target);
                RAISE(jbuff, IOError);
            }
            break;
        
        case String:
            if(fprintf(target, "\"%s\" ", state->stack->content[i].val.instr) < 0){
                fclose(target);
                RAISE(jbuff, IOError);
            }
            break;
        
        case Integer:
            if(fprintf(target, "%ld ", state->stack->content[i].val.ival) < 0){
                fclose(target);
                RAISE(jbuff, IOError);
            }
            break;

        case Floating:
            if(fprintf(target, "%lf ", state->stack->content[i].val.fval) < 0){
                fclose(target);
                RAISE(jbuff, IOError);
            }
            break;

        case Boolean:
            if(fprintf(target, "%s ", BOOL[state->stack->content[i].val.ival]) < 0){
                fclose(target);
                RAISE(jbuff, IOError);
            }
            break;

        case None:
            if(fprintf(target, "none ") < 0){
                fclose(target);
                RAISE(jbuff, IOError);
            }
            break;

        case Type:
            if(fprintf(target, "%s ", TYPES[state->stack->content[i].val.ival]) < 0){
                fclose(target);
                RAISE(jbuff, IOError);
            }
            break;

        default:
            UNREACHABLE;
        }
    }
    if(fclose(target) != 0)
        RAISE(jbuff, IOError);
}

void op_dip(struct ProgramState* state, struct ExceptionHandler* jbuff){
    if (state->stack->next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack->next -= 1;
    if (state->stack->content[state->stack->next].type != Instruction) {
        state->stack->next += 1;
        RAISE(jbuff, InvalidOperands);
    }
    char* mem = state->stack->content[state->stack->next].val.instr;
    state->stack->next -= 1;
    struct StackElem temp = state->stack->content[state->stack->next];
    add_memory(jbuff, mem);
    add_backtrace(jbuff);
    parse_script(state, mem, strlen(mem), jbuff);
    remove_memory(jbuff, mem);
    remove_backtrace(jbuff);
    push_Stack(state->stack, temp, jbuff);
}

void op_if(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack->next < 3)
        RAISE(jbuff, StackUnderflow);
    state->stack->next -= 1;
    if(state->stack->content[state->stack->next].type != Instruction){
        state->stack->next += 1;
        RAISE(jbuff, InvalidOperands);
    }
    char *memf = state->stack->content[state->stack->next].val.instr;
    state->stack->next -= 1;
    if(state->stack->content[state->stack->next].type != Instruction){
        state->stack->next += 2;
        RAISE(jbuff, InvalidOperands);
    }
    char *memt = state->stack->content[state->stack->next].val.instr;
    state->stack->next -= 1;
    if(state->stack->content[state->stack->next].type != Boolean){
        state->stack->next += 3;
        RAISE(jbuff, InvalidOperands);
    }
    add_memory(jbuff, memt);
    add_memory(jbuff, memf);
    add_backtrace(jbuff);

    switch(state->stack->content[state->stack->next].val.ival){
        case 1:
            parse_script(state, memt, strlen(memt), jbuff);
        break;

        case 0:
            parse_script(state, memf, strlen(memf), jbuff); 
        break;

        default:
        UNREACHABLE;
    }

    remove_memory(jbuff, memf);
    remove_memory(jbuff, memt);
    remove_backtrace(jbuff);
}

void brop_if(struct ProgramState *state, char *cond, size_t condlen, struct ExceptionHandler *jbuff){
    if(state->stack->next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack->next -= 1;
    if(state->stack->content[state->stack->next].type != Instruction){
        state->stack->next += 1;
        RAISE(jbuff, InvalidOperands);
    }
    char *memf = state->stack->content[state->stack->next].val.instr;
    state->stack->next -= 1;
    if(state->stack->content[state->stack->next].type != Instruction){
        state->stack->next += 2;
        RAISE(jbuff, InvalidOperands);
    }
    char *memt = state->stack->content[state->stack->next].val.instr;
    add_memory(jbuff, memt);
    add_memory(jbuff, memf);
    add_backtrace(jbuff);
    parse_script(state, cond, condlen, jbuff);
    if(state->stack->next < 1)
        RAISE(jbuff, StackUnderflow);
    state->stack->next -= 1;
    if(state->stack->content[state->stack->next].type != Boolean){
        state->stack->next += 1;
        RAISE(jbuff, InvalidOperands);
    }
    switch(state->stack->content[state->stack->next].val.ival){
        case 1:
            parse_script(state, memt, strlen(memt), jbuff);
        break;

        case 0:
            parse_script(state, memf, strlen(memf), jbuff);
        break;

        default:
        UNREACHABLE;
    }
    remove_memory(jbuff, memt);
    remove_memory(jbuff, memf);
    remove_backtrace(jbuff);
}

void op_loop(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack->next == 0)
        RAISE(jbuff, StackUnderflow);
    state->stack->next -= 1;
    if(state->stack->content[state->stack->next].type != Instruction){
        state->stack->next += 1;
        RAISE(jbuff, InvalidOperands);
    }
    char *mem = state->stack->content[state->stack->next].val.instr;
    add_memory(jbuff, mem);
    add_backtrace(jbuff);
    while (1){
        parse_script(state, mem, strlen(mem), jbuff);
        state->stack->next -= 1;
        if(state->stack->content[state->stack->next].type != Boolean){
            state->stack->next += 1;
            RAISE(jbuff, InvalidOperands);
        }
        if (state->stack->content[state->stack->next].val.ival == 0) {
            break;
        }
    }
    remove_memory(jbuff, mem);
    remove_backtrace(jbuff);
}

void brop_loop(struct ProgramState *state, char *cond, size_t condlen, struct ExceptionHandler *jbuff){
    if(state->stack->next == 0)
        RAISE(jbuff, StackUnderflow);
    state->stack->next -= 1;
    if(state->stack->content[state->stack->next].type != Instruction){
        state->stack->next += 1;
        RAISE(jbuff, InvalidOperands);
    }
    char *mem = state->stack->content[state->stack->next].val.instr;
    add_memory(jbuff, mem);
    add_backtrace(jbuff);
    while (1){
        parse_script(state, cond, condlen, jbuff);
        state->stack->next -= 1;
        if(state->stack->content[state->stack->next].type != Boolean){
            state->stack->next += 1;
            RAISE(jbuff, InvalidOperands);
        }
        if (state->stack->content[state->stack->next].val.ival == 0) {
            break;
        }
        parse_script(state, mem, strlen(mem), jbuff);
    }
    remove_memory(jbuff, mem);
    remove_backtrace(jbuff);
}

void op_try(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack->next == 0)
        RAISE(jbuff, StackUnderflow);
    state->stack->next -= 1;
    if(state->stack->content[state->stack->next].type != Instruction){
        state->stack->next += 1;
        RAISE(jbuff, InvalidOperands);
    }
    struct ExceptionHandler *try_buf = init_ExceptionHandler();
    if(try_buf == NULL) {
        state->stack->next += 1;
        RAISE(jbuff, ProgramPanic);
    }
    char *mem = state->stack->content[state->stack->next].val.instr;
    struct StackElem result;
    result.type = Boolean;
    TRY(try_buf){
        parse_script(state, mem, strlen(mem), try_buf);
        result.val.ival = 1;
    }CATCHALL{
        result.val.ival = 0;
    }
    free(mem);
    free(try_buf);
    push_Stack(state->stack, result, jbuff);
}

void op_apply(struct ProgramState *state, struct ExceptionHandler *jbuff){
    if(state->stack->next == 0)
        RAISE(jbuff, StackUnderflow);
    state->stack->next -= 1;
    if(state->stack->content[state->stack->next].type != Instruction){
        state->stack->next += 1;
        RAISE(jbuff, InvalidOperands);
    }
    char *mem = state->stack->content[state->stack->next].val.instr;
    add_memory(jbuff, mem);
    add_backtrace(jbuff);
    parse_script(state, mem, strlen(mem), jbuff);
    remove_memory(jbuff, mem);
    remove_backtrace(jbuff);
}

void brop_dup(struct ProgramState *state, char *comand, size_t clen, struct ExceptionHandler *jbuff){
    parse_script(state, comand, clen, jbuff);
    if(state->stack->next < 1)
        RAISE(jbuff, StackUnderflow);
    state->stack->next -= 1;
    if(state->stack->content[state->stack->next].type == Integer) {
        if (state->stack->content[state->stack->next].val.ival >= state->stack->next)
            RAISE(jbuff, StackUnderflow);
        struct StackElem copy;
        size_t index = state->stack->next - 1 - state->stack->content[state->stack->next].val.ival;
        copy.type = state->stack->content[index].type;
        if (copy.type == Instruction || copy.type == String) {
            size_t srclen = strlen(state->stack->content[index].val.instr) + 1;
            copy.val.instr = malloc(srclen);
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
    }else{
        state->stack->next += 1;
        RAISE(jbuff, InvalidOperands);
    }
}

void brop_swap(struct ProgramState *state, char *comand, size_t clen, struct ExceptionHandler *jbuff){
    parse_script(state, comand, clen, jbuff);
    if(state->stack->next < 2)
        RAISE(jbuff, StackUnderflow);
    state->stack->next -= 1;
    if(state->stack->content[state->stack->next].type == Integer) {
        if(state->stack->content[state->stack->next].val.ival >= state->stack->next)
            RAISE(jbuff, StackUnderflow);
        size_t index1 = state->stack->next - 1;
        size_t index2 = index1 - state->stack->content[state->stack->next].val.ival;
        struct StackElem temp;
        temp = state->stack->content[index1];
        state->stack->content[index1] = state->stack->content[index2];
        state->stack->content[index2] = temp;
    }else{
        state->stack->next += 1;
        RAISE(jbuff, InvalidOperands);
    }

}

void brop_dig(struct ProgramState* state, char* number, size_t numberlen, struct ExceptionHandler* jbuff) {
    add_backtrace(jbuff);
    parse_script(state, number, numberlen, jbuff);
    state->stack->next -= 1;
    if (state->stack->content[state->stack->next].type != Integer) {
        state->stack->next += 1;
        RAISE(jbuff, InvalidOperands);
    }
    if (state->stack->next <= state->stack->content[state->stack->next].val.ival){
        state->stack->next += 1;
        RAISE(jbuff, StackUnderflow);
    }
    size_t index = state->stack->next - 1;
    size_t indextar = state->stack->next - 1 - state->stack->content[state->stack->next].val.ival;
    struct StackElem temp = state->stack->content[indextar];
    for (size_t i = indextar; i < index; i++) {
        state->stack->content[i] = state->stack->content[i + 1];
    }
    state->stack->content[index] = temp;
    remove_backtrace(jbuff);
}

void brop_isdef(struct ProgramState *state, char *funcname, size_t fnlen, struct ExceptionHandler *jbuff){
    struct StackElem elem;
    elem.type = Boolean;
    char **out = malloc(sizeof(char *));
    elem.val.ival = get_Environment(state->env, funcname, fnlen, out);
    free(out);
    push_Stack(state->stack, elem, jbuff);
}

void brop_define(struct ProgramState *state, char *funcname, size_t fnlen, struct ExceptionHandler *jbuff){
    if(state->stack->next == 0)
        RAISE(jbuff, StackUnderflow);
    state->stack->next -= 1;
    if(state->stack->content[state->stack->next].type != Instruction) {
        state->stack->next += 1;
        RAISE(jbuff, InvalidOperands);
    }
    for (size_t i = 0; i < fnlen; i++){
        if(RESERVED_CHAR(funcname[i]))
            RAISE(jbuff, InvalidNameDefine);
    }
    set_Environment(state->env, funcname, fnlen, state->stack->content[state->stack->next].val.instr, jbuff);
}

void brop_delete(struct ProgramState *state, char *funcname, size_t fnlen, struct ExceptionHandler *jbuff){
    remove_Environment(state->env, funcname, fnlen);
}
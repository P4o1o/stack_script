//
// Created by P4o1o on 09/05/2024.
//
#include "programstate.h"

static struct Stack *init_Stack(const size_t capacity){
    struct Stack *res = malloc(sizeof(struct Stack));
    if(res == NULL)
        return NULL;
    res->capacity = capacity;
    res->next = 0;
    res->content = malloc(sizeof(struct StackElem) * capacity);
    if(res->content == NULL)
        return NULL;
    return res;
}

inline void free_Stack(struct Stack *stack){
    for(size_t i = 0; i < stack->next; i++){
        if (stack->content[i].type == Instruction || stack->content[i].type == String) {
            free(stack->content[i].val.instr);
        }
        if (stack->content[i].type == InnerStack) {
            free_Stack(stack->content[i].val.stack);
        }
    }
    free(stack->content);
    stack->capacity = 0;
    stack->next = 0;
    free(stack);
}

static inline struct Environment *init_Environment(size_t capacity){
    struct Environment *res = malloc(sizeof(struct Environment));
    if(res == NULL)
        return NULL;
    res->content = malloc(sizeof(struct EnvElem *) * capacity);
    if(res->content == NULL)
        return NULL;
    res->capacity = capacity;
    for (size_t i = 0; i < res->capacity; i++) {
        res->content[i] = NULL;
    }
    return res;
}

static inline void free_Environment(struct Environment *env){
    for (size_t i = 0; i < env->capacity; i++) {
        struct EnvElem *elem = env->content[i];
        while(elem != NULL){
            struct EnvElem *temp = elem->next;
            free(elem->key);
            free(elem->value);
            free(elem);
            elem = temp;
        }
    }
    free(env->content);
    env->capacity = 0;
    free(env);
}

struct ProgramState init_PrgState(size_t stack_capacity, size_t env_capacity){
    struct ProgramState res;
    res.stack = init_Stack(stack_capacity);
    res.env = init_Environment(env_capacity);
    return res;
}

void free_PrgState(struct ProgramState *inter){
    free_Stack(inter->stack);
    free_Environment(inter->env);
}

struct ExceptionHandler *init_ExceptionHandler(){
    struct ExceptionHandler* try_buf = malloc(sizeof(struct ExceptionHandler));
    try_buf->not_exec = malloc(sizeof(char *) * BT_VEC_CAPACITY);
    try_buf->bt_size = 1;
    try_buf->bt_capacity = BT_VEC_CAPACITY;
    try_buf->openmemmap = malloc(sizeof(struct OpenMemMap *) * OM_VEC_CAPACITY);
    for(size_t i = 0; i < OM_VEC_CAPACITY; i++){
        try_buf->openmemmap[i] = NULL;
    }
    try_buf->stack_num = 0;
    return try_buf;
}

void reload_Exceptionhandler(struct ExceptionHandler *try_buf){
    for(size_t i = 0; i < OM_VEC_CAPACITY; i++){
        while(try_buf->openmemmap[i] != NULL){
            struct OpenMemMap *temp = try_buf->openmemmap[i];
            try_buf->openmemmap[i] =  try_buf->openmemmap[i]->next;
            free(temp->openmem);
            free(temp);
        }
    }
    try_buf->bt_capacity = BT_VEC_CAPACITY;
    free(try_buf->not_exec);
    try_buf->not_exec = malloc(sizeof(char *) * BT_VEC_CAPACITY);
    try_buf->bt_size = 1;
    for (size_t i = 0; i < try_buf->stack_num; i++){
        if(try_buf->inject_err[i] != NULL){
            free_ExceptionHandler(try_buf->inject_err[i]);
        }
    }
    if(try_buf->stack_num != 0){
        free(try_buf->inject_err);
    }
    try_buf->stack_num = 0;
}

void free_ExceptionHandler(struct ExceptionHandler *try_buf){
    free(try_buf->not_exec);
    for(size_t i = 0; i < OM_VEC_CAPACITY; i++){
        while(try_buf->openmemmap[i] != NULL){
            struct OpenMemMap *temp = try_buf->openmemmap[i];
            try_buf->openmemmap[i] =  try_buf->openmemmap[i]->next;
            free(temp->openmem);
            free(temp);
        }
    }
    free(try_buf->openmemmap);
    for (size_t i = 0; i < try_buf->stack_num; i++){
        if(try_buf->inject_err[i] != NULL){
            free_ExceptionHandler(try_buf->inject_err[i]);
        }
    }
    if(try_buf->stack_num != 0){
        free(try_buf->inject_err);
    }
    free(try_buf);
}

void print_Exception(struct ExceptionHandler *exc) {
    char *excstr;
    switch(exc->exit_value){
        case ProgramPanic:
            excstr = "Error while allocating memory";
            break;
        case InvalidChar:
            excstr = "Invalid character";
            break;
        case InvalidInstruction:
            excstr = "Invalid instruction";
            break;
        case InvalidOperands:
            excstr = "Invalid operands";
            break;
        case SquaredParenthesisError:
            excstr = "Squared Parenthesis number mismatch";
            break;
        case RoundParenthesisError:
            excstr = "Squared Parenthesis number mismatch";
            break;
        case CurlyParenthesisError:
            excstr = "Curly Parenthesis number mismatch";
            break;
        case StringQuotingError:
            excstr = "String quoting marks number mismatch";
            break;
        case StackUnderflow:
            excstr = "Stack underflow";
            break;
        case ValueError:
            excstr = "Value Error";
            break;
        case IOError:
            excstr = "I/O Error";
            break;
        case FileNotFound:
            excstr = "File not found";
            break;
        case FileNotCreatable:
            excstr = "File not creatable";
            break;
        case InjectError:
            excstr = "inject failed";
            break;
        case InvalidNameDefine:
            excstr = "define: invalid name";
            break;
        default:
            UNREACHABLE;
    }
    printf("%s not executed: %12s\n", excstr, exc->not_exec[0]);
    if(exc->bt_size > 1){
        printf("Backtrace:\n");
        char *tabs = malloc(sizeof(char) * exc->bt_size);
        tabs[0] = '\t';
        for(size_t i = 1; i < exc->bt_size; i++){
            tabs[i] = '\0';
            printf("%s%s\n", tabs, exc->not_exec[i]);
            tabs[i] = '\t';
        }
        free(tabs);
    }
    if(exc->exit_value == InjectError){
        for (size_t i = 0; i < exc->stack_num; i++){
            if(exc->inject_err[i] != NULL){
                printf("Inject Failed in stack %ld:\n", i);
                print_Exception(exc->inject_err[i]);
                printf("\n");
            }
        }
        
    }
}
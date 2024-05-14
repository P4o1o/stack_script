//
// Created by P4o1o on 09/05/2024.
//
#include "programstate.h"

static struct Stack init_Stack(const size_t capacity){
    struct Stack res;
    res.capacity = 0;
    res.next = 0;
    res.content = malloc(sizeof(struct StackElem) * capacity);
    res.capacity = capacity * (res.content != NULL);
    return res;
}

static inline void free_Stack(struct Stack stack){
    for(size_t i = 0; i < stack.next; i++){
        if (stack.content[i].type == Instruction) {
            free(stack.content[i].val.instr);
        }
    }
    free(stack.content);
    stack.capacity = 0;
    stack.next = 0;
}

static inline struct Environment init_Environment(size_t capacity){
    struct Environment res;
    res.content = malloc(sizeof(struct EnvElem *) * capacity);
    res.capacity = (res.content != NULL) * capacity;
    for (size_t i = 0; i < res.capacity; i++) {
        res.content[i] = NULL;
    }
    return res;
}

static inline void free_Environment(struct Environment env){
    for (size_t i = 0; i < env.capacity; i++) {
        if(env.content[i] != NULL){
            free(env.content[i]->key);
            free(env.content[i]->value);
            struct EnvElem *elem = env.content[i]->next;
            free(env.content[i]);
            while(elem != NULL){
                struct EnvElem *temp = elem->next;
                free(elem->key);
                free(elem->value);
                free(elem);
                elem = temp;
            }
        }
    }
    free(env.content);
    env.capacity = 0;
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
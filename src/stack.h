//
// Created by P4o1o on 14/05/2024.
//

#ifndef SSCRIPT_STACK_H
#define SSCRIPT_STACK_H

#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>

#ifdef __GNUC__
	#define UNREACHABLE __builtin_unreachable()
#else
#ifdef _MSC_VER
	#define UNREACHABLE __assume(0);
#else
	[[noreturn]] inline void unreachable(){}
	#define UNREACHABLE unreachable()
#endif
#endif

#include "memdebug.h"

enum ElemType{
    Instruction,
    Integer,
    Floating,
    BoolTrue,
    BoolFalse,
    String
};

union ElemVal{
    char *instr;
    int64_t ival;
    double fval;
};

struct StackElem{
    enum ElemType type;
    union ElemVal val;
};

struct Stack{
    struct StackElem *content;
    size_t capacity;
    size_t next;
};

#endif //SSCRIPT_STACK_H

cmake_minimum_required(VERSION 3.27)
project(sscript C)

set(CMAKE_C_STANDARD 11)

add_executable(sscript
        src/main.c
        src/programstate.c
        src/interpreter.c
        src/stack.h
        src/environment.c
        src/programstate.h
        src/interpreter.h
        src/environment.h "src/buildutils.c" "src/buildutils.h")

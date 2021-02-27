cmake_minimum_required(VERSION 3.10)
project(ralloc)

include_directories(include)

set(SRC src/ralloc.c)
add_library(ralloc STATIC ${SRC})

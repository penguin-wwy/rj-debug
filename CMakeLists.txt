cmake_minimum_required(VERSION 3.5)
project(rj-debug)

if(NOT DEFINED ENV{JAVA_HOME})
    message(FATAL_ERROR "not define environment variable:JAVA_HOME")
endif()

if(UNIX)
    include_directories(
            $ENV{JAVA_HOME}/include
            $ENV{JAVA_HOME}/include/linux
    )
endif()

link_directories(${CMAKE_CURRENT_SOURCE_DIR}/target/debug)
add_library(rjdebug SHARED src/debug.c)
target_link_libraries(rjdebug "-Wl,--whole-archive" rj_debug "-Wl,--no-whole-archive")
#set(CMAKE_LINK_LIBRARY_SUFFIX "${CMAKE_LINK_LIBRARY_SUFFIX} -lpthread -ldl")
#add_executable(main src/debug.c)
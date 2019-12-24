//
// Created by penguin on 19-12-24.
//

#ifndef NATIVE_TOOLS_TOOLS_H
#define NATIVE_TOOLS_TOOLS_H

#include "bridge.h"

static char buffer[128] = {0};

#define RESET_STRING(MESSAGE...)            \
    memset(buffer, 0, 128);                 \
    sprintf(buffer, MESSAGE)

#define ERROR_EXIT(EXIT_CODE, MESSAGE...)   \
    do {                                    \
        RESET_STRING(MESSAGE);              \
        error_log(buffer);                  \
        exit(EXIT_CODE);                    \
    } while (0)

#define INFO(MESSAGE...)					\
	do {                                    \
        RESET_STRING(MESSAGE);              \
        info_log(buffer);                  	\
    } while (0)

#define WARNING(MESSAGE...)                 \
    do {
        RESET_STRING(MESSAGE);              \
        warn_log(buffer);                   \
    } while (0)
#endif //NATIVE_TOOLS_TOOLS_H

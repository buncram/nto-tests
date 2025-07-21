/*
 * Copyright 2023 The TensorFlow Authors. All Rights Reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// This file provides a reference implementation of the TFLM debug logging
// functions. It is designed to replicate the behavior of the original
// tflm_debug_log.c but in C++, using a platform-specific `print_string`
// function for output.

#include "tensorflow/lite/micro/debug_log.h"

// We use the TF_LITE_STRIP_ERROR_STRINGS macro to allow the user to compile
// out all logging code, reducing binary size.
#ifndef TF_LITE_STRIP_ERROR_STRINGS
#include <cstdarg>
#include <cstdio>
#include <cstring>
#endif

// Forward-declare the platform-specific C function for printing a string.
// We use extern "C" to ensure C-style name mangling, so the linker can find
// this C function when linking this C++ object file.
extern "C" void print_string(const char *s);

// This is the function that the TFLM library will call for logging.
// It must have C-linkage to be discoverable by the C-based TFLM library.
// The function signature matches the one in `tensorflow/lite/micro/debug_log.h`.
extern "C" void DebugLog(const char *format, va_list args)
{
#ifndef TF_LITE_STRIP_ERROR_STRINGS
    // A buffer to hold the formatted string.
    // Make sure this is large enough for your logging needs.
    char log_buffer[256];

    // Use vsnprintf to safely format the string into the buffer.
    vsnprintf(log_buffer, sizeof(log_buffer), format, args);

    // Ensure the buffer is null-terminated, just in case vsnprintf truncated it.
    log_buffer[sizeof(log_buffer) - 1] = '\0';

    // Use the platform-specific print function to output the formatted string.
    print_string(log_buffer);

    // Add a newline if the formatted string doesn't already have one.
    // This is helpful for readability in serial terminals.
    const int length = strlen(log_buffer);
    if (length > 0 && log_buffer[length - 1] != '\n')
    {
        print_string("\r\n");
    }
#endif
}

extern "C" void DebugSane(const char *format, ...)
{
    va_list args;
    va_start(args, format);
    // A buffer to hold the formatted string.
    // Make sure this is large enough for your logging needs.
    char log_buffer[256];

    // Use vsnprintf to safely format the string into the buffer.
    vsnprintf(log_buffer, sizeof(log_buffer), format, args);

    // Ensure the buffer is null-terminated, just in case vsnprintf truncated it.
    log_buffer[sizeof(log_buffer) - 1] = '\0';

    // Use the platform-specific print function to output the formatted string.
    print_string(log_buffer);

    // Add a newline if the formatted string doesn't already have one.
    // This is helpful for readability in serial terminals.
    const int length = strlen(log_buffer);
    if (length > 0 && log_buffer[length - 1] != '\n')
    {
        print_string("\r");
    }
    va_end(args);
}

#ifndef TF_LITE_STRIP_ERROR_STRINGS
// This function is also part of the TFLM debug logging API and is used
// by the MicroVsnprintf function in micro_log.h. It provides a C-linkage
// wrapper around the standard vsnprintf.
extern "C" int DebugVsnprintf(char *buffer, size_t buf_size,
                              const char *format, va_list vlist)
{
    return vsnprintf(buffer, buf_size, format, vlist);
}
#endif // TF_LITE_STRIP_ERROR_STRINGS

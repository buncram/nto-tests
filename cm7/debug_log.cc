#include "tensorflow/lite/micro/micro_log.h"

#include <cstdarg>
#include <cstdio>
#include <cstring>

// Your platform-specific function for printing a string
extern "C" void print_string(const char *s);

// This is the implementation of the low-level logging function that TFLM
// expects. It formats the string and prints it.
// This function MUST have C linkage to be found by the TFLM library.
extern "C" void VDebugLog(const char *format, va_list args)
{
    // A buffer to hold the formatted string. 256 is a safe size.
    constexpr int kMaxLogLen = 256;
    char log_buffer[kMaxLogLen];

    vsnprintf(log_buffer, kMaxLogLen, format, args);

    // Ensure the buffer is null-terminated, even if truncated.
    log_buffer[kMaxLogLen - 1] = '\0';

    print_string(log_buffer);

    // Add a newline if the formatted string didn't already have one.
    if (strlen(log_buffer) > 0 && log_buffer[strlen(log_buffer) - 1] != '\n')
    {
        print_string("\r\n");
    }
}

// The TFLM library calls these C-linkage functions when it needs to log.
// We will implement them to call our core VDebugLog function.
extern "C" void DebugLog(const char *format, ...)
{
    va_list args;
    va_start(args, format);
    VDebugLog(format, args);
    va_end(args);
}

extern "C" void VMicroPrintf(const char *format, va_list args)
{
    VDebugLog(format, args);
}

extern "C" void MicroPrintf(const char *format, ...)
{
    va_list args;
    va_start(args, format);
    VDebugLog(format, args);
    va_end(args);
}
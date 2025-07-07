// In your file: tflm_debug_log.c

#include <stdio.h>
#include <stdarg.h>
#include <string.h>

// Your platform's function to print a string
extern void print_string(const char *s);

// This is the function TFLM will call. We give it C-linkage
// to ensure the name matches what the linker is looking for.
void DebugLog(const char *format, ...)
{
    // A buffer to hold the formatted string.
    char log_buffer[256];

    va_list args;
    va_start(args, format);
    vsnprintf(log_buffer, sizeof(log_buffer), format, args);
    va_end(args);

    // Ensure null termination
    log_buffer[sizeof(log_buffer) - 1] = '\0';

    print_string(log_buffer);

    // Add a newline if the formatted string didn't already have one.
    if (strlen(log_buffer) > 0 && log_buffer[strlen(log_buffer) - 1] != '\n')
    {
        print_string("\r\n");
    }
}
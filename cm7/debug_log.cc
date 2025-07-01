/* Copyright 2024 The TensorFlow Authors. All Rights Reserved.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
==============================================================================*/

#include "tensorflow/lite/micro/debug_log.h"

#include <cstdarg>
#include <cstdio>

// Include your platform-specific headers
#include "daric_util.h"
#include "mbox.h"

extern "C" void DebugLog(const char *format, ...)
{
    // Buffer for formatted string - adjust size as needed for your platform
    constexpr int kMaxLogLen = 256;
    char log_buffer[kMaxLogLen];

    va_list args;
    va_start(args, format);

    // Use vsnprintf to format the string safely
    int chars_written = vsnprintf(log_buffer, kMaxLogLen, format, args);
    va_end(args);

    // Ensure null termination
    if (chars_written >= kMaxLogLen)
    {
        log_buffer[kMaxLogLen - 1] = '\0';
        chars_written = kMaxLogLen - 1;
    }

    // Output the formatted string using your platform's UART function
    // Based on your code, you have __uart_putchar available
    for (int i = 0; i < chars_written && log_buffer[i] != '\0'; ++i)
    {
        __uart_putchar(log_buffer[i]);
    }

    // Add newline if not present
    if (chars_written > 0 && log_buffer[chars_written - 1] != '\n')
    {
        __uart_putchar('\n');
    }
}

// Alternative implementation using your existing print_string function
// You can use this instead if you prefer simpler logging without formatting
/*
extern "C" void DebugLog(const char* format, ...) {
  // For simpler implementation, just output the format string
  // This loses printf-style formatting but is more lightweight
  print_string(format);
}
*/
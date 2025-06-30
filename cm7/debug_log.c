// File: debug_log.c

#include "tensorflow/lite/micro/debug_log.h"

#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>

// Forward-declare the low-level character output function from mbox.c
extern void __uart_putchar(char ch);

// Forward-declare our custom string formatting function
int DebugVsnprintf(char *buffer, size_t buf_size, const char *format, va_list vlist);

// This is the function that TFLM calls for logging.
void DebugLog(const char *format, va_list args)
{
#ifndef TF_LITE_STRIP_ERROR_STRINGS
    // To keep the binary size small and avoid heap allocation, we format
    // the string into a stack-allocated buffer.
    char buffer[256];
    DebugVsnprintf(buffer, sizeof(buffer), format, args);

    // Print the formatted string out to the UART character by character.
    for (char *p = buffer; *p != '\0'; ++p)
    {
        __uart_putchar(*p);
    }
#endif
}

// A simple integer-to-string function.
// Note: This is a helper for our vsnprintf and is intentionally kept simple.
static void simple_itoa(long long int value, char *str, int base)
{
    char *p = str;
    if (value < 0 && base == 10)
    {
        *p++ = '-';
        value = -value;
    }

    // Use a temporary buffer to reverse the string
    char temp[20];
    int i = 0;
    if (value == 0)
    {
        temp[i++] = '0';
    }
    else
    {
        while (value > 0)
        {
            temp[i++] = "0123456789abcdef"[value % base];
            value /= base;
        }
    }

    while (i > 0)
    {
        *p++ = temp[--i];
    }
    *p = '\0';
}

// A basic, C-compatible vsnprintf to format strings.
// It supports %c, %s, %d, %u, %x, and %%.
int DebugVsnprintf(char *buffer, size_t buf_size, const char *format,
                   va_list vlist)
{
    if (!buffer || buf_size == 0)
        return 0;

    char *out = buffer;
    char *const end = buffer + buf_size - 1;

    while (*format && out < end)
    {
        if (*format == '%')
        {
            format++;
            if (*format == '\0')
                break;

            if (*format == '%')
            {
                *out++ = '%';
            }
            else if (*format == 's')
            {
                const char *s = va_arg(vlist, const char *);
                if (s == NULL)
                    s = "(null)";
                while (*s && out < end)
                    *out++ = *s++;
            }
            else if (*format == 'd')
            {
                int val = va_arg(vlist, int);
                char num_buf[12];
                simple_itoa(val, num_buf, 10);
                char *p = num_buf;
                while (*p && out < end)
                    *out++ = *p++;
            }
            else if (*format == 'u')
            {
                unsigned int val = va_arg(vlist, unsigned int);
                char num_buf[12];
                simple_itoa(val, num_buf, 10);
                char *p = num_buf;
                while (*p && out < end)
                    *out++ = *p++;
            }
            else if (*format == 'x')
            {
                unsigned int val = va_arg(vlist, unsigned int);
                char num_buf[12];
                simple_itoa(val, num_buf, 16);
                char *p = num_buf;
                while (*p && out < end)
                    *out++ = *p++;
            }
            else if (*format == 'c')
            {
                *out++ = (char)va_arg(vlist, int);
            }
        }
        else
        {
            *out++ = *format;
        }
        format++;
    }
    *out = '\0';
    return out - buffer;
}
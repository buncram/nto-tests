#ifndef MBOX_H
#define MBOX_H
#include <stdint.h>

// This declares the function for any file that needs it.
void print_string(const char *s);
void send_u32_hex(uint32_t x);

#endif // MBOX_H
// File: system_setup.c

#include "tensorflow/lite/micro/system_setup.h"

// Your mbox.c file provides the enable_fpu() function.
// We declare it here so we can call it from this file.
extern void enable_fpu(void);

// This function is called by the TFLM framework before the main application
// logic begins. It's the ideal place for any hardware initialization
// that TFLM depends on.
void InitializeTarget(void)
{
    // Your `Reset_Handler` and `main_loop` in mbox.c already handle most of
    // the system initialization (clocks, memory, etc.).
    //
    // Enabling the FPU is critical for good performance on any Cortex-M core
    // that has one (like the CM7) when using floating-point models.
    enable_fpu();
}
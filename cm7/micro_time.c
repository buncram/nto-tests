// File: micro_time.c

#include "tensorflow/lite/micro/micro_time.h"
#include "core_cm7.h" // For SysTick registers and SystemCoreClock declaration

// SystemCoreClock is a global variable that holds the core clock frequency.
// It must be defined in one of your C files, which appears to be mbox.c.
extern uint32_t SystemCoreClock;

// The TFLM framework requires this function to be implemented.
// It should return a 32-bit tick count that increases over time.
uint32_t GetCurrentTimeTicks(void)
{
    // This implementation assumes you have configured the SysTick timer in your
    // setupTicks() function to be a free-running, 24-bit down-counter that
    // reloads automatically.
    //
    // To get an INCREASING tick count from a DECREASING counter (like SysTick),
    // we subtract the current counter value from its maximum reload value.
    // This gives us the number of ticks that have elapsed since the last reload.
    // Note: For measuring longer intervals, you would typically use the SysTick
    // interrupt to increment a 64-bit variable to avoid rollover issues.
    // For TFLM operator profiling, this is often sufficient.
    return SysTick->LOAD - SysTick->VAL;
}
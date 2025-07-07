#include "tensorflow/lite/micro/micro_time.h"

// Include platform-specific headers
#include "daric_cm7.h"
#include "tx_api.h" // ThreadX API

// Option 1: Using DWT (Data Watchpoint and Trace) cycle counter
// This provides high precision timing using Cortex-M7 debug features.
static bool dwt_initialized = false;

static void InitializeDWT()
{
  if (!dwt_initialized)
  {
    // Enable DWT if not already enabled
    CoreDebug->DEMCR |= CoreDebug_DEMCR_TRCENA_Msk;

    // Reset the cycle counter
    DWT->CYCCNT = 0;

    // Enable the cycle counter
    DWT->CTRL |= DWT_CTRL_CYCCNTENA_Msk;

    dwt_initialized = true;
  }
}

extern "C" uint32_t tflite_micro_time()
{
  InitializeDWT();

  // Return cycle count. This is all TFLM needs for profiling (measuring deltas).
  return DWT->CYCCNT;
}

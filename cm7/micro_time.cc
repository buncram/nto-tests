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

#include "tensorflow/lite/micro/micro_time.h"

// Include platform-specific headers
#include "daric_cm7.h"
#include "tx_api.h" // ThreadX API

// Option 1: Using DWT (Data Watchpoint and Trace) cycle counter
// This provides high precision timing using Cortex-M7 debug features
// static bool dwt_initialized = false;

// static void InitializeDWT()
// {
//   if (!dwt_initialized)
//   {
//     // Enable DWT if not already enabled
//     CoreDebug->DEMCR |= CoreDebug_DEMCR_TRCENA_Msk;

//     // Reset the cycle counter
//     DWT->CYCCNT = 0;

//     // Enable the cycle counter
//     DWT->CTRL |= DWT_CTRL_CYCCNTENA_Msk;

//     dwt_initialized = true;
//   }
// }

// extern "C" uint32_t tflite_micro_time()
// {
//   InitializeDWT();

//   // Return cycle count - you may want to convert this to microseconds
//   // if your SystemCoreClock is known and stable
//   return DWT->CYCCNT;
// }

// Option 2: Using ThreadX timer (alternative implementation)
// Uncomment this section if you prefer to use ThreadX timing

extern "C" uint32_t tflite_micro_time()
{
  // ThreadX provides timer ticks - convert to microseconds
  // TX_TIMER_TICKS_PER_SECOND is usually defined in tx_port.h

  ULONG current_time = tx_time_get();

  // Convert ThreadX ticks to microseconds
  // Assuming 1000 ticks per second (adjust based on your ThreadX configuration)
  const uint32_t ticks_per_second = 1000; // Adjust this value
  const uint32_t microseconds_per_second = 1000000;

  return (current_time * microseconds_per_second) / ticks_per_second;
}

// Option 3: Using SysTick (alternative implementation)
// Uncomment this section if you prefer to use SysTick
/*
static volatile uint32_t systick_counter = 0;

extern "C" void SysTick_Handler(void) {
  systick_counter++;
}

extern "C" uint32_t tflite_micro_time() {
  // Convert SysTick ticks to microseconds
  // This assumes SysTick is configured to tick at 1kHz (1ms intervals)
  return systick_counter * 1000;  // Convert milliseconds to microseconds
}
*/
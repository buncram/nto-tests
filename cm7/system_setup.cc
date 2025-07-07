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

#include "tensorflow/lite/micro/system_setup.h"

// Include platform-specific headers
#include "daric_cm7.h"
#include "daric_util.h"
#include "tx_api.h" // Needed for DebugLog, which might use ThreadX-aware printf if hooked

// This function is called by the TFLM framework to perform target-specific setup.
// We keep it minimal here, focusing on essential TFLM requirements like FPU and UART.
// MPU and general cache setup are handled by the main application (mbox.c)
// to avoid conflicts and redundant configurations.
extern "C" void InitializeTarget()
{
    // 1. Initialize UART for debug logging (if not already done).
    // This ensures TFLM's DebugLog can output to the DUART.
    // Assuming initDUART is idempotent or safe to call multiple times.
    initDUART(115200); // Adjust baud rate as needed

    // 2. Enable FPU if using floating point operations in TFLM.
    // This is crucial for Cortex-M7 with FPU, as the sine model uses floats.
    enable_fpu();

    // NOTE: MPU and Cache configuration are intentionally *removed* from here.
    // These are handled by the main_loop in mbox.c for comprehensive system setup.
    // Placing them here would be redundant and could conflict with mbox.c's setup.

    // 3. Print initialization status using TFLM's DebugLog.
    DebugLog("TFLM target initialization complete (minimal setup).");
}

// Optional: Additional setup function for post-ThreadX initialization
// Call this after ThreadX has started if you need RTOS-aware initialization
extern "C" void InitializeTargetPostRTOS()
{
    // Any initialization that requires ThreadX to be running
    // For example:
    // - Creating ThreadX threads for ML inference
    // - Setting up RTOS-aware peripherals
    // - Allocating ThreadX memory pools for ML models

    DebugLog("TFLM post-RTOS initialization complete");
}

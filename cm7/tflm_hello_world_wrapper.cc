#include "tensorflow/lite/micro/micro_allocator.h"
#include "tensorflow/lite/micro/micro_interpreter.h"
#include "tensorflow/lite/micro/micro_mutable_op_resolver.h"
#include "tensorflow/lite/micro/system_setup.h"
#include "tensorflow/lite/schema/schema_generated.h"
#include "examples/hello_world/models/hello_world_float_model_data.h"

#include <cmath>

// Bring in the C headers for your project's HAL and utility functions
#include "daric_hal.h"
#include "daric_util.h"
#include "mbox.h"

// Your custom logger header
#include "tflm_debug_log.h"

extern "C" void run_tflm_hello_world_test(void)
{
    print_string("Entered TFLM C++ wrapper function.\r\n");

    // 1. Set up logging (using your custom function via the header)
    DebugLog("TFLM: 1. Logging ready.");

    // 2. Set up any platform-specific initializations
    tflite::InitializeTarget();
    DebugLog("TFLM: 2. Target initialized.");

    // 3. Map the model into a usable data structure
    const tflite::Model *model = tflite::GetModel(g_hello_world_float_model_data);
    if (model->version() != TFLITE_SCHEMA_VERSION)
    {
        DebugLog("Model schema version mismatch.");
        return;
    }
    DebugLog("TFLM: 3. Model mapped.");

    // 4. Pull in only the operation implementations we need
    tflite::MicroMutableOpResolver<1> micro_op_resolver;
    if (micro_op_resolver.AddFullyConnected() != kTfLiteOk)
    {
        DebugLog("Failed to add FullyConnected op.");
        return;
    }
    DebugLog("TFLM: 4. Op resolver created.");

    // 5. Define the Tensor Arena
    const int kTensorArenaSize = 4 * 1024;
    uint8_t tensor_arena[kTensorArenaSize];
    DebugLog("TFLM: 5. Tensor arena defined on stack.");

    // 6. Create the MicroAllocator
    tflite::MicroAllocator *allocator = tflite::MicroAllocator::Create(tensor_arena, kTensorArenaSize);
    if (allocator == nullptr)
    {
        DebugLog("Failed to create MicroAllocator!");
        return;
    }
    DebugLog("TFLM: 6. Allocator created.");

    // 7. Build an interpreter, passing it the allocator
    tflite::MicroInterpreter interpreter(model, micro_op_resolver, allocator);
    DebugLog("TFLM: 7. Interpreter built.");

    // 8. Allocate memory for the model's tensors
    if (interpreter.AllocateTensors() != kTfLiteOk)
    {
        DebugLog("AllocateTensors() failed.");
        return;
    }
    DebugLog("TFLM: 8. Tensors allocated.");

    // 9. Obtain pointers to the model's input and output tensors
    TfLiteTensor *input = interpreter.input(0);
    TfLiteTensor *output = interpreter.output(0);
    DebugLog("TFLM: 9. Input/output tensors obtained.");

    DebugLog("Starting inference...");

    // Define how many test points you want
    const int kNumSteps = 20;

    // Create and populate the test data array
    float x_test[kNumSteps];
    const float kTwoPi = 2.0f * 3.14159265359f;
    for (int i = 0; i < kNumSteps; ++i)
    {
        x_test[i] = (static_cast<float>(i) / (kNumSteps - 1)) * kTwoPi;
    }

    // Run inference for each test point
    for (int i = 0; i < kNumSteps; ++i)
    {
        float x_val = x_test[i];
        input->data.f[0] = x_val;

        if (interpreter.Invoke() != kTfLiteOk)
        {
            DebugLog("Invoke failed on x: %f", static_cast<double>(x_val));
            continue;
        }

        float y_val = output->data.f[0];
        DebugLog("x_val: %f, inferred y_val: %f, actual y_val: %f",
                 static_cast<double>(x_val),
                 static_cast<double>(y_val),
                 static_cast<double>(sin(x_val)));
    }
}
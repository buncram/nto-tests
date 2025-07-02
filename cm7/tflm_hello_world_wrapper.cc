#include "tensorflow/lite/micro/micro_allocator.h"
#include "tensorflow/lite/micro/micro_interpreter.h"
#include "tensorflow/lite/micro/micro_mutable_op_resolver.h"
#include "tensorflow/lite/micro/system_setup.h"
#include "tensorflow/lite/micro/tflite_bridge/micro_error_reporter.h"
#include "tensorflow/lite/schema/schema_generated.h"
#include "examples/hello_world/models/hello_world_float_model_data.h"

#include <cmath>

// Bring in the C headers for your project's HAL and utility functions
#include "daric_hal.h"
#include "daric_util.h"
#include "mbox.h"

extern "C" void run_tflm_hello_world_test(void)
{
    print_string("Entered TFLM C++ wrapper function.\r\n");

    // 1. Set up logging
    tflite::MicroErrorReporter micro_error_reporter;
    tflite::ErrorReporter *error_reporter = &micro_error_reporter;
    TF_LITE_REPORT_ERROR(error_reporter, "TFLM: 1. Logging ready.");

    // 2. Set up any platform-specific initializations
    tflite::InitializeTarget();
    TF_LITE_REPORT_ERROR(error_reporter, "TFLM: 2. Target initialized.");

    // 3. Map the model into a usable data structure
    const tflite::Model *model = tflite::GetModel(g_hello_world_float_model_data);
    if (model->version() != TFLITE_SCHEMA_VERSION)
    {
        TF_LITE_REPORT_ERROR(error_reporter, "Model schema version mismatch.");
        return;
    }
    TF_LITE_REPORT_ERROR(error_reporter, "TFLM: 3. Model mapped.");

    // 4. Pull in only the operation implementations we need
    tflite::MicroMutableOpResolver<1> micro_op_resolver;
    if (micro_op_resolver.AddFullyConnected() != kTfLiteOk)
    {
        TF_LITE_REPORT_ERROR(error_reporter, "Failed to add FullyConnected op.");
        return;
    }
    TF_LITE_REPORT_ERROR(error_reporter, "TFLM: 4. Op resolver created.");

    // 5. Define the Tensor Arena
    const int kTensorArenaSize = 4 * 1024;
    uint8_t tensor_arena[kTensorArenaSize];
    TF_LITE_REPORT_ERROR(error_reporter, "TFLM: 5. Tensor arena defined on stack.");

    // 6. Create the MicroAllocator
    tflite::MicroAllocator *allocator = tflite::MicroAllocator::Create(tensor_arena, kTensorArenaSize);
    if (allocator == nullptr)
    {
        TF_LITE_REPORT_ERROR(error_reporter, "Failed to create MicroAllocator!");
        return;
    }
    TF_LITE_REPORT_ERROR(error_reporter, "TFLM: 6. Allocator created.");

    // 7. Build an interpreter, passing it the allocator
    tflite::MicroInterpreter interpreter(model, micro_op_resolver, allocator);
    TF_LITE_REPORT_ERROR(error_reporter, "TFLM: 7. Interpreter built.");

    // 8. Allocate memory for the model's tensors
    if (interpreter.AllocateTensors() != kTfLiteOk)
    {
        TF_LITE_REPORT_ERROR(error_reporter, "AllocateTensors() failed.");
        return;
    }
    TF_LITE_REPORT_ERROR(error_reporter, "TFLM: 8. Tensors allocated.");

    // 9. Obtain pointers to the model's input and output tensors
    TfLiteTensor *input = interpreter.input(0);
    TfLiteTensor *output = interpreter.output(0);
    TF_LITE_REPORT_ERROR(error_reporter, "TFLM: 9. Input/output tensors obtained.");

    TF_LITE_REPORT_ERROR(error_reporter, "Starting inference...");

    float x_test[] = {0.0f, 1.0f, 1.570796f, 3.14159f, 4.7123889f, 6.283185f};
    int num_tests = sizeof(x_test) / sizeof(float);

    for (int i = 0; i < num_tests; ++i)
    {
        float x_val = x_test[i];
        input->data.f[0] = x_val;

        if (interpreter.Invoke() != kTfLiteOk)
        {
            TF_LITE_REPORT_ERROR(error_reporter, "Invoke failed on x: %f", static_cast<double>(x_val));
            continue;
        }

        float y_val = output->data.f[0];
        TF_LITE_REPORT_ERROR(error_reporter, "x_val: %f, inferred y_val: %f, actual y_val: %f",
                             static_cast<double>(x_val),
                             static_cast<double>(y_val),
                             static_cast<double>(sin(x_val)));
    }
}
#include "tensorflow/lite/micro/micro_allocator.h"
#include "tensorflow/lite/micro/micro_interpreter.h"
#include "tensorflow/lite/micro/micro_mutable_op_resolver.h"
#include "tensorflow/lite/micro/system_setup.h"
#include "tensorflow/lite/micro/tflite_bridge/micro_error_reporter.h"
#include "tensorflow/lite/schema/schema_generated.h"
#include "examples/hello_world/models/hello_world_float_model_data.h"

#include "tflm_hello_world_wrapper.h"
#include "debug_log.h" // Your new header

#include <cmath>

namespace
{
    const int kTensorArenaSize = 4 * 1024;
    alignas(16) uint8_t tensor_arena[kTensorArenaSize];
} // namespace

extern "C"
{
    void print_string(const char *s);
    void send_u32_hex(uint32_t x);
}

void run_direct_print_test()
{
    print_string("\r\n--- Direct C Function Print Test ---\r\n");

    // 1. Test integer argument passing
    int my_int = 12345; // Hex value is 0x3039
    print_string("Testing integer 12345 (should be 00003039): ");
    send_u32_hex(my_int);
    print_string("\r\n");

    // 2. Test float memory access by printing its raw bits
    float my_float = 3.14f; // IEEE 754 hex is 0x4048F5C3
    // Use a pointer cast to get the raw 32-bit integer representation of the float
    uint32_t float_as_int = *reinterpret_cast<uint32_t *>(&my_float);
    print_string("Testing float 3.14 (bits should be 4048F5C3): ");
    send_u32_hex(float_as_int);
    print_string("\r\n");

    print_string("--- Direct Print Test Complete ---\r\n\r\n");
}

void run_cpp_memory_test(tflite::ErrorReporter *error_reporter)
{
    TF_LITE_REPORT_ERROR(error_reporter, "--- C++ Memory Test ---");

    // Test local variables
    int my_int = 0;
    float my_float = 3.14f;
    TF_LITE_REPORT_ERROR(error_reporter, "Local int: %d, Local float: %f", my_int, static_cast<double>(my_float));

    // Test writing to and reading from an array on the stack
    float test_array[4];
    test_array[0] = 1.1f;
    test_array[1] = 2.2f;
    test_array[2] = -9.87f;

    if (test_array[1] == 2.2f)
    {
        TF_LITE_REPORT_ERROR(error_reporter, "Array read/write SUCCESS. Values: %f, %f, %f",
                             static_cast<double>(test_array[0]),
                             static_cast<double>(test_array[1]),
                             static_cast<double>(test_array[2]));
    }
    else
    {
        TF_LITE_REPORT_ERROR(error_reporter, "Array read/write FAILURE.");
    }
    TF_LITE_REPORT_ERROR(error_reporter, "--- C++ Memory Test Complete ---");
}

extern "C" void run_tflm_hello_world_test(void)
{
    // Use the standard error reporter. It will automatically find your DebugLog function.
    tflite::MicroErrorReporter micro_error_reporter;
    tflite::ErrorReporter *error_reporter = &micro_error_reporter;

    run_direct_print_test();

    TF_LITE_REPORT_ERROR(error_reporter, "--- TFLM Hello World Test ---");

    run_cpp_memory_test(error_reporter);

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

    float x_test[] = {0.0f, 1.0f, 1.570796f, 3.14159f}; // Shortened for brevity
    for (float x_val : x_test)
    {
        input->data.f[0] = x_val;

        if (interpreter.Invoke() != kTfLiteOk)
        {
            TF_LITE_REPORT_ERROR(error_reporter, "Invoke failed on x: %f", static_cast<double>(x_val));
            continue;
        }

        float y_val = output->data.f[0];
        TF_LITE_REPORT_ERROR(error_reporter, "x_val: %.4f, inferred y: %.4f, actual y: %.4f",
                             static_cast<double>(x_val),
                             static_cast<double>(y_val),
                             static_cast<double>(sin(x_val)));
    }
    TF_LITE_REPORT_ERROR(error_reporter, "--- Test Complete ---");
}

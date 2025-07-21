// tflm_person_detection_wrapper.cc

// TFLM-specific headers
#include "tflm/tensorflow/lite/c/common.h"
#include "tflm/tensorflow/lite/micro/micro_interpreter.h"
#include "tflm/tensorflow/lite/micro/micro_mutable_op_resolver.h"
#include "tflm/tensorflow/lite/micro/models/person_detect_model_data.h"
#include "tflm/tensorflow/lite/schema/schema_generated.h"

// Header for model settings and test data
#include "tflm/examples/person_detection/model_settings.h"
#include "tflm/examples/person_detection/testdata/person_image_data.h"
#include "tflm/examples/person_detection/testdata/no_person_image_data.h"
#include "tflm/examples/person_detection/testdata/bunnie_image_data.h" // New include

// Your custom logger
#include "debug_log.h"
#include <stdio.h> // Required for sprintf

#include "tx_api.h" // ADDED: ThreadX API for timing functions

// Create an area of memory to use for input, output, and intermediate arrays.
#if defined(XTENSA) && defined(VISION_P6)
constexpr int kTensorArenaSize = 352 * 1024;
#else
constexpr int kTensorArenaSize = 136 * 1024;
#endif
uint8_t tensor_arena[kTensorArenaSize];

// Expose a C-friendly function to run the test
extern "C" void run_tflm_person_detection_test(void)
{
    char log_buffer[256];
    DebugSane("--- Running TFLM Person Detection Test from Wrapper ---\r\n");

    // 1. Set up the model, Op Resolver, and Interpreter
    const tflite::Model *model = ::tflite::GetModel(g_person_detect_model_data);
    if (model->version() != TFLITE_SCHEMA_VERSION)
    {
        DebugSane("Error: Model schema version mismatch.\r\n");
        return;
    }
    tflite::MicroMutableOpResolver<5> micro_op_resolver;
    micro_op_resolver.AddAveragePool2D(tflite::Register_AVERAGE_POOL_2D_INT8());
    micro_op_resolver.AddConv2D(tflite::Register_CONV_2D_INT8());
    micro_op_resolver.AddDepthwiseConv2D(tflite::Register_DEPTHWISE_CONV_2D_INT8());
    micro_op_resolver.AddReshape();
    micro_op_resolver.AddSoftmax(tflite::Register_SOFTMAX_INT8());
    tflite::MicroInterpreter interpreter(model, micro_op_resolver, tensor_arena, kTensorArenaSize);
    if (interpreter.AllocateTensors() != kTfLiteOk)
    {
        DebugSane("Error: AllocateTensors() failed.\r\n");
        return;
    }
    TfLiteTensor *input = interpreter.input(0);
    TfLiteTensor *output = nullptr;       // Define once
    int8_t person_score, no_person_score; // Define once
    ULONG start_time_ms, duration_ms;     // ADDED: Variables for timing

    // --- TEST 1: INFERENCE WITH "PERSON" IMAGE ---
    DebugSane("\r\nRunning test with 'person' image...\r\n");
    memcpy(input->data.int8, g_person_image_data, input->bytes);

    start_time_ms = tx_time_get(); // ADDED: Get start time in milliseconds
    interpreter.Invoke();
    duration_ms = tx_time_get() - start_time_ms; // ADDED: Calculate duration

    output = interpreter.output(0);
    person_score = output->data.int8[kPersonIndex];
    no_person_score = output->data.int8[kNotAPersonIndex];

    sprintf(log_buffer, "Inference took %lu ms.\r\n", duration_ms); // ADDED: Print duration
    DebugSane(log_buffer);

    sprintf(log_buffer, "Scores -> Person: %d, No Person: %d\r\n", person_score, no_person_score);
    DebugSane(log_buffer);
    if (person_score > no_person_score)
        DebugSane("SUCCESS: Person correctly detected.\r\n");
    else
        DebugSane("FAILURE: Person score was not higher.\r\n");

    // --- TEST 2: INFERENCE WITH "NO PERSON" IMAGE ---
    DebugSane("\r\nRunning test with 'no person' image...\r\n");
    memcpy(input->data.int8, g_no_person_image_data, input->bytes);

    start_time_ms = tx_time_get(); // ADDED: Get start time in milliseconds
    interpreter.Invoke();
    duration_ms = tx_time_get() - start_time_ms; // ADDED: Calculate duration

    output = interpreter.output(0);
    person_score = output->data.int8[kPersonIndex];
    no_person_score = output->data.int8[kNotAPersonIndex];

    sprintf(log_buffer, "Inference took %lu ms.\r\n", duration_ms); // ADDED: Print duration
    DebugSane(log_buffer);

    sprintf(log_buffer, "Scores -> Person: %d, No Person: %d\r\n", person_score, no_person_score);
    DebugSane(log_buffer);
    if (no_person_score > person_score)
        DebugSane("SUCCESS: No person correctly detected.\r\n");
    else
        DebugSane("FAILURE: No person score was not higher.\r\n");

    // --- TEST 3: INFERENCE WITH "BUNNY" IMAGE ---
    DebugSane("\r\nRunning test with 'bunny' image...\r\n");
    memcpy(input->data.int8, g_bunnie_image_data, input->bytes);

    start_time_ms = tx_time_get(); // ADDED: Get start time in milliseconds
    interpreter.Invoke();
    duration_ms = tx_time_get() - start_time_ms; // ADDED: Calculate duration

    output = interpreter.output(0);
    person_score = output->data.int8[kPersonIndex];
    no_person_score = output->data.int8[kNotAPersonIndex];

    sprintf(log_buffer, "Inference took %lu ms.\r\n", duration_ms); // ADDED: Print duration
    DebugSane(log_buffer);

    sprintf(log_buffer, "Scores -> Person: %d, No Person: %d\r\n", person_score, no_person_score);
    DebugSane(log_buffer);
    if (no_person_score > person_score)
        DebugSane("SUCCESS: Bunny correctly classified as 'no person'.\r\n");
    else
        DebugSane("FAILURE: Bunny was likely classified as a person.\r\n");

    DebugSane("\r\n--- Test finished. ---\r\n");
}
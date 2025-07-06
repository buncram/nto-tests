#ifndef TFLM_HELLO_WORLD_WRAPPER_H
#define TFLM_HELLO_WORLD_WRAPPER_H

// This header declares the C++ test function so that C code can call it.
#ifdef __cplusplus
extern "C"
{
#endif

    void run_tflm_hello_world_test(void);

#ifdef __cplusplus
}
#endif

#endif // TFLM_HELLO_WORLD_WRAPPER_H
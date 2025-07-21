#ifndef TFLM_DEBUG_LOG_H_
#define TFLM_DEBUG_LOG_H_

// This allows the header to be used in both C and C++ code
#ifdef __cplusplus
extern "C"
{
#endif

    // The function declaration
    void DebugLog(const char *format, va_list args);

#ifdef __cplusplus
}
#endif

#endif // TFLM_DEBUG_LOG_H_
/* Platform Integration Header for TensorFlow Lite Micro
 * Cortex-M7 + ThreadX Platform
 */

#ifndef TFLM_PLATFORM_H
#define TFLM_PLATFORM_H

#ifdef __cplusplus
extern "C"
{
#endif

// Platform-specific configuration
#define TFLM_PLATFORM_CORTEX_M7
#define TFLM_PLATFORM_THREADX

// Memory configuration for TFLM
// Adjust these values based on your available RAM and model requirements
#define TFLM_TENSOR_ARENA_SIZE_KB 64 // 64KB for tensor arena (adjust as needed)
#define TFLM_MAX_LOG_LENGTH 256      // Maximum debug log message length

// Cache management macros for TFLM tensor operations
// These help manage cache coherency for ML tensor data
#define TFLM_CACHE_CLEAN_TENSOR_DATA(ptr, size)                      \
    do                                                               \
    {                                                                \
        SCB_CleanDCache_by_Addr((uint32_t *)(ptr), (int32_t)(size)); \
    } while (0)

#define TFLM_CACHE_INVALIDATE_TENSOR_DATA(ptr, size)                      \
    do                                                                    \
    {                                                                     \
        SCB_InvalidateDCache_by_Addr((uint32_t *)(ptr), (int32_t)(size)); \
    } while (0)

// Enable DCache for TFLM operations (use carefully with ThreadX)
#define TFLM_ENABLE_DCACHE_FOR_INFERENCE() \
    do                                     \
    {                                      \
        SCB_EnableDCache();                \
    } while (0)

// Disable DCache after TFLM operations
#define TFLM_DISABLE_DCACHE_AFTER_INFERENCE() \
    do                                        \
    {                                         \
        SCB_DisableDCache();                  \
    } while (0)

    // Function declarations
    void InitializeTarget(void);
    void InitializeTargetPostRTOS(void);

    // Utility functions for TFLM integration
    void tflm_enable_caches_for_inference(void);
    void tflm_disable_caches_after_inference(void);
    void tflm_platform_print_memory_info(void);

#ifdef __cplusplus
}
#endif

#endif // TFLM_PLATFORM_H
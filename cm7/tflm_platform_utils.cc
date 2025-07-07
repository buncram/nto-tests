/* Platform Utility Functions for TensorFlow Lite Micro
 * Cortex-M7 + ThreadX Platform
 */

#include "tflm_platform.h"
#include "ARMCM7.h"     // Or system_ARMCM7.h based on your project's CMSIS structure
#include "daric_util.h" // For print_string and SystemCoreClock

// --- CORRECTED TFLM header includes based on 'find' output ---
#include "tensorflow/lite/micro/debug_log.h"
#include "tensorflow/lite/micro/tflite_bridge/micro_error_reporter.h" // <--- CORRECTED PATH

#include <cstdio>  // For snprintf, vsnprintf
#include <cstdarg> // For va_list, va_start, va_end
#include <cstring> // For memset
#include <cstdint> // For uint32_t

// Declare SystemCoreClock as external, as it's defined in mbox.c or daric_util.c
extern uint32_t SystemCoreClock;

// TFLM Debug Log Wrapper
// The DebugLog function in TFLM's debug_log.h expects a va_list.
// This wrapper provides a printf-like interface.
void TFLM_DebugPrint(const char *format, ...)
{
    va_list args;
    va_start(args, format);
    DebugLog(format, args);
    va_end(args);
}

// Implement your MicroErrorReporter
namespace tflite
{
    namespace micro
    {

        class DaricErrorReporter : public MicroErrorReporter
        {
        public:
            DaricErrorReporter() : buffer_index_(0)
            {
                buffer_[0] = '\0'; // Initialize buffer
            }

            // --- ADD THIS MACRO TO PREVENT IMPLICIT DESTRUCTOR DELETION ---
            TF_LITE_REMOVE_VIRTUAL_DELETE

            int Report(const char *format, va_list args) override
            {
                int chars_written = vsnprintf(buffer_ + buffer_index_,
                                              TFLM_MAX_LOG_LENGTH - buffer_index_,
                                              format, args);
                if (chars_written > 0)
                {
                    buffer_index_ += chars_written;
                    if (buffer_index_ >= TFLM_MAX_LOG_LENGTH - 1)
                    {
                        buffer_index_ = TFLM_MAX_LOG_LENGTH - 1;
                    }
                    buffer_[buffer_index_] = '\0'; // Ensure null termination
                    print_string(buffer_);
                    buffer_index_ = 0; // Reset buffer for next log
                }
                return chars_written;
            }

        private:
            char buffer_[TFLM_MAX_LOG_LENGTH];
            int buffer_index_;
        };

        // Global instance of the error reporter
        DaricErrorReporter daric_error_reporter;

        MicroErrorReporter *GetMicroErrorReporter()
        {
            return &daric_error_reporter;
        }

    } // namespace micro
} // namespace tflite

extern "C" void tflm_enable_caches_for_inference(void)
{
    TFLM_DebugPrint("TFLM: ICache status: %s", (SCB->CCR & SCB_CCR_IC_Msk) ? "Enabled" : "Disabled");

#ifdef TFLM_USE_DCACHE_DURING_INFERENCE
    if (!(SCB->CCR & SCB_CCR_DC_Msk))
    {
        TFLM_DebugPrint("TFLM: Enabling DCache for inference");
        SCB_EnableDCache();
    }
    else
    {
        TFLM_DebugPrint("TFLM: DCache already enabled for inference");
    }
#else
    TFLM_DebugPrint("TFLM: DCache not enabled for inference (TFLM_USE_DCACHE_DURING_INFERENCE not defined)");
#endif
}

extern "C" void tflm_disable_caches_after_inference(void)
{
#ifdef TFLM_USE_DCACHE_DURING_INFERENCE
    if (SCB->CCR & SCB_CCR_DC_Msk)
    {
        TFLM_DebugPrint("TFLM: Disabling DCache after inference");
        SCB_DisableDCache();
    }
    else
    {
        TFLM_DebugPrint("TFLM: DCache already disabled after inference");
    }
#endif
    TFLM_DebugPrint("TFLM: ICache status: %s", (SCB->CCR & SCB_CCR_IC_Msk) ? "Enabled" : "Disabled");
}

extern "C" void tflm_platform_print_memory_info(void)
{
    TFLM_DebugPrint("=== TFLM Platform Memory Info ===");
    TFLM_DebugPrint("System Core Clock: %lu Hz", SystemCoreClock);
    TFLM_DebugPrint("SRAM Base: 0x%08lX", 0x61000000UL);
    TFLM_DebugPrint("SRAM Size: %d MB", 2);
    TFLM_DebugPrint("Tensor Arena Size: %d KB", TFLM_TENSOR_ARENA_SIZE_KB);

    uint32_t ccr = SCB->CCR;
    TFLM_DebugPrint("ICache: %s", (ccr & SCB_CCR_IC_Msk) ? "Enabled" : "Disabled");
    TFLM_DebugPrint("DCache: %s", (ccr & SCB_CCR_DC_Msk) ? "Enabled" : "Disabled");
    TFLM_DebugPrint("=====================================");
}

static uint8_t tensor_arena[TFLM_TENSOR_ARENA_SIZE_KB * 1024] __attribute__((aligned(16)));
static bool tensor_arena_initialized = false;

extern "C" uint8_t *tflm_get_tensor_arena(void)
{
    if (!tensor_arena_initialized)
    {
        memset(tensor_arena, 0, sizeof(tensor_arena));
        tensor_arena_initialized = true;
        TFLM_DebugPrint("TFLM: Initialized tensor arena at 0x%08lX, size %lu bytes",
                        (uint32_t)tensor_arena, (unsigned long)sizeof(tensor_arena));
    }
    return tensor_arena;
}

extern "C" size_t tflm_get_tensor_arena_size(void)
{
    return sizeof(tensor_arena);
}

extern "C" void tflm_platform_error_handler(const char *error_msg)
{
    TFLM_DebugPrint("TFLM ERROR: %s", error_msg);
    tflm_disable_caches_after_inference();
    while (1)
    {
        __asm__("nop");
    }
}

extern "C" void InitializeTarget(void)
{
    TFLM_DebugPrint("TFLM: InitializeTarget called.");
}

extern "C" void InitializeTargetPostRTOS(void)
{
    TFLM_DebugPrint("TFLM: InitializeTargetPostRTOS called.");
}
# vim: noet:ts=4:sw=4:

fullmf := $(abspath $(lastword $(MAKEFILE_LIST)))
pathmf := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))

include makefile.daric.common

## Use g++ for C++ projects
LD      :=  $(TOOLCHAIN)g++
CXX    :=  $(TOOLCHAIN)g++ # Explicitly define CXX for C++ compilation

TARGET = mbox
LD_FILES ?= $(pathmf)/gcc_daric_mbox.ld

FIDO2_ROOT := ../../threadx_mono

THREADX_COMMON_PATH := $(FIDO2_ROOT)/daric_sdk/middleware/threadx/kernel/common/src
THREADX_PORT_PATH := $(FIDO2_ROOT)/daric_sdk/middleware/threadx/kernel/ports_arch/ARMv7-M/threadx/gnu/src
THREADX_PORT_EXAMPLE_PATH := $(FIDO2_ROOT)/daric_sdk/middleware/threadx/kernel/ports_arch/ARMv7-M/threadx/gnu/example_build
THREADX_PORT_INCLUDE_PATH := $(FIDO2_ROOT)/daric_sdk/middleware/threadx/kernel/ports_arch/ARMv7-M/threadx/inc
LIBC_HOOK_PATH := $(FIDO2_ROOT)/daric_sdk/middleware/libs/libc/newlib
APP_SETUP_PATH := .

TFLM_ROOT := $(pathmf)/tflm

# Discover all .su, .c, and .cc files under tflm/ that are not tests or internal to signal/micro/kernels
# We are being more selective now, as many core files are at the top level
# Exclude test files (*_test.cc)
TFLM_CORE_SRCS_C := $(shell find $(TFLM_ROOT) -name "*.c" ! -name "*_test.c")
TFLM_CORE_SRCS_CXX := $(shell find $(TFLM_ROOT) -name "*.cc" ! -name "*_test.cc")

# Remove some specific .su files that might be just symbolic links or duplicates if find picks them up
# and we are explicitly listing their actual .cc/.c counterparts.
# This list might need adjustment based on compilation errors.
TFLM_EXCLUDE_SRCS := \
    $(TFLM_ROOT)/examples/person_detection/main.cc \
    $(TFLM_ROOT)/tensorflow/lite/micro/debug_log.cc \

TFLM_CORE_SRCS_C := $(filter-out $(TFLM_EXCLUDE_SRCS), $(TFLM_CORE_SRCS_C))
TFLM_CORE_SRCS_CXX := $(filter-out $(TFLM_EXCLUDE_SRCS), $(TFLM_CORE_SRCS_CXX))

# Explicitly add the example-specific source files that are clearly present
# We see these under tflm/examples/person_detection directly.
PERSON_DETECTION_SRCS_CXX := \
    $(TFLM_ROOT)/examples/person_detection/main.cc \
    $(TFLM_ROOT)/examples/person_detection/main_functions.cc \
    $(TFLM_ROOT)/examples/person_detection/model_settings.cc \
    $(TFLM_ROOT)/examples/person_detection/detection_responder.cc \
    $(TFLM_ROOT)/examples/person_detection/image_provider.cc

HELLO_WORLD_SRCS_CXX := \
    $(TFLM_ROOT)/examples/hello_world/hello_world_test.cc

# Signal library sources (needed by micro_speech)
# SIGNAL_SRCS_CXX := \
#   $(TFLM_ROOT)/signal/src/circular_buffer.cc \
#   $(TFLM_ROOT)/signal/src/energy.cc \
#   $(TFLM_ROOT)/signal/src/fft_auto_scale.cc \
#   $(TFLM_ROOT)/signal/src/filter_bank.cc \
#   $(TFLM_ROOT)/signal/src/filter_bank_log.cc \
#   $(TFLM_ROOT)/signal/src/filter_bank_spectral_subtraction.cc \
#   $(TFLM_ROOT)/signal/src/filter_bank_square_root.cc \
#   $(TFLM_ROOT)/signal/src/irfft_float.cc \
#   $(TFLM_ROOT)/signal/src/irfft_int16.cc \
#   $(TFLM_ROOT)/signal/src/irfft_int32.cc \
#   $(TFLM_ROOT)/signal/src/log.cc \
#   $(TFLM_ROOT)/signal/src/max_abs.cc \
#   $(TFLM_ROOT)/signal/src/msb_32.cc \
#   $(TFLM_ROOT)/signal/src/msb_64.cc \
#   $(TFLM_ROOT)/signal/src/overlap_add.cc \
#   $(TFLM_ROOT)/signal/src/pcan_argc_fixed.cc \
#   $(TFLM_ROOT)/signal/src/rfft_float.cc \
#   $(TFLM_ROOT)/signal/src/rfft_int16.cc \
#   $(TFLM_ROOT)/signal/src/rfft_int32.cc \
#   $(TFLM_ROOT)/signal/src/square_root_32.cc \
#   $(TFLM_ROOT)/signal/src/square_root_64.cc \
#   $(TFLM_ROOT)/signal/src/window.cc

# MICRO_SPEECH_TEST_SRCS_CXX := \
#     $(TFLM_ROOT)/examples/micro_speech/testdata/no_1000ms_audio_data.cc \
#     $(TFLM_ROOT)/examples/micro_speech/testdata/silence_1000ms_audio_data.cc \
#     $(TFLM_ROOT)/examples/micro_speech/testdata/yes_1000ms_audio_data.cc \
#     $(TFLM_ROOT)/examples/micro_speech/testdata/noise_1000ms_audio_data.cc \
#     $(TFLM_ROOT)/examples/micro_speech/testdata/yes_30ms_audio_data.cc \
#     $(TFLM_ROOT)/examples/micro_speech/testdata/no_30ms_audio_data.cc


# KissFFT library sources (part of third_party, needed by signal)
KISSFFT_SRCS_C := $(TFLM_ROOT)/third_party/kissfft/kiss_fft.c

# Add your existing project C++ sources that you defined
PROJECT_CXX_SRCS := \
    micro_time.cc \
    system_setup.cc \
    tflm_platform_utils.cc \
    dso_handle.cc \
    tflm_hello_world_wrapper.cc \
    #debug_log.cc

# --- Automatically find all source files in those directories ---
THREADX_COMMON_SRCS_C := $(wildcard $(THREADX_COMMON_PATH)/*.c)
THREADX_PORT_SRCS_C   := $(wildcard $(THREADX_PORT_PATH)/*.c)
THREADX_PORT_SRCS_S   := $(wildcard $(THREADX_PORT_PATH)/*.S)
THREADX_INIT_SRC_S  := $(wildcard $(THREADX_PORT_EXAMPLE_PATH)/tx_initialize_low_level.S)

# --- Create final source lists ---
PROJECT_SRCS = mbox.c daric_util.c reram.c threadx_app_setup.c tflm_debug_log.c
LIBC_HOOK_SRCS = libc-hooks.c

C_SRCS := \
    $(PROJECT_SRCS) \
    $(APP_SRCS) \
    $(LIBC_HOOK_SRCS) \
    $(notdir $(THREADX_COMMON_SRCS_C)) \
    $(notdir $(THREADX_PORT_SRCS_C)) \
    $(TFLM_CORE_SRCS_C) \
    $(KISSFFT_SRCS_C) # Added KissFFT C source

# Combine all C++ sources
CXX_SRCS := \
    $(PROJECT_CXX_SRCS) \
    $(TFLM_CORE_SRCS_CXX) \
    $(HELLO_WORLD_SRCS_CXX) \
    # $(PERSON_DETECTION_SRCS_CXX) \
    # $(SIGNAL_SRCS_CXX) \
    # $(MICRO_SPEECH_TEST_SRCS_CXX)


S_SRCS := \
    $(notdir $(THREADX_PORT_SRCS_S)) \
    $(notdir $(THREADX_INIT_SRC_S))

# --- Create the final object lists from the source lists ---
OBJS_C := $(patsubst %.c,%.o,$(C_SRCS))
OBJS_S := $(patsubst %.S,%.o,$(S_SRCS))
OBJS_CXX := $(patsubst %.cc,%.o,$(CXX_SRCS)) # New object list for C++ sources
OBJS   := $(OBJS_C) $(OBJS_S) $(OBJS_CXX) # Combined object list

# --- Add all source directories to VPATH so 'make' can find the files ---
VPATH := $(APP_SETUP_PATH) \
         $(LIBC_HOOK_PATH) \
         $(THREADX_COMMON_PATH) \
         $(THREADX_PORT_PATH) \
         $(THREADX_PORT_EXAMPLE_PATH) \
         $(TFLM_ROOT) \
         $(TFLM_ROOT)/tensorflow/lite/micro \
         $(TFLM_ROOT)/tensorflow/lite/micro/tflite_bridge \
         $(TFLM_ROOT)/tensorflow/lite/micro/kernels \
         $(TFLM_ROOT)/tensorflow/lite/micro/tools/make/downloads/cmsis \
         $(TFLM_ROOT)/tensorflow/lite/micro/tools/make/downloads/kissfft \
         $(TFLM_ROOT)/tensorflow/lite/micro/tools/make/downloads/flatbuffers/include \
         $(TFLM_ROOT)/tensorflow/lite/micro/compatibility \
         $(TFLM_ROOT)/examples/person_detection \
         $(TFLM_ROOT)/examples/micro_speech \
         $(TFLM_ROOT)/examples/micro_speech/testdata \
         $(TFLM_ROOT)/examples/hello_world \
         $(TFLM_ROOT)/signal/src \
         $(TFLM_ROOT)/signal/src/kiss_fft_wrappers \
         $(TFLM_ROOT)/third_party \
         $(TFLM_ROOT)/third_party/kissfft \
         $(TFLM_ROOT)/tensorflow/lite/ \
         $(TFLM_ROOT)/tensorflow/lite/c \
         $(TFLM_ROOT)/tensorflow/lite/core \
         $(TFLM_ROOT)/tensorflow/lite/kernels \
         $(TFLM_ROOT)/tensorflow/lite/schema \
         $(TFLM_ROOT)/third_party/flatbuffers/include \
         $(TFLM_ROOT)/third_party/gemmlowp \
         $(TFLM_ROOT)/third_party/gemmlowp/fixedpoint \
         $(TFLM_ROOT)/third_party/gemmlowp/internal \
         $(TFLM_ROOT)/third_party/ruy \
         $(TFLM_ROOT)/third_party/ruy/ruy

# Add these flags to enable floating point support in printf/scanf
PRINTF_FLOAT_LDFLAGS := -Wl,-u,vfprintf
PRINTF_FLOAT_LDLIBS  := -lprintf_flt

# --- Finalized Compiler Flags ---
# Use soft-float for libc compatibility, but tell the toolchain the FPU exists for assembly/context-switching.
COMMONFLAGS += -mfloat-abi=hard -mfpu=fpv5-d16 -DARMCM7 -D__FPU_PRESENT=1
COMMONFLAGS += -time -O3 -g -fno-exceptions

CXXFLAGS += -fno-strict-aliasing -fstack-usage -std=c++17 -fno-rtti

# Add TFLM-specific defines
TFLM_DEFS := \
    -DTF_LITE_STATIC_MEMORY \
    -DTF_LITE_MICRO_USE_CONSTANT_ARENA \
    -DTF_LITE_DISABLE_RTTI \
    -DTF_LITE_DISABLE_EXCEPTIONS \
    -DTF_LITE_MCU_NAME=CORTEX_M7 \
    -DTFLM_USE_CMSIS_NN \
    -DTFLM_USE_FFT \
    # -DTF_LITE_OPTIMIZE_DEBUG_LOGS

FIDO2_CONFIG_DEFS := \
    -DCONFIG_ARCH=arm -DCONFIG_ARM=y -DCONFIG_AW9523B=y -DCONFIG_BATT_CHG=y \
    -DCONFIG_BOARD=Daric-EVB -DCONFIG_BOARD_EVB=y -DCONFIG_CPU_CORTEX_M7=y \
    -DCONFIG_DISABLE_CACHE=y -DCONFIG_FINGERPRINT_IRQ_PIN=GPIO_PIN_1 \
    -DCONFIG_FINGERPRINT_IRQ_PORT=GPIOD -DCONFIG_FINGERPRINT_MFM017RD1_UART=y \
    -DCONFIG_FINGERPRINT_POWER_METHOD=1 -DCONFIG_FINGERPRINT_POWER_PIN=GPIO_PIN_9 \
    -DCONFIG_FINGERPRINT_POWER_PORT=GPIOB -DCONFIG_FINGERPRINT_UART_ID=1 \
    -DCONFIG_FINGERPRINT_UART_RX_PIN=GPIO_PIN_13 -DCONFIG_FINGERPRINT_UART_RX_PORT=GPIOD \
    -DCONFIG_FINGERPRINT_UART_TX_PIN=GPIO_PIN_14 -DCONFIG_FINGERPRINT_UART_TX_PORT=GPIOD \
    -DCONFIG_GSENSOR=y -DCONFIG_LCD_BUSY_IS_REQUIRED=false \
    -DCONFIG_LCD_BUSY_PIN=GPIO_PIN_13 -DCONFIG_LCD_BUSY_PORT=GPIOC -DCONFIG_LCD_COMMON=y \
    -DCONFIG_LCD_DC_IS_REQUIRED=true -DCONFIG_LCD_DC_PIN=GPIO_PIN_9 \
    -DCONFIG_LCD_DC_PORT=GPIOD -DCONFIG_LCD_RST_IS_REQUIRED=false \
    -DCONFIG_LCD_RST_PIN=GPIO_PIN_12 -DCONFIG_LCD_RST_PORT=GPIOD -DCONFIG_LCD_SSD1685_SPI=y \
    -DCONFIG_LCD_VCIEN_IS_REQUIRED=false -DCONFIG_LCD_VCIEN_PIN=GPIO_PIN_11 \
    -DCONFIG_LCD_VCIEN_PORT=GPIOD -DCONFIG_MAIN_TASK_STACK_SIZE=10240 \
    -DCONFIG_MBEDTLS_LIB=y -DCONFIG_NEWLIB_LIBC=y -DCONFIG_NEWLIB_LIBC_NANO=y \
    -DCONFIG_PMIC=y -DCONFIG_PMIC_AXP223=y -DCONFIG_PMIC_I2C_ID=DARIC_I2C1_ID \
    -DCONFIG_PMIC_I2C_SPEED=100 -DCONFIG_RGB_LED=y -DCONFIG_RGB_LED_CHIP=1 \
    -DCONFIG_RGB_LED_I2C_ID=DARIC_I2C0_ID -DCONFIG_RGB_LED_I2C_SPEED=100 \
    -DCONFIG_RTC_I2C_ID=DARIC_I2C0_ID -DCONFIG_RTC_I2C_SPEED=100 \
    -DCONFIG_RTC_INS5T8900=y -DCONFIG_SOC="Daric MPW A" -DCONFIG_SOC_SERIES=daric \
    -DCONFIG_SYS_CLOCK_HW_CYCLES_PER_SEC=250000000 -DCONFIG_SYS_CLOCK_TICKS_PER_SEC=1000 \
    -DCONFIG_TOUCH=y -DCONFIG_TOUCH_IIC_ID=DARIC_I2C0_ID -DCONFIG_TOUCH_IIC_SPEED=400 \
    -DCONFIG_TOUCH_INT_PIN=GPIO_PIN_2 -DCONFIG_TOUCH_INT_PORT=GPIOB \
    -DCONFIG_TOUCH_RST_PIN=GPIO_PIN_1 -DCONFIG_TOUCH_RST_PORT=GPIOB -DCONFIG_USB_CDC=y \
    -DCONFIG_USB_DETEC=y -DCONFIG_USB_HID=y -DTX_INCLUDE_USER_DEFINE_FILE

FIDO2_CONFIG_INCLUDES := \
    -I$(FIDO2_ROOT)/daric_sdk/drivers/Inc \
    -I$(FIDO2_ROOT)/daric_sdk/middleware/cmsis/CMSIS/Core/Include \
    -I$(FIDO2_ROOT)/daric_sdk/devices/Include \
    -I$(FIDO2_ROOT)/project/fido2/user/inc/config \
    -I$(FIDO2_ROOT)/daric_sdk/devices/Include \
    -I$(THREADX_PORT_PATH) \
    -I$(THREADX_PORT_INCLUDE_PATH) \
    -I$(FIDO2_ROOT)/daric_sdk/middleware/threadx/include \
    -I$(FIDO2_ROOT)/daric_sdk/middleware/threadx/kernel/common/inc

TFLM_INCLUDES := \
    -I$(TFLM_ROOT) \
    -I$(TFLM_ROOT)/tensorflow/lite \
    -I$(TFLM_ROOT)/tensorflow/lite/c \
    -I$(TFLM_ROOT)/tensorflow/lite/core \
    -I$(TFLM_ROOT)/tensorflow/lite/kernels \
    -I$(TFLM_ROOT)/tensorflow/lite/micro \
    -I$(TFLM_ROOT)/tensorflow/lite/micro/tflite_bridge \
    -I$(TFLM_ROOT)/tensorflow/lite/micro/kernels \
    -I$(TFLM_ROOT)/tensorflow/lite/micro/tools/make/downloads/cmsis \
    -I$(TFLM_ROOT)/tensorflow/lite/micro/tools/make/downloads/kissfft \
    -I$(TFLM_ROOT)/tensorflow/lite/micro/tools/make/downloads/flatbuffers/include \
    -I$(TFLM_ROOT)/tensorflow/lite/micro/compatibility \
    -I$(TFLM_ROOT)/examples/person_detection \
    -I$(TFLM_ROOT)/examples/micro_speech \
    -I$(TFLM_ROOT)/examples/micro_speech/testdata \
    -I$(TFLM_ROOT)/examples/hello_world \
    -I$(TFLM_ROOT)/examples/hello_world/models \
    -I$(TFLM_ROOT)/signal/micro/kernels \
    -I$(TFLM_ROOT)/signal/src \
    -I$(TFLM_ROOT)/signal/src/kiss_fft_wrappers \
    -I$(TFLM_ROOT)/third_party \
    -I$(TFLM_ROOT)/third_party/kissfft \
    -I$(TFLM_ROOT)/third_party/flatbuffers/include \
    -I$(TFLM_ROOT)/third_party/gemmlowp \
    -I$(TFLM_ROOT)/third_party/gemmlowp/fixedpoint \
    -I$(TFLM_ROOT)/third_party/gemmlowp/internal \
    -I$(TFLM_ROOT)/third_party/ruy \
    -I$(TFLM_ROOT)/third_party/ruy/ruy


# Combine all flags. CFLAGS are used for both .c and .S files.
CFLAGS   += $(INCLUDES) $(FIDO2_CONFIG_INCLUDES) $(TFLM_INCLUDES) $(FIDO2_CONFIG_DEFS) $(COMMONFLAGS) $(COMMONDEFS) $(CLFLAGS) $(TFLM_DEFS)
CXXFLAGS += $(INCLUDES) $(FIDO2_CONFIG_INCLUDES) $(TFLM_INCLUDES) $(FIDO2_CONFIG_DEFS) $(COMMONFLAGS) $(COMMONDEFS) $(CLFLAGS) $(TFLM_DEFS)

# --- Finalized Linker Flags ---
MALLOC_UNDEFINES := -Wl,-u,malloc,-u,calloc,-u,free,-u,realloc,-u,_malloc_r,-u,_calloc_r,-u,_free_r,-u,_realloc_r,-u,aligned_alloc
LDFLAGS += $(COMMONFLAGS) -Wl,--gc-sections,$(addprefix -T,$(LD_FILES)) --debug $(MALLOC_UNDEFINES) --specs=nano.specs -u _printf_float
LDLIBS   = -lm -lstdc++

######### Rules #########
.PHONY: all clean show cleanup

all: $(TARGET).bin
	@echo "Build complete. Cleaning up intermediate files..."
	@$(MAKE) cleanup

show:
	@echo "--- C Sources ---"
	@$(foreach file,$(C_SRCS),$(info $(file)))
	@echo "--- Asm Sources ---"
	@$(foreach file,$(S_SRCS),$(info $(file)))
	@echo "--- CXX Sources ---"
	@$(foreach file,$(CXX_SRCS),$(info $(file)))
	@echo "--- Objects ---"
	@$(foreach file,$(OBJS),$(info $(file)))

# Rule for assembling .S files (using CC)
%.o: %.S
	@echo "CC (for assembly) $<"
	@$(CC) -c $(CFLAGS) -o $@ $<

# Rule for compiling libc-hooks.c with special flags
libc-hooks.o: libc-hooks.c
	@echo "CC (custom rule) $<"
	@$(CC) -c -o $@ $(CFLAGS) -fno-builtin $<

# Rule for compiling all other .c files
%.o: %.c
	@echo "CC $<"
	@$(CC) -c -o $@ $(CFLAGS) $<

# Rule for compiling .cc (C++) files
%.o: %.cc
	@echo "CXX $<"
	@$(CXX) -c -o $@ $(CXXFLAGS) $<

# Rule for linking the final executable
$(TARGET).elf: $(OBJS)
	@echo "LD $@"
	@$(LD) -o $@ $^ $(LDFLAGS) $(LDLIBS)

# Rule for creating the .bin file
%.bin: %.elf
	@echo "OBJCOPY $@"
	@$(OBJCOPY) $(CPFLAGS) $^ $@
	@$(OBJDUMP) $(ODFLAGS) $^ > $(@:.bin=.dmp)

# Rule to clean intermediate files (.o and .su)
cleanup:
	@echo "Cleaning intermediate object and stack usage files..."
	@rm -f $(OBJS)
	@find . -name "*.su" -delete
	@find . -name "*.o" -delete

# Rule to clean all build artifacts
clean:
	@$(MAKE) cleanup
	@echo "Cleaning all build artifacts..."
	@rm -f $(TARGET).bin $(TARGET).elf $(TARGET).dmp $(TARGET).map

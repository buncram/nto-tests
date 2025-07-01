// dso_handle.cc
// Provides a dummy __dso_handle for bare-metal C++ builds.
// This symbol is required by the C++ runtime for global constructors/destructors,
// but is not needed for a statically linked, bare-metal application.

// Use extern "C" to prevent C++ name mangling for this symbol.
extern "C"
{
    // Define a weak symbol to allow a real implementation to override it if needed.
    // Initialize it to 0, as it's not used in this environment.
    void *__dso_handle __attribute__((weak)) = 0;
}
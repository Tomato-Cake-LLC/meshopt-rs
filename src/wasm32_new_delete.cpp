// wasm32 has no C++ runtime, so meshoptimizer's default allocator goes through Rust's.
#include <stddef.h>

extern "C" void* meshopt_rs_alloc(size_t size);
extern "C" void meshopt_rs_free(void* ptr);

void* operator new(size_t size)
{
	void* ptr = meshopt_rs_alloc(size);
	// There are no exceptions to throw std::bad_alloc with.
	if (!ptr)
		__builtin_trap();
	return ptr;
}

void operator delete(void* ptr) noexcept
{
	meshopt_rs_free(ptr);
}

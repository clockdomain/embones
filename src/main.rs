// - The `cfg_attr` attribute allows you to conditionally apply other attributes
// depending on the value of configuration flags passed to the compiler. In this
// case, we're saying that if the "std" feature is not enabled, then apply the
// `no_std` attribute.
// - The `feature` attribute is used to enable or disable compiler features. In
// this case, we're saying that if the "std" feature is enabled, then enable the
// "std" feature of the `core` library. This is necessary because the `core`
// library is `no_std` by default, so we need to enable its "std" feature to
// make it compatible with standard Rust programs.
// - The `no_std` attribute tells the Rust compiler to compile the code without including
// the standard library. It's used in the context of writing "bare-metal" or embedded
// systems programs.
// This allows you to write code that can work in both standard Rust
// programs and "no_std" environments, depending on how it's configured during
// compilation.
#![cfg_attr(not(feature = "std"), no_std)]

mod rv_cpu;
use rv_cpu::TrapRecord;

#[cfg(feature = "std")]
pub fn main() {}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {
        // your code goes here
    }
}

#[no_mangle]
#[inline(never)]
#[allow(clippy::empty_loop)]
extern "C" fn exception_handler(trap_record: &TrapRecord) {}

#[no_mangle]
#[inline(never)]
#[allow(clippy::empty_loop)]
extern "C" fn nmi_handler(trap_record: &TrapRecord) {}

// This function is called on panic.
// Writing embedded Rust code that is panic-free is highly desirable for several reasons:
// Predictable Behavior: In embedded systems, predictability is critical. Panicking (unwinding) in
// response to an error can be problematic because it can lead to unpredictable behavior. Panic
// unwinding involves running destructors for objects, which may not be well-defined in some embedded
// environments.
// Resource Management: Embedded systems often have limited resources such as memory and processing
// power. Unwinding in response to a panic can lead to resource leaks or additional resource consumption
// due to running destructors. By avoiding panic, you have more control over resource management.
// Safety and Reliability: Embedded systems are used in safety-critical applications, where reliability
// is paramount. Panicking can lead to system failures, which are unacceptable in many embedded contexts.
// Writing panic-free code helps improve the reliability of embedded systems.
// Real-Time Constraints: Some embedded systems have real-time constraints, meaning they must respond within a certain time frame. Unwinding due to a panic can introduce unpredictable delays, which are incompatible with real-time requirements.
// Error Handling Strategies: Embedded systems often have their own error handling strategies tailored to
// the specific requirements of the application. These strategies might include error codes, state
// machines, or simple fail-fast mechanisms. Panic can disrupt these strategies and make it harder to
// manage errors.
// Minimalism: Embedded systems typically strive for minimalism to reduce code size and complexity. By avoiding panic and using custom error handling, you can keep your codebase smaller and more focused on essential functionality.

// To write panic-free code in embedded Rust, you often use techniques like:

// Custom error handling using Result types.
// Avoiding dynamic memory allocation (e.g., using no_std).
// Minimizing or avoiding dependencies on the standard library.
// Creating a fail-safe design that handles errors gracefully without unwinding.
// While it may not be possible to completely eliminate the risk of panics in all embedded systems, making an effort to write panic-free code is a good practice to ensure the reliability and predictability of your embedded applications. It's important to carefully consider the requirements of your specific embedded system and design error-handling strategies that align with those requirements.
#[panic_handler]
#[inline(never)]
#[cfg(not(feature = "std"))]
#[allow(clippy::empty_loop)]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

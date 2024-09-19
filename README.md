### Intel hardware RNG.

Generate a random 64-bit number using the Intel RDRAND instruction with the `_rdrand64_step` intrinsic. 

The RDRAND instruction is part of Intel's Secure Key technology, which provides high-quality random numbers directly from the CPU.
[[Intel _rdrand64_step Docs]](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_rdrand64_step)

This function requires an Intel processor that supports the RDRAND instruction. Ensure your CPU architecture is either `x86` or `x86_64`. Otherwise, the code will fail to run.

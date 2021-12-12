## Notes

### Terminology

A process has its own data and execution space.

Threads within a process share the same data space, effectively *lightweight* processes.

An OS has a pool of threads to allocate, which it can share between the various OS processes.

### Conditions we are trying to avoid

1. Race conditions — where multiple threads of execution are attempting to access a shared resource in an unspecified and inconsistent manner.
1. Deadlocks — where two threads are waiting for each other to complete. Often caused by one, or both, holding a resource the other wants.
1. Intermittent bugs or behaviour — where the execution sequence results in differing behaviour. Ad-hoc results.

### Types of threads

+ Operating system allocated threads.
+ VM allocated threads, e.g., Java VM.

Rust uses a 1:1 mapping to the OS threads.  
There are libraries (crates) that allow M:N thread mappings.

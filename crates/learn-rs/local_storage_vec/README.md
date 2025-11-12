| | |
| --- | ---- |
| **Exercise** | https://teach-rs.trifectatech.org/exercises/2-foundations-of-rust/ |4-traits-and-generics/index.html
| **GitHub** | https://github.com/trifectatechfoundation/teach-rs |
| **Run tests** | [from project root] : `cargo test -p local_storage_vec` |

## At a High Level

Create a hybrid, array-like data structure that stores elements **on the stack** when the size is below a specified bound, and automatically transitions to **heap storage** when that bound is exceeded.

When the array grows beyond the stack capacity, its contents are transparently moved to the heap.

**This behavior should be completely opaque to the caller.**

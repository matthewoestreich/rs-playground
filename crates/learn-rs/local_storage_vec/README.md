**Exercise**  : https://teach-rs.trifectatech.org/exercises/2-foundations-of-rust/4-traits-and-generics/index.html </br>
**GitHub**     : https://github.com/trifectatechfoundation/teach-rs </br>
**Run tests**  : [from project root] : `cargo test -p local_storage_vec` </br>

## At a High Level

Create an array-like data structure that resides on the stack if the size of array is under some specified bound, or on the heap if the size of the array is above some specified bound.

If the array is on the stack and it's size surpasses the bound, it should be automatically moved to the heap.

# Rust Iterator Out-of-Bounds Access

This repository demonstrates a common error in Rust when working with iterators: accessing elements beyond the iterator's range. The code in `bug.rs` illustrates this issue, while `bugSolution.rs` provides a solution.

The problem arises because iterators only provide access to elements within their valid range.  Attempts to go beyond this lead to undefined behavior, typically resulting in a panic.  This is different from some languages where out-of-bounds access might return a default value or throw an exception.

The solution demonstrates safe iterator usage through various techniques including checking for iterator exhaustion.
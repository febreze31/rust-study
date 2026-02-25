# rust-study

Rust system programming practice for building high-performance and fault-tolerant trading infrastructure.

---

## 🎯 Goal

Transition from embedded automotive systems engineering to high-performance Rust systems development (crypto / trading infrastructure / low-latency services).

Focus areas:
- Memory safety without GC
- Ownership and borrow semantics
- Concurrency safety
- Performance-oriented data structures

---

## 📚 Learning Log

### Day 1-2
- Rust project initialization using Cargo
- Basic ownership and move semantics
- Immutable and mutable borrowing
- Function-based ownership transfer
- `clone()` vs move behavior

### Day 3-4
- Deep dive into borrow rules
- Mutable borrow exclusivity (`&mut`)
- Non-Lexical Lifetimes (NLL)
- Understanding lifetime ending at last usage

### Day 5
- `Vec<T>` memory layout (stack metadata + heap allocation)
- Capacity vs length and reallocation behavior
- `iter()`, `iter_mut()`, `into_iter()` differences
- Desugaring of `for` loop into `IntoIterator`
- Ownership transfer during iteration
- Copy vs non-Copy types in iteration
- Designing APIs using slices (`&[T]`) instead of `&Vec<T>`
- Deref coercion (`&Vec<T>` → `&[T]`, `&String` → `&str`)
- Abstraction principle: accept the most general borrow type

---

## 🧠 Key Concepts Practiced

- Move semantics and heap ownership
- Borrow checker constraints
- Lifetime reasoning
- Data race prevention at compile time
- Slice-based flexible API design
- Stack vs heap mental model
- Iterator ownership models
- Deref coercion and implicit reference conversions
- API abstraction and type generalization principles

---

## 🚀 Next Steps

- Struct and trait-based design patterns
- Zero-cost abstractions
- Error propagation patterns (`?` operator)
- Interior mutability (`RefCell`, `Mutex`)
- Multi-threading with `Arc`, `Mutex`, and channels
- Building a mini trading order book CLI
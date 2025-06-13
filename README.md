# scopeshare

**Scoped shared access for Rust — single-threaded and thread-safe variants with ergonomic APIs.**

`scopeshare` provides two small, safe, and ergonomic wrappers for shared state in Rust:

- 🧩 **`ScopeShare<T>`** — single-threaded, using `RefCell<T>`, for lightweight scoped interior mutability.
- 🔐 **`SyncShare<T>`** — thread-safe, using `RwLock<T>`, for safe shared access across threads.

Both types offer clean `.with()` and `.with_mut()` access patterns, avoiding the boilerplate of manual borrow guards.

---

## ✨ Features

- ✅ Scoped immutable and mutable access
- ✅ Optional Serde support (`SyncShare`)
- ✅ Trait implementations: `Clone`, `Debug`, `Serialize`, `Deserialize`
- ✅ Zero dependencies (except optional `serde`)
- ✅ Simple, minimal API

---

## 🔧 Examples

### Single-threaded: `ScopeShare<T>`

```rust
use scopeshare::ScopeShare;

let state = ScopeShare::new(vec![1, 2, 3]);
state.with_mut(|v| v.push(4));
state.with(|v| println!("{:?}", v));
```

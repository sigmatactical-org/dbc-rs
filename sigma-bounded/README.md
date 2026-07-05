# sigma-bounded

Bounded **`Vec`**, **`String`**, and **`BTreeMap`** (`alloc` vs **`heapless`**) shared by **`dbc-rs`** (via **`compat/`** re-exports).

Enable **`heapless`** when firmware has no global allocator; **`alloc`** otherwise. See **[`dbc-rs` `ARCHITECTURE.md`](../ARCHITECTURE.md#embedded-first)** for usage context.

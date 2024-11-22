# This is `search` grep lib

### Example of using `search`:

```rust
use search::{search, search_case_insensitive};

fn main() {
    let query = "RUst";
    let contents = "Rust is good language";
    let results = search_case_insensitive(query, contents);
    assert_eq!(results, vec!["Rust"]) // -> True
}
```
#### Version `search` -- 0.1.4

#### Note:
- This is part of the `grep` project
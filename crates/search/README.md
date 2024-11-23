# This is `search` grep lib

### Example of using `search`:

```rust
use search::{search, search_case_insensitive};

fn main() {
    let query = "RUst";
    let contents = "Rust is good language";
    let results = search_case_insensitive(query, contents, false, false);
    assert_eq!(results, vec!["Rust"]) // -> True
}
```
#### Version `search` -- 0.1.6

#### Note:
- This is part of the `grep` project
#### Update version -- 0.1.6
- Update functions: search, search_case_insensitive
- Update README.md
Allows you to pass a lock with it's resource.

```rust
use locked_resource::{LockedResource, WithLock};
use std::io::{stdin, BufRead, Stdin, StdinLock};

fn use_stdin<'l>(
    mut locked_stdin: LockedResource<Stdin, StdinLock<'l>>,
) -> LockedResource<Stdin, StdinLock<'l>> {
    let mut line = String::new();
    locked_stdin.read_line(&mut line).unwrap();
    locked_stdin
}

fn main() {
    let mut locked_stdin = stdin().with_lock();

    let mut line = String::new();
    locked_stdin.read_line(&mut line).unwrap();
    let locked_resource = use_stdin(locked_stdin);

    let mut line = String::new();
    locked_stdin.read_line(&mut line).unwrap();
}
```

### TODO
- Implement for Stdout
- Implement for Stderr
- etc..

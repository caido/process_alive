# Process Alive

This is a small cross platform crate to check if a process is alive.
Inspired by [sysinfo](https://github.com/GuillaumeGomez/sysinfo), but without the need to parse through all the processes to verify one.
Since there can be some errors related to permissions, the state can be `Unknown` and you can decide how you want to handle it.

```rust
use process_alive::{State, Pid};

pub fn main() {
  let pid = Pid::from(1234);
  let state = process_alive::state(pid);
  println("Process {} is {}", pid, state);
}
```

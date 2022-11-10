# Process Alive
[<img alt="github" src="https://img.shields.io/badge/github-caido/process_alive-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/caido/process_alive)
[<img alt="crates.io" src="https://img.shields.io/crates/v/process_alive?color=fc8d62&logo=rust&style=for-the-badge" height="20">](https://crates.io/crates/process_alive)

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

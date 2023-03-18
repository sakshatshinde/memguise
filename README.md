# memguise
### A simple tool to get info of all the memory in your system
---

To run the benchmark use for `get_multi_process_info_single_t` vs `get_multi_process_info`

```rust
pub fn get_multi_process_info(processes: Vec<&str>) -> Vec<Vec<ProcessInfo>> {
    let multi_process_info = processes
        .par_chunks(8) // * chunk size 8 seems to give the best performance, try changing this value

```
then run
```sh
cargo bench
```

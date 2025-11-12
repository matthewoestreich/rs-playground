| | |
| --- | --- |
| **Exercise** | https://teach-rs.trifectatech.org/exercises/2-foundations-of-rust/5-closures-and-dynamic-dispatch/index.html |
| **GitHub** | https://github.com/trifectatechfoundation/teach-rs/tree/main/exercises/2-foundations-of-rust/5-closures-and-dynamic-dispatch/1-config-reader |
| **Parse .json** | [from root of project] `cargo run -p config_reader -- ./crates/learn-rs/config_reader/config.json` |
| **Parse .yml** | [from root of project] `cargo run -p config_reader -- ./crates/learn-rs/config_reader/config.yml` |

## At a High Level

In this exercise, you'll work with dynamic dispatch to deserialize with `serde_json` or `serde_yaml`, depending on the file extension. The starter code is in `exercises/2-foundations-of-rust/5-closures-and-dynamic-dispatch/1-config-reader`. Fix the todo's in there.

To run the program, you'll need to pass the file to deserialize to the binary, not to Cargo. To do this, run

```shell
cargo run -- <FILE_PATH>
```

Deserializing both `config.json` and `config.yml` should result in the Config being printed correctly.
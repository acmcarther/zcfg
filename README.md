# zcfg

zcfg is a simple configuration library. It lets you define configurable values for your Rust library, without coupling you to a particular parsing format, or requiring plumbing in the binary.

Unlike other common configuration or flag libraries, you do not need to propagate internal details of your application into `main`, and you do not need to force users to configure internal details of your library if they choose not to.

It is inspired by [gflags](https://github.com/gflags/gflags) and similar global configuration systems.

## Features
- Define configuration values right where they're relevant
```rust
#![feature(used)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate zcfg;

define_pub_cfg!(net_protocol_timeout_ms, u32, 20000,
                "How long the server or client should wait before considering \
                this connection timed out.")
```
- Access the values while initializing your domain objects
```rust
struct NetProtocolClient {
  timeout_ms: u32,
}

impl Default for NetProtocolClient {
  fn default() -> NetProtocolClient {
    NetProtocolClient {
      timeout_ms: net_protocol_timeout_ms::CONFIG.get_value(),
    }
  }
}
```
- Let binaries decide how they'd like to bring in config values
```rust
extern crate zcfg;
extern crate zcfg_flag_parser;

use zcfg_flag_parser::FlagParser;

fn main() {
  // Parse flags for all linked crates via command line
  FlagParser::new().parse_from_args(env::args().skip(1)).unwrap();
}
```
- Make any of your types `configurable`
``` rust
use zcfg::ConfigParsable;

enum BuildStrategy {
  Local,
  Remote {
    addr: String,
  }
}
impl ConfigParsable for BuildStrategy { ... }

define_cfg!(use_build_strategy, BuildStrategy, BuildStrategy::Local,
            "Defines how the build planner performs compilation. Options are \
             [Local] or [Remote(\"some_address\")].")
```

## Best Practices

- Limit access of configs to object initialization under default confitions to preserve testability
- Consider `define_pub_cfg` if you'd like other modules to use the config.
- Ensure that default values are useful -- users may not perform config population at all.


## Roadmap

- A "Help" functionality that emits config names, default values, and config descriptions for public consumption
- Build on stable! `zcfg` currently depends on the experimental [used feature](https://github.com/rust-lang/rust/issues/40289) to prevent config initializers from being dropped by the compiler.

## How it works

Really dark stuff. `zcfg` uses a combination of linker flags and the `used` experimental feature to enqueue all flags defined in linked libraries in a global list, which `main` can populate without knowing about.

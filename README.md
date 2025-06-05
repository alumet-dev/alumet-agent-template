# Alumet agent template

Example of a custom Alumet agent.

## Key parts

This is a Rust project managed by [Cargo](https://doc.rust-lang.org/cargo/).
It creates a binary app, defined in `src/main.rs`.

- `Cargo.toml` declares a dependency on `alumet` (Alumet core), some Alumet plugins, and some third-party libraries.
- `main.rs` performs a complete setup of Alumet, as it:
  - loads the plugins
  - loads the configuration file
  - applies default configs if needed
  - creates the measurement pipeline
  - runs until you press Ctrl+C (or send SIGTERM in another way).

Command-line arguments are not provided.
To do that, we suggest you to have a look at [clap](https://docs.rs/clap/latest/clap/) or other libraries.

## How to add your plugin

1. Add your plugin as a dependency of the project, for instance with `cargo add`.
2. Import the plugin's struct in `main.rs`.
3. Add the plugin's type to the list of plugins (the `static_plugins!` macro) in `main.rs`.
4. Compile everything together with `cargo build`. The binary is generated in `target/`. Note that, by default, you are in `debug` (unoptimized) mode.

Pro-tip: you can compile and run quickly with `cargo run`. To pass arguments to the Alumet agent, use `--`. For instance, `cargo run -- plugins list`.

For more information on plugins, read the [Alumet Developer Book](https://alumet-dev.github.io/developer-book/).

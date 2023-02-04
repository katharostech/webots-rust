# Webots `libcontroller` Rust Bindings

This is a reference project that shows how to build controllers for the [Webots robot simulator](https://cyberbotics.com) using the Rust programming language.

## Running The Example

First you must install [Webots](https://cyberbotics.com) & [Rust](https://rust-lang.org).

You can run the example by:

1. Install Webots & Rust.
1. Clone this repository and switch folders to it.
1. Run `cargo build --example rust_epuck_controller`
1. Open up the `example` world in the `sample_project` folder in Webots, and run the simulation.

If you make changes, simply re-build the example and restart the simulation!

## Using in Your Own Project

> **Warning:** There is a `webots` crates.io package, but that is not this crate. Currently this
> crate is only published on GitHub, and we may change the name to prevent confusion with the
> crates.io crate.

**`Cargo.toml`:**

```toml
[dependencies]
webots = { git = "https://github.com/katharostech/webots-rust" }
```

Now you can compile your crate to create a Webots controller.

Once you have compiled the crate, you must copy or link your controller to a Webots project folder.

Webots projects always have a folder named `worlds` and you will need to have a `controllers` folder next to that. Inside of that create a folder named after your controller, such as `my_rust_controller`. Finally, copy your built controller into that folder, and make sure it has the same name as your folder.

Now, in the Webots program, you can select `my_rust_controller` as the controller for any robot!

## How it Works

[Bindgen](https://github.com/rust-lang/rust-bindgen) is used to generate bindings to the Webots C library at compile time. It will look in standard OS installation directories, or else use the `WEBOTS_PATH` environment variable to find your local Webots installation. For now, you must have it installed locally to compile this crate. We may change that soon.

You can find the raw, generated bindings in the `webots::sys` module, and the rest of the crate contains idiomatic Rust bindings. Though the idiomatic bindings are incomplete, they are easy to write, and we will be extending them as time permits. It's simple to see how they are made, and to add your own if necessary. Pull requests are welcome!

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.

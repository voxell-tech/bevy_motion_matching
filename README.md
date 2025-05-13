# Bevy Motion Matching

> [!Note]
> This project's initial phase research has been completed!
> If anyone is interested in continuing making this into a publishable product, feel free to reach out to us!
> We might revisit this project in the future, but for now, our focus has shifted elsewhere...

Motion matching enables characters to smoothly transition between animations by finding the best matching pose and trajectory from an extensive database, without the need to create state machines.
Gameplay logic can be embedded side by side with motion matching by querying animations with the desired attributes.

## Showcase

### Configuration

![config](./.github/assets/config.png)

### Play Mode

![play-mode](./.github/assets/play-mode.png)

## Development

### Prerequisites

- Rust
  - MSRV: v1.85.0
- Cargo
- Linux: [Bevy dependencies](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md)
- Optional, for the Visual Studio Code `start` task: `cargo-watch`
  - `cargo install cargo-watch` or `cargo binstall cargo-watch`

### Building

```bash
cargo build
```

### Running

```bash
cargo run --features bevy/dynamic_linking
```

## Reference

- [Learned Motion Matching](https://static-wordpress.ubisoft.com/montreal.ubisoft.com/wp-content/uploads/2020/07/09154101/Learned_Motion_Matching.pdf)
- [Machine Learning for Motion Synthesis and Character Control](https://www.youtube.com/watch?v=zuvmQxcCOM4)
- [LAFAN1 Animation Dataset](https://github.com/ubisoft/ubisoft-laforge-animation-dataset)
- [Unity Kinematica](https://docs.unity3d.com/Packages/com.unity.kinematica@0.8/manual/index.html)

## License

`bevy_motion_matching` is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

This means you can select the license you prefer!
This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to include both.

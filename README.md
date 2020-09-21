# Rust Tetris Clone

This was a learning project where I made tetris in Rust using the ggez
framework.

To play (requires Rust tooling install):

```bash
cargo run
```

There's also tests! The piece moving logic is particularly atrocious so I wrote
tests for it. I would not use that as a reference ;)

```bash
cargo test
```

Originally had audio as well, so you will see some references and dependencies
for dealing with that. I removed the audio files as I couldn't remember where
they're from and not sure how they are licensed.

## License

Code is [MIT](./LICENSE).

Most assets I believe I made myself and can be licensed similarly.

The included font is a [Google
Font](https://fonts.google.com/specimen/Press+Start+2P) so their license applies
there.

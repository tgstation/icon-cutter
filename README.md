# icon-cutter

icon-cutter is a library which creates .dmi files tailored to use with an 8-bit bitmasking system for smooth icons in in the [DM] game programming language.

A pre-compiled exe version can be found on the [releases page], but you can build your own from this repo at your preference.

## Dependencies

The [Rust] compiler:

1. Install the Rust compiler's dependencies (primarily the system linker):

   * Ubuntu: `sudo apt-get install gcc-multilib`
   * Windows (MSVC): [Build Tools for Visual Studio 2017][msvc]
   * Windows (GNU): No action required

1. Use [the Rust installer](https://rustup.rs/), or another Rust installation method,
   or run the following:

    ```sh
    curl https://sh.rustup.rs -sSfo rustup-init.sh
    chmod +x rustup-init.sh
    ./rustup-init.sh
    ```

## Compiling

The [Cargo] tool handles compilation, as well as automatically downloading and
compiling all Rust dependencies. To compile in release mode (recommended for speed):

```sh
cargo build --release
```

## Running

Simply drag your input .png file(s) onto the produced executable. You can also run it from the command line and provide the file path(s) as arguments.

In addition, there needs to be a `config.yaml` file present in the executable's directory that complies with the format specified in `examples/config-documentation.yaml`.

Example formats for the .png and config.yaml files can be found in `examples/`.
All you have to do is copy the template and config file from one of them into the same folder in where the program will be executed.

**Note:** The final products in dmi forms are fully functional but uncompressed. In order to reduce memory usage, open the DMI with DreamMaker and save it without changing anything. The file size should be noticeably reduced.

If you're still having problems, ask in the [Coderbus Discord]'s
`#tooling-questions` channel.

[releases page]: https://github.com/tgstation/icon-cutter/releases
[DM]: https://secure.byond.com/
[Rust]: https://rust-lang.org
[Cargo]: https://doc.rust-lang.org/cargo/
[rustup]: https://rustup.rs/
[msvc]: https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=15
[Coderbus Discord]: https://discord.gg/Vh8TJp9

## License

This project is licensed under the [AGPL license](https://en.wikipedia.org/wiki/Affero_General_Public_License).

See [LICENSE](./LICENSE) for more details.

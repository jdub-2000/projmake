# projmake 
 
A very basic utility to setup predicatble directory patterns
for starting new projects.  Also, an excuse to learn `Rust`!

This project is just getting started, and I hope to use it as an application
and framework to learn more `Rust` along the way.  In the meantime, I'll try to make
this useful with what I know, and update as I go along my journey. :)

## Installation and Setup 

1. Clone the `projmake` repository, and enter the repo directory

```
git clone git@github.com:jdub-2000/projmake.git &&
cd projmake
```

2. Compile `projmake` using `cargo` ; if you don't have `Rust` or `cargo` installed, see the instructions, [here](https://doc.rust-lang.org/cargo/getting-started/installation.html).

```
cargo build --release
```

if all goes well, a binary will placed in the following directory:

`/projmake/target/release/projmake`

3. Make a copy of that binary and place it in the `/usr/bin` directory:

```
sudo cp <full_path_to>/projmake/target/release/projmake /usr/bin/
```

4. Exit the terminal

```
exit
```

You should now be able to invoke the compiled binary no matter where you "are" in the filesystem.  See below for Usage instructions.

## Usage

To make a new project:

```
projmake foo
```

This yields a directory with the following structure:

```
foo
├── CODE
├── DAT
├── DOC
└── PROD
```

I know, not too imppressive, but its a start and a big timesaver for me :)
I hope to make some new feature available soon!

## Additional Resources

- For help with `Rust/Cargo` installation, consult their very nice guide, [ here](https://www.rust-lang.org/tools/install).

- `Rust` standard [documentation](https://doc.rust-lang.org/std/fs/index.html) for the `fs` Module

# mblocks
> This is a fork of [mblocks](https://gitlab.com/mhdy/mblocks), which adds some useful functions, such as allowing Interval to be triggered by a signal, and allowing to obtain a semaphore. This is useful for dwm to implement status bar clicks.


This program is a multi-threaded memory-safe status monitor written in Rust.
It updates the status only when there is a change.

![example](./screenshots/screenshot_1.png) 

## Installation

First, clone this repository.
```
git clone https://github.com/oh-wind/mblocs.git
```

Then, configure the status blocks by editing `src/config.rs`.

Next, build a release binary.
```
cargo build --release
```

If the build succeeds, the binary can be found in `target/release/mblocks`.

Finally, move the executable to one directory of your PATH directories, and add `mblocks &` to your `~/.xinitrc`.

## Configuration

The status monitor can be configured directly in the source `src/config.rs`.
Examples of `src/config.rs` can be found [here](https://gitlab.com/mhdy/mblocks/-/blob/master/src/config.rs) and [here](https://gitlab.com/mhdy/mde/-/blob/master/mblocks/src/config.rs).

Status blocks are defined in the `BLOCKS` vector.
Each block has a kind, executes a command, and has a prefix and a suffix for formatting. 

There are 3 kinds of blocks:

- Once: blocks labeled with this kind are executed once at the start of the program.
- Interval(N): blocks of this type are executed every N seconds. If the signal field is not None, it will fire immediately when this signal is received.
- Signal{ S: signal }: these blocks are executed when the signal S is sent to the mblocks process. To send a signal, you can use `kill -$((34 + S)) $(pidof mblocks)` where `S` is the argument given to Signal, and it should not exceed 15 (1 <= S <= 15). This means you can define at most 15 Signal blocks, which is large enough.

The `command` attribute corresponds to the command to be executed and can be one of the following:

- Shell(COMMAND): this executes the shell command given as arguments (e.g. ` Shell(&["date", "+%a, %b %d %Y"])`). Note the `&` preceding the array of arguments!
- Function(F): this executes the Rust function F given as argument. These latter can be placed in the directory `src/blocks/`. Check out the example functions.

The configuration file also defines the `SEPARATOR` between blocks, which could be any string. The generated status starts with the value of `PREFIX` and ends with the value of `SUFFIX`.

## Misc.

- If an empty string `""` is returned by a block execution, then the corresponding block will not be displayed.
- If the exit status of a `Shell` command is non-zero, then `failed` will be shown in the status (in the corresponding block).
- If a `Function` command returns `None`, then `failed` will be shown in the status (in the corresponding block).
- mblocks is memory-safe; it is validated with valgrind. The memory leaks shown by valgrind are false positives related to the signal-hook crate.

## License

MIT.

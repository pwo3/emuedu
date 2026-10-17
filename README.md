# EmuEdu

A collection of emulators written from scratch in Rust, compiled to WebAssembly and playable in the browser.

This is a **learning project**. The goal is to understand how classic machines work (CPU, memory, graphics, sound, timing) by building them step by step, and to document the journey along the way.

> Work in progress. Nothing is playable yet.

## Goals

- Write every core from scratch, without copying existing emulators.
- Share one common interface between all cores, so a single front end can drive any of them.
- Provide a debugger that makes the inner workings visible (registers, memory, tiles, disassembly).
- Validate each core with public test ROMs.

## Roadmap

- [ ] Project setup (Rust workspace, CLI harness, web front end)
- [ ] CHIP-8
- [ ] Game Boy / Game Boy Color
- [ ] NES
- [ ] Master System
- [ ] Game Boy Advance
- [ ] Maybe more (SNES, Mega Drive, PS1)

## Planned structure

```
cores/     # emulation cores, independent from the web
cli/       # native harness to run test ROMs headless
wasm/      # WebAssembly bindings
web/       # Vite + TypeScript front end (canvas, audio, input, debugger)
docs/      # notes and write-ups
```

## Building

Instructions will be added once there is something to build.

## Use of AI

An AI assistant (Claude, by Anthropic) is part of the learning process, but **it does not write the project's code**. The rules it follows are in [`CLAUDE.md`](CLAUDE.md).

- **Tutor**: it explains the theory (CPU architecture, instruction sets, graphics and audio hardware, timing) and points me to reference documentation.
- **Reviewer**: it reads my code and tells me where and why something is wrong, without giving the fix. When I am stuck, it gives progressive hints instead of solutions.
- **Not used for**: writing or modifying the emulation cores, the CLI, the WASM bindings or the front end. All of that is written by hand.
- **Documentation**: the initial README and agent instructions were drafted with AI assistance, then reviewed by me.

## Legal

This repository does **not** contain any commercial ROM, BIOS or copyrighted game data, and never will. The online demo (when available) only runs freely licensed homebrew and test ROMs. You load your own files locally.

## Contributing

This project exists for my own learning, so I write all the code myself and I am not accepting pull requests with code. Issues, questions, corrections and pointers to good documentation are very welcome.

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
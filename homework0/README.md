# Homework 0

Setting Up Cargo and Hello World!

This assignment is merely setting up Rust and the programming environment for the course. Cargo is the package manager and build tool for rust (no makefiles here!).

1. **Email me your GitHub username you plan to use for submitting the assignments!**
2. Set up a Linux System
3. Set up Rust

### Operating System

Low-level systems programs sometimes rely on OS implementation details to work. Specifically we will be focusing in this class on Linux x86-64. You may use any Linux distro you like for this (it shouldn't make a difference), but Ubuntu 18.04 LTS is probably easiest.
Developing (or at least testing) your implementation on Linux before submission is required for all assignments.

**Note:** If your implementation acts differently on the grading machine because of subtle differences or bugs across distros or OS versions, don't worry. We will figure it out together and no harm will come to your grade.

### Rust Versions

We will use the latest stable versions of Rust (and associated tools like Cargo, etc.): as of January 2021, that's 1.49.0.

## Setup

### Installing Linux

If you are working from Linux or already have a computer with a Linux environment running, great!
Otherwise, you will have to figure out how to get one running virtually.
There are many guides on the internet, but here are some pointers.
If you run into any trouble, don't worry! Let me know and I will try to help you get one running as quickly as possible.

- **On Windows,** I recommend using Windows Subsystem for Linux (WSL 2), which has matured in recent years and seems to be the de facto standard. You can follow [this guide](https://docs.microsoft.com/en-us/windows/wsl/install-win10). You will also need to decide how you want to edit your code -- from within or from outside WSL. Either way can work, though from within is more setup (you need X11 forwarding to run GUI programs -- I've used Xming in the past. Try [this guide](https://medium.com/swlh/get-wsl2-working-on-windows-10-2ee84ef8ed43)).

- **Using your ENIAC account** and logging in through SSH may be the easiest solution on MacOS. Follow the instructions [here](https://cets.seas.upenn.edu/answers/remote.html) to set up remote access.

- **VirtualBox** is recommended if the above don't work or apply. You can download it [here](https://www.virtualbox.org/) and you can get an Ubuntu 18.04 LTS image (`.iso`) by downloading from [here](https://www.ubuntu.com/download/desktop). Basically you create a virtual machine, decide how much memory and storage it needs, then when you launch it you select the `.iso` image to load Ubuntu.

### Installing Rust

This is the easy part! Follow the directions [on the Rust website](https://www.rust-lang.org/tools/install)
or [from the rust book](https://doc.rust-lang.org/book/ch01-01-installation.html).
That is, on your Linux platform, run

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This installs something called `rustup`, which is the Rust tool chain manager. However, you don't actually need to know about `rustup` -- almost always you will interact with Rust through `cargo` from the command line.
(Rustup will be relevant if you want to have several Rust versions at once, seemlessly switch between them, or run Nightly Rust features.)

To see if it's all working, run `cargo --version` -- you should get something like

```
cargo 1.49.0 (d00d64df9 2020-12-05)`
```

If you'd like, you're welcome to install Rust on your non-Linux system as well to play with there. However, you will need to test all your code on Linux in the end, so having it on your Linux installation is the most important.

## Assignment

Cargo is the rust package manager and build tool. For now, make sure you're able to create a new project: go into a directory and run

```bash
cargo new hello-world
cd hello-world
```

Then, inside the project, edit the `src/main.rs` file. You should see something like
```
fn main() {
    println!("Hello, world!");
}
```

and you can edit what it prints out.
Finally, try running your program:

```bash
cargo run
```

That's it!
For more detailed steps please see the Rust book's [introduction to cargo](https://doc.rust-lang.org/book/second-edition/ch01-03-hello-cargo.html). You may also be interested in the [Cargo Book](https://doc.rust-lang.org/cargo/) as a reference.

## Submission

This assignment will **NOT** be submitted. It is merely for setting up your system. Assignment 1 will be the first true assignment.

## Help

Do not hesitate to get help for this assignment. Setting up environments can be a major pain point of development!

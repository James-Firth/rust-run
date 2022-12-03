# Rust Run (Compile and Execute)

While reading [A Gentle Introduction to Rust](https://stevedonovan.github.io/rust-gentle-intro/1-basics.html) I wanted a small project.

Initially I created a bash alias
```bash
alias rrun=rustc $1.rs && ./$1
```

based on their comment in the Introduction.


However, I realized this could be a great way to learn a few things about Rust, and here I am!

## Usage

`rrun /path/to/file` will compile the program at the given path (defaults to your current working directory for output)
then executes the newly compiled program assuming there were no errors.

Future features may include defining outputs.

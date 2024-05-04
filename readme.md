# Tokenizerer
## About the Tokenizerer
This is a personal project I've been playing with. It is my first attempt at a (very) small language. I wrote this program with no knowledge as to how languages should be coded, so it ended up turning out more like a calculator with variable names than an actual language.

Even though it isn't exactly what I was going for, it's still cool and I had fun making it. In the future, maybe after a programming languages class, I hope to have another go at making a small language.

## Building
This is a Rust project, so you will need Rust and Cargo installed to build this program. If you just want to run the program, feel free to only download the binary.

In order to build:
```git clone https://github.com/kyle-gardner/Tokenizer ```
```cd ./Tokenizer/ ```
```cargo run```

or you can get the binary I've built for you :)
## How to use 
The Tokenizerer expects there to be a file called ```input``` in the same directory as the program.
The input file needs to be formatted a specific way for the Tokenizerer to read and execute it. Your code should:
- Be written on a single line.
- Have no spaces.
- Each statement/expression should end in a semi-colon.
- Only use the operators +-/*

Sample input is included in the binary download and in the repo, so you can model any new code you write off of that.

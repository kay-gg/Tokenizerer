# Tokenizerer
## About the Tokenizerer
This is a personal project I've been playing with. It is my first attempt at a (very) small language. I wrote this program with no knowledge as to how languages should be coded, so it ended up turning out more like a calculator with variable names than an actual language. 

Even though it isn't exactly what I was going for, it's still cool and I had fun making it. In the future, maybe after a Programming Languages class, I hope to have another go at making a small language.

## Building
This is a Rust project, so you will need Rust and Cargo installed to build this program. If you just want to run the program, feel free to only download the binary.

In order to build:

```git clone https://github.com/kyle-gardner/Tokenizer ```

```cd ./Tokenizer/ ```

```cargo run```

or you can [get the binary I've built for you :)](https://github.com/kyle-gardner/Tokenizerer/releases/tag/v.1.0.0)
## How to use 
The Tokenizerer expects there to be a file called ```input``` in the same directory as the program.
The input file needs to be formatted a specific way for the Tokenizerer to read and execute it. Your code should:
- Be written on a single line.
- Have no spaces.
- Each statement/expression should end in a semi-colon.
- Only use the operators +-/*

Sample input is included in the binary download and in the repo, so you can model any new code you write off of that.


## How it works
So, for a broad overview, the Tokenizerer does 3 main things,

1. Lexes code
2. Switching the order of the tokens into [Reverse Polish Notation, or postfix](https://en.wikipedia.org/wiki/Reverse_Polish_notation)
3. Evaluates the tokens

### Lexing Stage
So this part turns your input into Lexemes, or as I call them in my code, ```Token```s.

Here's an example, for the input ```x=2;``` The Lexing stage will turn this into an ```Vec``` of ```Token```, namely {Identifier, Operator, Literal} 
You may notice that the semi-colon doesnt get a ```Token```. 
While I was writing the next stage, I figured it would be easier if every statement was it's own ```Vec<Token>```, so I rewrote it to just discard any semi-colon tokens.

Another funny thing that I came upon while writing this stage was that Literals and Identifiers look the same, so how was I supposed to differentiate between them?
I decided that Identifiers should be alphabetical, and Literals could be numeric. 

This kind of cemented the Tokenizerer as more of a calculator than a full fledged language looking back on it, but oh well haha.

### RPN Stage
This stage takes a ```Vec<Token>``` and re-orders them so that they are in RPN or postfix notation. It does this via the [Shunting Yard algorithm](https://en.wikipedia.org/wiki/Shunting_yard_algorithm)

Sticking with our last example, ```x=2;```, we have the ```Vec<Token>```: {Identifier, Operator, Literal}

Following the Shunting Yard algorithm gives us a re-ordered ```Vec<Token>```: {Identifier, Literal, Operator}

This was definitely the most challenging part of development and it went through a lot of rewrites until I got it working. I still don't think it works COMPLETELY, but for most of the simpleish inputs I give it, it works.

### Evaluation stage
This stage will take a ```Vec<Vec<<Token>>``` and evaluate the entire program. 
The reason that it takes a Vec of Vecs unlike the last stage, is that it allows me to keep all of the variables in memory easier.
The algorithm I use to evaluate the RPN is found [here](https://www.geeksforgeeks.org/evaluation-of-postfix-expression/)

Sticking with the previous example, we have a ```Vec<Token>```: {Identifier, Literal, Operator}
As we get to the Operator token, we pop 2 tokens off the stack, and preform the operation.
In this case, the operation we want to do is assignment. We change the value of the variable ```x``` in our ```variables``` HashMap

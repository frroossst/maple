# MAZER 🍁🌰
## Mazer's Accessible Zero-hassle Equations Rendering

<img alt="mazer-logo" src="https://raw.githubusercontent.com/frroossst/mazer/master/mazer-logo.png" width=25% height=25%>

A minimal, simple, and opinionated math markup language that emits HTML, written in Rust.  

It's like LaTeX, but with a very stripped down feature set, mainly intended for note taking and simple math expressions.

Mainly, intended for quick and dirty math notes, and evaluating simple expressions.

# Motivations

- **Simple:** Natural Language alternative to math markup like LaTeX or MathML  

- **Opinionated:** This is what works for me, your preferences may be different  

- **Portable:** Plain text files that compile to HTML  

- **Verbose:** Typing speed is not a concern which enables expressive and flexible syntax  
  
- **Minimal:** Limited features (more to be added as I need them)  


# Installation

> cargo install mazer


# Usage

> mazer \<path to file>  

Outputs a HTML file of the same name  

> mazer \<path to file> --serve  

Starts a server to serve HTML file and watch for file changes  
 
> mazer \<path to file> --serve --open  

Starts a server and opens the page in the default web browser (open only works in conjunction with serve)  

> mazer \<path to file> --dry-run  

Runs the mazer compiler without creating the HTML file, useful for debugging!

# Mazel 

Meet out mascot, they are called Mazel! (Maple + Hazel)
They are very excited to meet you!

![mazel-mascot](https://raw.githubusercontent.com/frroossst/mazer/master/mazer-mascot.png)

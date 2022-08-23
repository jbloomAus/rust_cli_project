# Rust Command Line Project. 

I'm teaching myself Rust using this tutorial: https://rust-cli.github.io/book/tutorial/cli-args.html

To run the code currently, I:

1. Set the logger level to info.
2. look for the presence of `main` (my pattern)
3. look in the `./src/main.rs` file (seems a s good as any)

```bash
RUST_LOG=info cargo run main ./src/main.rs
```

## Learnings
- VSCode started showing inferred types called "inlay hints". I updated my vscode to toggle these off when I press `ctrl+ alt` as they seem useful but possibly annoying if I can't toggle them. I'm curious whether I can auto-add them to code and if other crustaceans don't like/want that then why not? 
- Anyhow library can be used to create a chain of errors  via the "context" trait. I like the sound of this because it sounds similar to the trace in python which I love. 
- The `indicatif` package sounds like it would be very useful for longer running programs (to make a progress bar). I'll consider using this. 
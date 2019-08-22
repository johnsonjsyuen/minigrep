## To test:

cargo run -- let src/main.rs && cat src/main.rs | cargo run -- let

This runs it in both file mode and pipe mode.

## Extensions

1. Read in a streaming way, instead of all input into memory.
1. Allow regex in the pattern, instead of just seeing if input contains it.
This is an implementation of the ["Build Your Own Redis" Challenge](https://codecrafters.io/challenges/redis).

[![progress-banner](https://backend.codecrafters.io/progress/redis/2ce965b7-29c6-49aa-96e2-0207e00f6247)](https://app.codecrafters.io/users/mikesievers?r=2qF)

# Sources

The following sources have been useful in the project

- Tokio tutorial https://tokio.rs/tokio/tutorial (which happens to be along implementing redis, so don't fall into the trap of just copying)
- Redis command refrence https://redis.io/docs/latest/commands/redis-8-8-commands/

# Next steps

- Implement SET parameters
  - Maybe use a parser

## RESP protocol parsing

- Redis protocol spec: https://redis.io/docs/latest/develop/reference/protocol-spec/
- Sample RESP parser from redis-oxide: https://dpbriggs.ca/blog/Implementing-A-Copyless-Redis-Protocol-in-Rust-With-Parsing-Combinators/
- Tokio docs https://tokio.rs/
  - especially tutorial https://tokio.rs/tokio/tutorial
  - Framing (with Redis example) https://tokio.rs/tokio/tutorial/framing

# From original project

In this challenge, you'll build a toy Redis clone that's capable of handling
basic commands like `PING`, `SET` and `GET`. Along the way we'll learn about
event loops, the Redis protocol and more.

**Note**: If you're viewing this repo on GitHub, head over to
[codecrafters.io](https://codecrafters.io) to try the challenge.

# Passing the first stage

The entry point for your Redis implementation is in `src/main.rs`. Study and
uncomment the relevant code, then run the command below to execute the tests on
our servers:

```sh
codecrafters submit
```

That's all!

# Stage 2 & beyond

Note: This section is for stages 2 and beyond.

1. Ensure you have `cargo (1.95)` installed locally
1. Run `./your_program.sh` to run your Redis server, which is implemented in
   `src/main.rs`. This command compiles your Rust project, so it might be slow
   the first time you run it. Subsequent runs will be fast.
1. Run `codecrafters submit` to submit your solution to CodeCrafters. Test
   output will be streamed to your terminal.

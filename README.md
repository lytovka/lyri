# Introduction

This repository holds an implementation of [Genius API](https://docs.genius.com/#/getting-started-h1) wrapper.

Ultimately my aim is to have a handy abstraction that I plan to utilize for future data processing projects. For the time being, this implementation should not be considered as something of an exceptional value: this is just me learning Rust fundamentals by building something with it.

## Getting started

1. Get a Genius API client for your application
Please advice API [Client management page](https://genius.com/api-clients) to obtain your keys. You will need to authorize with Genius before it allows to register new application.

2. Update `.env` file
This project uses [`dotenv`](https://crates.io/crates/dotenv) crate to load environment variables from a `.env` file. Upon generating client access token, simply bind its value to `GENIUS_ACCESS_TOKEN`.

3. Run the app

```bash
cargo build
cargo run
```
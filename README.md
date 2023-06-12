# Introduction

This repository holds an implementation of [Genius API](https://docs.genius.com/#/getting-started-h1) wrapper in Rust, along with a lyrics scraper for Genius pages.

The purpose of this implementation is to create a convenient abstraction for interacting with the Genius API and scraping lyrics from Genius pages. This can be useful for various data processing projects. Please note that the implementation of the lyrics scraper is for educational purposes and should be used responsibly and in compliance with Genius's terms of service.

## Getting started
To get started with this wrapper and lyrics scraper, follow the steps below:

1. Obtain a Genius API client for your application

Please visit the [API Client management](https://genius.com/api-clients) page to obtain your API keys. You will need to authorize with Genius before you can register a new application.

2. Update `.env` file

This project utilizes the [`dotenv`](https://crates.io/crates/dotenv) crate to load environment variables from a `.env` file. Once you have generated a client access token, bind its value to the `GENIUS_ACCESS_TOKEN` variable in the `.env` file.

3. Build and run the application

```bash
cargo run -- artist "oasis" # or any other artist
```

### Manual

```
Retrieves lyrics for a specific artist

Usage: lyri artist [OPTIONS] --name <NAME>

Options:
  -n, --name <NAME>                Name of the artist
  -l, --limit <LIMIT>              Number of songs to retrieve. If not specified, all songs will be retrieved
  -a, --antipattern <ANTIPATTERN>  Filter songs by anti-pattern for title
  -f, --features <FEATURES>        Include features in the results. If not specified, features will be excluded [possible values: true, false]
  -h, --help                       Print help
```

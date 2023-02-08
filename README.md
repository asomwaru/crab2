# Crab2

A crappy `bunnylol` (also known as [`bunny1`](https://github.com/ccheever/bunny1)) clone written in Rust for my personal needs.

This "search engine" has support for these different sites:

- [crates.io](https://crates.io)
- [google.com](https://google.com)
- [duckduckgo.com](https://duckduckgo.com)
- [github.com](https://github.com)
- [youtube.com](https://youtube.com)

## Todos

- [X] Setup Axum
- [X] Create a simple frontend to view
- [ ] Create redirects for all major services
- [ ] Make into browser extension
- [ ] Have it identified as a search engine for browsers

## Technologies

This "engine" uses Rust with Axum, Tokio, and Serde. It also has usage logging with tracing and tracing-subscriber.

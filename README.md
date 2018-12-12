# Rust Web API sample

A Web API sample for application with Rust.

Warning: In this sample used custom Router for Iron, but best way is use [iron/router](https://github.com/iron/router).

## Development

- rustc 1.31.0 (2018-12-04)
- diesel_cli 1.3.1

## TODO

- [ ] Authentication
    - [x] Iron middleware for sessions
    - [x] Routes wrapper
    - [ ] JWT Generation
    - [ ] SignUp controller
    - [ ] SignIn controller
    - [ ] SignOut controller
    - [ ] Facebook auth
- [ ] Database (PostgreSQL)
    - [ ] Define user model
- [ ] Errors chaining
- [ ] Tests
- [ ] GraphQL version

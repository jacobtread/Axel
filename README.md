# Axel 
A rust web framework intended to be the [Laravel](https://laravel.com/) for rust. Extends upon the extremely fast
[Actix (https://actix.rs/)](https://actix.rs/).

> This framework is very opinionated

## Goal
The goal of Axel is to be like Laravel but for Rust to provide a large set of pre-made tooling and functionality that 
can be easily implemented into your web application to speed up and simply the process of creating backends

## Sub Projects

- axel-cli *Command line tool for generating projects, running commands and such related to axel*
- axel-derive *Proc macros for code generation*
- example *Example template project using axel*

## Intended File Structure

- src *Sources*
  - routes *Routing handlers and functionality*
  - models *Models for requests, responses, errors, and Database*
  - stores *Central stores for storing app data*
  - utils *Utility classes and other functionality*
  - startup *Logic for starting the server and setting it up*

## Planned Features
- Hashing
- Encryption
- Authentication
- Authorization
- Caching
- File storage & Managing
- Localization
- Easy Mailing
- Notifications
- Queues
- Rate Limiting
- Task Scheduling
- Controllers
- Easy Middleware
- Sessions
- Logging
- Database (Seeding, Helpers, etc)
- Payment implementations (Paypal, Stripe)
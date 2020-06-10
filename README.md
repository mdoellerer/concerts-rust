# API concerts-rust
Simple API application created originally in Laravel, now being implemented also in Rust. 

## Requirements
    rustc 1.42.0 or higher
    diesel client instaled 
    a postgres db running

## Instalation
* On Postgres create a database called **concerts_test**

* Clone or download this repository

* Set a .env file with the connection data to your database. 

* Go inside of directory where the file `cargo.toml` is located and

* Then, in the terminal type:
    * to compile: **cargo build**
    * to install diesel client: **cargo install diesel_cli**
    * to create tables: **diesel migration run**
    * and finally, to start the api: **cargo run**
    * to stop it, press ctrl+c in the terminal

## Usage
You can run with a REST client (like a browser extension) or even by command line using curl.

### To do 
* Enforce the Bearer Authentication
* Better management of error situations (Bad Requests, Unauthorized)
* Improve this file with more meaningful explanations
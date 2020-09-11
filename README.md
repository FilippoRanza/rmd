# rmd

[![Build Status](https://travis-ci.com/FilippoRanza/rmd.svg?branch=master)](https://travis-ci.com/FilippoRanza/rmd) ![crates.io](https://img.shields.io/crates/v/rmd.svg) [![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT) [![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=round-square)](http://makeapullrequest.com)

An improved rm implementation able to remove duplicate files

## Description 
**rmd** is an **rm** reimplementation made in pure Rust. It
is able to:
- Do standard **rm** work
- Recursively remove duplicate files 
- Recursively remove files by size 
- Recursively remove files by last access date


## Installation

This tool can be easily installed from sources:
```bash
cargo install rmd
```

### Compile from source
It is also possible to directly clone the repository and compile **rmd** from there.
In this case it is recommended to run all tests before compile **rmd** for production.
A convenient way to do that is using make
```bash
make build
```
This will run all cargo tests (both unit and integration) and cli tests before compile rmd for 
production.


## Usage 

A quick guide can be found running: 
```bash
rmd --help
```

For a more complete reference please read the online documentation.
https://filipporanza.github.io/rmd/

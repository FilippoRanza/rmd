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

This tool can be easly installed from sources:
```bash
cargo install rmd
```


## Usage 

A quick guide can be found running: 
```bash
rmd --help
```

For a more complete reference please read the online documentation.
https://filipporanza.github.io/rmd/

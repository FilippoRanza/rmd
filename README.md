# rmd

[![Build Status](https://travis-ci.com/FilippoRanza/rmd.svg?branch=master)](https://travis-ci.com/FilippoRanza/rmd) ![crates.io](https://img.shields.io/crates/v/rmd.svg)

An improved rm implementation able to remove duplicate files

## Description 
**rmd** is an **rm** reimplementation made in pure Rust. It
is able to remove files and directories as usual but 
it also able to remove duplicate file in given directories recursively. 

Duplicate file are found by comparing **SHA256** hashes. 
When a file has an hash that has already been found it is removed immediately.

## Installation

This tool can be easly installed from sources:
```bash
cargo instal rmd
```


## Usage 
It works in an almost compatible way with the standard rm. To get a full help run:

```bash
rmd --help
```

But the most common scenarios includes:

- remove files, for example:
```bash
rmd FILE_A FILE_B 
```

- remove a directory, for example:
```bash
rmd -rf DIR_A
```

- remove duplicates files in the current directory, and all sub directoris:
```bash
rmd -d
```
or remove duplicates files in a specified directory:
```bash
rmd -d /PATH/TO/DIRECTORY
```
### Note
When working in *interactive* mode and a  remove file is a
directory **rmd** prompts only once for the root directory



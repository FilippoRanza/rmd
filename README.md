# rmd

[![Build Status](https://travis-ci.com/FilippoRanza/rmd.svg?branch=master)](https://travis-ci.com/FilippoRanza/rmd) ![crates.io](https://img.shields.io/crates/v/rmd.svg) [![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

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
cargo install rmd
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
### Additional Features
#### Remove Duplicates
- remove duplicates files in the current directory, and all sub directoris:
```bash
rmd -d
```
or remove duplicates files in a specified directory:
```bash
rmd -d /PATH/TO/DIRECTORY
```

#### Remove by Last Access
This functionality allows to remove file **older** or **newer** then a given 
*time-specification*.

Remove File *older* then *time-spec*
```bash 
rmd --older <time-spec> [directory...]
```

Remove File *newer* then *time-spec*
```bash 
rmd --newer <time-spec> [directory...]
```

**rmd** checks if the last access is **before** (so the file is **older**) or **after** 
(so the file is **newer**) then the time described by the *time-specification*.
*time-specification* describes a relative amount of time (in **seconds**) in the past 
from the moment when the program is run. 

*time-specification* format
```
[NT]+
```
Where:
- **N** is a number (1-9)
- **T** is a time descriptor
- **+** means one or more

Time Descriptor Table
| Short Format | Long  Format| Meaning | Value         |
|:-------------|-------------|---------|---------------|
| s            | second      | second  | 1 second      |
| m            | minute      | minute  | 60 seconds    |
| h            | hour        | hour    | 60 minutes    |
| d            | day         | day     | 24 hours      |
| w            | week        | week    | 7 days        |      
| m            | month       | month   | 30 days       |
| y            | year        | year    | 365 days      |
 
##### Examples
```bash
rmd --older 2y4m5d
```
will remove in the current directory, and recursivelly in all sub directories, file 
with a last access time equal or before *2 year, 4 month and 5 days* in the past from
the time when the program is run. 

```bash 
rmd --newer 4h30m
```
will remove in the current directory, and recursivelly in all sub directories, file 
with a last access time equal or after *4 hour and 30 minutes* in the past from
the time when the program is run. 


```bash 
rmd --older 1m15d /home/user/temp-store
```
will remove in **/home/user/temp-store** and recursivelly in all sub directories, file 
with a last access time equal or before *1 mounth and 15 days* in the past from
the time when the program is run. 

```bash 
rmd --newer 30s /home/user/wrong-downloads
```
will remove in **/home/user/wrong-downloads** and recursivelly in all sub directories, file 
with a last access time equal or after *30 seconds* in the past from
the time when the program is run. 



### Note
- When working in *interactive* mode and a  remove file is a
directory **rmd** prompts only once for the root directory
- *newer*, *older*, *recursive* are mutually exclusive.


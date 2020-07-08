# rmd

[![Build Status](https://travis-ci.com/FilippoRanza/rmd.svg?branch=master)](https://travis-ci.com/FilippoRanza/rmd) ![crates.io](https://img.shields.io/crates/v/rmd.svg) [![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT) [![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=round-square)](http://makeapullrequest.com)

An improved rm implementation able to remove duplicate files

## Description 
**rmd** is an **rm** reimplementation made in pure Rust. It
is able to remove files and directories as usual.

**rmd** is also able to:

- Recursively remove duplicate file. Duplicates are found 
by comparing *SHA256* file hash.

- Recursively remove file by size

- Recusively remove file by last access time.


## Installation

This tool can be easly installed from sources:
```bash
cargo install rmd
```

### Compile from source
It is also possible to directly clone di repository and compile **rmd** from there.
In this case it is recommended to run all tests before compile **rmd** for production.
A convenient way to do that is using make
```bash
make build
```
This will run all cargo tests (both unit and integration) and cli tests before compile rmd for 
production.



## Usage 
It works in an almost compatible way with the standard rm. To get a full help run:

```bash
rmd --help
```

### Standard Features (Standard Mode)

But the most common scenarios includes:

- remove files, for example:
```bash
rmd FILE_A FILE_B 
```

- remove a directory, for example:
```bash
rmd -rf DIR_A
```

- enable verbose mode:
```bash
rmd -v FILE_A
```
simple verbose just output the name of removed files and directory.
For a more verbose:
```bash
rmd -vv FILE_A
```
 **rmd** shows also statistics about removed files, specifing if the removed
 file is a directory or a regular file, in the latter case **rmd** also shows 
 the size of the removed file.


- enable *log* mode:
```bash
rmd -l FILE_A
```
output the same information as *-v* to syslog.
For more log:
```bash
rmd -ll FILE_A
```
output the same information as *-vv* to syslog.


### Additional Features (Automatic Mode)
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
[N+T]+
```
Where:
- **N** is a number (1-9)
- **T** is a time descriptor
- **+** means one or more

Time Descriptor Table

| Short Format | Long  Format| Meaning | Value         |
|--------------|-------------|---------|---------------|
| s            | second      | second  | 1 second      |
| m            | minute      | minute  | 60 seconds    |
| h            | hour        | hour    | 60 minutes    |
| d            | day         | day     | 24 hours      |
| w            | week        | week    | 7 days        |      
| M            | month       | month   | 30 days       |
| y            | year        | year    | 365 days      |
 
##### Examples
```bash
rmd --older 2y4M5d
```
will remove in the current directory, and recursivelly in all sub directories, file 
with a last access time equal or before *2 year, 4 month and 5 days* in the past from
the time when the program is run. 

```bash 
rmd --newer '4h+30m'
```
will remove in the current directory, and recursivelly in all sub directories, file 
with a last access time equal or after *4 hour and 30 minutes* in the past from
the time when the program is run. 


```bash 
rmd --older '1M 15d' /home/user/temp-store
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

#### Remove by Size

This functionality allows to remove file **smaller** or **larger** then a given 
*size-specification*.

Remove File *smaller* then *size-spec*
```bash 
rmd --smaller <size-spec> [directory...]
```

Remove File *larger* then *size-spec*
```bash 
rmd --larger <size-spec> [directory...]
```

**rmd** checks if the file size, in bytes. If **larger** mode is used **rmd** checks,
for each file in the specified directory, and recursivelly in all sub directories,
if the size is **larger or equal** to the size decribed in *size-spec* and if so **rmd**
remove the file. Of course if **smaller** mode is used **rmd** checks for file **smaller or equal** to the size in *size-spec*. 

*size-specification* format
```
[N+S]+
```
Where:
- **N** is a number (1-9)
- **T** is a size descriptor
- **+** means one or more

Deciamal Size Descriptor Table

| Short Format | Long  Format| Meaning | Value         |
|--------------|-------------|---------|---------------|
|         b    |             | byte    | 1 byte        |
|kb            |kilo         |kilobyte |1000 byte      |
|mb            |mega         |megabyte |1000 kilobyte  |
|gb            |giga         |gigabyte |1000 megabyte  |
|tb            |tera         |terabyte |1000 gigabyte  |
|pb            |peta         |petabyte |1000 terabyte  |

Binary Size Descriptor Table

| Short Format | Long  Format| Meaning | Value         |
|--------------|-------------|---------|---------------|
|         b    |             | byte    | 1 byte        |
|kib           |kibi         |kibibyte |1024 byte      |
|mib           |mebi         |mebibyte |1024 kibibyte  |
|gib           |gibi         |gibibyte |1024 mebibyte  |
|tib           |tebi         |tebibyte |1024 gibibyte  |
|pib           |pebi         |pebibyte |1024 tebibyte  |

Decimal and Binary size descriptor **can** be use together

##### Examples

```bash
rmd --smaller '2kb,56mib'
```
will remove in the current directory, and recursivelly in all sub directories, file 
with a size smaller or equal to *56 Mebibytes and 2 Kilobytes*.
```bash 
rmd --larger 4gb30mb
```
will remove in the current directory, and recursivelly in all sub directories, file 
with a size larger or equal to  *4 Gibabytes and 30 Megabytes*.


```bash 
rmd --larger '1 mebi 15 kibi' /home/user/temp-store
```
will remove in **/home/user/temp-store** and recursivelly in all sub directories, file 
with a size larger or equal to *1 Mebibytes and 15 Kibibytes*. 

```bash 
rmd --smaller 30kb /home/user/useless-files
```
will remove in **/home/user/useless-files** and recursivelly in all sub directories, file 
with a size smaller or equal to *30 Kilobytes*.

#### Skip Files
Sometimes you may need to skip some files or directories 
from been removed, for example you may want to preserve 
any *.bak* file or to completely ignore directories like *.git*. In these cases **rmd** provides two usefull options:
- *--ignore-extensions* allows to specify a list of extensions that will be ignored by **rmd** 

```bash
rmd --ignore-extensions bak --duplicates
```
will remove any duplicate file in the current directory and recursivelly in all sub directories ignoring any file with *.bak* extension. So if to equal file "file.rs.bak" and "copy-file.rs.bak" will be preserved. Also the original
"file.bak" (if it is unique) will be preserved because *.bak* file are completely ignored.

```bash
rmd --ignore-extensions bak pdf mp3 --larger 40kb project
```
will remove all file larger or equal to *40 Kilobytes* in the *project* directory, and recursivelly in all sub directories, but files with *.bak*,
*.pdf* and *.mp3* extensions. So, for example, *project/docs.pdf* a 4 Mb file
will not be removed.


- *--ignore-directories* allows to specify a list of directory names (just 
the last component in the path string) that will be ignored by **rmd**

```bash
rmd --clean --ignore-directories xmas_photos --older 1y documents
```
will remove any file older than one year in *documents* directory 
and recursivelly in all sub directories, ignoring any directory named *xmas_photo*. If *xmas_photo* is empty it will not be removed. **rmd**
simply will never open any directory named *xmas_photo* in the directory tree 
rooted in *documents*.

```bash
rmd --clean --ignore-directories important_files .git --duplicates /home/user
```
will remove any duplicate file in the user home, and recursivelly in all sub
directories, ignoring any directory named *.git* or *important_files*. 

It is allowed to use *--ignore-directories* and *--ignore-extensions* together. 

### Note
- When working in *interactive* mode and a  remove file is a
directory **rmd** prompts only once for the root directory
- *newer*, *older*, *duplicates*, *smaller*, *larger* are mutually exclusive.
- Specification String, in both time and size remove, can contain any number of
non alphanumeric characters between a number and a descriptor or between a descriptor and
a number, those characters are simply treated as separators.
The important thing are to **NOT** put sperators into numbers or into descriptors and to 
properly quote the specification string so it will be treated as a unique argument.

- *-c*/*--clean* flag deletes directories left empty after an automatic file removal,
(i.e. **rmd** run with *newer*, *older*, *duplicates*, *smaller*, *larger*).
This operation is done from the bottom of the directory tree, so directories that contains only directoies (recursively) without any file are considered empty.
So, although not technically empty
those directoies will be removed. Pay attention ;-). Clean flag does not 
take any effect where uses in standard mode.

- Verbose flag can be set in standard or in automatic mode.

- Log flag can be set in standard or in automatic mode.

- Log and Verbose mode can work together.

- Output generated by *log* and *verbose* is the same, it just changes where this 
output is sent. *log* send its output to *syslog*, *verbose* to stdout.

- *--ignore-extensions* and *--ignore-directories* can be used only with an
automatic remover


### Advices

It is very likely that you will end up using ```--ignore-extenions``` and/or ```--ignore-directories``` with the same arguments over and over. In this scenario a good idea could be add an alias to your shell configuration file like
```bash
alias rmdd='rmd --ignore-extensions bak --ignore-directories .git .hg'
```

or a shell function like
```bash
function rmd() {
    rmd --ignore-extensions bak --ignore-directories .git .hg -- "$@"
}
```

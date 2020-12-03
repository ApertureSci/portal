# Portal

> "Hello and, again, welcome to the Aperture Science computer-aided enrichment center." 

`Portal` is a simple & fast tool to help you create "portal" in a POSIX shell. It's designed to used along with command `cd` or something like that (e.g. `zoxide`).

A "portal" is closely similar to a "bookmark", allowing you save paths and re-enter them quickly from shell. However, it's still called a "portal" because I'm a BIG fan of Valve's Portal Series. 

## Install

```shell
cargo install portal
```

## Usage

```shell
$ /home/chell> portal /your-path
$ /your-path> portal -b
$ /home/chell> 
```

```shell
# assume that the working dir is `/home/chell`,
# and the pid of current shell is `1498`
# list all portals
$ /home/chell> portal list # p -l
*1498: /tmp

# use with `cd` (default behaviour)
$ /home/chell> portal /your-path # p /your-path
# every `portal cd` will memorized the current path
$ /your-path> portal --list # p -l
*1498: /home/chell
       /tmp

# portal to the latest path of the current shell
$ /your-path> portal --back # p -b
# or enter a portal interactively
$ /home/chell> portal --interactive # p -i
# clear portal history of this shell
$ /home/chell> portal --clear # p -c

```

## Configuration

```toml
[general]
default_exec = "/bin/cd"

[history]
keep_after_exit = false;

[back]
global = false;
```



# inmo

A cli tool help generate templates for problems from leetcode and codeforeces.

Have fun coding ;)

## Configuration

> `inmo` generates default config file at `~/.config/inmo/config.toml` when config is not found

```toml
leetcode = "$HOME/inmo/leetcode"
codeforces = "$HOME/inmo/codeforces"
cache = "$HOME/.inmo"
lang = cpp
```

- leetcode: the directory for Leetcode solutions
- codeforeces: the directory for codeforeces solutions
- cache: the directory for problem caches
- lang: the default Language to use for generating solution templates

## Supported languages

- rust
- cpp
- python3
- typescript
- javascript
- unknown

## Usage

```
inmo

USAGE:
    inmo <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    codeforces
    help          Print this message or the help of the given subcommand(s)
    leetcode
```

### Leetcode

```
inmo-leetcode

USAGE:
    inmo leetcode [OPTIONS]
    inmo leetcode <SUBCOMMAND>

OPTIONS:
    -h, --help           Print help information
    -i, --id <ID>        problem id
        --lang <LANG>    generate template of given language [possible values: rust, cpp, python3,
                         typescript, javascript, unknown]
        --open           open with $EDITOR
        --related        show related problems
        --solve          solve problem
        --tags           show tags of one problem, do not generate template

SUBCOMMANDS:
    help    Print this message or the help of the given subcommand(s)
    list    list local todos or solutions
    pick    pick one problem
    tag     list all problems of a given tag
    tags    list all types of tags
```

#### Workflow

1. Generate a solution file containing problem description for the given frontend id

```
inmo leetcode 1
```

2. Open the problem with $EDITOR

```
inmo leetcode 1 --open
```

3. Mark the problem solved in specified langauge as solved

```
inmo leetcode 1 --lang cpp --solve
```

## Why

1. It's inconvenient to manage the AC submissions in LeetCode
2. I may lose access to some questions that are free before, which means I would lose all my submitted code. And I do have lost access to these questions.

## RoadMap

### Leetcode

- [x] generate solution template for given languages
- [x] pick one problem within conditions
- [x] list local solutions
  - [x] tree view for langs
  - [x] list format refactor
- [x] list local todos
  - [x] tree view for langs
  - [x] list format refactor
- [x] list problems by topic
  - [x] state of each problem (solved/todos)
- [x] list all topic tags
- [x] solve to-do solution of given langauge
- [x] open with $EDITOR
- [x] list similar questions

### Configuration

- [x] support config for leetcode dir
- [x] support config for codeforeces dir
- [x] cache directory
- [x] default language

### Eye Candy

- [x] Download spinner

### Codeforces

- [ ] fetch problems
- [ ] fetch problems

## Acknowledgement

This project is inspired by:

- [zcong1993/leetcode-tool](https://github.com/zcong1993/leetcode-tool)

# Evaluate Markdown

This is a tool to evaluate code from Markdown files.

Why? Because I like writing Markdown files with code snippets (it's good with
Obsidian too).

With smaller code snippets and documentation in a markdown file, the file will
be easily consumed by the reader, yet it can be executed without copy-paste. If
the file has more than one code block with given language, `eval-md` will
combine them and evaluate as one script.

## Install

```bash
cargo install eval-md
```

## Supported languages

* Bash
* Python3
* Ruby
* Zsh

## Examples

The following examples will use `example/test.md`:

```bash
❯ eval-md zsh example/test.md -- --random-flag
nice in zsh
Arguments: --random-flag

❯ eval-md bash example/test.md
nice in bash
Arguments:

❯ eval-md python example/test.md -- --hype-level=awesomeness
awesome
Arguments: ['-', '--hype-level=awesomeness']

❯ eval-md ruby example/test.md -- --debug
it works :)
Arguments: ["--debug"]

❯ eval-md --debug python example/test.md -- --hype-level=awesomeness
 -- Target Language: python
 -- Source file: example/test.md
 -- Arguments: ["--hype-level=awesomeness"]
awesome
Arguments: ['-', '--hype-level=awesomeness']
```

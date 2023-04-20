# Evaluate Markdown

This is a tool to evaluate or export code from Markdown files.

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

## Export

With the `--export` flag, target language can be anything, it will not evaluate
the final code. It will print out the content to `stdout`. Ideal to generate
content from a Markdown file, for example configuration files. In the example we
have a JSON configuration file for a service, and we can add extra comments
about sections of the configuration file.

In the output, a header will be added, for example Python scripts get
`#!/usr/bin/env python3` on `--export`.

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

❯ eval-md json example/test.md --export
{
  "enable_registration": true,
  "debug": false,
  "hostname": "efertone.me",
  "port": 9999
}
```

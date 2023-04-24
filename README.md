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

* JavaScript (node, deno)
* Lua
* Python3
* Ruby
* Shell (bash, zsh)

## Export

With the `--export` flag, target language can be anything, it will not evaluate
the final code. It will print out the content to `stdout`. Ideal to generate
content from a Markdown file, for example configuration files. In the example we
have a JSON configuration file for a service, and we can add extra comments
about sections of the configuration file.

In the output, a header will be added, for example Python scripts get
`#!/usr/bin/env python3` on `--export`.

## Custom Tag

Custom tag and executor can be defined with `:`. The first part will be the
string tag to extract code blocks, and the second part will be the language that
will evaluate the extracted code.

* `py:python` => will parse `py` and run as `python`
* `js` => will parse `js` and run as `js` (`js` is an alias to javascript)
* `js:deno` => will parse `js` and run with `deno`

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

❯ eval-md lua example/test.md
Value:  15

❯ eval-md something:javascript example/test.md
Fancy
```

### Install and Bootstrap Flux

Obviously the whole documentation can live in a shell script as comment. In case
you want to add images and links to other pages, you can still "execute" the
documentation about how to install [Flux](https://fluxcd.io/flux/installation/).
Of course this example installs and bootstraps only one specific way.

```bash
❯ eval-md bash example/install-flux.md
 !! git repository is not defined.
 --help            This help message
 --repo <repo>     Git repository. (example: git@github.com/org/repo)
 --branch <branch> Git branch. [default: main]
 --path <path>     Path to the cluster. [default: ./clusters/management]

❯ eval-md bash example/install-flux.md -- --repo git@github.com/yitsushi/cluster-conf --branch production
...
```

### Deploy Helm Release with Flux

The `example/flux-helm-repo.md` example shows a simple scenario where we
describe how to deploy helm charts on a cluster. Usually that comes with a lot
of copy-paste, but we can be smart with `eval-md`.

This simple command will evaluate all `bash` in the markdown file. The only
`bash` snippet is the one that exports `yaml` blocks and pass it to `kubectl`.
At the end, we have a nice documentation (not this one, it's mostly just filler
text, did not really spend much time on it).

```bash
❯ eval-md bash example/flux-helm-repo.md
helmrepository.source.toolkit.fluxcd.io/podinfo created
helmrelease.helm.toolkit.fluxcd.io/podinfo created
```

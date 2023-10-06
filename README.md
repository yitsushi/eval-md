# Evaluate Markdown

This is a tool to evaluate or export code from Markdown files.

Why? Because I like writing Markdown files with code snippets (it's good with
Obsidian too).

With smaller code snippets and documentation in a markdown file, the file will
be easily consumed by the reader, yet it can be executed without copy-paste. If
the file has more than one code block with given language, `eval-md` will
combine them and evaluate as one script.

[![Quality Check](https://github.com/yitsushi/eval-md/actions/workflows/quality-check.yaml/badge.svg)](https://github.com/yitsushi/eval-md/actions/workflows/quality-check.yaml)
[![codecov](https://codecov.io/gh/yitsushi/eval-md/branch/main/graph/badge.svg?token=QTTVYOULF1)](https://codecov.io/gh/yitsushi/eval-md)

## Install

```bash
cargo install eval-md
```

## Supported languages

* JavaScript (node, deno)
* Lua
* PHP
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

### Group Filter

With a special marker, key-value pairs can be defined on code blocks:


    ```bash #group=a something=else
    echo "This is group A"
    ```
    
    ```bash #group=b
    echo "This is group B"
    ```
    
    ```bash
    echo "This one does not belon anywhere"
    ```

Without any extra arguments, all `bash` blocks will be evaluated:

```bash
❯ eval-md bash a.md
This is group A
This is group B
This one does not belon anywhere
```

If we want to evaluate only a specific group, we can set the filter with the
`--group` flag:

```bash
❯ eval-md bash a.md --group=a
This is group A

❯ eval-md bash a.md --group=b
This is group B
```

If `--group` flag is specified, but with empty value, it means evaluate
everything that has no group.

```bash
❯ ./target/release/eval-md bash a.md --group=
This one does not belon anywhere
```
### Pick mode

With the `--pick` flag, we'll be asked about each group to add it to the script
or discard it. This is good if we have more than one code blocks, but we don't
want all of them to be evaluated.

Group selection and Pick mode works together, so it is possible to filter on a
group and discard parts from it before evaluation.

Pick mode questions are printed to `stderr`, the program output can be
redirected to a file or pipe, it is useful with `--export`

```
❯ eval-md bash a.md --group=a --pick --export > myscript.sh

---
echo "This is group A"
---
 --> Do you want to add this block? (yes/no) yes

---
echo "This is group A again"
---
 --> Do you want to add this block? (yes/no) no

❯ cat myscript.sh
#!/usr/bin/env bash

echo "This is group A"
```

### Evaluate All as One Script

This is a weird case, but someone said they would use it and that would be cool.
If the name is set to `all` with a specified interpreter like `all:python`, then
all code blocks will be used in the document regardless the language marker on
them, and it will be evaluated with the specified interpreter (in this case
`python`).

See the `example/use-all.md` example:

```bash
❯ eval-md all:python example/use-all.md
Do something with:


<html>
  <head><title>Nice</title></head>
  <body>
    <div id="app">awesome</div>
  </body>
</html>

```

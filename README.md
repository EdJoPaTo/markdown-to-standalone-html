# Markdown to Standalone HTML

Create a standalone HTML file from Markdown with basic CSS, table of contents and source code highlighting.
The HTML is meant to be readable and self-contained, much like PDF intends to do.

## Features

- table of contents
- source code highlighting
- self-contained (inline assets like images and CSS via [monolith](https://github.com/Y2Z/monolith))
- simple HTML / CSS template built in
- bring your own template


# Install

This tool uses [monolith](https://github.com/Y2Z/monolith) for inlining assets like images or CSS.
If you want to have that feature make sure monolith is also installed and in the path.

## Prebuilt

### Arch Linux (AUR)

`paru -S markdown-to-standalone-html` or `yay -S markdown-to-standalone-html`

### Other

Check the [Releases](https://github.com/EdJoPaTo/markdown-to-standalone-html/releases).

## From Source

- Clone this repository
- `cargo install --path .`


# Usage

```bash
markdown-to-standalone-html input.md > output.html
cat input.md | markdown-to-standalone-html - > output.html
```

```plaintext
Markdown to Standalone HTML 0.4.0
EdJoPaTo <markdown-to-standalone-html@edjopato.de>
Create a standalone HTML file from Markdown with basic CSS, table of contents and source
code highlighting.

USAGE:
    markdown-to-standalone-html [FLAGS] [OPTIONS] <FILE>
    markdown-to-standalone-html [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help
            Prints help information

    -i, --no-inline
            Don't try to inline assets. Normally assets are inlined with monolith. When
            monolith is not in PATH a warning is shown. This warning is also suppressed
            with this flag.
    -V, --version
            Prints version information


OPTIONS:
    -t, --template <FILE>
            Template file to be used instead of the builtin one. Use the subcommand
            template to print the builtin template to stdout.

ARGS:
    <FILE>
            Markdown file to be parsed. Use - to read from stdin instead.


SUBCOMMANDS:
    help        Prints this message or the help of the given subcommand(s)
    raw         Only parse the markdown to html without any further modifications
    template    Print the included template to stdout.
```


# Alternatives

If you consider creating books from markdown (to HTML, PDF or EPUB) check out [crowbook](https://github.com/lise-henry/crowbook).

If you want to download articles and webpages as standalone HTML files check out [monolith](https://github.com/Y2Z/monolith).

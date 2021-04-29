# Markdown to Standalone HTML

This tool creates an HTML file from Markdown.
The HTML is meant to be standalone, much like PDF intends to do.
They also should include some basic Table of Contents in a simple HTML/CSS setup.

This is still work in progress…
Some things work, some things don't.

# Usage

```bash
markdown-to-standalone-html input.md > output.html
cat input.md | markdown-to-standalone-html - > output.html
```

```plaintext
Markdown to Standalone HTML 0.2.0
EdJoPaTo <markdown-to-standalone-html@edjopato.de>
Create a standalone HTML file from Markdown with basic CSS

USAGE:
    markdown-to-standalone-html [OPTIONS] <FILE>
    markdown-to-standalone-html [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help
            Prints help information

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

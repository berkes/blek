# Blek

Blek le Rat is a fast, simple template tool for the commandline

It accepts [Jinja2](https://jinja.palletsprojects.com/en/2.11.x/)
templates and replaces variables with those you pass along.

`invoice.j2`:
```
Date: {{ date }}
Number: {{ number }}
Dear {{ name | default(value="valued customer") }}, this the invoice for 1 pizza. Please pay ฿{{ amount }}.
```

```
$ blek invoice.j2 --var amount=10,000 --var "name=John Doe" --var number=$(openssl rand -base64 6)
Date: 2021-01-29
Number: KqSYp872
Dear John Doe, this the invoice for 1 pizza. Please pay ฿10,000.
```

## Features

* Simple, fast, small.
* Binary. Just download and run (or compile and distribute)
* Jinja2: A familiar (Liquid, Django, Twig), well documented templating language. We use
    [Tera](https://tera.netlify.app/docs/#macros).
* Any file can be a template.
* Comes with ever growing set of basic variables (currently 2... 😋)
* Unix philosophy; very easy to automate or use in automation.

## Quickstart

Requirements:

TODO: cargo install

TODO: release binaries.

Build from source (TODO how to set up Rust and cargo):

    git checkout https://git.webschuur.com/berkes/blek
    cd blek
    cargo build

### Run

After installing, from the place where it is installed:

    blek --help

This builds and runs the platform locally.

### Test

After downloading the source:

    cargo test

This builds and runs the tests locally. There are very few tests, since
there are very few features.

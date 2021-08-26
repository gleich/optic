<!-- DO NOT REMOVE - contributor_list:data:start:["gleich"]:end -->

# kiwi

⚠️ Kiwi is under active development and documentation is **not** done. ⚠️

[![lint](https://github.com/gleich/kiwi/actions/workflows/lint.yml/badge.svg)](https://github.com/gleich/kiwi/actions/workflows/lint.yml)
[![build](https://github.com/gleich/kiwi/actions/workflows/build.yml/badge.svg)](https://github.com/gleich/kiwi/actions/workflows/build.yml)
[![test](https://github.com/gleich/kiwi/actions/workflows/test.yml/badge.svg)](https://github.com/gleich/kiwi/actions/workflows/test.yml)

- [kiwi](#kiwi)
  - [👋 Introduction](#-introduction)
    - [❓ What is kiwi?](#-what-is-kiwi)
    - [❓ Why schoolwork as code?](#-why-schoolwork-as-code)
    - [🚀 Install](#-install)
  - [🤖 Automation](#-automation)
  - [🌲 Templates](#-templates)
  - [🙌 Contributing](#-contributing)
  - [👥 Contributors](#-contributors)

## 👋 Introduction

### ❓ What is kiwi?

Kiwi is a tool to help you write your schoolwork (e.g. worksheets, essays, lab reports, etc) as code using [LaTeX](https://en.wikipedia.org/wiki/LaTeX) or markdown and with a number of [automation features](#-automation) and a powerful [template system](#-templates). It is primarily controlled from the command line but editor extensions and a desktop app are planned for the future. Read more to see how kiwi could benefit you! Here are some core features:

- Creating documents.
- Writing markdown that is then transpiled to LaTeX.
- Organizing documents.
- Powerful templates

### ❓ Why schoolwork as code?

tl;dr coding is cool and school is boring (also LaTeX documents look hot)

### 🚀 Install

Using [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) you can install kiwi by running the following command in your terminal of choice:

```sh
cargo install kiwi-cli
```

Once that is done create a folder where you want to create your kiwi project and then run the following terminal command from inside that folder:

```sh
kiwi setup
```

## 🤖 Automation

If you try to write LaTeX manually at a large scale you will quickly realize that it is pretty repetitive. Having a tool to automatically create, organize, and build documents for you saves a ton of time and removes a ton of pain points. Kiwi can do all of this for you with only two commands:

| **Command**  | **Description**                                           |
| ------------ | --------------------------------------------------------- |
| `kiwi new`   | Create a new document (and put it in an organized folder) |
| `kiwi build` | Build the last updated document                           |

## 🌲 Templates

A core part of kiwi is the template system. It consists of two parts:

1. branches (LaTeX or Markdown): where you actually write your documents.
2. roots (LaTeX only): larger templates that the branch code gets put in.

The reason for this two part system so that branches that are markdown can still be styled with LaTeX code. To read more about how templates work and how to set them up please see the [templates documentation](docs/templates.md).

## 🙌 Contributing

We would love to have you contribute! Please read the [contributing guide](CONTRIBUTING.md) before submitting a pull request. Thank you in advance!

<!-- prettier-ignore-start -->
<!-- DO NOT REMOVE - contributor_list:start -->
## 👥 Contributors


- **[@gleich](https://github.com/gleich)**

<!-- DO NOT REMOVE - contributor_list:end -->
<!-- prettier-ignore-end -->

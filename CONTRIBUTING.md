# Contributing to heim

The following is a set of guidelines for contributing to heim.

## Table of contents

 1. [Code of conduct](#code-of-conduct)
 2. [I just have a question!](#i-just-have-a-question)
 3. [What should I know before I get started?](#what-should-i-know-before-i-get-started)
 4. [How can I contribute?](#how-can-i-contribute)
 5. [Styleguides](#styleguides)

## Code of conduct

This project and everyone participating in it is covered by [Contributor Covenant Code of Conduct](https://github.com/heim-rs/heim/blob/master/CODE_OF_CONDUCT.md).
By participating, you are expected to uphold this code.

## I just have a question!

First of all: do not file an issue to ask a question about how to use heim!

It is not a project goal right now to provide any kind of information
about how to work with `Futures`, use heim with `actix`, or any other such things.\
Such issues will be closed immediately.

Use any [Rust community channel available](https://www.rust-lang.org/community#contribute-talk) to get help
or check out the [Gitter](https://gitter.im/heim-rs/heim) chat.

## What should I know before I get started?

### Crates system

heim is split into multiple crates, each one responsible for a specific system component
(ex. `heim-cpu` provides information about CPUs, `heim-memory` about system memory, you got it).

The `heim` crate acts as a facade to all the `heim-*` crates and does nothing more
than re-export all of them.

### Design decisions

Since heim is a very young project, the public API should not be considered stable.

There is no separate repository for RFC proposals (yet?), so if you have an idea,
just [create an issue](https://github.com/heim-rs/heim/issues/new) in the main repository.

## How can I contribute?

### Reporting bugs

Before creating bug reports, please check [this list](https://github.com/heim-rs/heim/issues)
as you might find out that you don't need to create one.
When you are creating a bug report, please include as many details as possible.

> **Note**: Note: If you find a closed issue that seems like it is the same thing that you're experiencing,
> open a new issue and include a link to the original issue in the body of your new one.

### How Do I Submit A Bug Report?

Bugs are tracked as [GitHub issues](https://github.com/heim-rs/heim/issues).

Explain the problem and include additional details to help maintainers reproduce the problem:

 1. Use a clear and descriptive title for the issue to identify the problem.

 2. Describe the exact steps which reproduce the problem in as many details as possible.

 3. Provide specific examples to demonstrate the steps. Include links to files or GitHub projects,
    or copy/pasteable snippets, which you use in those examples.
    If you're providing snippets in the issue, use [Markdown code blocks](https://help.github.com/articles/markdown-basics/#multiple-lines).

 4. Describe the behavior you observed after following the steps and point out what exactly is the problem with that behavior.

 5. Explain which behavior you expected to see instead and why.

Include details about your configuration and environment:

 1. Which version of `heim` crate are you using?
 2. What's the name and version of the OS you're using?

### Code Contribution

You can start by looking through these [`good-first-issue` issues](https://github.com/heim-rs/heim/issues?q=is%3Aissue+is%3Aopen+label%3AC-good-first-issue):

Code you are contributing should pass the following checks:

 1. Should change only one specific thing
 2. Not raising any compiler errors or warnings
 3. Conforms to `rustfmt` rules (see [`.rustfml.toml`](https://github.com/heim-rs/heim/blob/master/.rustfmt.toml) file)
 4. Not raising any [`clippy`](https://github.com/rust-lang/rust-clippy) warnings
 5. Should pass the [CI tests](https://dev.azure.com/heim-rs/heim/_build/latest)

#### Pull Requests

 1. Create a GitHub Pull Request with patch
 2. Ensure the Pull Request description clearly describes the problem and solution.
    Include the relevant issue number if applicable.
 3. Ensure that all checks are passing

#### Did you fix whitespace, format code, or make a purely cosmetic patch?

Changes that are cosmetic in nature and do not add anything substantial to the stability, functionality,
or testability will generally not be accepted.

## Styleguides

### Git Commit Messages

 * Provide precise information about change made
 * [Refer the issues and pull requests](https://help.github.com/en/articles/closing-issues-using-keywords)

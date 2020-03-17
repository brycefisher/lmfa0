![lmfa0](logo.png)

> Skip portions of your CI run for source code that hasn't changed

## Use Cases

 - Only apply terraform when the .tf files change
 - Only build/deploy documentation when the docs/ directory changes
 - Only test a portion of mono-repo when that microservice changes

**Warning: 0% test coverage today. Because 0 tests today. It works on my machine**

## Installation

Currently requires building using `cargo`:

```
cargo install --git https://gitlab.com/bff/lmfa0.git
```

**Optional**: Use `strip` to further shrink the binary size a up to 100kb (roughly 850kb without this as of 3/17/2020).

```
$ strip "$(which lmfa0)"
```

## Setup

 1. Map path **roots** --> shell **commands**
 2. Cache & restore the .lmfa0/ directory per branch in CI
 3. Invoke `lmfa0 [rule]` throughout CI config anywhere you want to skip runs

### Requirements

 * Git
 * A system that allows you cache/restore specific paths based on the branch being built

The following CI systems should work with `lmfa0` using their "caching" features, and others likely do as well: CircleCI, TravisCI, GitlabCI.

Additionally, you could probably use a serverless database like AWS DynamoDB or Azure CosmoDB to replicate CI caching in a CI system that lacks configurable caching.

## How do I configure `lmfa0`?

Create a `lmfa0.toml` file in the root of your project, adjacent to your .git/ directory. It should have this structure:

```toml
[rules.doc]
root = "docs/"
command = "cargo doc"

[rules.deploy]
root = "terraform/"
command = "terraform apply --force"

[rules.microserviceA]
root = "service-a/"
command = "py.test service-a/"

[rules.microserviceB]
root = "service-b/"
command = "py.test service-b/"
```

Each rule goes inside a heading `[rules.<your rule name>]` and has the keys `root` and `command`.

## Running a Rule

This executes a rule named "doc" (see example above):

```
$ lmfa0 doc
```

## Contributions Welcome

If you're interested in this project, feel free to open an issue and propose your idea.

### Particular Contributions of Interest

 * Sexy documentation site (a la gitlab pages)
 * Cucmber tests
 * Unit tests
 * Code coverage
 * Config options for:
   * Making `root` into a glob to only consider file suffixes
   * Configuring repo location
   * A way to build a DAG by composing rules together (similar to Make)
 * Posting to Reddit
 * Blogging about this project!
 * Contriving a meaning for this an acronym?

## License

Distributed under an Apache 2.0 license. See LICENSE.txt for full license.

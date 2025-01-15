# hq

hq is a CLI to help you query HTML documents like `jq`.

```
$ hq --help

hq is a CLI to help you query HTML documents like `jq`

Usage: hq [OPTIONS] <QUERY> [PATH]

Arguments:
  <QUERY>  The query to run
  [PATH]   Path to the HTML file to query (defaults to stdin)

Options:
  -a, --all-matches  Select all matches (default)
  -f, --first-match  Select the first match
  -o, --outer-html   Return outer HTML (default)
  -i, --inner-html   Return inner HTML
  -t, --text         Return text content
  -d, --debug        Debug mode
      --indent       Indent output
  -h, --help         Print help
  -V, --version      Print version
```

## Status

🚧 This is a work in progress. 🚧


## Installation

```
cargo install --git https://github.com/a-poor/hq
```

## Examples

Find the `span` elements in [test.html](./test.html).

```
$ hq span test.html

<span class="baz">paragraph</span>
<span>paragraph</span>
```

Use with `curl` to query the contents of a webpage.

```
$ curl https://austinpoor.com/ | hq title

<title>AustinPoor.com</title>
```

How many `h1`, `h2`, and `h3` elements are there on the [Wikipedia page for HTML](https://en.wikipedia.org/wiki/HTML)?

```
$ curl https://en.wikipedia.org/wiki/HTML | hq 'h1, h2, h3' | wc -l

29
```


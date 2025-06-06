# Test snippets

This directory contains two sets of test snippets which can be run in Python.
The `snippets/` directory contains functional tests, and the `benchmarks/`
directory contains snippets for use in benchmarking AzLang's performance.

## Setup

Our testing depends on [pytest](https://pytest.org), which you can install
using pip.

## Running

Simply run `pytest -v` in this directory, and the tests should run (and hopefully
pass). If it hangs for a long time, that's because it's building AzLang in
release mode, which should take less time than it would to run every test
snippet with AzLang compiled in debug mode.

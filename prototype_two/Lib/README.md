# Standard Library for AzLang

This directory contains all of the Python files that make up the standard
library for AzLang.

Most of these files are copied over from the CPython repository (the 3.7
branch), with slight modifications to allow them to work under AzLang. The
current goal is to complete the standard library with as few modifications as
possible. Current modifications are just temporary workarounds for bugs/missing
feature within the AzLang implementation.

The first big module we are targeting is `unittest`, so we can leverage the
CPython test suite.

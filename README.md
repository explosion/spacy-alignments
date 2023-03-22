<a href="https://explosion.ai"><img src="https://explosion.ai/assets/img/logo.svg" width="125" height="125" align="right" /></a>

# spacy-alignments: Align tokenizations for spaCy + transformers

A spaCy package for Yohei Tamura's Rust
[tokenizations](https://github.com/tamuhey/tokenizations/) library with Python
bindings.

## Installation

```
pip install -U pip setuptools wheel
pip install spacy-alignments
```

If no binary wheel is available for your platform, you will need to [install
Rust](https://www.rust-lang.org/tools/install) in order to build
`spacy-alignments` from source.

## spacy-alignments vs. pytokenizations

The `spacy_alignments` module is a drop-in replacement for `tokenizations`:

```python
import spacy_alignments as tokenizations
a2b, b2a = tokenizations.get_alignments(["å", "BC"], ["abc"])
assert a2b == [[0], [0]]
assert b2a == [[0, 1]]
```

The only difference between this package and the original
[`pytokenizations`](https://pypi.org/project/pytokenizations/) is that it
switches the build system to `setuptools-rust` to make it easier for us at
Explosion to build source and binary packages for a wider range of platforms.

## Bug reports and other issues

Please use [spaCy's issue tracker](https://github.com/explosion/spaCy/issues) to report a bug, or open a new thread on the
[discussion board](https://github.com/explosion/spaCy/discussions)
for any other issue.

#!/usr/bin/env python
import sys

from setuptools import setup
from setuptools_rust import RustExtension


setup(
    name="spacy-alignments",
    version="0.8.3",
    packages=["spacy_alignments", "spacy_alignments.tests"],
    rust_extensions=[RustExtension("spacy_alignments.tokenizations")],
)

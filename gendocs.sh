#!/bin/sh
cargo doc
cp -rv target/doc/* docs/

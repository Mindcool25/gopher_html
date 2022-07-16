#!/bin/sh

cargo build
cp target/debug/gopher_html cgi-bin/
python3 -m http.server --cgi

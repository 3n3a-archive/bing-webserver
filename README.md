# Bing Webserver

A webserver that _bings_ the 🛎️.

> This is in no way a fully implemented HTTP-Webserver `;)`

## Release Status

| Workflow | Status |
| -------- | ------ |
| Build and release | [![.github/workflows/release.yml](https://github.com/3n3a/bing-webserver/actions/workflows/release.yml/badge.svg)](https://github.com/3n3a/bing-webserver/actions/workflows/release.yml) |

## Features

* Answer `POST`, `GET` and `HEAD` http-requests
* Serve Static Files, if enabled
    * when a Markdown file is requested, it converts it to HTML
* concurrent request handling, e.g can handle multiple clients at same time

## Usage

1. Get the binary from github releases
2. Put it in `/usr/local/bin/`
3. You now have the bing webserver!!
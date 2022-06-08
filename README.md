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

### Example

Can be found in [examples folder](./examples).

Just copy the `bing-webserver` binary there and execute it from within that folder. In the config.env file there you can change any variable you'd like.

### Docker

```sh
docker run -p 7878:7878 3n3a/bing-webserver:latest
```

### Environment Variables

| Key | Usage |
| --- | --- |
| `BWS_IP` | `BWS_IP="127.0.0.1"` | 
| `BWS_PORT` | `BWS_PORT="7878"` | 
| `BWS_RING_BELL_ON_REQUEST` | `BWS_RING_BELL_ON_REQUEST="false"` | 
| `BWS_SERVE_STATIC_FILES` | `BWS_SERVE_STATIC_FILES="false"` | 
| `BWS_STATIC_FILE_PATH` | `BWS_STATIC_FILE_PATH="."` | 
| `BWS_ALLOWED_STATIC_FILE_EXTENSIONS` | `BWS_ALLOWED_STATIC_FILE_EXTENSIONS="html md css js jpg jpeg webp png avif"` |
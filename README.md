# Timelapse

A raspberry pi based timelapse camera software. Takes images periodically on the
pi and presents them in a web UI for viewing and downloading.

## Getting started

Requires rust and node/npm.

### Backend

Requires raspistill in your path to be able to capture images (you will see a
harmless error in the logs without it). But it can be run locally without this
(just won't generate new images). Requires a `data` directory which images will
be served from and you can place any images in there for testing.

```bash
UPSTREAM=https://49a5b645a954379cec90fa82218cc69f.balena-devices.com/images
mkdir data
wget "$UPSTREAM/2020-09-30T22:08:35.471236400+00:00.jpg" -P data/
wget "$UPSTREAM/2020-09-30T22:09:37.202233452+00:00.jpg" -P data/
wget "$UPSTREAM/2020-09-30T22:10:38.934747486+00:00.jpg" -P data/
wget "$UPSTREAM/2020-09-30T22:11:40.668463506+00:00.jpg" -P data/

cargo run
```

### Frontend

> TODO

## Deployment

The repo is designed to be deployed with [Balena]. To get started follow their
[getting started guide] switching out the example repo for this one.

The frontend is a static site that can be deployed to any static hosting
provider such as [Netlify].

[Balena]: https://www.balena.io/
[getting started guide]: https://www.balena.io/docs/learn/getting-started/raspberrypi3/rust/
[Netlify]: https://www.netlify.com/

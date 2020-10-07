# Timelapse

A raspberry pi based timelapse camera software. Takes images periodically on the
pi and presents them in a web UI for viewing and downloading.

## Getting started

Requires rust (for the backend) and node/npm (for the frontend).

### Backend

There is a mock raspistill script in `mock-bin` which downloads a placeholder
image. Add this to your path to stop the timelapse binary from erroring.

```bash
PATH=$PWD/mock-bin:$PATH cargo run
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

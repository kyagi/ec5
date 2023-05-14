# Setup multi-platform image build environment on Mac OS Ventura
Install tonistiigi/binfmt, then create a builder instance named `mybuilder`.
```
➜ docker run --privileged --rm tonistiigi/binfmt --install all
```
```
➜ docker buildx create --name mybuilder --driver docker-container --bootstrap
➜ docker buildx use mybuilder
➜ docker buildx ls
➜ docker buildx inspect
```

# Build
Build multi-platform images(cache only)
```
➜ docker buildx build --platform linux/amd64,linux/arm64 -t kyagi/ec5:latest
```

Build multi-platform images and send them to container registry with `--push` option
```
➜ docker buildx build --platform linux/amd64,linux/arm64 -t kyagi/ec5:latest --push .
```
Build single-platform image(as default, without buildx)
```
➜ docker image build -t ec5:customized .
```

# References
- https://docs.docker.com/build/building/multi-platform/
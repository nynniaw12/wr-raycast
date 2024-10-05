# WGPU and Raylib Raycasting Game Engine 

Two backends one in webgpu and another in raylib are provided
for raycasting. Wgpu compiles to web with wasm and it has a
much more lower level api.

Both the raylib and wgpu backends share rendering and offloading
a pixel buffer. Details of Wgpu implementation are in w-pixbuf repo.

## Getting Started
```bash
make run # run natively

make build # build natively
make TARGET=web build # build for web 
```

## [Demo](https://curious-semifreddo-32a300.netlify.app)

## Other
License is MIT. Feel free to contribute.

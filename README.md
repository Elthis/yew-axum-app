# yew + trunk + tailwind + axum = 💗

## Introduction
Skeleton of an application in which [yew](https://github.com/yewstack/yew) + [Trunk](https://trunkrs.dev/) + [tailwindcss](https://tailwindcss.com/) is used to create Single Page App (SPA) which is then served by [axum](https://github.com/tokio-rs/axum). 


## Running the app 

### Prerequisites
- [just](https://github.com/casey/just) - used to organize commands to build/run the solution
- [docker](https://www.docker.com/) - used to build 

### Starting

To start the app first build it by running following command:
```sh
just docker-build
```

Then run following command to start the container:
```sh
just docker-run
```

App should be available at http://localhost:9999.


## Developing & Changing

### Prerequisites
- [rust](https://www.rust-lang.org/tools/install) - required to develop with rust language
- [just](https://github.com/casey/just) - used to organize commands to build/run the solution
- [trunk](https://trunkrs.dev/) - building the client side package 
- [tailwindcss](https://tailwindcss.com/) - managing the css on client side

Moreover `wasm32-unknown-unknown` toolchain is required to build frontend part. It can be installed with rustup (which should be installed along with rust):

```sh
rustup target add wasm32-unknown-unknown
```

### Running locally
To build & run the application execute following command:

```sh
just local-run
```

App should be available at http://localhost:9999.
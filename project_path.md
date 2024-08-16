# explore_rust_notebook_evcxr inside vscode

## create rust project folder

```bash

cd 
cd workspace_rust
mkdir explore_rust_notebook_evcxr && cd $_
```

## init project

```bash
touch README.md \
&& ln -s README.md README \
&& cargo init "." \
&& cargo add rustfmt \
&& rustup component add rustfmt \
&& mkdir examples \
&& cp src/main.rs examples/example.rs \
&& sed -i -e 's/world/example/g' examples/example.rs \
&& rustup  show \
&& rustup  check \
&& rustup toolchain uninstall stable \
&& rustup toolchain install stable \
&& rustup update  --force \
&& rustup show \
&& cargo add rustfmt \
&& rustup component add rustfmt \
&& rustup show \
&& touch FROM_HERE.md 
```

## build plain project

```bash
cargo build
```

## add cargo-watch#

```bash
cargo install cargo-watch
```

## install extension manually

- prettier
- rust-analyser
- code-spell-checker

## install evcxr_jupyter

[evcxr_jupyter](https://github.com/evcxr/evcxr/blob/main/evcxr_jupyter/README.md)

```bash
# cargo add evcxr
cargo install evcxr_jupyter
evcxr_jupyter --install
# reload the vscode ide complete

```

## add nessery dependencies

```bash
sudo apt update
sudo apt upgrade --Yes 
sudo apt install --Yes pkg-config libfreetype6-dev libfontconfig1-dev
```

## add plotter

```bash

cargo add plotters
cargo add csv

```
## builf project

```
cargo build
cargo update

```

## add stock data folder - e.g. FROM HERE

```bash
# change to project folder
mkdir stock_data
# inside a browser
# https://stooq.com/q/d/l/?s=TREX.US&i=d&d1=19900101&d2=20241231

```

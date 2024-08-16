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

## install evcxr_jupyter

[evcxr_jupyter](https://github.com/evcxr/evcxr/blob/main/evcxr_jupyter/README.md)

```bash
# cargo add evcxr
cargo install evcxr_jupyter
evcxr_jupyter --install
# reload the vscode ide complete

cargo add csv

```

## add stock data folder - e.g. FROM HERE

```bash
# change to project folder
mkdir stock_data
# inside a browser
# https://stooq.com/q/d/l/?s=TREX.US&i=d&d1=19900101&d2=20241231

```

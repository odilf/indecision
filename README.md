# indecision

Python library and exploratory notebooks to make super selective or negatively selective particles. 

## Running

This project uses `uv` as a package manager. You can install `uv` here: https://docs.astral.sh/uv/getting-started/installation/

You also need to install a Rust toolchain for building the Rust code. You can do so here: https://www.rust-lang.org/learn/get-started

Changes in the Rust code will be automatically picked up by `uv`. 

### Nix

If using nix, you can run `nix develop .#uv` to get into the `uv` shell for development. 

## Notebooks

Run jupyter from command line with

```bash
cd notebooks/
uv run --with jupyter jupyter lab
```

To use jupyter vscode, I think you can run:

```bash
cd notebooks/
uv run --with jupyter code .
```

You can also run marimo with

```bash
cd notebooks/
uv run --with marimo marimo
```

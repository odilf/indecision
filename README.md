# indecision

Python library and exploratory notebooks to make super selective or negatively selective particles. 

## Running

This project uses `uv` as a package manager. 

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
uv run --with jupyter
code .
```


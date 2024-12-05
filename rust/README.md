# indecision-rs

The `indecision` library, built with Rust, used from Python. This gives very singificant speedups. 

The building is done with `maturin` which is handleded by `uv`. 

The type definitions need to be updated manually. First, you can auto-generate type definitions by running `cargo run --bin stub_gen`. That drops the type definitions into a `core.pyi` file. Then, you need to import it and create the module structure manually with regular `.py` files. Is this far from ideal, but apparently you can't do better. Such is life. 

Since you can't have Rust generics in the Python interface, you need to monomorphise all generics with the specific particle you have in mind. To make this more DRY, you can use the `monomorphise!` macro. 

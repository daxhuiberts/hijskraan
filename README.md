# Hijskraan

Explorations in Cranelift code generation.

`cargo run --bin compile` generates an object file with two functions.

`cargo run --bin exec` then links the generated object file from the previous step in a rust application and calls them.

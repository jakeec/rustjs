## Rustjs

Seeing how far I can get building a recursive descent parsing interpreter for JavaScript syntax in Rust.

### Values

Values are stored against and identifier in a HashMap. Types are defined in the Types enum:

```rust
enum Types {
    Null,
    Undefined,
    Number(f64),
    Boolean(bool),
    TextString(String),
    Object(HashMap<String, Types>),
    Function(String),
}
```

Fortunately, plain JavaScript has a very manageable set of types that we need to worry about. Primitives like strings, numbers, bools, hold their equivalent value in Rust primitives. More complex types like Object and Function are treated a bit differently.

### Functions

Not sure of the best approach for this so for the time being only arrow functions are being parsed and are stored as a value the same way as any other statement. However the value they store is just a string id wrapped in a Function enum tuple struct. The id is then used to retrieve a Function struct from a separate scope table. For now this just holds arguments (not yet implemented), and `code` (the string value of the scope block), which is lazily executed only when the function is called (not yet implemented).

### Objects (not yet implemented)

Objects simply hold a HashMap of Types. Hopefully this is enough for more advanced aspect of Objects such as prototype-based inheritance.

### Control Constructs (not yet implemented)

- if/else
- while
- for
- ternary operator

### Modules (not yet implemened)

For debugging it would be most beneficial to first implement console.log(). Seeing as the node console class calls into c this doesn't seem feasible to parse the entire dependency chain so for this I will just defer to println!(). Other modules will have to just run the js portions, any std lib stuff will have to be excluded.

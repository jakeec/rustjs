## Rustjs

Seeing how far I can get building a recursive descent parsing interpreter for JavaScript syntax in Rust.

### Notes

- Should try pushing function scope onto a stack and popping when it goes out of stack rather than recursively spinning up a new compiler
- Maintain two pointers, lookahead and current
- What happens if lookahead and current pointers get out of sync?
- In JavaScript arrays are objects but it might be easier for me to treat them as a different type entirely
- How should vars/lets/consts be stored? Like this?
  ```rust
  Declaration(Type)
  // e.g.
  Const(Type::Number(Num::F64(10.0)))
  Var(Type::Number(Num::F64(10.0)))
  Let(Type::Number(Num::F64(10.0)))
  ```

### Values

Values are stored against and identifier in a HashMap. Types are defined in the Types enum:

```rust
enum Num {
    NaN,
    F64(f64)
}

enum Type {
    Null,
    Undefined,
    Number(Num),
    Boolean(bool),
    TextString(String),
    Object(HashMap<String, Types>),
    Function(String),
}
```

Fortunately, plain JavaScript has a very manageable set of types that we need to worry about. Primitives like strings, numbers, bools, hold their equivalent value in Rust primitives. More complex types like Object and Function are treated a bit differently.

### Functions

Not sure of the best approach for this so for the time being only arrow functions are being parsed and are stored as a value the same way as any other assignment. However the value they store is just a string id wrapped in a Function enum tuple struct. The id is then used to retrieve a Function struct from a separate scope table. For now this just holds arguments (not yet implemented), and `code` (the string value of the scope block), which is lazily executed only when the function is called (not yet implemented).

### Objects (not yet implemented)

Objects simply hold a HashMap of Types. Hopefully this is enough for more advanced aspect of Objects such as prototype-based inheritance.

### Control Constructs (not yet implemented)

- if/else
- while
- for
- ternary operator

### Modules (not yet implemened)

For debugging it would be most beneficial to first implement console.log(). Seeing as the node console class calls into c this doesn't seem feasible to parse the entire dependency chain so for this I will just defer to println!(). Other modules will have to just run the js portions, any std lib stuff will have to be excluded.

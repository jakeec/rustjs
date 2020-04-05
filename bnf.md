## Current

```bnf
<program> = [<assign> | <expression>]*
<assign> = [<assign-keyword>]? <ident> "=" <expression>
<assign-keyword> = "var" | "let" | "const"
<ident> = <literal>
<expression> = <factor> [<operator> <factor>]*
```

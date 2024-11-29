# string-literal

We all have been tired arguing about which of the following is the best practice:

```rust
"foo".to_owned()
"foo".to_string()
String::from("foo")
format!("foo")
```

It's time to finish this...

## Usage

```toml
string-literal = "1"
```

```rust
use string_literal::string;

let s = string!("foo");
let props = vec![
    Foo {
        key: string!("key1"),
        value: string!("value1"),
    }
];
```

# the-string-macro

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
the-string-macro = "1"
```

```rust
use the_string_macro::string;

let s = string!("foo");
let props = vec![
    Foo {
        key: string!("key1"),
        value: string!("value1"),
    }
];
```

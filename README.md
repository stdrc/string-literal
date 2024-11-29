# string-literal

We all have been tired arguing about which of the following is the best practice:

```rust
"foo".to_owned()
"foo".to_string()
String::from("foo")
format!("foo")
```

It's time to finish this...

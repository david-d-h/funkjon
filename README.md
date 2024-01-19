# Funkjon - a bad macro

With funkjon, you too can create your very own function.

## Usage:

```rust
funkjon!(greet :: name {
    println!("Hello, {name}.");
} as (String) -> Unit);

greet("David".to_string());
```

By the way, `Unit` is the return type. It is equivalent to `()`.
You can change it by just putting something other than `Unit` next to the right arrow (`->`).

### Usage with generics:

```rust
funkjon!(greet ::<T: std::fmt::Display>:: name {
    println!("Hello, {name}.");
} as (T) -> Unit);

greet("David");
```

Or with a where clause

```rust
funkjon!(greet ::<T>:: name {
    println!("Hello, {name}.");
} as (T) -> Unit
    where T: std::fmt::Display,
);

greet("David");
```

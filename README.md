# Rust Swap Library Documentation

This is a minimal Rust library providing a generic swap function for mutable references.

## Function

### `swap<T: Copy>(a: &mut T, b: &mut T)`

Swaps the values of two mutable references.

#### Parameters:
- `a`: First mutable reference to swap
- `b`: Second mutable reference to swap

#### Type Parameters:
- `T`: The type of values to swap. Must implement the `Copy` trait.

#### Examples:

```rust
use your_swap_library::swap;

let mut x = 5;
let mut y = 10;
swap(&mut x, &mut y);
assert_eq!(x, 10);
assert_eq!(y, 5);
```

#### Safety:
This function is safe as it only operates on mutable references and requires the type to be Copy, ensuring no ownership issues arise.

#### Performance:
The operation is O(1) as it performs a simple copy and assignment.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
fiano_swap = "0.1.0"
```


# Likh

**likh** is library to read and write to text and CSV files.

Please review source code carefully before production usage.
# Usage

- Read a file

```rust 
read_from_file("");
```
---

- Write to a file

```rust 
read_from_file("filename.txt","Hello!");
```
---

- Read from a CSV

```rust 
read_from_csv("filename.csv");
```

The output will be in the form of a 2d Vec -

```rust
vec![vec![1,2,3],vec![4,5,6]];
```
---

- Read from a CSV

```rust 
let c = vec![vec![1,2,3],vec![4,5,6]];

write_to_csv("filename.csv",c);
```
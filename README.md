# Analit: Analog Literals for Rust
For all of Rust's great improvements on the status quo of systems languages, one feature has been sorely missing. No, it's not higher-kinded-types, or integer generics, or having a name that includes the letter 'C'. It's analog literals. Ever since Eelis debuted "Analog Literals" in 2005 [as a C++ utility](http://www.eelis.net/C++/analogliterals.xhtml), the ability to draw geometry out using ASCII art has set a new bar for what constitutes a minimal-viable-language.

## Examples
Analog literals follow a simple syntactic pattern. The pointy corners of things are marked by '+' characters, one unit in the x-axis is represented by '--', and one unit in the Y and Z axes are represented by '|' and '/', respectively. The reason two characters are used to represent just one unit in the x-axis is to compensate for the rectangular shape that characters are rendered with. Analog expressions return a tuple reflecting the dimensions of the drawing. But enough talk--let's see some examples:

### One-Dimensional
A line of length one:
```rust
assert_eq!(1, analit!(
    +--+
));
```

A line of length four:
```rust
assert_eq!(1, analit!(
    +--------+
));
```

### Two-Dimensional
Two dimensional literals have proven themselves especially valuable to GUI programmers. A square:
```rust
assert_eq!((1,1),analit!(
    +--+
    |  |
    +--+
));
```
A delicious bar of chocolate:
```rust
assert_eq!((8,1),analit!(
    +----------------+
    |                |
    +----------------+
));
```
### Three-Dimensional
It's time to dig those red-and-blue glasses out from between the couch cushions; we're going into the next dimension!
```rust
assert_eq!((1,1,3),analit!(
        +--+
       /  /|
      /  / +
     /  / /
    +--+ /
    |  |/
    +--+
));
```
It's like it's coming right out of the screen!

## Using Analog Literals in Your Next Project*
Simply add the following to your `Cargo.toml`

```toml
[dependencies.analit]
git = "https://github.com/jswrenn/analit"
```

Or, from the registry:
```toml
[dependencies]
analit = "*"
```
*Please don't.

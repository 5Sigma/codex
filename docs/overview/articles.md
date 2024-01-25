---
title: Article format
subtitle: Overview
---

# Text formatting

Text can be formatted as bold or italic, or strike through.

## Example

```
**Bold**, _italic_, ~~dfsdfsdf~~ 
```

**Bold**, _italic_, ~~dfsdfsdf~~ 

# Links

## Example

```
[Codex](https://codex.5sigma.io)
```
[Codex](https://codex.5sigma.io)

# Code formatting

## Code blocks

Write code blocks using a set of three backticks.

### Example

<Codeexample />

```Rust
fn hello(name: &str) -> String {
  format!("Hello, {}", name)  ;
}
```

## Inline code

Inline code is written surrounded by single back ticks. Example: `SomeType`.


# Tables

## Example

```
| **Name**    | **Age** |
| ----------- | ------- |
| Alice       | 18      |
| Bob         | 19      |
| Charles     | 20      |
```

| **Name**    | **Age** |
| ----------- | ------- |
| Alice       | 18      |
| Bob         | 19      |
| Charles     | 20      |

# Lists 

## Unordered lists

### Example

```
- Alice
    - Nested
- Bob
- Charles
```
- Alice
    - Something
- Bob
- Charles

## Ordered lists

### Example

```
1. Alice
    1a. Nested
2. Bob
3. Charles
```

1. Alice
2. Bob
3. Charles


## Task list

### Example
```
- [x] Task One
- [x] Task Two
- [ ] Task Three
```

- [x] Task One
- [x] Task Two
- [ ] Task Three

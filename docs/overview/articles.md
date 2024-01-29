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

Hyper links can be added using standard markdown sytntax. 

All headers automatically have anchors that allow links to link to a section 
on the current or another page. These headers are built out of the header 
text, in lowercase, with all spaces and underscores converted to hypens.

## Example

```
[Codex](https://codex.5sigma.io)
```
[Codex](https://codex.5sigma.io)

# Block quotes

Block quotes can be used to make a paragraph stand out.
Use a greater than sign to denote a block quote.

## Example 

```
> Here is a paragraph that draws attention.
```

> Here is a paragraph that draws attention.

# Code formatting

## Code blocks
 
Write code blocks using a set of three back ticks. Optionally, you an add a 
language for syntax highlighting. 

### Example

<Codeexample />

```Rust
fn hello(name: &str) -> String {
  format!("Hello, {}", name)  ;
}
```

## Inline code

Inline code is written surrounded by single back ticks. Example: \`SomeType\`.


### Example

```
Here is an exmaple of inline `code`
```

Here is an example of inline `code`

# Tables

Tables can be created using standard markdown syntax. Table cells will be 
evaluated for internal markdown, such as bold, italic, and links. 

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

 Lists can be written either as ordered or unordered. Items can also be 
 nested with tabs.

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

Task lists are a special list that render check marks or X's, useful for 
road maps or some other set of progress items.

### Example
```
- [x] Task One
- [x] Task Two
- [ ] Task Three
```

- [x] Task One
- [x] Task Two
- [ ] Task Three

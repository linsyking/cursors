# Cursors

A Rust library that lets you edit `String` like in a modern editor (Vim, VS Code, etc.)

:fire: WIP :fire:

## Features to support

- Multiple cursors
- Multiple selections
- Move semantics for cursors
- Select by regex
- Copy, paste, replace semantics
- Iterators over selections and cursor

## Usage scenarios

- Manipulate document-like strings
- Multi-substring replacement
- Regex based replacement

## Examples

### Regex substitution

```c
#include <stdlib>
#include <stdio>

int main(){
  return 0;
}
```

We want to replace all headers with `(header).h`, i.e. we expect the result to be:

```c
#include <stdlib.h>
#include <stdio.h>

int main(){
  return 0;
}
```
It's possible to use the powerful `regex` library to implement this. (`replace_all`)

If we are in a modern editor like `VS Code`, we can do this by: search by regex, select all text that matches, move cursor to the end of the selections, and type `.h`.

To do this in an elegant way, we can create some cursors like you as if you are in a modern editor.

Sample code:

```rs
fn rep(s: String) -> String {
  let mut doc = Doc::from(s);
  doc.select_regex_all("#include <(.*?)>") // Select all that matches the regex
    .add_cursor_right() // Add cursors to the right for each selection
    .insert(".h") // Insert `.h` to all selections
    .content() // Return the content
}
```

### Insert vertically

```text
1 + 1 <
2 + 2 =
10 + 10 >
```

We want to add ` ?` to the end of each line, i.e. we expect

```text
1 + 1 < ?
2 + 2 = ?
10 + 10 > ?
```

Sample code:

```rs
fn rep(s: String) -> String {
  let mut doc = Doc::from(s);
  let lines = doc.lines();
  doc.cursors()
    .add(0) // Create a cursor at the beginning of the string
    .duplicate_down(lines - 1) // Duplicate cursors
    .move_it(CursorMove::EndOfLine) // Move to the end
    .insert(String::from(" ?")); // Insert ` ?` to all cursors
    .content() // Return the content
}
```

## Issues

- It seems it's not very suitable to implement this by using `Rc<RefCell>` because it has runtime overhead.

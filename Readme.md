# Cursors

A Rust library that lets you edit `String` like in a modern editor (Vim, VS Code, etc.)

## Features to support

- Multiple cursors
- Multiple selections
- Move semantics for cursors & docs
- Select by regex
- Copy, paste, replace semantics

## Usage scenarios

- Manipulate document-like strings
- Multi-substring replcement
- Regex based replacement

It's not convenient to directly manipulate a string by using functions in the `String` module.

For example, consider we have this string:

```c
#include <stdlib>
#include <stdio>

int main(){
  return 0;
}
```

We want to replace all include to `(itself).h`, i.e. we expect the result to be:

```c
#include <stdlib.h>
#include <stdio.h>

int main(){
  return 0;
}
```

To do this, we have to first find all subexpressions by regex `#include <(.*?)>` and then get their positions and finally add `.h`.
This process is quite complicated by using bare `String` and `regex` library code.

However, if we are in a modern editor like `vscode`, we can do this by first search all selections by regex, and select all text that matches, and move cursor to the end of the selections, and type `.h`.

This library is trying to provide a similar API that lets you do that as if you are in a modern editor.

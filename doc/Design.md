# Data Structure

## `DocRef` and `StringRef`

Those are just `Rc<RefCell<T>>`.

I use that because I want to implement mutually recursive types.

## `Doc<T>`

There are three modes of `Doc`: `DocMode`, `CursorMode`, `SelectionMode`.

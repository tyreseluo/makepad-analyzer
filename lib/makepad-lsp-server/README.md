# Makepad DSL Language Server

## Vision

we want to create a language server that can be used to provide code completion, hover, and definition capabilities for the Makepad DSL.

of course, we not only want to provide these basic language capabilities, but we also want to provide code style, formatting and linting capabilities in the future.

Maybe even a full-blown language server that can be used to compile and run Makepad DSL code, generate a base live view, just like web devtools, where you can tick and see the relevant properties and structures.

The experience might be better if it's integrated with `makepad-studio`, but for now we'll focus more on vscode in preparation for `makepad-studio`.

> [!NOTE]
> ⚠️ makepad-lsp-server is a work-in-progress that doesn't yet support all features.

## Capabilities

- [x] Syntax Highlighting: use `rust-analyzer`, not custom implemented yet
- [ ] Code Completion
- [ ] Hover
- [ ] Definition

### Code Completion

- [ ] Complete module paths

1. `use` - Include a module
2. `dep` - Include a static resource

```rust
use link::theme::*;
use link::shaders::*;
use link::widgets::*

use crate::shared::style::*;

IMG = dep("crate://self/resources/img.jpg")

```
- [ ] Complete components

  ...

```rust
pub <Home> {}
pub RoomHome = <View> {}
pub RoomHome = {{RoomHome}}<View> {}
```

### Hover
### Definition

# regex-tui

## Structure

```
src/
├── app.rs     -> holds the states and renders the widgets
├── event.rs   -> handles the terminal events (key press, mouse click, resize, etc.)
├── handler.rs -> handles the key press events and updates the application
├── lib.rs     -> module definitions
├── main.rs    -> entry-point
└── tui.rs     -> initializes/exits the terminal interface
```

## Credits
Thanks to [orhun](https://github.com/orhun) for making it easy to get started with tui-rs:
https://github.com/orhun/rust-tui-template

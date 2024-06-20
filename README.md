<h1 align="center">ratatui.ts</h1>

## Examples

```ts
import { Terminal, Stdout, Paragraph, Event } from "@lambda-group/ratatui.ts";
import type { Frame } from "@lambda-group/ratatui.ts";

using terminal = new Terminal(); // We leave terminal.restore() to [Symbol.dispose], although it may be done manually.

terminal.setup([Stdout.EnterAlternateScreen]);

while (true) {
  terminal.draw(renderApp);
  if (shouldQuit()) break;
}

function renderApp(frame: Frame) {
  frame.renderWidget(
    new Paragraph("Hello, World! (press 'q' to quit)"),
    frame.size(),
  );
}

function shouldQuit() {
  if (Event.poll(250)) return Event.read()?.code == KeyCode.char("q");
}

// here `terminal.restore([Stdout.LeaveAlternateScreen]);` is unnecessary because of the [Symbol.dispose] implementation for the Terminal
```

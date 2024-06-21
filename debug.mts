import { Frame, Paragraph, Stdout, Terminal } from ".";

const terminal = new Terminal();

terminal.setup(Stdout.EnterAlternateScreen);

let count = 0;
while (true) {
  terminal.draw((frame: Frame) => {
    frame.renderWidget(new Paragraph(`Hello, from Javascript! ${count++}`), frame.size());
  });

  await new Promise((resolve) => setTimeout(resolve, 10));
}

# Usage

## Setup

```rs
use bevy_tui::*;

fn main() {
    //setup for the tui_logger
    setup_logger();

    App::new().add_plugins((
      //Turn off LogPlugin so it doesn't clash with the tui_logger
      DefaultPlugins.build().disable::<LogPlugin>(),
      tui::TUIPlugin
    ))
    .run();
}
```

## Printing
use ```info!("Hello World!")``` instead of ```println!("Hello World!")```

## Custom Commands
Add a unique command like this:
First create a struct which will parse the input string
```rs
use bevy_tui::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MyConsoleCommand;
```
Then create a FromStr implementation to parse it from a string
```rs
impl FromStr for MyConsoleCommand {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.to_lowercase().trim().eq("hello world") {
            Ok(MyConsoleCommand)
        } else {
            Err(format!("Couldn't parse MyConsoleCommand, input: {}", s))
        }
    }
}
```
Then implement the listener which will execute the command
```rs
pub fn clear_command(
    mut command_reader: MessageReader<TUICommand>,
    //...System Params
) {
    for command in command_reader.read() {
        if command.parse::<MyConsoleCommand>().is_ok() {
            info!("Hello World!")
        }
    }
}
```

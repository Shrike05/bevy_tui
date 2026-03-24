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
import ```use log::info``` then
use ```info!("Hello World!")``` instead of ```println!("Hello World!")```

## Custom Commands
A simple way is to create a system to listen to the TUICommand message
```rs
pub fn my_command(
    mut command_reader: MessageReader<TUICommand>,
    //...System Params
) {
    for command in command_reader.read() {
        //print first argument
        info!("{}", command.args[0])
    }
}
```

Or alternatively the proper way to do it is to
1. create a struct which will parse the input string
```rs
use bevy_tui::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MyConsoleCommand;
```
2. Then create a FromStr implementation to parse it from a string
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
3. Then implement the listener which will execute the command by parsing it
```rs
pub fn my_command(
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

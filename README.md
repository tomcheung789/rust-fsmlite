Rust-fsmlite
============
fsmlite is a simple finite state machine library for Rust.

Install
============
Add fsmlite to your Cargo.toml.
```
[dependencies.fsmlite]

git = "https://github.com/tomcheung789/rust-fsmlite.git"
```

Usage
============
Import fsmlite into your project:
```rust
extern crate fsmlite;
use fsmlite::{Machine, State, Event};
use std::default::Default;
```

Simple example:
```rust
fn callback(){
  println!("callback()");
}
let mut fsm = Machine{
  name: "demo".to_string(),
  initial_state: Some("Open".to_string()),
  final_state: Some("Close".to_string()),
  states: vec![
    State{
      name: "Open".to_string(),
      enter:Some(callback),   //optional
      leave:Some(callback)    //optional
    },
    State{
      name: "Close".to_string(),
      ..Default::default()
    }
  ],
  events: vec![
    Event{
      name: "closeDoor".to_string(),
      from_state: vec!["Open".to_string()],
      to_state: "Close".to_string(),
      before:Some(callback),  //optional
      after:Some(callback)    //optional
    },
    Event{
      name: "openDoor".to_string(),
      from_state: vec!["Close".to_string()],
      to_state: "Open".to_string(),
      ..Default::default()
    }
  ],
  ..Default::default()
};
match fsm.build(){
  Ok(_) => {},
  Err(x) => println!("{}",x)
}
match fsm.fire("closeDoor"){
  Ok(_) => {},
  Err(x) => println!("{}",x)
}
```

State members
============
* `name: String`          - default `""`    
* `enter: Option<fn()>`   - optional callback, default `None`
* `leave: Option<fn()>`   - optional callback, default `None`

Event members
============
* `name: String`          - default `""`  
* `from_state: Vec<String>` - default empty list`Vec::new()`
* `to_state: String`      - default `""`
* `before: Option<fn()>`  - optional callback, default `None`
* `after: Option<fn()>`   - optional callback, default `None`

Machine members
============
* `name: String`          - default `""`
* `initial_state: Option<String>`  - default `None`
* `final_state: Option<String>`    - default `None`
* `current_state: Option<String>`  - private member
* `states: Vec<State>`  - default empty list`Vec::new()`
* `events: Vec<Event>`  - default empty list`Vec::new()`
* `ready: bool` - private member

Functions:
* `.build() -> Result<(), String>` : Build the machine before you use it. Return  Ok(()) or Err(Message)
* `.fire(event_name) -> Result<(),String>` : Fire the event. Return the result Ok(()) or Err(Message);
* `.can_fire(event_name) -> bool` : return true if the event can be fired
* `.current_state() -> String` : return current state name 
* `.is_finished() -> bool` : return true if current_state same to final_state

Callbacks
============
There have 4 types of callback in the state machine.
* State -> enter    //fired on entering the new state
* State -> leave    //fired on leaving the old state
* Event -> before   //fired before the event
* Event -> after    //fired after the event

The order of the callbacks:

1. Event -> before 
2. Old State -> leave 
3. New State -> enter 
4. Event -> after 

License
============
Rust-fsmlite is licensed under the [MIT license](https://github.com/tomcheung789/rust-fsmlite/blob/master/LICENSE).

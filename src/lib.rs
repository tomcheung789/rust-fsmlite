pub mod fsm_lite{

  use std::default::Default;

  pub struct State{
    pub name: String,
    pub enter: Option<fn()>,
    pub leave: Option<fn()>
  }

  pub struct Event{
    pub name: String,
    pub from_state: Vec<String>,
    pub to_state: String,
    pub before: Option<fn()>,
    pub after: Option<fn()>
  }

  pub struct FSM{
    pub name: String,
    pub initial_state: Option<String>,
    pub final_state: Option<String>,
    current_state: Option<String>,
    pub states: Vec<State>,
    pub events: Vec<Event>,
    ready: bool
  }

  impl Default for State{
    fn default() -> State{
      State{
        name: "".to_string(),
        enter: None,
        leave: None
      }
    }
  }

  impl Default for Event{
    fn default() -> Event{
      Event{
        name: "".to_string(),
        from_state: Vec::new(),
        to_state: "".to_string(),
        before: None,
        after: None
      }
    }
  }

  impl Default for FSM{
    fn default() -> FSM{
      FSM{
        name: "".to_string(),
        initial_state: None,
        final_state: None,
        current_state: None,
        states: Vec::new(),
        events: Vec::new(),
        ready: false
      }
    }
  }

  impl FSM{
    pub fn build(&mut self) -> Result<(), String>{
      /*
      check initial_state not null
      check state not null
      check state not duplicate
      check event not null
      check event not duplicate
      if final_state yes , check event to_state == final_state >= 1
      */
      if self.initial_state.is_none() { return Err("Initial state cannot be none.".to_string()); }
      if self.states.len() == 0 { return Err("No State is defined.".to_string()); }
      for s in self.states.iter(){
        let mut count = 0i;
        for sl in self.states.iter(){
          if s.name == sl.name {count+=1}
        }
        if count != 1 { return Err(format!("Duplicate state definition: {}.",s.name)); }
      }
      if self.events.len() == 0 { return Err("No Event is defined.".to_string()); }
      {
        let state_exists = |state: String| -> bool {
          for s in self.states.iter(){
            if s.name == state { return true; }
          }
          false
        };
        for e in self.events.iter(){
          let mut count = 0i;
          for el in self.events.iter(){
            if e.name == el.name {count+=1}
          }
          if count != 1 { return Err(format!("Duplicate event definition: {}.",e.name)); }
          if e.from_state.len() == 0 { return Err(format!("No from state is defined in Event {}.", e.name));}
          for fs in e.from_state.iter(){
            if state_exists(fs.clone()) == false { return Err(format!("State {} is not defined.", fs)); }
          }
          if state_exists(e.to_state.clone()) == false { return Err(format!("State {} is not defined.", e.to_state)); }
        }
      }
      if self.final_state.is_some() {
        let final_state = self.final_state.clone().unwrap();
        let mut count = 0i;
        for e in self.events.iter(){
          if e.to_state == final_state {count+=1}
        }
        if count == 0 { return Err("No event is connected to final state.".to_string()); }
      }
      //after checks
      self.current_state = self.initial_state.clone();
      self.ready = true;
      Ok(())
    }

    pub fn fire(&mut self, event: &str) -> Result<(),String>{
      //before > leave > enter > after
      if self.can_fire(event) {
        let current = self.current_state();
        let mut c_event: &Event;
        let mut c_from_state: &State;
        let mut c_to_state: &State;
        for e in self.events.iter() {
          if e.name.as_slice() == event {
            c_event = e; //get event obj
            match c_event.before { //fire before event
              Some(x) => x(),
              None => {}
            }
            for s in self.states.iter(){
              if s.name == current {
                c_from_state = s; //get current state obj
                match c_from_state.leave { //fire leave event
                  Some(x) => x(),
                  None => {}
                }
                break;
              }
            }
            for s in self.states.iter(){
              if s.name == c_event.to_state {
                c_to_state = s; //get to state obj
                match c_to_state.enter { //fire enter event
                  Some(x) => x(),
                  None => {}
                }
                break;
              }
            }
            match c_event.after { //fire after event
              Some(x) => x(),
              None => {}
            }
            self.current_state = Some(c_event.to_state.clone());
            break;
          }
        }
        return Ok(());
      }

      if self.ready == false { return Err("State machine is not ready.".to_string()); }
      if self.is_finished() { return Err("State machine is finished.".to_string()); }
      Err(format!("Event {} cannot be fired.", event))
    }

    pub fn can_fire(&mut self, event: &str) -> bool{
      if self.ready == false { return false; }
      if self.is_finished() { return false; }
      let current = self.current_state();
      for e in self.events.iter() {
        if e.name.as_slice() == event {
          for fs in e.from_state.iter(){
            if fs == &current { return true; }
          }
        }
      }
      false
    }

    pub fn current_state(&mut self) -> String{
      match self.current_state{
        Some(ref x) => x.clone(),
        None => "".to_string(),
      }
    }

    pub fn is_finished(&mut self) -> bool{
      match self.current_state{
        Some(ref cs) => {
            match self.final_state{
              Some(ref fs) => {
                  if cs.as_slice() == "" { false }
                  else if cs == fs { true }
                  else { false }
                },
              None => false
            }
          },
        None => false
      }
    }
  }
}

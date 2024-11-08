# Rust State Machine

This project is a Rust-based state machine to model a simple call lifecycle, transitioning between states like `Idle`, `Dialing`, `Ringing`, `Connected`, and `Disconnected` in response to specific events. It uses closures to define transitions and supports flexible state changes.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Usage](#usage)
- [Testing](#testing)
- [License](#license)

## Overview

This state machine models a simple call flow with states and events. For example, the machine starts in an `Idle` state, and transitions occur when events like `Dial`, `Incoming`, `Answer`, `HangUp`, or `Reset` are received.

### States

- `Idle`: Initial state before a call begins.
- `Dialing`: When a call is initiated.
- `Ringing`: When an incoming call is received.
- `Connected`: Call is successfully connected.
- `Disconnected`: Call is ended or hung up.

### Events

- `Dial`: Initiate a call.
- `Incoming`: Receive an incoming call.
- `Answer`: Answer a call.
- `HangUp`: Hang up a call.
- `Reset`: Reset the state machine.

## Features

- Easily extensible to add new states and transitions.
- Transitions are stored in a `HashMap` for efficient lookup.
- Error handling for invalid transitions.

## Usage

The `StateMachine` struct models the call lifecycle and allows state transitions via the `handle_event` method. Below is a simple usage example:

```rust
use std::collections::HashMap;
use crate::state_machine::{StateMachine, CallState, CallEvent};

fn main() {
    let context = HashMap::new();
    let mut sm = StateMachine::new(context);

    // Start dialing
    sm.handle_event(&CallEvent::Dial).expect("Failed to transition from Idle to Dialing");
    println!("Current state: {:?}", sm.current_state);

    // Hang up
    sm.handle_event(&CallEvent::HangUp).expect("Failed to transition from Dialing to Disconnected");
    println!("Current state: {:?}", sm.current_state);

    // Reset the state machine
    sm.handle_event(&CallEvent::Reset).expect("Failed to transition from Disconnected to Idle");
    println!("Current state: {:?}", sm.current_state);
}
```

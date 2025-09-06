# pencall

PenCall - The World's First Pen Polymorphic Function Call.

## Overview
Pencall is a Rust library that simulates resource allocation and delivery with exponential release patterns. It provides a safe, formal interface for defining allocations and plugging in custom delivery providers.

## Features
- Register and activate allocations
- Doubling-release simulation model
- Delivery provider trait for lawful integrations
- Built-in safety caps and policy hooks

## Example
Run the included demo with:
```bash
cargo run --example basic
```

This will print release events to the console.
```
ReleaseEvent: ReleaseEvent { allocation_id: "demo1", release_time: ..., units: 1 }
ReleaseEvent: ReleaseEvent { allocation_id: "demo1", release_time: ..., units: 2 }
ReleaseEvent: ReleaseEvent { allocation_id: "demo1", release_time: ..., units: 4 }
...

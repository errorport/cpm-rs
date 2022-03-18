# cpm-rs

Single crate for Critical Path Method calculation.

## Functionality

- File parser for predefined tasks. (May be removed later.)
- Critical path calculation.
- Calculation of number of maximum parallel tasks at a time.

## Future functionality

- Shiftable tasks, critical path recalculation.
- Graph visualization.
- Crate features.

## Usage

```rust

fn main() {
    let mut scheduler = scheduler::Scheduler::new();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide an input file path!");
        exit(1);
    }
    match input_parser::parse_input_file(&args[1]) {
        Ok(task_list) => { scheduler.schedule(task_list); },
        Err(e) => {eprintln!("Error: {}", e); exit(1);},
    }
}

```

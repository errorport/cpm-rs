# cpm-rs

Single crate for Critical Path Method calculation.

[![Crates.io](https://img.shields.io/crates/v/cpm-rs.svg)](https://crates.io/crates/cpm-rs)

## Functionality

- File parser for predefined tasks. (May be removed later.)
- Critical path calculation.
- Calculation of number of maximum parallel tasks at a time.

## Future functionality

- Time unit type templates. (integer / float / std::time)
- Dependency circle check.
- Shiftable tasks.
- Graph visualization.
- Crate features.

## Limitations

- Does not check circles in task dependencies.
- Does not utilize multiple utilize multiple threads for path calculations.
- Does not have a depth / performance limit on recursive path calculations.

## Usage

### Example 1: read tasks from file

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

### Example 2: add tasks from code

```rust

fn main() {
    let mut scheduler = Scheduler::new();
    scheduler.add_task(CustomTask::new(
        "Task_A".to_string()
        , 1
        , vec!{}
    ));
    scheduler.add_task(CustomTask::new(
        "Sidetask_B".to_string()
        , 3
        , vec!{"Task_A".to_string()}
    ));
    scheduler.add_task(CustomTask::new(
        "Sidetask_C".to_string()
        , 2
        , vec!{"Task_B".to_string()}
    ));
    scheduler.add_task(CustomTask::new(
        "Finish".to_string()
        , 1
        , vec!{"Sidetask_B".to_string(), "Sidetask_C".to_string()}
    ));
    match scheduler.schedule() {
        Ok(()) => {},
        Err(e) => {eprintln!("Error: {}", e); exit(1);},
    }

}

```

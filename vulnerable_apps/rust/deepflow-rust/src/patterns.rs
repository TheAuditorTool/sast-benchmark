//! Pattern matching and struct/enum variations for SAST extraction testing.
//!
//! This module demonstrates patterns that TheAuditor extracts:
//! - Tuple structs
//! - Unit structs
//! - Enum variants (unit, tuple, struct, discriminants)
//! - Match expressions (critical for CFG)
//! - Destructuring patterns (tuple, struct, slice, ref, mut)
//! - Where clauses and trait bounds

use crate::sinks;
// Grouped import (TheAuditor extracts each item)
use std::collections::{HashMap, HashSet, BTreeMap};
// Alias import
use std::collections::BTreeSet as OrderedSet;
// Self import
use std::io::{self, Read, Write};

// ============================================================================
// TUPLE STRUCTS - Structs with unnamed fields
// ============================================================================

/// Simple tuple struct
#[derive(Debug, Clone)]
pub struct Point(pub i32, pub i32);

/// Tuple struct with visibility modifiers
#[derive(Debug, Clone)]
pub struct Coordinates(pub f64, pub f64, f64); // Third field is private

/// Newtype pattern (single-field tuple struct)
#[derive(Debug, Clone)]
pub struct UserId(pub u64);

/// Newtype for tainted data
#[derive(Debug, Clone)]
pub struct TaintedString(pub String);

/// Tuple struct with generic
#[derive(Debug, Clone)]
pub struct Wrapper<T>(pub T);

/// Tuple struct with multiple generics
#[derive(Debug, Clone)]
pub struct Pair<A, B>(pub A, pub B);

/// Tuple struct with lifetime
#[derive(Debug)]
pub struct BorrowedPair<'a>(pub &'a str, pub &'a str);

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }

    pub fn x(&self) -> i32 {
        self.0
    }

    pub fn y(&self) -> i32 {
        self.1
    }

    pub fn to_string(&self) -> String {
        format!("({}, {})", self.0, self.1)
    }
}

impl TaintedString {
    pub fn new(s: String) -> Self {
        Self(s)
    }

    pub fn inner(&self) -> &str {
        &self.0
    }

    /// TAINT SINK: Newtype flows to sink
    pub fn to_sink(&self) -> Result<String, String> {
        sinks::write_to_file("/tmp/tainted_string.txt", &self.0)
    }
}

impl<T: Clone> Wrapper<T> {
    pub fn unwrap(self) -> T {
        self.0
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Wrapper<U> {
        Wrapper(f(self.0))
    }
}

// ============================================================================
// UNIT STRUCTS - Structs with no fields
// ============================================================================

/// Unit struct (marker type)
#[derive(Debug, Clone, Copy, Default)]
pub struct Marker;

/// Unit struct for type state pattern
#[derive(Debug, Clone, Copy)]
pub struct Unvalidated;

#[derive(Debug, Clone, Copy)]
pub struct Validated;

/// Unit struct with trait implementation
pub struct Logger;

impl Logger {
    pub fn log(&self, message: &str) {
        // TAINT SINK: Log message
        let _ = sinks::log_data("LOGGER", message);
    }
}

/// State machine using unit structs
pub struct Request<State> {
    data: String,
    _state: std::marker::PhantomData<State>,
}

impl Request<Unvalidated> {
    pub fn new(data: String) -> Self {
        Self {
            data,
            _state: std::marker::PhantomData,
        }
    }

    pub fn validate(self) -> Result<Request<Validated>, String> {
        if self.data.is_empty() {
            Err("Empty data".to_string())
        } else {
            Ok(Request {
                data: self.data,
                _state: std::marker::PhantomData,
            })
        }
    }
}

impl Request<Validated> {
    pub fn process(&self) -> Result<String, String> {
        // TAINT SINK: Validated data flows to sink
        sinks::execute_query(&format!("INSERT INTO requests VALUES ('{}')", self.data))
    }
}

// ============================================================================
// ENUM VARIANTS - All variant types
// ============================================================================

/// Enum with unit variants only
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Pending,
    Processing,
    Completed,
    Failed,
}

/// Enum with explicit discriminants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Priority {
    Low = 1,
    Medium = 5,
    High = 10,
    Critical = 100,
}

/// Enum with tuple variants
#[derive(Debug, Clone)]
pub enum Message {
    Text(String),
    Number(i64),
    Pair(String, String),
    Triple(i32, i32, i32),
}

/// Enum with struct variants
#[derive(Debug, Clone)]
pub enum Event {
    Click { x: i32, y: i32 },
    KeyPress { key: char, modifiers: u8 },
    Scroll { delta_x: f64, delta_y: f64 },
    Custom { name: String, data: HashMap<String, String> },
}

/// Enum with mixed variants
#[derive(Debug, Clone)]
pub enum Command {
    // Unit variant
    Quit,
    // Tuple variant
    Echo(String),
    // Struct variant
    Execute { program: String, args: Vec<String> },
    // Tuple variant with multiple fields
    Move(i32, i32, i32),
    // Unit variant with discriminant
    Noop,
}

/// Enum with generics
#[derive(Debug, Clone)]
pub enum Result2<T, E> {
    Ok(T),
    Err(E),
    Pending,
}

/// Recursive enum (like AST nodes)
#[derive(Debug, Clone)]
pub enum Expr {
    Literal(i64),
    Variable(String),
    BinaryOp {
        op: char,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    UnaryOp {
        op: char,
        expr: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
}

// ============================================================================
// MATCH EXPRESSIONS - Critical for CFG extraction
// ============================================================================

/// Simple match on enum
pub fn process_status(status: Status) -> String {
    match status {
        Status::Pending => "Waiting...".to_string(),
        Status::Processing => "Working...".to_string(),
        Status::Completed => "Done!".to_string(),
        Status::Failed => "Error!".to_string(),
    }
}

/// Match with guards
pub fn process_priority(priority: Priority, urgent: bool) -> String {
    match priority {
        Priority::Critical => "CRITICAL".to_string(),
        Priority::High if urgent => "URGENT HIGH".to_string(),
        Priority::High => "HIGH".to_string(),
        Priority::Medium if urgent => "URGENT MEDIUM".to_string(),
        Priority::Medium => "MEDIUM".to_string(),
        Priority::Low => "LOW".to_string(),
    }
}

/// Match on tuple variants
pub fn process_message(msg: Message) -> Result<String, String> {
    let result = match msg {
        Message::Text(s) => format!("TEXT: {}", s),
        Message::Number(n) => format!("NUMBER: {}", n),
        Message::Pair(a, b) => format!("PAIR: {} + {}", a, b),
        Message::Triple(x, y, z) => format!("TRIPLE: {}, {}, {}", x, y, z),
    };
    // TAINT SINK: Message content flows to sink
    sinks::write_to_file("/tmp/message.txt", &result)
}

/// Match on struct variants
pub fn process_event(event: Event) -> Result<String, String> {
    let result = match event {
        Event::Click { x, y } => format!("Click at ({}, {})", x, y),
        Event::KeyPress { key, modifiers } => format!("Key '{}' with mods {}", key, modifiers),
        Event::Scroll { delta_x, delta_y } => format!("Scroll ({}, {})", delta_x, delta_y),
        Event::Custom { name, data } => format!("Custom '{}': {:?}", name, data),
    };
    // TAINT SINK: Event data flows to sink
    sinks::log_data("EVENT", &result);
    Ok(result)
}

/// Match with binding and @ pattern
pub fn process_with_binding(msg: Message) -> String {
    match msg {
        Message::Text(ref s) if s.len() > 100 => format!("Long text: {}...", &s[..100]),
        Message::Text(s) => format!("Text: {}", s),
        Message::Number(n @ 0..=100) => format!("Small number: {}", n),
        Message::Number(n @ 101..=1000) => format!("Medium number: {}", n),
        Message::Number(n) => format!("Large number: {}", n),
        other => format!("Other: {:?}", other),
    }
}

/// Match with wildcard patterns
pub fn process_command(cmd: Command) -> Result<String, String> {
    match cmd {
        Command::Quit => {
            // TAINT SINK: Quit command triggers log
            sinks::log_data("COMMAND", "Quit received");
            Ok("Quitting".to_string())
        }
        Command::Echo(text) => {
            // TAINT SINK: Echo content flows to shell
            sinks::execute_shell(&format!("echo {}", text))
        }
        Command::Execute { program, args } => {
            // TAINT SINK: Execute command (dangerous!)
            sinks::execute_program(&program, &args)
        }
        Command::Move(x, y, z) => Ok(format!("Moving to ({}, {}, {})", x, y, z)),
        Command::Noop | _ => Ok("No operation".to_string()),
    }
}

/// Nested match expressions
pub fn evaluate_expr(expr: &Expr) -> Result<i64, String> {
    match expr {
        Expr::Literal(n) => Ok(*n),
        Expr::Variable(name) => {
            // TAINT SINK: Variable lookup
            let _ = sinks::log_data("EVAL", &format!("Looking up: {}", name));
            Ok(0) // Placeholder
        }
        Expr::BinaryOp { op, left, right } => {
            let l = evaluate_expr(left)?;
            let r = evaluate_expr(right)?;
            match op {
                '+' => Ok(l + r),
                '-' => Ok(l - r),
                '*' => Ok(l * r),
                '/' if r != 0 => Ok(l / r),
                '/' => Err("Division by zero".to_string()),
                _ => Err(format!("Unknown operator: {}", op)),
            }
        }
        Expr::UnaryOp { op, expr } => {
            let v = evaluate_expr(expr)?;
            match op {
                '-' => Ok(-v),
                '+' => Ok(v),
                _ => Err(format!("Unknown unary operator: {}", op)),
            }
        }
        Expr::Call { name, args } => {
            // TAINT SINK: Function call name flows to log
            let _ = sinks::log_data("EVAL", &format!("Calling: {} with {} args", name, args.len()));
            Ok(0) // Placeholder
        }
    }
}

/// Match on Option
pub fn process_optional(opt: Option<String>) -> Result<String, String> {
    match opt {
        Some(s) if s.is_empty() => Err("Empty string".to_string()),
        Some(s) => {
            // TAINT SINK: Optional value flows to sink
            sinks::write_to_file("/tmp/optional.txt", &s)
        }
        None => Ok("None".to_string()),
    }
}

/// Match on Result
pub fn process_result<T: std::fmt::Debug, E: std::fmt::Debug>(
    result: Result<T, E>,
) -> String {
    match result {
        Ok(value) => format!("Success: {:?}", value),
        Err(error) => format!("Error: {:?}", error),
    }
}

/// Match on references
pub fn process_ref(data: &[String]) -> Result<String, String> {
    match data {
        [] => Ok("Empty".to_string()),
        [single] => {
            // TAINT SINK: Single element flows to sink
            sinks::write_to_file("/tmp/single.txt", single)
        }
        [first, second] => {
            // TAINT SINK: Two elements flow to sink
            sinks::write_to_file("/tmp/pair.txt", &format!("{}, {}", first, second))
        }
        [first, .., last] => {
            // TAINT SINK: First and last flow to sink
            sinks::write_to_file("/tmp/range.txt", &format!("{} ... {}", first, last))
        }
    }
}

// ============================================================================
// DESTRUCTURING PATTERNS - For assignment extraction
// ============================================================================

/// Tuple destructuring
pub fn destructure_tuple(input: String) -> Result<String, String> {
    let pair = (input.clone(), input.len());
    let (data, len) = pair;

    // TAINT SINK: Destructured data flows to sink
    sinks::write_to_file("/tmp/tuple_destruct.txt", &format!("{} (len: {})", data, len))
}

/// Nested tuple destructuring
pub fn destructure_nested_tuple(input: String) -> Result<String, String> {
    let nested = ((input.clone(), 1), (2, input));
    let ((a, _b), (_c, d)) = nested;

    // TAINT SINK: Nested destructured data flows to sink
    sinks::execute_query(&format!("INSERT INTO nested VALUES ('{}', '{}')", a, d))
}

/// Struct destructuring
pub fn destructure_struct(event: Event) -> Result<String, String> {
    // Destructure in let binding
    if let Event::Click { x, y } = event {
        // TAINT SINK: Destructured struct fields flow to sink
        return sinks::write_to_file("/tmp/click.txt", &format!("x={}, y={}", x, y));
    }
    Ok("Not a click".to_string())
}

/// Struct destructuring with rename
pub fn destructure_with_rename(event: Event) -> String {
    match event {
        Event::KeyPress { key: k, modifiers: m } => format!("Key: {}, Mods: {}", k, m),
        Event::Click { x: col, y: row } => format!("Position: ({}, {})", col, row),
        _ => "Other event".to_string(),
    }
}

/// Slice pattern destructuring
pub fn destructure_slice(items: &[String]) -> Result<String, String> {
    let result = match items {
        [] => "empty".to_string(),
        [only] => format!("single: {}", only),
        [first, rest @ ..] => format!("first: {}, rest count: {}", first, rest.len()),
    };
    // TAINT SINK: Slice pattern result flows to sink
    sinks::log_data("SLICE", &result);
    Ok(result)
}

/// Ref pattern in destructuring
pub fn destructure_ref(data: &(String, i32)) -> Result<String, String> {
    let (ref text, ref num) = data;

    // TAINT SINK: Ref pattern data flows to sink
    sinks::write_to_file("/tmp/ref_destruct.txt", &format!("{}: {}", text, num))
}

/// Mut pattern in destructuring
pub fn destructure_mut(mut data: (String, i32)) -> Result<String, String> {
    let (ref mut text, ref mut num) = data;

    // Mutate the destructured data
    text.push_str("_modified");
    *num += 1;

    // TAINT SINK: Modified data flows to sink
    sinks::write_to_file("/tmp/mut_destruct.txt", &format!("{}: {}", text, num))
}

/// Combined patterns
pub fn destructure_combined(input: String) -> Result<String, String> {
    // Complex destructuring
    let wrapper = Wrapper(TaintedString::new(input));
    let Wrapper(TaintedString(inner)) = wrapper;

    // TAINT SINK: Deeply destructured data flows to sink
    sinks::execute_shell(&format!("echo {}", inner))
}

/// Destructuring in function parameters
pub fn param_destructure(Point(x, y): Point) -> Result<String, String> {
    // TAINT SINK: Destructured params flow to sink
    sinks::write_to_file("/tmp/param_destruct.txt", &format!("x={}, y={}", x, y))
}

/// Destructuring in for loop
pub fn loop_destructure(pairs: Vec<(String, i32)>) -> Result<String, String> {
    let mut results = Vec::new();

    for (name, value) in pairs {
        // TAINT SINK: Loop destructured data
        let _ = sinks::log_data("LOOP", &format!("{}: {}", name, value));
        results.push(format!("{}={}", name, value));
    }

    Ok(results.join(", "))
}

// ============================================================================
// WHERE CLAUSES AND TRAIT BOUNDS
// ============================================================================

/// Function with simple trait bound
pub fn process_display<T: std::fmt::Display>(item: T) -> String {
    format!("DISPLAY: {}", item)
}

/// Function with multiple trait bounds
pub fn process_multi_bound<T: std::fmt::Display + std::fmt::Debug + Clone>(item: T) -> String {
    format!("DISPLAY: {}, DEBUG: {:?}", item.clone(), item)
}

/// Function with where clause
pub fn process_where<T, U>(t: T, u: U) -> String
where
    T: std::fmt::Display,
    U: std::fmt::Debug,
{
    format!("T: {}, U: {:?}", t, u)
}

/// Function with complex where clause
pub fn process_complex_where<T, U, V>(t: T, u: U, v: V) -> Result<String, String>
where
    T: std::fmt::Display + Send,
    U: std::fmt::Debug + Clone,
    V: AsRef<str>,
{
    let result = format!("T: {}, U: {:?}, V: {}", t, u, v.as_ref());
    // TAINT SINK: Generic data flows to sink
    sinks::write_to_file("/tmp/where_clause.txt", &result)
}

/// Struct with where clause
pub struct Container<T>
where
    T: Clone,
{
    data: Vec<T>,
}

impl<T> Container<T>
where
    T: Clone + std::fmt::Debug,
{
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
    }

    pub fn process(&self) -> String {
        format!("{:?}", self.data)
    }
}

impl<T: Clone + std::fmt::Debug> Default for Container<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait with associated type and where clause
pub trait Processor {
    type Output;

    fn process(&self) -> Self::Output;
}

/// Impl with where clause on associated type
impl<T> Processor for Container<T>
where
    T: Clone + std::fmt::Debug,
{
    type Output = String;

    fn process(&self) -> Self::Output {
        self.process()
    }
}

/// Function with trait bound on associated type
pub fn process_with_output<P>(processor: P) -> P::Output
where
    P: Processor,
    P::Output: std::fmt::Debug,
{
    processor.process()
}

// ============================================================================
// COMBINED PATTERNS TO SINK
// ============================================================================

/// Process all pattern types to sinks
pub fn patterns_to_sink(input: String) -> Result<String, String> {
    // Tuple struct
    let tainted = TaintedString::new(input.clone());
    let _ = tainted.to_sink()?;

    // Match expression
    let msg = Message::Text(input.clone());
    let _ = process_message(msg)?;

    // Destructuring
    let _ = destructure_tuple(input.clone())?;

    // Enum with struct variant
    let event = Event::Custom {
        name: "test".to_string(),
        data: HashMap::new(),
    };
    let _ = process_event(event)?;

    // Final combined result
    sinks::write_to_file("/tmp/patterns_combined.txt", &input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_struct() {
        let p = Point::new(10, 20);
        assert_eq!(p.x(), 10);
        assert_eq!(p.y(), 20);
    }

    #[test]
    fn test_unit_struct() {
        let _marker = Marker;
        let _logger = Logger;
    }

    #[test]
    fn test_status_match() {
        assert_eq!(process_status(Status::Completed), "Done!");
    }

    #[test]
    fn test_priority_match() {
        assert_eq!(process_priority(Priority::Critical, false), "CRITICAL");
        assert_eq!(process_priority(Priority::High, true), "URGENT HIGH");
    }

    #[test]
    fn test_wrapper() {
        let w = Wrapper(42);
        let mapped = w.map(|x| x * 2);
        assert_eq!(mapped.unwrap(), 84);
    }

    #[test]
    fn test_container() {
        let mut c = Container::new();
        c.push(1);
        c.push(2);
        assert_eq!(c.process(), "[1, 2]");
    }
}

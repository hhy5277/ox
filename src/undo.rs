// Undo.rs - Utilities for undoing, redoing and storing events
use crate::{Direction, Position, Row};

// Enum for the the types of banks
#[derive(Debug, Clone)]
pub enum BankType {
    Line,   // For holding lines from the document
    Cursor, // For holding cursor positions
}

// Event enum to store the types of events that occur
#[derive(Debug, Clone)]
pub enum Event {
    Store(BankType, usize),                // Store an item in a bank
    Load(BankType, usize),                 // Load an item from a bank
    SpliceUp(Position),                    // Delete from start
    SplitDown(Position),                   // Return from middle of the line
    InsertLineAbove(usize),                // Return key in the middle of line
    InsertLineBelow(usize),                // Return on the end of line
    Deletion(Position, char),              // Delete from middle
    Insertion(Position, char),             // Insert character
    DeleteLine(usize, Box<Row>),           // For deleting a line
    UpdateLine(usize, Box<Row>, Box<Row>), // For holding entire line updates
    MoveCursor(i128, Direction),           // For moving the cursor
    GotoCursor(Position),                  // For setting the cursor position
    Overwrite(Vec<Row>, Vec<Row>),         // Overwrite document
    New,                                   // New document
    Open(Option<String>),                  // Open document
    Save(Option<String>, bool),            // Save document
    SaveAll,                               // Save all documents
    Undo,                                  // Undo event
    Redo,                                  // Redo event
    Commit,                                // Commit undo event
    Quit(bool),                            // Quit document
    QuitAll(bool),                         // Quit all
    NextTab,                               // Next tab
    PrevTab,                               // Previous tab
}

// A struct for holding all the events taken by the user
#[derive(Debug)]
pub struct EventStack {
    history: Vec<Vec<Event>>,  // For storing the history of events
    current_patch: Vec<Event>, // For storing the current group
}

// Methods for the EventStack
impl EventStack {
    pub fn new() -> Self {
        // Initialise an Event stack
        Self {
            history: vec![],
            current_patch: vec![],
        }
    }
    pub fn push(&mut self, event: Event) {
        // Add an event to the event stack
        self.current_patch.insert(0, event);
    }
    pub fn append(&mut self, patch: Vec<Event>) {
        self.history.push(patch);
    }
    pub fn pop(&mut self) -> Option<Vec<Event>> {
        // Take a patch off the event stack
        self.history.pop()
    }
    pub fn empty(&mut self) {
        // Empty the stack
        self.history.clear();
    }
    pub fn commit(&mut self) {
        // Commit patch to history
        if !self.current_patch.is_empty() {
            self.history.push(self.current_patch.clone());
            self.current_patch.clear();
        }
    }
}

pub fn reverse(before: Event) -> Option<Event> {
    // Turn an event into the opposite of itself
    // Used for undo
    Some(match before {
        Event::SpliceUp(pos) => Event::SplitDown(pos),
        Event::SplitDown(pos) => Event::SplitDown(pos),
        Event::InsertLineAbove(y) => Event::DeleteLine(y, Box::new(Row::from(""))),
        Event::InsertLineBelow(y) => Event::DeleteLine(y.saturating_add(1), Box::new(Row::from(""))),
        Event::Deletion(pos, ch) => Event::Insertion(pos, ch),
        Event::Insertion(pos, ch) => Event::Deletion(pos, ch),
        Event::DeleteLine(y, before) => Event::UpdateLine(y, Box::new(Row::from("")), before),
        Event::UpdateLine(y, before, after) => Event::UpdateLine(y, after, before),
        Event::Overwrite(before, after) => Event::Overwrite(after, before),
        _ => return None,
    })
}

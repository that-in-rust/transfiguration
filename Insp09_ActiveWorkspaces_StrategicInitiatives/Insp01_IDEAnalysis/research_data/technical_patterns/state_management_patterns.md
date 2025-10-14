# State Management and Data Flow Patterns in Development Environments

## Executive Summary

This document analyzes proven state management architectures used in complex IDE applications, focusing on data synchronization patterns, undo/redo system implementations, and real-time collaboration patterns. Based on analysis of 8+ major development environments, we identify scalable patterns for managing complex application state in Rust/WASM implementations.

## Research Methodology

### Analysis Framework
- **State Architecture**: Centralized vs distributed state management approaches
- **Data Synchronization**: Patterns for keeping UI and backend state consistent
- **Undo/Redo Systems**: Command pattern implementations and state history management
- **Real-time Collaboration**: Conflict resolution and operational transformation patterns
- **Performance Optimization**: State update batching and selective rendering strategies

### Reference Implementations
- **VS Code**: Event-driven architecture with centralized state management
- **Zed**: CRDT-based collaborative state with GPU-accelerated rendering
- **Figma**: Operational transformation with real-time multiplayer state
- **IntelliJ IDEA**: Command-based architecture with sophisticated undo/redo
- **Sublime Text**: Lightweight state management with efficient text operations
- **Atom**: Redux-inspired state management (legacy analysis)
- **Xi Editor**: Rope-based text representation with async state updates
- **Monaco Editor**: Immutable state with efficient diff algorithms

## Core State Management Patterns

### 1. Event Sourcing Pattern

**Pattern: Immutable Event Log with State Reconstruction**

Event sourcing treats state changes as a sequence of immutable events, enabling powerful features like time travel debugging, audit trails, and distributed synchronization.

#### Core Implementation
```rust
// Event definitions for IDE operations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum IDEEvent {
    // Document events
    DocumentOpened {
        document_id: DocumentId,
        path: PathBuf,
        content: String,
        timestamp: DateTime<Utc>,
    },
    TextInserted {
        document_id: DocumentId,
        position: Position,
        text: String,
        timestamp: DateTime<Utc>,
    },
    TextDeleted {
        document_id: DocumentId,
        range: Range,
        deleted_text: String,
        timestamp: DateTime<Utc>,
    },
    
    // Editor events
    CursorMoved {
        document_id: DocumentId,
        old_position: Position,
        new_position: Position,
        timestamp: DateTime<Utc>,
    },
    SelectionChanged {
        document_id: DocumentId,
        selection: Selection,
        timestamp: DateTime<Utc>,
    },
    
    // Workspace events
    FileCreated {
        path: PathBuf,
        content: Option<String>,
        timestamp: DateTime<Utc>,
    },
    FileDeleted {
        path: PathBuf,
        timestamp: DateTime<Utc>,
    },
    
    // UI events
    PanelToggled {
        panel_id: PanelId,
        visible: bool,
        timestamp: DateTime<Utc>,
    },
    ThemeChanged {
        theme_id: String,
        timestamp: DateTime<Utc>,
    },
}

// Event store for persistence and replay
#[async_trait]
pub trait EventStore: Send + Sync {
    async fn append_event(&self, event: IDEEvent) -> Result<EventId, EventStoreError>;
    async fn get_events_since(&self, event_id: Option<EventId>) -> Result<Vec<IDEEvent>, EventStoreError>;
    async fn get_events_for_document(&self, document_id: DocumentId) -> Result<Vec<IDEEvent>, EventStoreError>;
    async fn create_snapshot(&self, state: &IDEState) -> Result<SnapshotId, EventStoreError>;
    async fn get_latest_snapshot(&self) -> Result<Option<(SnapshotId, IDEState)>, EventStoreError>;
}

// State reconstruction from events
pub struct StateProjector {
    event_store: Arc<dyn EventStore>,
    snapshot_interval: usize, // Create snapshot every N events
}

impl StateProjector {
    pub async fn reconstruct_state(&self) -> Result<IDEState, ProjectionError> {
        // Start with latest snapshot if available
        let (mut state, last_snapshot_id) = match self.event_store.get_latest_snapshot().await? {
            Some((snapshot_id, snapshot_state)) => (snapshot_state, Some(snapshot_id)),
            None => (IDEState::default(), None),
        };
        
        // Apply events since snapshot
        let events = self.event_store.get_events_since(last_snapshot_id).await?;
        
        for event in events {
            self.apply_event(&mut state, &event)?;
        }
        
        Ok(state)
    }
    
    fn apply_event(&self, state: &mut IDEState, event: &IDEEvent) -> Result<(), ProjectionError> {
        match event {
            IDEEvent::DocumentOpened { document_id, path, content, .. } => {
                let document = Document::new(*document_id, path.clone(), content.clone());
                state.documents.insert(*document_id, document);
            }
            
            IDEEvent::TextInserted { document_id, position, text, .. } => {
                if let Some(document) = state.documents.get_mut(document_id) {
                    document.insert_text(*position, text)?;
                }
            }
            
            IDEEvent::TextDeleted { document_id, range, .. } => {
                if let Some(document) = state.documents.get_mut(document_id) {
                    document.delete_range(*range)?;
                }
            }
            
            IDEEvent::CursorMoved { document_id, new_position, .. } => {
                if let Some(editor_state) = state.editor_states.get_mut(document_id) {
                    editor_state.cursor_position = *new_position;
                }
            }
            
            // ... handle other events
        }
        
        Ok(())
    }
}

// Event-driven state manager
pub struct EventDrivenStateManager {
    current_state: Arc<RwLock<IDEState>>,
    event_store: Arc<dyn EventStore>,
    projector: Arc<StateProjector>,
    subscribers: Arc<RwLock<HashMap<String, Vec<Box<dyn StateSubscriber>>>>>,
}

impl EventDrivenStateManager {
    pub async fn apply_event(&self, event: IDEEvent) -> Result<(), StateError> {
        // Store the event
        let event_id = self.event_store.append_event(event.clone()).await?;
        
        // Apply to current state
        {
            let mut state = self.current_state.write().await;
            self.projector.apply_event(&mut *state, &event)?;
        }
        
        // Notify subscribers
        self.notify_subscribers(&event).await?;
        
        Ok(())
    }
    
    pub async fn get_state_snapshot(&self) -> IDEState {
        self.current_state.read().await.clone()
    }
    
    async fn notify_subscribers(&self, event: &IDEEvent) -> Result<(), StateError> {
        let subscribers = self.subscribers.read().await;
        
        for (event_type, handlers) in subscribers.iter() {
            if self.event_matches_type(event, event_type) {
                for handler in handlers {
                    if let Err(e) = handler.handle_event(event).await {
                        log::error!("Subscriber error: {}", e);
                    }
                }
            }
        }
        
        Ok(())
    }
}
```

#### Time Travel and Debugging
```rust
// Time travel debugging capabilities
pub struct TimeTravelDebugger {
    state_manager: Arc<EventDrivenStateManager>,
    event_store: Arc<dyn EventStore>,
    current_position: Arc<RwLock<Option<EventId>>>,
}

impl TimeTravelDebugger {
    pub async fn jump_to_event(&self, event_id: EventId) -> Result<IDEState, DebuggerError> {
        // Reconstruct state up to the specified event
        let events = self.event_store.get_events_since(None).await?;
        let mut state = IDEState::default();
        
        for event in events {
            if event.id() <= event_id {
                self.state_manager.projector.apply_event(&mut state, &event)?;
            } else {
                break;
            }
        }
        
        *self.current_position.write().await = Some(event_id);
        Ok(state)
    }
    
    pub async fn step_forward(&self) -> Result<Option<IDEState>, DebuggerError> {
        let current_pos = *self.current_position.read().await;
        let events = self.event_store.get_events_since(current_pos).await?;
        
        if let Some(next_event) = events.first() {
            self.jump_to_event(next_event.id()).await.map(Some)
        } else {
            Ok(None)
        }
    }
    
    pub async fn step_backward(&self) -> Result<Option<IDEState>, DebuggerError> {
        let current_pos = *self.current_position.read().await;
        
        if let Some(pos) = current_pos {
            if pos > EventId(0) {
                self.jump_to_event(EventId(pos.0 - 1)).await.map(Some)
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}
```

### 2. Command Pattern with Undo/Redo

**Pattern: Reversible Operations with Command History**

The command pattern enables sophisticated undo/redo functionality by encapsulating operations as objects that can be executed, undone, and redone.

#### Command Interface and Implementation
```rust
// Command trait for reversible operations
#[async_trait]
pub trait Command: Send + Sync + std::fmt::Debug {
    async fn execute(&self, state: &mut IDEState) -> Result<(), CommandError>;
    async fn undo(&self, state: &mut IDEState) -> Result<(), CommandError>;
    async fn redo(&self, state: &mut IDEState) -> Result<(), CommandError> {
        // Default implementation: redo is the same as execute
        self.execute(state).await
    }
    
    fn can_merge_with(&self, other: &dyn Command) -> bool {
        false // Default: commands cannot be merged
    }
    
    fn merge_with(&self, other: Box<dyn Command>) -> Result<Box<dyn Command>, CommandError> {
        Err(CommandError::CannotMerge)
    }
    
    fn description(&self) -> String;
}

// Text editing commands
#[derive(Debug, Clone)]
pub struct InsertTextCommand {
    document_id: DocumentId,
    position: Position,
    text: String,
    timestamp: DateTime<Utc>,
}

#[async_trait]
impl Command for InsertTextCommand {
    async fn execute(&self, state: &mut IDEState) -> Result<(), CommandError> {
        if let Some(document) = state.documents.get_mut(&self.document_id) {
            document.insert_text(self.position, &self.text)?;
            
            // Update cursor position
            if let Some(editor_state) = state.editor_states.get_mut(&self.document_id) {
                editor_state.cursor_position = Position {
                    line: self.position.line,
                    column: self.position.column + self.text.len(),
                };
            }
            
            Ok(())
        } else {
            Err(CommandError::DocumentNotFound(self.document_id))
        }
    }
    
    async fn undo(&self, state: &mut IDEState) -> Result<(), CommandError> {
        if let Some(document) = state.documents.get_mut(&self.document_id) {
            let end_position = Position {
                line: self.position.line,
                column: self.position.column + self.text.len(),
            };
            
            document.delete_range(Range {
                start: self.position,
                end: end_position,
            })?;
            
            // Restore cursor position
            if let Some(editor_state) = state.editor_states.get_mut(&self.document_id) {
                editor_state.cursor_position = self.position;
            }
            
            Ok(())
        } else {
            Err(CommandError::DocumentNotFound(self.document_id))
        }
    }
    
    fn can_merge_with(&self, other: &dyn Command) -> bool {
        if let Some(other_insert) = other.as_any().downcast_ref::<InsertTextCommand>() {
            // Can merge if same document, adjacent positions, and within time window
            self.document_id == other_insert.document_id &&
            self.is_adjacent_to(other_insert) &&
            self.timestamp.signed_duration_since(other_insert.timestamp).num_seconds().abs() < 5
        } else {
            false
        }
    }
    
    fn merge_with(&self, other: Box<dyn Command>) -> Result<Box<dyn Command>, CommandError> {
        if let Ok(other_insert) = other.into_any().downcast::<InsertTextCommand>() {
            Ok(Box::new(InsertTextCommand {
                document_id: self.document_id,
                position: self.position,
                text: format!("{}{}", self.text, other_insert.text),
                timestamp: self.timestamp,
            }))
        } else {
            Err(CommandError::CannotMerge)
        }
    }
    
    fn description(&self) -> String {
        format!("Insert '{}' at {}:{}", self.text, self.position.line, self.position.column)
    }
}

// Composite command for complex operations
#[derive(Debug)]
pub struct CompositeCommand {
    commands: Vec<Box<dyn Command>>,
    description: String,
}

impl CompositeCommand {
    pub fn new(description: String) -> Self {
        CompositeCommand {
            commands: Vec::new(),
            description,
        }
    }
    
    pub fn add_command(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }
}

#[async_trait]
impl Command for CompositeCommand {
    async fn execute(&self, state: &mut IDEState) -> Result<(), CommandError> {
        for command in &self.commands {
            command.execute(state).await?;
        }
        Ok(())
    }
    
    async fn undo(&self, state: &mut IDEState) -> Result<(), CommandError> {
        // Undo in reverse order
        for command in self.commands.iter().rev() {
            command.undo(state).await?;
        }
        Ok(())
    }
    
    fn description(&self) -> String {
        self.description.clone()
    }
}
```

#### Command History Management
```rust
// Command history with undo/redo stack
pub struct CommandHistory {
    executed_commands: Vec<Box<dyn Command>>,
    undo_stack: Vec<Box<dyn Command>>,
    max_history_size: usize,
    merge_timeout: Duration,
}

impl CommandHistory {
    pub fn new(max_history_size: usize) -> Self {
        CommandHistory {
            executed_commands: Vec::new(),
            undo_stack: Vec::new(),
            max_history_size,
            merge_timeout: Duration::from_secs(5),
        }
    }
    
    pub async fn execute_command(
        &mut self,
        command: Box<dyn Command>,
        state: &mut IDEState
    ) -> Result<(), CommandError> {
        // Try to merge with the last command if possible
        if let Some(last_command) = self.executed_commands.last() {
            if last_command.can_merge_with(&**command) {
                let merged = last_command.merge_with(command)?;
                
                // Undo the last command and execute the merged one
                if let Some(last) = self.executed_commands.pop() {
                    last.undo(state).await?;
                }
                
                merged.execute(state).await?;
                self.executed_commands.push(merged);
                
                // Clear redo stack since we modified history
                self.undo_stack.clear();
                
                return Ok(());
            }
        }
        
        // Execute the command
        command.execute(state).await?;
        
        // Add to history
        self.executed_commands.push(command);
        
        // Clear redo stack since we executed a new command
        self.undo_stack.clear();
        
        // Limit history size
        if self.executed_commands.len() > self.max_history_size {
            self.executed_commands.remove(0);
        }
        
        Ok(())
    }
    
    pub async fn undo(&mut self, state: &mut IDEState) -> Result<Option<String>, CommandError> {
        if let Some(command) = self.executed_commands.pop() {
            command.undo(state).await?;
            let description = command.description();
            self.undo_stack.push(command);
            Ok(Some(description))
        } else {
            Ok(None)
        }
    }
    
    pub async fn redo(&mut self, state: &mut IDEState) -> Result<Option<String>, CommandError> {
        if let Some(command) = self.undo_stack.pop() {
            command.redo(state).await?;
            let description = command.description();
            self.executed_commands.push(command);
            Ok(Some(description))
        } else {
            Ok(None)
        }
    }
    
    pub fn can_undo(&self) -> bool {
        !self.executed_commands.is_empty()
    }
    
    pub fn can_redo(&self) -> bool {
        !self.undo_stack.is_empty()
    }
    
    pub fn get_undo_description(&self) -> Option<String> {
        self.executed_commands.last().map(|cmd| cmd.description())
    }
    
    pub fn get_redo_description(&self) -> Option<String> {
        self.undo_stack.last().map(|cmd| cmd.description())
    }
}
```

### 3. Reactive State Management Pattern

**Pattern: Observable State with Automatic UI Updates**

Reactive state management automatically propagates state changes to interested components, reducing boilerplate and ensuring UI consistency.

#### Observable State Implementation
```rust
use futures::stream::{Stream, StreamExt};
use tokio::sync::broadcast;

// Observable value that notifies subscribers of changes
pub struct Observable<T> {
    value: Arc<RwLock<T>>,
    sender: broadcast::Sender<T>,
}

impl<T> Observable<T> 
where 
    T: Clone + Send + Sync + 'static,
{
    pub fn new(initial_value: T) -> Self {
        let (sender, _) = broadcast::channel(1000);
        Observable {
            value: Arc::new(RwLock::new(initial_value)),
            sender,
        }
    }
    
    pub async fn set(&self, new_value: T) {
        *self.value.write().await = new_value.clone();
        let _ = self.sender.send(new_value);
    }
    
    pub async fn update<F>(&self, updater: F) 
    where 
        F: FnOnce(&mut T),
    {
        let new_value = {
            let mut value = self.value.write().await;
            updater(&mut *value);
            value.clone()
        };
        let _ = self.sender.send(new_value);
    }
    
    pub async fn get(&self) -> T {
        self.value.read().await.clone()
    }
    
    pub fn subscribe(&self) -> impl Stream<Item = T> {
        let receiver = self.sender.subscribe();
        tokio_stream::wrappers::BroadcastStream::new(receiver)
            .filter_map(|result| async move { result.ok() })
    }
    
    pub fn map<U, F>(&self, mapper: F) -> Observable<U>
    where
        F: Fn(&T) -> U + Send + Sync + 'static,
        U: Clone + Send + Sync + 'static,
    {
        let mapped_initial = {
            let value = futures::executor::block_on(self.value.read());
            mapper(&*value)
        };
        
        let mapped = Observable::new(mapped_initial);
        let mapped_sender = mapped.sender.clone();
        
        let mut stream = self.subscribe();
        tokio::spawn(async move {
            while let Some(value) = stream.next().await {
                let mapped_value = mapper(&value);
                let _ = mapped_sender.send(mapped_value);
            }
        });
        
        mapped
    }
}

// Reactive document state
pub struct ReactiveDocument {
    content: Observable<String>,
    cursor_position: Observable<Position>,
    selection: Observable<Option<Selection>>,
    is_dirty: Observable<bool>,
    language: Observable<Language>,
}

impl ReactiveDocument {
    pub fn new(initial_content: String, language: Language) -> Self {
        ReactiveDocument {
            content: Observable::new(initial_content),
            cursor_position: Observable::new(Position::default()),
            selection: Observable::new(None),
            is_dirty: Observable::new(false),
            language: Observable::new(language),
        }
    }
    
    pub async fn insert_text(&self, position: Position, text: &str) {
        self.content.update(|content| {
            // Insert text at position (simplified)
            content.insert_str(position.offset, text);
        }).await;
        
        self.is_dirty.set(true).await;
        
        // Update cursor position
        let new_position = Position {
            line: position.line,
            column: position.column + text.len(),
        };
        self.cursor_position.set(new_position).await;
    }
    
    pub async fn delete_range(&self, range: Range) {
        let deleted_text = {
            let content = self.content.get().await;
            content[range.start.offset..range.end.offset].to_string()
        };
        
        self.content.update(|content| {
            content.drain(range.start.offset..range.end.offset);
        }).await;
        
        self.is_dirty.set(true).await;
        self.cursor_position.set(range.start).await;
    }
    
    // Computed observables
    pub fn word_count(&self) -> Observable<usize> {
        self.content.map(|content| {
            content.split_whitespace().count()
        })
    }
    
    pub fn line_count(&self) -> Observable<usize> {
        self.content.map(|content| {
            content.lines().count()
        })
    }
}
```

#### Reactive State Store
```rust
// Central reactive state store
pub struct ReactiveStateStore {
    documents: Observable<HashMap<DocumentId, ReactiveDocument>>,
    active_document: Observable<Option<DocumentId>>,
    workspace_path: Observable<Option<PathBuf>>,
    theme: Observable<Theme>,
    ui_state: Observable<UIState>,
}

impl ReactiveStateStore {
    pub fn new() -> Self {
        ReactiveStateStore {
            documents: Observable::new(HashMap::new()),
            active_document: Observable::new(None),
            workspace_path: Observable::new(None),
            theme: Observable::new(Theme::default()),
            ui_state: Observable::new(UIState::default()),
        }
    }
    
    pub async fn open_document(&self, path: PathBuf, content: String) -> DocumentId {
        let document_id = DocumentId::new();
        let language = detect_language(&path);
        let document = ReactiveDocument::new(content, language);
        
        self.documents.update(|docs| {
            docs.insert(document_id, document);
        }).await;
        
        self.active_document.set(Some(document_id)).await;
        
        document_id
    }
    
    pub async fn close_document(&self, document_id: DocumentId) {
        self.documents.update(|docs| {
            docs.remove(&document_id);
        }).await;
        
        // If this was the active document, clear active document
        let current_active = self.active_document.get().await;
        if current_active == Some(document_id) {
            self.active_document.set(None).await;
        }
    }
    
    // Computed state: is any document dirty?
    pub fn has_unsaved_changes(&self) -> Observable<bool> {
        let documents = self.documents.clone();
        
        // This is a simplified example - in practice, you'd want more efficient
        // change detection that doesn't recompute for every document change
        documents.map(|docs| {
            docs.values().any(|doc| {
                futures::executor::block_on(doc.is_dirty.get())
            })
        })
    }
}
```

**Benefits of Reactive Patterns:**
- Automatic UI updates when state changes
- Declarative data flow and transformations
- Reduced boilerplate for state synchronization
- Easy composition of derived state

**Implementation Considerations:**
- Memory usage from subscriptions and streams
- Potential for circular dependencies
- Debugging complexity with reactive chains
- Performance impact of frequent updates### 4
. CRDT-Based Collaborative State Pattern

**Pattern: Conflict-Free Replicated Data Types for Real-Time Collaboration**

CRDTs enable multiple users to edit the same document simultaneously without conflicts, making them ideal for collaborative development environments.

#### Text CRDT Implementation
```rust
use std::collections::BTreeMap;

// Position identifier for CRDT operations
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PositionId {
    site_id: u32,
    counter: u64,
}

impl PositionId {
    pub fn new(site_id: u32, counter: u64) -> Self {
        PositionId { site_id, counter }
    }
    
    pub fn between(&self, other: &PositionId) -> PositionId {
        // Generate a position between self and other
        // This is a simplified implementation
        PositionId {
            site_id: self.site_id,
            counter: (self.counter + other.counter) / 2,
        }
    }
}

// CRDT character with position
#[derive(Debug, Clone)]
pub struct CRDTChar {
    id: PositionId,
    character: char,
    visible: bool,
    author: UserId,
    timestamp: DateTime<Utc>,
}

// CRDT-based text document
pub struct CRDTDocument {
    site_id: u32,
    counter: Arc<AtomicU64>,
    characters: Arc<RwLock<BTreeMap<PositionId, CRDTChar>>>,
    tombstones: Arc<RwLock<HashSet<PositionId>>>, // Deleted characters
    subscribers: Arc<RwLock<Vec<Box<dyn CRDTSubscriber>>>>,
}

impl CRDTDocument {
    pub fn new(site_id: u32) -> Self {
        CRDTDocument {
            site_id,
            counter: Arc::new(AtomicU64::new(0)),
            characters: Arc::new(RwLock::new(BTreeMap::new())),
            tombstones: Arc::new(RwLock::new(HashSet::new())),
            subscribers: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    pub async fn insert_char(&self, position: usize, character: char, author: UserId) -> CRDTOperation {
        let counter = self.counter.fetch_add(1, Ordering::SeqCst);
        let char_id = PositionId::new(self.site_id, counter);
        
        // Find position IDs to insert between
        let (prev_id, next_id) = self.find_position_ids(position).await;
        
        // Generate position between prev and next
        let actual_id = match (prev_id, next_id) {
            (Some(prev), Some(next)) => prev.between(&next),
            (Some(prev), None) => PositionId::new(self.site_id, prev.counter + 1000),
            (None, Some(next)) => PositionId::new(self.site_id, next.counter - 1000),
            (None, None) => char_id,
        };
        
        let crdt_char = CRDTChar {
            id: actual_id.clone(),
            character,
            visible: true,
            author,
            timestamp: Utc::now(),
        };
        
        // Insert the character
        self.characters.write().await.insert(actual_id.clone(), crdt_char);
        
        let operation = CRDTOperation::Insert {
            id: actual_id,
            character,
            author,
            timestamp: Utc::now(),
        };
        
        // Notify subscribers
        self.notify_subscribers(&operation).await;
        
        operation
    }
    
    pub async fn delete_char(&self, position: usize, author: UserId) -> Option<CRDTOperation> {
        let char_id = self.find_char_id_at_position(position).await?;
        
        // Mark as deleted (tombstone)
        if let Some(mut crdt_char) = self.characters.write().await.get_mut(&char_id) {
            crdt_char.visible = false;
            self.tombstones.write().await.insert(char_id.clone());
            
            let operation = CRDTOperation::Delete {
                id: char_id,
                author,
                timestamp: Utc::now(),
            };
            
            // Notify subscribers
            self.notify_subscribers(&operation).await;
            
            Some(operation)
        } else {
            None
        }
    }
    
    pub async fn apply_remote_operation(&self, operation: CRDTOperation) -> Result<(), CRDTError> {
        match operation {
            CRDTOperation::Insert { id, character, author, timestamp } => {
                let crdt_char = CRDTChar {
                    id: id.clone(),
                    character,
                    visible: true,
                    author,
                    timestamp,
                };
                
                self.characters.write().await.insert(id, crdt_char);
            }
            
            CRDTOperation::Delete { id, .. } => {
                if let Some(mut crdt_char) = self.characters.write().await.get_mut(&id) {
                    crdt_char.visible = false;
                    self.tombstones.write().await.insert(id);
                }
            }
        }
        
        // Notify subscribers of remote change
        self.notify_subscribers(&operation).await;
        
        Ok(())
    }
    
    pub async fn get_text(&self) -> String {
        let characters = self.characters.read().await;
        
        characters
            .values()
            .filter(|c| c.visible)
            .map(|c| c.character)
            .collect()
    }
    
    async fn find_position_ids(&self, position: usize) -> (Option<PositionId>, Option<PositionId>) {
        let characters = self.characters.read().await;
        let visible_chars: Vec<_> = characters
            .iter()
            .filter(|(_, c)| c.visible)
            .collect();
        
        let prev_id = if position > 0 {
            visible_chars.get(position - 1).map(|(id, _)| (*id).clone())
        } else {
            None
        };
        
        let next_id = visible_chars.get(position).map(|(id, _)| (*id).clone());
        
        (prev_id, next_id)
    }
    
    async fn notify_subscribers(&self, operation: &CRDTOperation) {
        let subscribers = self.subscribers.read().await;
        for subscriber in subscribers.iter() {
            if let Err(e) = subscriber.on_operation(operation).await {
                log::error!("CRDT subscriber error: {}", e);
            }
        }
    }
}

// CRDT operations for synchronization
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CRDTOperation {
    Insert {
        id: PositionId,
        character: char,
        author: UserId,
        timestamp: DateTime<Utc>,
    },
    Delete {
        id: PositionId,
        author: UserId,
        timestamp: DateTime<Utc>,
    },
}

// Collaborative document manager
pub struct CollaborativeDocumentManager {
    documents: Arc<RwLock<HashMap<DocumentId, CRDTDocument>>>,
    network_sync: Arc<NetworkSynchronizer>,
    conflict_resolver: Arc<ConflictResolver>,
}

impl CollaborativeDocumentManager {
    pub async fn create_document(&self, document_id: DocumentId, site_id: u32) -> Result<(), CollabError> {
        let crdt_doc = CRDTDocument::new(site_id);
        
        // Set up network synchronization
        let network_sync = self.network_sync.clone();
        let doc_id = document_id;
        
        crdt_doc.subscribe(Box::new(move |operation| {
            let network_sync = network_sync.clone();
            let doc_id = doc_id;
            Box::pin(async move {
                network_sync.broadcast_operation(doc_id, operation).await
            })
        })).await;
        
        self.documents.write().await.insert(document_id, crdt_doc);
        
        Ok(())
    }
    
    pub async fn handle_remote_operation(
        &self,
        document_id: DocumentId,
        operation: CRDTOperation
    ) -> Result<(), CollabError> {
        if let Some(document) = self.documents.read().await.get(&document_id) {
            document.apply_remote_operation(operation).await?;
        }
        
        Ok(())
    }
}
```

#### Operational Transformation for Rich Text
```rust
// Operational Transformation for more complex text operations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TextOperation {
    Retain { count: usize },
    Insert { text: String },
    Delete { count: usize },
}

impl TextOperation {
    // Transform operation against another operation
    pub fn transform(&self, other: &TextOperation, priority: Priority) -> (TextOperation, TextOperation) {
        match (self, other) {
            (TextOperation::Insert { text: text1 }, TextOperation::Insert { text: text2 }) => {
                match priority {
                    Priority::Left => (
                        self.clone(),
                        TextOperation::Retain { count: text1.len() }.compose(&other)
                    ),
                    Priority::Right => (
                        TextOperation::Retain { count: text2.len() }.compose(&self),
                        other.clone()
                    ),
                }
            }
            
            (TextOperation::Insert { text }, TextOperation::Delete { count }) => {
                (self.clone(), TextOperation::Retain { count: text.len() }.compose(&other))
            }
            
            (TextOperation::Delete { count }, TextOperation::Insert { text }) => {
                (TextOperation::Retain { count: text.len() }.compose(&self), other.clone())
            }
            
            // ... handle other combinations
            _ => (self.clone(), other.clone()),
        }
    }
    
    fn compose(&self, other: &TextOperation) -> TextOperation {
        // Compose two operations into one
        match (self, other) {
            (TextOperation::Retain { count: c1 }, TextOperation::Retain { count: c2 }) => {
                TextOperation::Retain { count: c1 + c2 }
            }
            (TextOperation::Insert { text: t1 }, TextOperation::Insert { text: t2 }) => {
                TextOperation::Insert { text: format!("{}{}", t1, t2) }
            }
            _ => other.clone(), // Simplified
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Priority {
    Left,
    Right,
}

// Operational transformation engine
pub struct OTEngine {
    document_state: Arc<RwLock<String>>,
    pending_operations: Arc<RwLock<Vec<TextOperation>>>,
    applied_operations: Arc<RwLock<Vec<TextOperation>>>,
}

impl OTEngine {
    pub async fn apply_local_operation(&self, operation: TextOperation) -> Result<(), OTError> {
        // Apply operation locally
        self.apply_operation_to_document(&operation).await?;
        
        // Transform against pending remote operations
        let transformed_op = self.transform_against_pending(&operation).await?;
        
        // Send to other clients
        self.broadcast_operation(transformed_op).await?;
        
        Ok(())
    }
    
    pub async fn apply_remote_operation(&self, operation: TextOperation) -> Result<(), OTError> {
        // Transform against local pending operations
        let transformed_op = self.transform_against_local(&operation).await?;
        
        // Apply to document
        self.apply_operation_to_document(&transformed_op).await?;
        
        Ok(())
    }
    
    async fn transform_against_pending(&self, operation: &TextOperation) -> Result<TextOperation, OTError> {
        let pending = self.pending_operations.read().await;
        let mut transformed = operation.clone();
        
        for pending_op in pending.iter() {
            let (new_transformed, _) = transformed.transform(pending_op, Priority::Left);
            transformed = new_transformed;
        }
        
        Ok(transformed)
    }
}
```

### 5. Immutable State with Structural Sharing

**Pattern: Persistent Data Structures for Efficient State Updates**

Immutable state with structural sharing provides efficient updates while maintaining immutability, enabling features like time travel and easy state comparison.

#### Immutable Document Representation
```rust
use im::{HashMap, Vector}; // Immutable data structures

// Immutable document state
#[derive(Debug, Clone, PartialEq)]
pub struct ImmutableDocument {
    content: Vector<char>,
    metadata: HashMap<String, String>,
    version: u64,
    checksum: u64,
}

impl ImmutableDocument {
    pub fn new(content: String) -> Self {
        let chars: Vector<char> = content.chars().collect();
        let checksum = Self::calculate_checksum(&chars);
        
        ImmutableDocument {
            content: chars,
            metadata: HashMap::new(),
            version: 1,
            checksum,
        }
    }
    
    pub fn insert_text(&self, position: usize, text: &str) -> Self {
        let mut new_content = self.content.clone();
        
        for (i, ch) in text.chars().enumerate() {
            new_content.insert(position + i, ch);
        }
        
        let checksum = Self::calculate_checksum(&new_content);
        
        ImmutableDocument {
            content: new_content,
            metadata: self.metadata.clone(),
            version: self.version + 1,
            checksum,
        }
    }
    
    pub fn delete_range(&self, start: usize, end: usize) -> Self {
        let mut new_content = self.content.clone();
        
        for _ in start..end {
            if start < new_content.len() {
                new_content.remove(start);
            }
        }
        
        let checksum = Self::calculate_checksum(&new_content);
        
        ImmutableDocument {
            content: new_content,
            metadata: self.metadata.clone(),
            version: self.version + 1,
            checksum,
        }
    }
    
    pub fn set_metadata(&self, key: String, value: String) -> Self {
        let mut new_metadata = self.metadata.clone();
        new_metadata.insert(key, value);
        
        ImmutableDocument {
            content: self.content.clone(),
            metadata: new_metadata,
            version: self.version + 1,
            checksum: self.checksum,
        }
    }
    
    pub fn get_text(&self) -> String {
        self.content.iter().collect()
    }
    
    pub fn get_line(&self, line_number: usize) -> Option<String> {
        let text = self.get_text();
        text.lines().nth(line_number).map(|s| s.to_string())
    }
    
    pub fn line_count(&self) -> usize {
        self.get_text().lines().count()
    }
    
    fn calculate_checksum(content: &Vector<char>) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        for ch in content.iter() {
            ch.hash(&mut hasher);
        }
        hasher.finish()
    }
}

// Immutable state store with time travel
pub struct ImmutableStateStore {
    current_state: Arc<RwLock<IDEState>>,
    state_history: Arc<RwLock<Vector<IDEState>>>,
    max_history_size: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IDEState {
    documents: HashMap<DocumentId, ImmutableDocument>,
    active_document: Option<DocumentId>,
    workspace_path: Option<PathBuf>,
    ui_state: UIState,
    timestamp: DateTime<Utc>,
}

impl ImmutableStateStore {
    pub fn new(max_history_size: usize) -> Self {
        let initial_state = IDEState {
            documents: HashMap::new(),
            active_document: None,
            workspace_path: None,
            ui_state: UIState::default(),
            timestamp: Utc::now(),
        };
        
        ImmutableStateStore {
            current_state: Arc::new(RwLock::new(initial_state.clone())),
            state_history: Arc::new(RwLock::new(Vector::unit(initial_state))),
            max_history_size,
        }
    }
    
    pub async fn update_state<F>(&self, updater: F) -> Result<IDEState, StateError>
    where
        F: FnOnce(&IDEState) -> IDEState,
    {
        let new_state = {
            let current = self.current_state.read().await;
            let mut updated = updater(&*current);
            updated.timestamp = Utc::now();
            updated
        };
        
        // Update current state
        *self.current_state.write().await = new_state.clone();
        
        // Add to history
        {
            let mut history = self.state_history.write().await;
            history.push_back(new_state.clone());
            
            // Limit history size
            if history.len() > self.max_history_size {
                history.pop_front();
            }
        }
        
        Ok(new_state)
    }
    
    pub async fn get_current_state(&self) -> IDEState {
        self.current_state.read().await.clone()
    }
    
    pub async fn get_state_at_version(&self, version: usize) -> Option<IDEState> {
        let history = self.state_history.read().await;
        history.get(version).cloned()
    }
    
    pub async fn revert_to_version(&self, version: usize) -> Result<IDEState, StateError> {
        let target_state = {
            let history = self.state_history.read().await;
            history.get(version)
                .cloned()
                .ok_or(StateError::VersionNotFound(version))?
        };
        
        *self.current_state.write().await = target_state.clone();
        
        Ok(target_state)
    }
    
    // Efficient diff calculation between states
    pub async fn calculate_diff(&self, from_version: usize, to_version: usize) -> Result<StateDiff, StateError> {
        let history = self.state_history.read().await;
        
        let from_state = history.get(from_version)
            .ok_or(StateError::VersionNotFound(from_version))?;
        let to_state = history.get(to_version)
            .ok_or(StateError::VersionNotFound(to_version))?;
        
        Ok(StateDiff::calculate(from_state, to_state))
    }
}

// Efficient diff calculation for immutable structures
#[derive(Debug, Clone)]
pub struct StateDiff {
    document_changes: HashMap<DocumentId, DocumentDiff>,
    active_document_changed: bool,
    workspace_path_changed: bool,
    ui_state_changed: bool,
}

impl StateDiff {
    pub fn calculate(from: &IDEState, to: &IDEState) -> Self {
        let mut document_changes = HashMap::new();
        
        // Find changed documents
        for (doc_id, to_doc) in &to.documents {
            if let Some(from_doc) = from.documents.get(doc_id) {
                if from_doc != to_doc {
                    document_changes.insert(*doc_id, DocumentDiff::calculate(from_doc, to_doc));
                }
            } else {
                document_changes.insert(*doc_id, DocumentDiff::Added(to_doc.clone()));
            }
        }
        
        // Find removed documents
        for (doc_id, from_doc) in &from.documents {
            if !to.documents.contains_key(doc_id) {
                document_changes.insert(*doc_id, DocumentDiff::Removed(from_doc.clone()));
            }
        }
        
        StateDiff {
            document_changes,
            active_document_changed: from.active_document != to.active_document,
            workspace_path_changed: from.workspace_path != to.workspace_path,
            ui_state_changed: from.ui_state != to.ui_state,
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.document_changes.is_empty() &&
        !self.active_document_changed &&
        !self.workspace_path_changed &&
        !self.ui_state_changed
    }
}

#[derive(Debug, Clone)]
pub enum DocumentDiff {
    Added(ImmutableDocument),
    Removed(ImmutableDocument),
    Modified {
        from: ImmutableDocument,
        to: ImmutableDocument,
        text_changes: Vec<TextChange>,
    },
}

impl DocumentDiff {
    pub fn calculate(from: &ImmutableDocument, to: &ImmutableDocument) -> Self {
        if from.checksum == to.checksum {
            // Documents are identical
            return DocumentDiff::Modified {
                from: from.clone(),
                to: to.clone(),
                text_changes: Vec::new(),
            };
        }
        
        // Calculate text differences using Myers algorithm or similar
        let text_changes = calculate_text_diff(&from.get_text(), &to.get_text());
        
        DocumentDiff::Modified {
            from: from.clone(),
            to: to.clone(),
            text_changes,
        }
    }
}

// Text diff calculation (simplified Myers algorithm)
fn calculate_text_diff(from: &str, to: &str) -> Vec<TextChange> {
    // This is a simplified implementation
    // In practice, you'd use a more sophisticated diff algorithm
    let from_lines: Vec<&str> = from.lines().collect();
    let to_lines: Vec<&str> = to.lines().collect();
    
    let mut changes = Vec::new();
    
    // Simple line-by-line comparison
    let max_len = from_lines.len().max(to_lines.len());
    
    for i in 0..max_len {
        match (from_lines.get(i), to_lines.get(i)) {
            (Some(from_line), Some(to_line)) => {
                if from_line != to_line {
                    changes.push(TextChange::Modified {
                        line: i,
                        from: from_line.to_string(),
                        to: to_line.to_string(),
                    });
                }
            }
            (Some(from_line), None) => {
                changes.push(TextChange::Deleted {
                    line: i,
                    content: from_line.to_string(),
                });
            }
            (None, Some(to_line)) => {
                changes.push(TextChange::Added {
                    line: i,
                    content: to_line.to_string(),
                });
            }
            (None, None) => break,
        }
    }
    
    changes
}

#[derive(Debug, Clone)]
pub enum TextChange {
    Added { line: usize, content: String },
    Deleted { line: usize, content: String },
    Modified { line: usize, from: String, to: String },
}
```

**Benefits of Immutable State Patterns:**
- Efficient structural sharing reduces memory usage
- Easy state comparison and diff calculation
- Natural support for undo/redo and time travel
- Thread-safe by default (immutable data)
- Simplified reasoning about state changes

**Implementation Considerations:**
- Memory overhead from persistent data structures
- Learning curve for developers unfamiliar with immutable patterns
- Potential performance impact for large documents
- Need for efficient diff algorithms for UI updates

This comprehensive analysis of state management patterns provides the foundation for implementing robust, scalable state management in the Kiro Rust/WASM implementation, with support for collaboration, undo/redo, and efficient UI updates.
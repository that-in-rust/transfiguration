# A010: User Journey 01 - Parseltongue (Claude Code Evolution)

## Primary User Persona

### "The Rust Developer Joining an Existing Codebase"

**Profile:**
- **Experience**: Solid Rust developer (comfortable with ownership, lifetimes, traits)
- **Context**: New to a specific codebase (5,000+ lines of Rust code)
- **Three main scenarios**:
  A. **Issue Resolution**: Debugging and fixing problems in unfamiliar code
  B. **Feature Development**: Adding new functionality to existing architecture
  C. **Refactoring**: Improving code structure, performance, or maintainability

**Key Pain Points:**
- Understanding complex module interactions and dependency chains
- Navigating unfamiliar architectural patterns
- Identifying the "right way" to make changes without breaking existing invariants
- Mental overhead of building context in a large, established codebase
- Risk-averse changes (fear of introducing regressions)

**Success Metrics:**
- Time to first productive contribution
- Confidence in making changes without breaking things
- Ability to understand codebase architecture and patterns quickly
- Quality of contributions (bug-free, idiomatic Rust)

## L1 PRD: Parseltongue Core User Journey

1. User arrives at the codebase via claude-code fork called Parseltongue
2. Interface Signature Graph is created in RAM as ISG_current
    - each node is an interface signature with a unique identifier
        - the identifier is filePath-fileName-InterfaceName where InterfaceName is defined such a way that it remains unique
    - any interface is a limited set of things but ALL of them should be at first level of distance from filename, which means interfaces which are inside other interfaces are not of relevant concern
    - the interface signature is enriched with meta-data from rust analyzer
        - might include HIR information
        - might include any information that helps understand what is the utility of this interface
        - might include any dependencies or etc. analytics related to its relationships with other interfaces
        - classification label of whether it is a test interface used for TDD or a normal interface
    - a persistent copy is created in a relevant graph database maybe CosData
    - a persistent copy is created in a JSON file
    - a visualization in HTML based on ISG which can help the user understand
        - control flow of codebase
        - overall structure of code
3. The codebase is copied into SQLlite database
    - Table Codebase has following columns
        - id
        - ISG_current_ind (0 or 1)
        - ISG_future_ind (0 or 1)
        - filePath-fileName-interfaceName as unique identifier
        - Current_Code
        - Future_Code (empty at initiation)
        - Future_Action (empty at initiation, but filled with whether to suit the PRD change we will edit or delete or create this new interface)
4. The user is asked to create a PRD
    - the LLM asks the user to refine the PRD in context of ISG_current
    - the PRD is created
5. The LLM is tasked with creating a new ISG_future which does not have a persistent copy based on
    - ISG_current + PRD
    - the LLM is asked if
        - Do we need to revise the PRD?
            - Yes
                - ISG_future is possible or NOT or should we ask for a PRD change based on whether the change is too big or too complicated - especially in terms of risks of different nature
            - No
                - if ISG_future is possible then lets have base value of ISG_current as a default value and then change it according to what you think is the correct logic
                    - what test-interfaces will be deleted, edited, created
                    - what non-test-interfaces will be deleted, edited, created
                - now reflect these ISG_future changes in the SQLlite database in ISG_current_ind, ISG_future_ind, Future_Code and Future_Action columns
                - now use the rubber duck debugging menthod to look at ISG_current + PRD + ISG_future + those rows in SQLlite database which have Future_Code and Future_Action columns as not null
                - if the LLM thinks that we need to refine the solutioning further, make changes to ISG_future and repeat the process
                - if the LLM thinks that we need to refine the PRD further then go back to previous step
                - if finally the LLM feels very confident of the changes, we reflect the changes in the SQLlite database in the codebase
                - now we run all the tests and compile the codebase
                - if the tests fail then we go back to previous step
                - if the tests pass then we show the visualization of changes in ISG to the user + results of compilation + tests + request behavorial confirmation
                - if user gives go ahead then we
                    - make a commit with list of changes
                    - recreate ISG_current from ISG_future; update the SQLlite database according to the current codebase from scratch

## L2 Analysis: Deep Architectural Intelligence

### 🎯 Core Innovation: Dual-State Architecture
This system manages **parallel universes** of code:
- **ISG_current**: Reality (what exists now)
- **ISG_future**: Vision (what should exist)
- **SQLite Bridge**: Transformation mechanism between states

### 🧠 Multi-Loop Refinement System
The workflow implements sophisticated feedback control:

1. **PRD Refinement Loop**: LLM + user co-create requirements in architectural context
2. **Feasibility Assessment Loop**: Risk/benefit analysis before committing changes
3. **Solution Design Loop**: Rubber duck validation against ISG_current + PRD + ISG_future
4. **Testing Loop**: Empirical validation of proposed changes
5. **Human Approval Loop**: Final behavioral confirmation

### 🔥 Multi-Layer Persistence Strategy
- **ISG_current**: In-memory for sub-millisecond queries
- **Graph DB**: Relationship topology (CosData for complex queries)
- **JSON**: Interoperability and backup
- **SQLite**: Source of truth for interface content and state
- **HTML Visualization**: Human-understandable architectural insight

### 💡 Strategic Brilliance: PRD-First Development
Inverted development model:
- **Traditional**: Code → Requirements (retrofit documentation)
- **Parseltongue**: Requirements → Code (intentional architecture)

### ⚡ Intelligent Risk Management
- **Complexity Assessment**: Automated "is this change too big?" analysis
- **Multi-dimensional Risk**: Technical, architectural, dependency risks
- **Test Validation**: Empirical feedback before permanent changes
- **Human Confirmation**: Final approval gate with behavioral verification

### 🎨 Developer Experience: Visualization-Driven
- **Before/After ISG Comparison**: See architectural transformation
- **Control Flow Understanding**: Understand impact of changes
- **Test Results Integration**: Trust through verification
- **Behavioral Confirmation**: Human-in-the-loop validation

### 🔧 Atomic Operations Model
Nothing changes permanently until all gates pass:
1. **Logic Validation**: Rubber duck method verifies internal consistency
2. **Test Results**: Empirical proof of correctness
3. **User Approval**: Human behavioral confirmation
4. **Atomic Commit**: Irreversible but validated changes

### 🚀 Problem-Solution Alignment

**Core Software Development Problems Solved:**
- **Fear of Change**: Unknown consequences eliminated through visualization
- **Complexity Management**: Large codebases made navigable through ISG
- **Requirements Drift**: PRD ↔ Code alignment maintained continuously
- **Testing Anxiety**: Comprehensive validation before integration

### 🎯 User Journey Integration

**Issue Resolution Scenario:**
1. ISG_current → Problem diagnosis
2. PRD creation → Define fix requirements
3. ISG_future → Design solution
4. Validation → Rubber duck + testing
5. Implementation → Atomic changes

**Feature Development Scenario:**
1. PRD → Feature requirements in context
2. ISG_future → Architectural design
3. Validation → Risk assessment + testing
4. Implementation → Safe integration

**Refactoring Scenario:**
1. ISG_current analysis → Understanding current state
2. ISG_future design → Improved architecture
3. Validation → Comprehensive testing
4. Safe transformation → Atomic refactoring

### 🐍 The "Parseltongue" Metaphor
The system speaks both languages fluently:
- **Human Language**: PRD requirements, visualization, confirmation
- **Computer Language**: Interface signatures, graph topology, atomic changes

Creating a reliable bridge between **intention** and **implementation** - the fundamental promise of intelligent development tools.
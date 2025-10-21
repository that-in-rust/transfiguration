**1. Objective**
To create a terminal-based user interface (TUI) that displays a side-by-side, interactive diff of two text files, similar to `git diff` or `vimdiff`.

**2. Core Logic & Data Flow**
The application will follow the standard Ratatui event loop pattern:

1.  **Initialization:**
    - Read the contents of two specified files into memory.
    - Use a diffing algorithm to process the two file contents and generate a structured list of changes (hunks).
    - Initialize the application state, including the diff data and a scroll offset set to zero.
    - Initialize the terminal backend (e.g., `crossterm`) and enter the alternate screen.
2.  **Event Loop:**
    - **Draw:** Render the UI based on the current application state.
    - **Handle Input:** Block and wait for a user event (e.g., a key press).
    - **Update State:** Based on the input, modify the application state (e.g., increment the scroll offset).
    - **Loop:** Repeat until a quit condition is met.
3.  **Cleanup:**
    - Restore the terminal to its original state.

**3. Application State (`App` Struct)**
The application's state will be the single source of truth, contained within a single `struct`. It must include:

- `diff_hunks`: A `Vec` of a custom struct (e.g., `DiffLine`) that represents each line of the diff. This struct should contain:
  - `tag`: An enum indicating if the line is `Unchanged`, `Added`, or `Removed`.
  - `content_left`: An `Option<String>` for the content of the left pane.
  - `content_right`: An `Option<String>` for the content of the right pane.
- `scroll_offset`: A `usize` to track the current vertical scroll position.
- `should_quit`: A `bool` to signal that the event loop should terminate.

**4. UI Rendering Logic**
The `ui` function will be responsible for drawing the entire interface.

- **Layout:** The screen will be divided into three vertical sections using `ratatui::layout::Layout`:
  - **Left Pane (45%):** Displays the original file.
  - **Gutter (Min 3):** A narrow center column to display `+`, `-`, or a space.
  - **Right Pane (45%):** Displays the modified file.
- **Rendering Engine:**
  - The function will iterate through the `diff_hunks` in the `App` state.
  - It will use the `scroll_offset` to determine the starting index for rendering, creating a "viewport" into the full diff.
  - For each visible `DiffLine`, it will render the content in the appropriate pane(s).
  - **Styling:** The `Style` of the text will be determined by the `tag`:
    - `Removed` lines: Red foreground.
    - `Added` lines: Green foreground.
    - `Unchanged` lines: Default color.
  - The rendering will be done by creating `Paragraph` widgets for each line and placing them at the correct `(x, y)` coordinates within the layout chunks.

**5. Event Handling**
The event loop will listen for keyboard events from `crossterm`.

- `KeyCode::Up` or `KeyCode::Char('k')`: Decrement `scroll_offset` (but not below 0).
- `KeyCode::Down` or `KeyCode::Char('j')`: Increment `scroll_offset` (but not past the end of the diff).
- `KeyCode::Char('q')`: Set `should_quit` to `true`.

---

### Prompt to Generate the Code

You can use the following prompt with a code-generation AI to produce the Rust program based on the design above.

**Prompt:**

"Generate a complete and runnable Rust program for an interactive, side-by-side file diff viewer using the Ratatui library.

**Objective:**
Create a TUI application that compares two hardcoded files (`file_a.txt` and `file_b.txt`) and displays the differences in a scrollable, three-pane layout.

**Technical Requirements:**

1.  **Dependencies:** Use the `ratatui`, `crossterm`, and `similar` crates. Provide the necessary `Cargo.toml` entries.

2.  **Application State (`App` Struct):**

    - Define a struct `App` to hold the application's state.
    - It must contain a `Vec<DiffLine>` where `DiffLine` is a struct with fields: `tag` (an enum from the `similar` crate), `content_left` (Option<String>), and `content_right` (Option<String>).
    - It must also contain a `scroll_offset: usize` and a `should_quit: bool`.

3.  **Initialization:**

    - In the `main` function, create an instance of the `App`.
    - Use the `similar` crate to read `file_a.txt` and `file_b.txt` and populate the `Vec<DiffLine>` in the `App` struct. Each line from the diff should be converted into your `DiffLine` struct.

4.  **UI Rendering (`ui` function):**

    - Create a function `ui(frame: &mut Frame, app: &App)`.
    - Inside, create a three-column horizontal layout: a left pane, a center gutter, and a right pane.
    - Iterate through the `app.diff_hunks`, starting from `app.scroll_offset`.
    - For each line, render its content in the left or right pane based on the `DiffLine`'s fields.
    - Apply styling: red for removed lines, green for added lines.
    - Render a `+`, `-`, or space in the gutter corresponding to the line's change tag.
    - Ensure that only lines visible within the terminal's height are rendered.

5.  **Event Loop:**

    - Implement the main event loop that continuously draws the UI and polls for events.
    - Handle the following key presses:
      - `j` or `Down`: Increment `app.scroll_offset`.
      - `k` or `Up`: Decrement `app.scroll_offset`.
      - `q`: Set `app.should_quit` to true to exit the loop.
    - Ensure the `scroll_offset` is clamped to valid bounds.

6.  **Boilerplate:**
    - Include all necessary setup and cleanup for the `crossterm` terminal backend (entering alternate screen, enabling raw mode, etc.) to ensure the terminal is restored correctly on exit.

Please provide the full source code for `src/main.rs` and the `Cargo.toml` file."

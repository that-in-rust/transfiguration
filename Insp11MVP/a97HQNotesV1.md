Yes, that logic is not only possible, it's the primary way to programmatically use `rust-analyzer`.

It's a two-phase process. You don't "run `rust-analyzer`" in a loop; you **run it once as a persistent server** and then send it a loop of queries.

---

## Phase 1: Discover All Top-Level Elements

First, you ask `rust-analyzer` for a "table of contents" for the file.

* **API Call:** `textDocument/documentSymbol`
* **Input:** The URI of your file (e.g., `file:///path/to/src/lib.rs`).
* **Output:** A JSON list (or tree) of all symbols in that file.

This list gives you everything you need for your loop. For each top-level item, you'll get:
1.  **`name`**: The item's name (e.g., `parse_cfg`).
2.  **`kind`**: The type of item (e.g., `Function`, `Struct`, `Trait`).
3.  **`selectionRange`**: The exact **position** (line and column) of the item's name. This is the "key" you'll use in the next step.

---

## Phase 2: Loop and Extract Metadata

Now you iterate over the list you just received. For *each symbol*, you use its **position** to send a batch of new LSP requests to extract every bit of metadata.

Here's the data you can get for each item at its `position`:

* **Doc Comments & Signature (`textDocument/hover`)**
    This is your richest summary. It returns the full function signature with all types inferred, plus all the `///` doc comments.

* **All Project-Wide Usages (`textDocument/references`)**
    This returns an aggregated list of every single location in your entire codebase that uses this item.

* **Call Hierarchy (`textDocument/callHierarchy`)**
    If the item is a function, this is a powerful aggregate query. You can ask for:
    * **`callHierarchy/incomingCalls`**: A list of all functions that call *this* function.
    * **`callHierarchy/outgoingCalls`**: A list of all functions that *this* function calls.

* **Implementation Details (`textDocument/implementation`)**
    If the item is a `Trait`, this will return a list of all `structs`/`enums` that implement it.

* **Type Information (`textDocument/typeDefinition`)**
    For a function, this will find the definition of its *return type*. For a struct, it just points to itself.

So, your generator's logic would look like this:

1.  Start the `rust-analyzer` LSP server.
2.  Send one `textDocument/documentSymbol` request for `your_file.rs`.
3.  Receive the `SymbolList`.
4.  **Loop** through each `symbol` in `SymbolList`:
    * Get `symbol.position`.
    * Send `textDocument/hover` with `position` -> Get docs/signature.
    * Send `textDocument/references` with `position` -> Get all usages.
    * If `symbol.kind == "Function"`, send `textDocument/callHierarchy` -> Get callers/callees.
    * ...etc.
5.  Aggregate all this metadata.

Can I run rust-analyzer in loop on all elements which are top level in a file- is such a logic possible - and extract every bit of meta info that rust analyzer can provide


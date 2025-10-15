Core Parseltongue Loop

- The LLM is tasked with creating a new ISG_future which does not have a persistent copy based on
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


Of course. Here is the expanded summary with the requested level of detail for the Parseltongue loop, integrated into the Minto Pyramid structure.

### **Essence: The Governing Idea**

The central insight of the conversation is the design of a sophisticated, AI-assisted framework in Rust, named "Parseltongue," for ingesting, analyzing, and evolving complex codebases and knowledge libraries. The architecture's core principle is to achieve high efficiency, accuracy, and creative potential by having a Large Language Model (LLM) reason over a structured, graph-based representation of the knowledge—the Interface Signature Graph (ISG)—rather than raw code. This dramatically reduces the LLM's context window, making the entire process faster, cheaper, and more precise.

---

### **Key Ideas: The Supporting Arguments**

The governing idea is supported by four key pillars that evolved throughout the conversation:

* **Hypothesis 1: A Unified, Graph-Centric Architecture is Superior.**
    The discussion concludes that the most effective architecture uses a single, unified database that can handle both graph relationships and relational data. This avoids the complexity of managing multiple databases for different types of information.

* **Hypothesis 2: Granular, Semantic Representation is Key.**
    To enable meaningful AI reasoning, the system must move beyond simple text chunking. The conversation proposes parsing code and documents into an "Interface Signature Graph" (ISG), where function signatures or key claims become nodes, and their relationships become edges. This semantic structure is the foundation for efficient analysis.

* **Hypothesis 3: An Iterative, Verifiable Workflow Ensures Reliability.**
    A robust, multi-stage workflow is necessary for evolving a codebase reliably. This involves a detailed, iterative loop of parsing, AI-driven planning based on a Product Requirements Document (PRD), generating a future state, "rubber duck debugging" with the LLM, and validating changes through automated tests and compilation before committing.

* **Hypothesis 4: Minimal-Context Interaction Maximizes LLM Efficiency.**
    The most critical principle discovered is that LLM performance is maximized when its context is surgically focused. Instead of feeding the entire codebase to the AI, the system queries the database for only the small subset of the ISG and code that is changing, reducing context by over 80% and leading to faster, more accurate, and more creative outputs.

---

### **Supporting Details & Alternatives**

Each key idea is built upon a foundation of detailed technical comparisons, tool selections, and process refinements.

* **Supporting the Unified Architecture Hypothesis:**
    * **Database Evolution:** The conversation explored several database types:
        * **KV Stores (`Sled`):** Found to be the fastest for raw ingestion but unsuitable for complex queries.
        * **Vector Databases (`Qdrant`, `CosData`, `OpenSearch`):** Deemed superior for semantic search (RAG) on code patterns, with `Qdrant` initially favored for its native Rust client.
        * **Graph Databases (`CozoDB`):** Ultimately selected as the ideal solution. Written in Rust, it can natively store both the graph structure of the ISG and the relational code blobs in a single, embedded database, queried efficiently with Datalog.

* **Supporting the Semantic Representation Hypothesis:**
    * **Parsing Tools:** The system should use `tree-sitter` for its ability to parse over 100 different programming languages and `syn` for high-fidelity, round-trip parsing of Rust code.
    * **Enrichment:** The parsed graph is enriched with metadata from `rust-analyzer`, providing the LLM with deep semantic context (like High-level Intermediate Representation or HIR) without needing the full source code.
    * **Genericity:** This approach is not limited to code. The same principles can be applied to a personal research library by treating document sections as "interfaces" and citations as "dependencies," turning a passive collection of PDFs and text files into a "second brain".

* **Supporting the Iterative Workflow Hypothesis:**
    * **The Parseltongue Loop:** The proposed workflow is a detailed, multi-step process designed for precision and validation:
        1.  **Ingestion:** The system ingests a codebase and creates the initial state representation, `ISG_current`.
        2.  **Goal Definition:** A goal is defined via a **Product Requirements Document (PRD)**.
        3.  **Future State Generation:** The LLM is tasked with creating a new, transient `ISG_future` based on the `ISG_current` and the PRD.
        4.  **PRD Feasibility Check:** The LLM first assesses the PRD:
            * **Question:** *Do we need to revise the PRD?*
            * **If YES:** The LLM determines if the `ISG_future` is achievable or if the proposed change is too large, complex, or risky. It may suggest a modification to the PRD.
            * **If NO:** The process continues to the detailed planning phase.
        5.  **Detailed Planning & Delta Calculation:**
            * The LLM uses `ISG_current` as a baseline and calculates the necessary changes to create `ISG_future`.
            * It explicitly lists which test-interfaces and non-test-interfaces will be **deleted, edited, or created**.
            * These proposed changes are reflected in specific columns (`ISG_current_ind`, `ISG_future_ind`, `Future_Code`, `Future_Action`) within the database for tracking.
        6.  **Rubber Duck Debugging & Refinement:**
            * The LLM reviews its own plan, examining the `ISG_current`, the PRD, the proposed `ISG_future`, and the specific database rows marked for future action.
            * **Refine Solution:** If the LLM identifies flaws in its logic, it modifies the `ISG_future` and the corresponding database entries, then repeats this debugging step.
            * **Refine PRD:** If the LLM concludes the requirements themselves are flawed, it loops back to step 4.
        7.  **Code Generation & Validation:**
            * Once the LLM is confident in the plan, the changes are written to the actual codebase.
            * The system then automatically **compiles the codebase and runs all associated tests**.
            * **If tests fail:** The system reverts the code changes and loops back to the refinement step (6), using the test failure as new context.
        8.  **User Confirmation & Visualization:**
            * **If tests pass:** The system presents a visualization of the changes between `ISG_current` and `ISG_future` to the user.
            * It also shows the successful compilation and test results.
            * It then requests final behavioral confirmation from the user.
        9.  **Commit & Reset:**
            * **Upon user go-ahead:**
                * A commit is made to the version control system with a detailed list of the changes.
                * The `ISG_future` becomes the new `ISG_current`.
                * The entire database is updated from scratch based on the newly modified codebase to ensure perfect consistency for the next loop.

* **Supporting the Minimal-Context Hypothesis:**
    * **The Core Insight:** The fundamental realization is that an LLM doesn't "read" files; it processes the text given to it. The ISG serves as a highly compressed, structured map of the codebase, allowing for precise queries.
    * **Querying the Delta:** The system is designed to query the database for only the small "delta" between the current and proposed future state. This minimal subset—containing the ISG changes and only the relevant code snippets—is then fed to the LLM for reasoning. This is the primary mechanism for achieving massive gains in efficiency and accuracy.


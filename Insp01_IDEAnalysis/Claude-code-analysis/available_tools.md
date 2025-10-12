Sure — that’s an interesting list! It looks like you’re referring to **capabilities or “rules” used in AI agent systems, LLM sandboxes, or workflow engines** (like OpenDevin, AutoGPT, or Cursor-like toolchains). Each of these corresponds to a specific action or “tool” the agent can invoke to interact with the environment. Let’s break them down one by one 👇

---

### 🧭 File and Command Tools

#### **Glob**

- **Purpose:** Pattern-based file matching.
- **What it does:** Returns a list of files matching a pattern — e.g., `"*.js"` will match all JavaScript files.
- **Example:**

  ```bash
  glob("src/**/*.ts")
  ```

  → Lists all `.ts` files recursively in `src`.

---

#### **Grep**

- **Purpose:** Search for text inside files.
- **What it does:** Finds lines that match a given regex or keyword.
- **Example:**

  ```bash
  grep("TODO", "src/**/*.ts")
  ```

  → Finds all occurrences of “TODO” in `.ts` files under `src`.

---

#### **LS**

- **Purpose:** List directory contents.
- **What it does:** Equivalent of the Unix `ls` command — shows files and directories.
- **Example:**

  ```bash
  ls("src/components")
  ```

  → Lists all files/folders in `src/components`.

---

#### **Read**

- **Purpose:** Read file contents.
- **What it does:** Opens and returns the content of a file.
- **Example:**

  ```bash
  read("package.json")
  ```

  → Returns the content of the `package.json` file as text.

---

### 📒 Data & Knowledge Tools

#### **NotebookRead**

- **Purpose:** Read notebook (like Jupyter `.ipynb`) files.
- **What it does:** Extracts code cells, markdown, or outputs from a notebook file so an agent can understand or modify it.
- **Example:**

  ```bash
  notebook_read("analysis.ipynb")
  ```

  → Returns code, text, and metadata from the notebook.

---

### 🌐 Web & Task Tools

#### **WebFetch**

- **Purpose:** Fetch raw data from a URL.
- **What it does:** Downloads or retrieves content from the web (HTML, JSON, API response, etc.).
- **Example:**

  ```bash
  web_fetch("https://api.github.com/repos/vercel/next.js")
  ```

  → Returns JSON data from the GitHub API.

---

#### **WebSearch**

- **Purpose:** Perform a search query on the web.
- **What it does:** Like a Google/Bing search; returns links and snippets for a keyword or question.
- **Example:**

  ```bash
  web_search("Next.js middleware examples")
  ```

---

#### **TodoWrite**

- **Purpose:** Write or log a TODO/task item.
- **What it does:** Adds a note or reminder to a “to-do” list or project tracker (internal memory or file).
- **Example:**

  ```bash
  todo_write("Refactor config-manager to support YAML")
  ```

---

### ⚙️ Shell / Process Control

#### **KillShell**

- **Purpose:** Terminate a running shell or subprocess.
- **What it does:** Stops a process the agent started earlier (like a runaway command).
- **Example:**

  ```bash
  kill_shell("build_process")
  ```

---

#### **BashOutput**

- **Purpose:** Execute a shell command and capture its output.
- **What it does:** Runs Bash commands and returns stdout/stderr.
- **Example:**

  ```bash
  bash_output("npm run build")
  ```

  → Executes the command and returns the terminal output.

---

### 🧩 Summary Table

| Rule             | Category    | Description                           |
| ---------------- | ----------- | ------------------------------------- |
| **Glob**         | File System | Match files by pattern                |
| **Grep**         | File System | Search text inside files              |
| **LS**           | File System | List directory contents               |
| **Read**         | File System | Read file content                     |
| **NotebookRead** | Data        | Parse `.ipynb` notebooks              |
| **WebFetch**     | Web         | Fetch content from a URL              |
| **WebSearch**    | Web         | Search the web                        |
| **TodoWrite**    | Task        | Add a note or to-do                   |
| **KillShell**    | System      | Terminate a running process           |
| **BashOutput**   | System      | Run a shell command and return output |

---

Would you like me to tell you how these rules **interact** (e.g., how an AI agent chains them together in a workflow)? That would give a good picture of how they’re used in practice.

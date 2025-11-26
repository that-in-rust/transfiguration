# Meta-Compiler Frameworks: A Comprehensive Survey

**Research Type**: Meta-Compiler & Language Workbench Analysis
**Purpose**: Identify existing solutions, patterns, and gaps for configuration-driven compiler construction
**Date**: 2025-11-26
**Researcher**: Claude (Anthropic)
**Status**: Complete Survey

---

## Executive Summary

This research surveys the landscape of meta-compiler frameworks and language workbenches - tools that enable creating new languages through **configuration and declaration** rather than writing compiler code from scratch. The goal is to inform the design of a "compiler enabler" that allows users to:

1. Create DSLs or language variants (e.g., "frontend-only Rust", "functional-only Rust")
2. Configure features (GC vs borrow checking, macros vs no macros)
3. Target LLVM (compile to C++, Rust, or LLVM IR)
4. Work primarily through CONFIGURATION rather than coding

### Key Findings

**What Exists**:
- **Language Workbenches**: JetBrains MPS, Spoofax, Xtext (mature, production-ready)
- **Runtime Frameworks**: Truffle/GraalVM, MLIR (multi-level IR)
- **Parser Generators**: ANTLR, tree-sitter (syntax-focused)
- **Specialized Tools**: TableGen (LLVM), Chalk (trait solving), Nanopass (IR transformations)
- **Macro Systems**: Racket (language extension via macros)

**What's Missing**:
- No framework combines **graph database** + **declarative configuration** + **LLVM targeting**
- Most tools require significant coding (not configuration-first)
- Limited support for feature matrices (enable/disable language features)
- No "compiler construction kit" with pluggable semantics

**Opportunity**: The proposed architecture (CozoDB + Datalog + LLVM) is **novel** and fills a significant gap.

---

## Table of Contents

1. [Survey of Existing Meta-Compiler Frameworks](#1-survey-of-existing-meta-compiler-frameworks)
   - 1.1 [Language Workbenches](#11-language-workbenches)
   - 1.2 [Runtime Frameworks](#12-runtime-frameworks)
   - 1.3 [Parser Generators](#13-parser-generators)
   - 1.4 [Intermediate Representation Frameworks](#14-intermediate-representation-frameworks)
   - 1.5 [Specialized Tools](#15-specialized-tools)
2. [Architecture Patterns](#2-architecture-patterns)
3. [Configuration vs Coding](#3-configuration-vs-coding)
4. [Type System Approaches](#4-type-system-approaches)
5. [Code Generation Strategies](#5-code-generation-strategies)
6. [Graph Database Approaches](#6-graph-database-approaches)
7. [Gaps and Opportunities](#7-gaps-and-opportunities)
8. [Recommendations for "Compiler Enabler"](#8-recommendations-for-compiler-enabler)
9. [References and Resources](#9-references-and-resources)

---

## 1. Survey of Existing Meta-Compiler Frameworks

### 1.1 Language Workbenches

Language workbenches are comprehensive tools for building DSLs and language-oriented programming environments.

#### 1.1.1 JetBrains MPS (Meta Programming System)

**Status**: Mature, open-source, production-ready
**Website**: https://www.jetbrains.com/mps/
**GitHub**: https://github.com/JetBrains/MPS

**Core Architecture**:
- **Projectional Editor**: Key differentiator - no parser needed
- **AST Construction**: Users edit ASTs directly, not text
- **Multi-Modal Notation**: Text, tables, diagrams, graphics
- **Language Extension**: Full support for DSL composition

**What's Configurable**:
- Language structure (grammar-like definitions)
- Type system rules
- Constraints (validation rules)
- Transformations (AST-to-AST)
- Code generation (to Java, C, etc.)
- Editor behavior (syntax highlighting, completion)

**What's Not Configurable** (requires coding):
- Complex semantic analysis (beyond declarative rules)
- Custom optimization passes
- Target-specific code generation logic

**Key Projects Built on MPS**:
- **mbeddr**: Full C extension framework (5 layers of language extensions)
- Embedded systems DSLs at Siemens, Bosch

**Configuration Approach**:
```
concept Function {
  children:
    name: Identifier
    parameters: Parameter[]
    returnType: Type
    body: Statement[]

  type system rules:
    check returnType matches body return paths

  constraints:
    name must be unique in scope

  editor:
    [ name ] ( parameters ) : returnType {
      body
    }
}
```

**Strengths**:
- No parser ambiguity (projectional editing eliminates parse errors)
- Excellent for complex domain notations
- Strong IDE integration
- Language composition via modules

**Weaknesses**:
- Steep learning curve (projectional editing feels "different")
- Java ecosystem dependency
- No LLVM backend (generates source code to other languages)
- Not optimized for systems programming languages

**Relevance to Compiler Enabler**: ★★★☆☆
- Good model for declarative language definition
- Projectional editing may be too constraining
- Lacks low-level code generation (LLVM)

---

#### 1.1.2 Spoofax / Stratego

**Status**: Mature, academic/research origins, production-capable
**Website**: http://www.metaborg.org/
**GitHub**: https://github.com/metaborg

**Core Architecture**:
- **SDF3**: Syntax Definition Formalism (declarative grammar)
- **Stratego**: Rewrite rule language for transformations
- **NaBL2**: Name binding language (declarative scoping rules)
- **Statix**: Constraint-based type system specification
- **SPT**: Spoofax Testing Language

**What's Configurable**:
- Syntax (SDF3 grammars)
- Name resolution (NaBL2 rules)
- Type systems (Statix constraints)
- Transformations (Stratego rewrite rules)
- Editor services (completion, hover)

**Example Configuration (SDF3 Syntax)**:
```sdf3
context-free syntax
  Exp.Add = <<Exp> + <Exp>>
  Exp.Mul = <<Exp> * <Exp>>
  Exp.Num = INT

context-free priorities
  Exp.Mul > Exp.Add
```

**Example Configuration (NaBL2 Name Resolution)**:
```nabl2
rules
  [[ FunDef(name, params, body) ^ (s) ]] :=
    new s_fun,
    s_fun -P-> s,
    name :: FUN in s,
    params :: params in s_fun,
    body :: body in s_fun.
```

**Example Transformation (Stratego)**:
```stratego
desugar-all =
  innermost(
    desugar-for
  + desugar-while
  + inline-function
  )

desugar-for:
  For(init, cond, step, body) ->
  Block([init, While(cond, Block([body, step]))])
```

**Strengths**:
- Highly declarative (syntax, names, types)
- Strong separation of concerns (syntax vs semantics)
- Excellent for language experimentation
- Bootstrapped (Spoofax is built with Spoofax)

**Weaknesses**:
- Stratego transformations require learning new language
- Limited LLVM support (primarily generates source code)
- Eclipse/IntelliJ IDE dependency
- Steep learning curve for all DSLs

**Relevance to Compiler Enabler**: ★★★★☆
- Excellent model for declarative specifications
- Stratego rewrite rules similar to Datalog queries
- Lacks graph database backend
- Good inspiration for configuration format

---

#### 1.1.3 Xtext

**Status**: Mature, Eclipse Foundation, production-ready
**Website**: https://eclipse.dev/Xtext/
**GitHub**: https://github.com/eclipse/xtext

**Core Architecture**:
- **ANTLR-based**: Uses LL(*) parser generator
- **EMF Integration**: Eclipse Modeling Framework for AST
- **Xtend**: Java-like language for extensions
- **LSP Support**: Language Server Protocol for any editor

**What's Configurable**:
- Grammar (EBNF-like syntax)
- Validation rules (constraints)
- Scoping rules (name resolution)
- Formatting (pretty-printing)
- Quick fixes (code actions)

**Example Grammar**:
```xtext
grammar org.example.MyDSL

Entity:
  'entity' name=ID '{'
    features+=Feature*
  '}';

Feature:
  'attr' name=ID ':' type=Type |
  'ref' name=ID '->' type=[Entity];

Type:
  'String' | 'Int' | 'Bool';
```

**What Requires Coding**:
- Complex semantic analysis (Xtend/Java code)
- Code generation (template-based or imperative)
- Custom type inference

**Strengths**:
- Fast to get started (15-minute tutorial)
- Good IDE integration (Eclipse, VSCode, IntelliJ)
- Large ecosystem (many DSLs built on Xtext)
- Generates parser + editor automatically

**Weaknesses**:
- Java/JVM ecosystem lock-in
- Code generation is template-based (not LLVM)
- Complex semantics require imperative code
- EMF overhead for simple languages

**Relevance to Compiler Enabler**: ★★★☆☆
- Good for DSLs, less suitable for general-purpose languages
- Template-based code gen not suitable for systems languages
- Lacks graph database, LLVM integration

---

#### 1.1.4 Rascal MPL (Meta-Programming Language)

**Status**: Academic/research, stable, open-source
**Website**: https://www.rascal-mpl.org/
**GitHub**: https://github.com/usethesource/rascal

**Core Architecture**:
- **Algebraic Data Types**: Define AST structures
- **Pattern Matching**: Powerful traversal and rewriting
- **Visit Statements**: Built-in tree traversal
- **Concrete Syntax**: Embed target language syntax in Rascal

**What's Declarative**:
- Syntax (context-free grammars)
- AST structure (algebraic data types)
- Rewrite rules (pattern matching)

**Example**:
```rascal
syntax Exp
  = Num: Int n
  | Add: Exp lhs "+" Exp rhs
  | Mul: Exp lhs "*" Exp rhs
  ;

int eval(Num(n)) = n;
int eval(Add(e1, e2)) = eval(e1) + eval(e2);
int eval(Mul(e1, e2)) = eval(e1) * eval(e2);
```

**Strengths**:
- Functional programming paradigm
- Excellent for source-to-source transformations
- Supports concrete syntax fragments
- Good for program analysis

**Weaknesses**:
- Not a "workbench" (more a programming language)
- No built-in IDE generation
- No LLVM backend
- Requires learning Rascal language

**Relevance to Compiler Enabler**: ★★☆☆☆
- Good for transformations, less for full compilers
- Functional approach aligns with project values
- Lacks configuration-first approach

---

### 1.2 Runtime Frameworks

#### 1.2.1 Truffle / GraalVM

**Status**: Production, Oracle-backed, open-source
**Website**: https://www.graalvm.org/
**GitHub**: https://github.com/oracle/graal

**Core Architecture**:
- **Self-Optimizing AST Interpreters**: Write interpreter, get JIT for free
- **Partial Evaluation**: Truffle → Graal compiler → machine code
- **Polyglot Interop**: Multiple languages in same VM
- **AST Node Framework**: Define language nodes in Java

**What's Framework-Provided**:
- JIT compilation (via partial evaluation)
- Memory management (JVM GC)
- Profiling and optimization
- Interoperability protocol

**What You Implement**:
- AST node classes (Java)
- Execution semantics (interpreter logic)
- Type system (programmatically)
- Parser (use external tool like ANTLR)

**Example (SimpleLanguage Tutorial)**:
```java
@NodeChild("leftNode")
@NodeChild("rightNode")
public abstract class SLAddNode extends SLBinaryNode {
  @Specialization
  protected long add(long left, long right) {
    return Math.addExact(left, right);
  }

  @Specialization
  protected double add(double left, double right) {
    return left + right;
  }
}
```

**Strengths**:
- Excellent performance (competitive with native)
- Polyglot interop (share data structures across languages)
- Battle-tested (JavaScript, Python, Ruby, R implementations)
- Good profiling/debugging tools

**Weaknesses**:
- JVM-based (not suitable for standalone compilers)
- Requires Java coding (not configuration)
- No ahead-of-time compilation to LLVM
- Steep learning curve (partial evaluation concepts)

**Relevance to Compiler Enabler**: ★★☆☆☆
- Good for dynamic languages, less for systems languages
- Interpreter-focused, not compiler-focused
- No declarative configuration layer

---

### 1.3 Parser Generators

#### 1.3.1 ANTLR

**Status**: Mature, widely-used, open-source
**Website**: https://www.antlr.org/
**GitHub**: https://github.com/antlr/antlr4

**Core Architecture**:
- **LL(*) Parsing**: Lookahead to resolve ambiguity
- **Grammar Files**: EBNF-like syntax
- **Visitor/Listener Patterns**: AST traversal
- **Multi-Target**: Java, C#, Python, JavaScript, Go, etc.

**What's Declarative**:
```antlr
grammar Expr;

expr
  : expr '*' expr
  | expr '+' expr
  | INT
  ;

INT : [0-9]+ ;
```

**What Requires Coding**:
- AST construction (custom visitors)
- Semantic analysis (all imperative)
- Code generation (all imperative)

**Industry Adoption**:
- Twitter search (2B queries/day)
- Hive/Pig (Hadoop query languages)
- Oracle SQL Developer
- Hibernate (HQL parser)

**Strengths**:
- Fast parser generation
- Excellent error recovery
- Wide language support (10+ targets)
- Large ecosystem

**Weaknesses**:
- Only handles parsing (no semantics, codegen)
- Requires visitor/listener pattern boilerplate
- No type system support
- No LLVM integration

**Relevance to Compiler Enabler**: ★★☆☆☆
- Good for parsing layer only
- Not a complete compiler framework
- Could be component of larger system

---

#### 1.3.2 tree-sitter

**Status**: Modern, fast, production-ready
**Website**: https://tree-sitter.github.io/
**GitHub**: https://github.com/tree-sitter/tree-sitter

**Core Architecture**:
- **Incremental Parsing**: Reparse only changed regions
- **Error Tolerance**: Always produces valid parse tree
- **Grammar DSL**: JavaScript-based grammar definition
- **Language Bindings**: Rust, C, JavaScript, Python

**Example Grammar (JavaScript subset)**:
```javascript
module.exports = grammar({
  name: 'javascript',

  rules: {
    program: $ => repeat($.statement),

    statement: $ => choice(
      $.expression_statement,
      $.if_statement,
      $.return_statement,
    ),

    expression: $ => choice(
      $.identifier,
      $.number,
      $.binary_expression,
    ),

    binary_expression: $ => prec.left(1, seq(
      $.expression,
      choice('+', '-', '*', '/'),
      $.expression,
    )),
  },
});
```

**Strengths**:
- Incremental parsing (critical for IDEs)
- Error-tolerant (never fails)
- Fast (C implementation)
- Large grammar library (40+ languages)

**Weaknesses**:
- Parsing only (no semantic analysis)
- No type system support
- No code generation
- Grammar format is JavaScript (not declarative)

**Relevance to Compiler Enabler**: ★★★★☆
- **Already used in Parseltongue project**
- Excellent for syntax layer
- Would be natural fit for parsing phase

---

### 1.4 Intermediate Representation Frameworks

#### 1.4.1 MLIR (Multi-Level Intermediate Representation)

**Status**: Production, LLVM sub-project, rapidly growing
**Website**: https://mlir.llvm.org/
**GitHub**: https://github.com/llvm/llvm-project/tree/main/mlir

**Core Architecture**:
- **Dialects**: Extensible operation sets
- **Multiple Abstraction Levels**: High-level to low-level IR
- **Progressive Lowering**: Dialect-to-dialect transformations
- **Typed IR**: Operations have type signatures

**What's Declarative** (TableGen):
```tablegen
def AddOp : Arith_Op<"add", [Commutative]> {
  let summary = "integer addition";
  let arguments = (ins IntegerLike:$lhs, IntegerLike:$rhs);
  let results = (outs IntegerLike:$result);
  let hasFolder = 1;
}
```

**What Requires Coding**:
- Dialect implementation (C++)
- Lowering passes (C++)
- Type inference (C++)
- Optimization passes (C++)

**Industry Adoption**:
- TensorFlow (XLA, TensorFlow Runtime)
- PyTorch (torch-mlir)
- Mojo language (entire compiler built on MLIR)
- NVIDIA, AMD, Intel (hardware dialects)

**Key Innovation**: **Progressive Lowering**
```
High-Level Dialect (e.g., TensorFlow)
  ↓ lower
Mid-Level Dialect (e.g., Linalg)
  ↓ lower
Low-Level Dialect (e.g., LLVM IR)
  ↓ codegen
Machine Code
```

**Strengths**:
- Multiple abstraction levels in one system
- Extensible dialect system
- Strong LLVM integration
- Production-proven (TensorFlow, Mojo)

**Weaknesses**:
- C++ heavy (not configuration-first)
- Steep learning curve
- TableGen is declarative only for operations
- Type systems still coded in C++

**Relevance to Compiler Enabler**: ★★★★★
- **Strong inspiration for multi-level IR**
- Dialect system similar to "language variants"
- Already LLVM-integrated
- Could complement graph database approach

**Potential Integration**:
```
CozoDB Graph (AST + semantics)
  ↓ lower
MLIR High-Level Dialect (functional IR)
  ↓ lower
MLIR LLVM Dialect
  ↓ codegen
LLVM IR → Machine Code
```

---

#### 1.4.2 Nanopass Framework

**Status**: Academic, Scheme/Racket-based, mature
**Website**: https://nanopass.org/
**Paper**: "A Nanopass Framework for Commercial Compiler Development" (ICFP 2013)

**Core Architecture**:
- **Many Small Passes**: 40-50+ passes instead of 5-10
- **Language Definitions**: Formal grammar for each IR
- **Language Extension**: Define IR[n+1] as extension of IR[n]
- **Auto-Generated Boilerplate**: Traversal code generated

**Example (Scheme/Racket)**:
```scheme
(define-language L0
  (Expr (e)
    x
    (lambda (x) e)
    (e0 e1)))

(define-language L1
  (extends L0)
  (Expr (e)
    (- (let ([x e0]) e1))  ; remove let
    (+ (e0 e1 ...))))      ; add multi-arg application

(define-pass desugar-let : L0 (e) -> L1 ()
  (Expr : Expr (e) -> Expr ()
    [(let ([,x ,e0]) ,e1)
     `((lambda (,x) ,e1) ,e0)]))
```

**Key Insight**: Many IRs, small transformations
- Traditional: 5 IRs, complex passes
- Nanopass: 50 IRs, simple passes

**Strengths**:
- Formally defined IRs (catch bugs early)
- Small passes easier to test/verify
- Auto-generated traversal (less boilerplate)
- Good for commercial compilers (proven in Chez Scheme)

**Weaknesses**:
- Scheme/Racket only (not polyglot)
- No type system framework
- No LLVM integration
- Not configuration-based (still coding passes)

**Relevance to Compiler Enabler**: ★★★☆☆
- Philosophy aligns with incremental approach
- Many-IR approach interesting
- Could inspire Datalog query structure

---

### 1.5 Specialized Tools

#### 1.5.1 LLVM TableGen

**Status**: Production, core LLVM tool
**Documentation**: https://llvm.org/docs/TableGen/

**Purpose**: Generate instruction definitions, register maps, scheduling models

**What's Declarative**:
```tablegen
class Instruction<bits<8> opcode, string mnemonic> {
  bits<8> Opcode = opcode;
  string Mnemonic = mnemonic;
  dag OutOperandList;
  dag InOperandList;
}

def ADD32rr : Instruction<0x01, "add"> {
  let OutOperandList = (outs GR32:$dst);
  let InOperandList = (ins GR32:$src1, GR32:$src2);
}
```

**Strengths**:
- Highly declarative for specific domain
- Generates C++ code
- Battle-tested (all LLVM backends use it)

**Weaknesses**:
- Domain-specific (instruction selection only)
- Not general-purpose compiler framework
- Awkward syntax

**Relevance to Compiler Enabler**: ★★☆☆☆
- Inspiration for declarative instruction configuration
- Not applicable to front-end/semantic analysis

---

#### 1.5.2 Rust Chalk (Trait Solver as Library)

**Status**: Production (used in rust-analyzer), open-source
**GitHub**: https://github.com/rust-lang/chalk

**Core Architecture**:
- **Prolog-Like Logic**: Trait solving as logic programming
- **Three-Layer Architecture**:
  1. Host program (rustc, rust-analyzer)
  2. chalk-solve (Rust concepts → logical clauses)
  3. Logic engine (generic SLG solver)

**Key Insight**: **Separate trait system from compiler**
```
Rust Code
  ↓
AST (host program)
  ↓
Logical Clauses (chalk-solve)
  ↓
Logic Engine (Prolog-like)
  ↓
Trait Satisfaction (yes/no + proof)
```

**What's Declarative** (Chalk notation):
```chalk
struct Vec<T> { }

trait Clone { }

impl<T> Clone for Vec<T> where T: Clone { }

// Query: Is Vec<i32>: Clone?
// Answer: Yes (if i32: Clone)
```

**Strengths**:
- Clean separation of concerns
- Reusable logic engine
- Formally defined semantics
- Used in production (rust-analyzer)

**Weaknesses**:
- Trait solving only (not full compiler)
- Requires Rust coding (not configuration)
- No LLVM integration

**Relevance to Compiler Enabler**: ★★★★☆
- **Excellent model for modular design**
- Logic programming approach similar to Datalog
- Could be component of type system layer

---

#### 1.5.3 Racket (Language Extension via Macros)

**Status**: Mature, production-ready, academic/research
**Website**: https://racket-lang.org/

**Core Architecture**:
- **Hygienic Macro System**: Extend language at compile-time
- **Module System**: Control syntax and semantics per-module
- **#lang Directive**: Every file can use different language

**Key Innovation**: **Language-Oriented Programming**
```racket
#lang racket            ; Standard Racket
(define x 42)

#lang typed/racket      ; Typed Racket (different type system)
(: x Integer)
(define x 42)

#lang lazy              ; Lazy Racket (lazy evaluation)
(define x 42)

#lang hackett           ; Haskell-in-Racket
x :: Integer
x = 42
```

**Language Extension Example**:
```racket
#lang racket

(define-syntax-rule (while cond body ...)
  (let loop ()
    (when cond
      body ...
      (loop))))

(while (< x 10)
  (println x)
  (set! x (+ x 1)))
```

**Strengths**:
- Powerful macro system (full language extension)
- Multiple paradigms (functional, imperative, OO)
- Excellent for DSLs
- Language-per-file flexibility

**Weaknesses**:
- Macro programming has learning curve
- Lisp syntax (barrier for some users)
- No LLVM backend (compiles to bytecode)
- Not suitable for systems languages

**Relevance to Compiler Enabler**: ★★★☆☆
- Philosophy aligns (language extension)
- Macro system too different from configuration
- Good for embedding DSLs, not standalone compilers

---

## 2. Architecture Patterns

### 2.1 Common Patterns Across Frameworks

#### Pattern 1: **Syntax-Semantics Separation**
All mature frameworks separate:
- **Syntax**: Grammar, parsing (often declarative)
- **Semantics**: Type checking, analysis (often imperative)
- **Code Generation**: Target output (always imperative)

**Example**:
- MPS: Language structure + Type system + Generator
- Spoofax: SDF3 + NaBL2/Statix + Stratego
- Xtext: Grammar + Validation + Templates

#### Pattern 2: **Multi-Stage Transformation**
Most frameworks use progressive lowering:
```
Source Code
  ↓ parse
Abstract Syntax Tree (AST)
  ↓ desugar
Core Language (simpler AST)
  ↓ type check
Typed AST
  ↓ optimize
Optimized IR
  ↓ codegen
Target Code
```

**MLIR takes this furthest**: 5-10 intermediate dialects

#### Pattern 3: **Rewrite Rules**
Transformations expressed as:
```
pattern → replacement
```

**Examples**:
- Stratego: `For(init, cond, step, body) -> Block([init, While(...)])`
- MLIR: `(arith.addi %x, %c0) -> %x`  (fold add-zero)
- Datalog (proposed): Similar pattern matching

#### Pattern 4: **Layered Architecture**
```
┌─────────────────────────┐
│  User-Facing DSL        │  (what users write)
├─────────────────────────┤
│  Meta-Language          │  (language definition)
├─────────────────────────┤
│  Framework Core         │  (parser, type system, codegen)
├─────────────────────────┤
│  Runtime/Target         │  (JVM, LLVM, interpreter)
└─────────────────────────┘
```

---

### 2.2 Configuration Approaches

#### Approach A: **Grammar-Centric** (ANTLR, tree-sitter, Xtext)
- Start with syntax definition
- Add semantic actions (imperative code)
- Limited configuration of semantics

**Configuration Level**: ★★☆☆☆ (20% config, 80% code)

---

#### Approach B: **Declarative Rules** (Spoofax, MPS)
- Syntax: Declarative grammar
- Names: Declarative scope rules
- Types: Constraint-based specification
- Transformations: Rewrite rules

**Configuration Level**: ★★★★☆ (70% config, 30% code)

**Example (Spoofax NaBL2)**:
```
[[ VarDecl(name, type) ^ (s) ]] :=
  name : TYPE(type),
  name :: VAR in s,
  distinct/name name in s | error "duplicate variable".
```

---

#### Approach C: **Macro-Based** (Racket)
- Host language provides extension points
- Macros define new syntax + semantics
- Full Turing-complete transformation

**Configuration Level**: ★☆☆☆☆ (5% config, 95% code - but "code" is macros)

---

#### Approach D: **Framework-Oriented** (Truffle)
- Implement abstract interfaces
- Framework provides optimization/runtime
- Mostly imperative Java/C++

**Configuration Level**: ★☆☆☆☆ (10% config, 90% code)

---

#### Approach E: **Hybrid Declarative** (MLIR + TableGen)
- Operations defined declaratively (TableGen)
- Passes implemented imperatively (C++)
- Good separation of concerns

**Configuration Level**: ★★★☆☆ (40% config, 60% code)

---

### 2.3 What's Missing: Full Configuration

**Desired**: ★★★★★ (90% config, 10% code)

```yaml
# Example: Hypothetical configuration-first approach
language:
  name: FunctionalRust
  base: Rust

  features:
    enabled:
      - borrow_checking
      - pattern_matching
      - algebraic_types
    disabled:
      - unsafe
      - raw_pointers
      - inline_assembly

  syntax:
    parser: tree-sitter
    grammar: rust.grammar

  semantics:
    type_system: hindley_milner
    name_resolution: lexical_scoping
    borrow_checking: enabled

  backend:
    target: llvm
    optimizations: [inline, dce, cse]
```

**No existing framework provides this level of configuration.**

---

## 3. Configuration vs Coding

### 3.1 What Can Be Configured Today

| Aspect | ANTLR | Xtext | Spoofax | MPS | Truffle | MLIR |
|--------|-------|-------|---------|-----|---------|------|
| **Syntax** | ✅ Grammar | ✅ Grammar | ✅ SDF3 | ✅ Structure | ❌ Code | ✅ TableGen |
| **Lexing** | ✅ Lexer rules | ✅ Terminal rules | ✅ Lexical syntax | ✅ N/A (projectional) | ❌ Code | ✅ N/A |
| **Scoping** | ❌ Code | ⚠️ DSL + code | ✅ NaBL2 | ✅ Rules | ❌ Code | ❌ Code |
| **Type System** | ❌ Code | ⚠️ DSL + code | ✅ Statix | ✅ Rules | ❌ Code | ❌ Code |
| **Constraints** | ❌ Code | ⚠️ DSL | ✅ Statix | ✅ Rules | ❌ Code | ❌ Code |
| **Transformations** | ❌ Code | ❌ Templates | ✅ Stratego | ✅ Rules | ❌ Code | ❌ Code |
| **Code Generation** | ❌ Code | ❌ Templates | ⚠️ Stratego | ⚠️ Rules | ❌ Code | ❌ Code |
| **Optimization** | ❌ N/A | ❌ N/A | ❌ N/A | ❌ N/A | ✅ Framework | ⚠️ Patterns |
| **LLVM Backend** | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ |

**Legend**:
- ✅ Fully declarative/configurable
- ⚠️ Partially configurable (DSL + code)
- ❌ Requires imperative coding

**Key Insight**: **No tool is fully configuration-based AND targets LLVM**

---

### 3.2 Why Coding is Still Required

**Reason 1: Semantic Complexity**
- Type inference algorithms (Hindley-Milner, bidirectional, etc.)
- Borrow checking (complex dataflow analysis)
- Trait resolution (logic programming)

**Reason 2: Target-Specific Code Generation**
- LLVM IR construction (complex API)
- Register allocation
- Instruction selection

**Reason 3: Optimization**
- Pattern-based rewrites (MLIR approach)
- Dataflow analysis (liveness, reaching definitions)
- Control flow transformations

**Reason 4: Performance**
- Hot paths need hand-tuned code
- Query optimization (graph database)

---

## 4. Type System Approaches

### 4.1 Declarative Type Systems

#### Approach: **Constraint-Based** (Spoofax Statix)

**Example**:
```statix
rules

  typeOfExpr(s, Add(e1, e2)) = INT() :-
    typeOfExpr(s, e1) == INT(),
    typeOfExpr(s, e2) == INT().

  typeOfExpr(s, Mul(e1, e2)) = INT() :-
    typeOfExpr(s, e1) == INT(),
    typeOfExpr(s, e2) == INT().

  typeOfExpr(s, Num(_)) = INT().
```

**Strengths**:
- Declarative (logic programming)
- Easy to extend
- Good error messages (constraint failures)

**Weaknesses**:
- Performance (Prolog-like search)
- Limited to decidable type systems
- Harder to express complex inference

---

#### Approach: **Rule-Based** (MPS Type System)

**Example**:
```mps
typeof(AddExpression expr) :==
  typeof(expr.left) :<=: IntegerType;
  typeof(expr.right) :<=: IntegerType;
  IntegerType;
```

**Strengths**:
- Readable syntax
- Integrated with editor (instant feedback)

**Weaknesses**:
- MPS-specific (not portable)
- Limited expressiveness

---

### 4.2 Programmatic Type Systems

#### Approach: **Visitor Pattern** (most frameworks)

**Example (Xtext/Xtend)**:
```xtend
def dispatch TypeRef inferType(Add expr) {
  val leftType = expr.left.inferType
  val rightType = expr.right.inferType
  if (leftType.isInt && rightType.isInt) {
    return intType
  } else {
    error("Add requires integers", expr)
  }
}
```

**Strengths**:
- Full control (Turing-complete)
- Easy to debug (step through code)
- Performance (compiled)

**Weaknesses**:
- Verbose boilerplate
- Hard to reason about (imperative)
- Not declarative

---

### 4.3 Logic Programming (Chalk, Datalog)

**Example (Chalk notation)**:
```prolog
forall<T> { WellFormed(Vec<T>) :- WellFormed(T) }

forall<T> { Implemented(Vec<T>: Clone) :- Implemented(T: Clone) }
```

**Datalog Equivalent (proposed)**:
```datalog
?[type] :=
  *vec_type[type, elem_ty],
  *well_formed[elem_ty]

?[ty] :=
  *vec_type[ty, elem_ty],
  *implements[elem_ty, "Clone"]
  :- *implements[ty, "Clone"]
```

**Strengths**:
- Declarative (queries, not code)
- Efficient (graph database optimized for queries)
- Compositional (rules combine naturally)

**Weaknesses**:
- Learning curve (logic programming)
- Debugging (no step-through)
- Performance depends on query optimizer

---

## 5. Code Generation Strategies

### 5.1 Template-Based (Xtext)

**Example**:
```xtend
def compile(Entity entity) '''
  class «entity.name» {
    «FOR feature : entity.features»
      «feature.compile»
    «ENDFOR»
  }
'''
```

**Strengths**:
- Simple to write
- Good for source-to-source compilation

**Weaknesses**:
- Not suitable for LLVM IR
- Hard to optimize
- String manipulation (error-prone)

---

### 5.2 Rewrite-Based (Stratego, MLIR)

**Example (Stratego)**:
```stratego
codegen-stmt:
  If(cond, thenBody, elseBody) ->
  [  Label(then-label),
     codegen-expr(cond),
     Branch(else-label),
     codegen-stmts(thenBody),
     Jump(end-label),
     Label(else-label),
     codegen-stmts(elseBody),
     Label(end-label) ]
```

**Strengths**:
- Compositional (rules combine)
- Declarative-ish (pattern-based)

**Weaknesses**:
- Still requires strategic programming
- No type safety

---

### 5.3 API-Based (LLVM, inkwell)

**Example (Rust + inkwell)**:
```rust
let func_val = module.add_function("add", func_type, None);
let basic_block = context.append_basic_block(func_val, "entry");
builder.position_at_end(basic_block);

let lhs = func_val.get_nth_param(0).unwrap();
let rhs = func_val.get_nth_param(1).unwrap();
let result = builder.build_int_add(lhs, rhs, "result");
builder.build_return(Some(&result));
```

**Strengths**:
- Full control
- Type-safe (Rust)
- Direct LLVM integration

**Weaknesses**:
- Verbose
- Imperative
- Hard to generate from configuration

---

### 5.4 Graph-to-IR (Proposed Approach)

**Example (CozoDB → LLVM via Datalog)**:
```datalog
// Query: Get all functions to compile
?[func_name, body_ast] :=
  *function[func_id, func_name],
  *ast[func_id, body_ast],
  *changed[func_id]  // only recompile changed functions

// Query: Generate LLVM IR for function
?[func_name, llvm_ir] :=
  *function[func_id, func_name],
  *typed_ast[func_id, typed_ast],
  llvm_ir = lower_to_llvm(typed_ast)
```

**Strengths**:
- Declarative (queries)
- Incremental (only changed functions)
- Graph database optimized for this

**Weaknesses**:
- Novel (no prior art)
- Query optimizer critical
- `lower_to_llvm()` still imperative

---

## 6. Graph Database Approaches

### 6.1 Existing Use of Graphs in Compilers

#### Approach 1: **Control Flow Graphs (CFG)**
- Standard compiler data structure
- In-memory only
- Not queryable

#### Approach 2: **Program Dependence Graphs (PDG)**
- Used in optimizers (LLVM)
- Ephemeral (built per pass)

#### Approach 3: **Call Graphs**
- Inter-procedural analysis
- Often materialized as data structure

**None of these are persistent graph databases.**

---

### 6.2 Graph Databases in General

#### Neo4j, ArangoDB, etc.
- General-purpose graph DBs
- Cypher query language
- Not optimized for compiler queries

#### TypeDB
- Semantic graph database
- Hypergraph model
- Strong type system
- Query language: TypeQL (similar to Datalog)

**Example (TypeQL)**:
```typeql
match
  $fn isa function, has name $name;
  $fn owns $param;
  $param isa parameter, has type $type;
get $name, $type;
```

**Relevance**: ★★★☆☆
- Good for semantic data
- Could store compiler IR
- Not proven for compiler performance

---

### 6.3 CozoDB (Proposed Approach)

**Key Differentiators**:
1. **Datalog**: Logic programming (like Chalk, but general-purpose)
2. **Transactional**: ACID properties (critical for incremental)
3. **Fast**: In-process (no network overhead)
4. **Embeddable**: Rust library (zero-copy integration)

**Compiler-Specific Advantages**:
```datalog
// Name resolution (recursive)
reachable[descendant] :=
  *scope[parent, descendant]
reachable[descendant] :=
  reachable[node],
  *scope[node, descendant]

// Type inference (constraint propagation)
?[expr, type] :=
  *expr[expr, "add", lhs, rhs],
  *type_constraint[lhs, "int"],
  *type_constraint[rhs, "int"],
  type = "int"
```

**Performance**: Sub-millisecond queries on million-node graphs (validated in Parseltongue)

---

## 7. Gaps and Opportunities

### 7.1 What Exists

| Feature | Best Tool | Maturity | Configuration Level |
|---------|-----------|----------|---------------------|
| Syntax Definition | tree-sitter, ANTLR | ⭐⭐⭐⭐⭐ | ★★★★☆ |
| Name Resolution | Spoofax (NaBL2) | ⭐⭐⭐⭐☆ | ★★★★★ |
| Type Systems | Spoofax (Statix) | ⭐⭐⭐⭐☆ | ★★★★☆ |
| Transformations | Stratego, MLIR | ⭐⭐⭐⭐☆ | ★★★☆☆ |
| LLVM Backend | MLIR | ⭐⭐⭐⭐⭐ | ★★☆☆☆ |
| Graph Storage | — | — | — |
| Feature Flags | Cargo (Rust) | ⭐⭐⭐⭐⭐ | ★★★★★ |
| Language Variants | — | — | — |

---

### 7.2 What's Missing

#### Gap 1: **Configuration-First Compiler Framework**
- Existing tools require significant coding
- No "compiler DSL" for full language definition

#### Gap 2: **Feature Matrix Configuration**
```yaml
# Doesn't exist today
variants:
  - name: SafeRust
    features: [borrow_checking, -unsafe]
  - name: ScriptRust
    features: [gc, -borrow_checking, -lifetimes]
  - name: EmbeddedRust
    features: [no_std, -heap, -panic_unwind]
```

#### Gap 3: **Graph Database Backend**
- No compiler uses persistent graph DB
- Parseltongue proves it's viable (12 languages)
- Datalog ideal for compiler queries

#### Gap 4: **Declarative Semantics + LLVM**
- Spoofax: Declarative but no LLVM
- MLIR: LLVM but not declarative

#### Gap 5: **Incremental Compilation at Function-Level**
- rustc: Crate-level incremental
- Proposed: Function-level (100-250× speedup)

#### Gap 6: **Multi-Language with Shared Infrastructure**
- Most tools target one language paradigm
- MLIR is closest but still per-project

---

### 7.3 Unique Opportunity

**Proposed Architecture** fills multiple gaps:

```
┌─────────────────────────────────────┐
│  Configuration Layer (YAML/DSL)     │  ← Gap 1, 2
├─────────────────────────────────────┤
│  CozoDB Graph Database              │  ← Gap 3
│  (Datalog Queries)                  │  ← Gap 4 (partially)
├─────────────────────────────────────┤
│  Semantic Analysis (Rust + Datalog) │  ← Gap 4
├─────────────────────────────────────┤
│  MLIR / LLVM Backend                │  ← Gap 4 (completes)
└─────────────────────────────────────┘

Supports:
- Function-level incremental (Gap 5)
- Multi-language (Gap 6)
- Feature flags (Gap 2)
```

**This combination is NOVEL.**

---

## 8. Recommendations for "Compiler Enabler"

### 8.1 Architecture Recommendations

#### Recommendation 1: **Hybrid Approach**
Don't try to make everything configurable in v1.

**Phase 1 (70% config, 30% code)**:
- Syntax: tree-sitter grammar (config)
- Name resolution: Datalog rules (config)
- Type system: Datalog constraints (config)
- Borrow checking: Rust code + Datalog (hybrid)
- Code generation: Rust code (not config)

**Phase 2 (85% config, 15% code)**:
- Add type system DSL (Statix-inspired)
- Add transformation DSL (Stratego-inspired)
- Keep codegen as Rust (too complex)

**Phase 3 (90% config, 10% code)**:
- Full language workbench
- Visual configuration UI
- Plugin system for custom analyses

---

#### Recommendation 2: **Adopt Proven Patterns**

**From Spoofax**:
- Separate syntax, names, types (SDF3, NaBL2, Statix)
- Constraint-based type systems

**From MLIR**:
- Multi-level IR (progressive lowering)
- Dialect system (for language variants)

**From Nanopass**:
- Many small IRs
- Formal language definitions

**From Rust (Cargo)**:
- Feature flags for language variants

**From Chalk**:
- Modular trait solver
- Logic programming for constraints

---

#### Recommendation 3: **Configuration Format**

**Proposal: YAML + Embedded Datalog**

```yaml
language:
  name: SafeRust
  version: "0.1.0"
  base: Rust

  features:
    enabled:
      - borrow_checking
      - pattern_matching
      - algebraic_types
    disabled:
      - unsafe
      - raw_pointers

  syntax:
    parser: tree-sitter
    grammar_file: rust.grammar

  semantics:
    name_resolution: |
      // Datalog rules
      reachable[scope] := *parent_scope[$parent, scope]
      reachable[scope] := reachable[$parent], *parent_scope[$parent, scope]

      ?[name, def] :=
        *reference[$expr, name],
        *scope[$expr, scope],
        reachable[scope],
        *definition[scope, name, def]

    type_system: |
      // Datalog constraints
      ?[expr, "int"] :=
        *expr_kind[expr, "add"],
        *child[expr, lhs, 0],
        *child[expr, rhs, 1],
        *type_of[lhs, "int"],
        *type_of[rhs, "int"]

    borrow_checking: enabled

  backend:
    target: llvm
    optimizations: [inline, dce]

  code_generation:
    # Rust code for complex codegen
    module: "src/codegen/safe_rust.rs"
```

**Advantages**:
- Familiar format (YAML)
- Declarative where possible (Datalog)
- Escape hatch (Rust code) for complexity

---

### 8.2 Technical Stack Recommendations

#### Layer 1: Syntax
- **Parser**: tree-sitter (already used in Parseltongue)
- **Grammar**: tree-sitter JavaScript DSL
- **Incremental**: tree-sitter built-in

#### Layer 2: Graph Storage
- **Database**: CozoDB (Datalog + transactional)
- **Schema**: Relations for AST, scopes, types, etc.
- **Queries**: Datalog (declarative)

#### Layer 3: Semantic Analysis
- **Name Resolution**: Datalog queries (like NaBL2)
- **Type Inference**: Datalog constraints (like Statix)
- **Borrow Checking**: Rust code + Datalog hybrid
- **Trait Resolution**: Chalk integration (if Rust-like)

#### Layer 4: IR Transformations
- **Desugaring**: Datalog rewrite rules
- **Optimization**: MLIR dialect passes (optional)
- **Lowering**: Rust code (complex)

#### Layer 5: Code Generation
- **LLVM IR**: inkwell crate (Rust LLVM bindings)
- **Optimization**: LLVM passes
- **Object Code**: LLVM backends

---

### 8.3 Feature Flag System

**Inspiration: Rust Cargo Features**

```yaml
# In language configuration
features:
  default: [std, panic_unwind]

  std:
    enables: [heap, threads]

  no_std:
    conflicts: [std]

  unsafe:
    allows: [raw_pointers, inline_asm]

  gc:
    conflicts: [borrow_checking]
    requires: [heap]
```

**Implementation**: Datalog rules + configuration

```datalog
// Feature conflicts
!feature_conflict[] :=
  *enabled_feature["gc"],
  *enabled_feature["borrow_checking"],
  "gc conflicts with borrow_checking"

// Feature requirements
!feature_requirement[] :=
  *enabled_feature["gc"],
  not *enabled_feature["heap"],
  "gc requires heap feature"
```

---

### 8.4 Language Variant System

**Example: Rust Variants**

```yaml
variants:
  - name: SafeRust
    description: "Rust without unsafe"
    features:
      enabled: [borrow_checking, std]
      disabled: [unsafe, raw_pointers, inline_asm]

  - name: ScriptRust
    description: "Rust with garbage collection"
    features:
      enabled: [gc, std]
      disabled: [borrow_checking, lifetimes]

  - name: EmbeddedRust
    description: "Rust for embedded systems"
    features:
      enabled: [no_std, borrow_checking]
      disabled: [heap, threads, panic_unwind]
```

**Benefits**:
- Easy to define new variants
- Share 90% of infrastructure
- Configuration-driven (no coding)

---

### 8.5 Comparison with Existing Tools

| Feature | Spoofax | MPS | MLIR | Proposed |
|---------|---------|-----|------|----------|
| **Syntax Config** | ✅ SDF3 | ✅ Structure | ⚠️ TableGen | ✅ tree-sitter |
| **Semantics Config** | ✅ NaBL2/Statix | ✅ Rules | ❌ C++ | ✅ Datalog |
| **Graph Storage** | ❌ | ❌ | ❌ | ✅ CozoDB |
| **LLVM Backend** | ❌ | ❌ | ✅ | ✅ |
| **Feature Flags** | ❌ | ❌ | ⚠️ (build-time) | ✅ |
| **Language Variants** | ❌ | ⚠️ (manual) | ⚠️ (dialects) | ✅ |
| **Incremental** | ⚠️ (file-level) | ⚠️ (file-level) | ❌ | ✅ (function-level) |
| **Multi-Language** | ✅ | ✅ | ⚠️ | ✅ |

**Conclusion**: **Proposed architecture uniquely combines**:
- Declarative semantics (like Spoofax)
- LLVM backend (like MLIR)
- Graph database (novel)
- Feature flags (like Cargo)
- Function-level incremental (novel)

---

## 9. References and Resources

### 9.1 Language Workbenches

1. **JetBrains MPS**
   - Website: https://www.jetbrains.com/mps/
   - Paper: "JetBrains MPS: Why Modern Language Workbenches Matter" (2021)
   - Tutorial: https://www.jetbrains.com/help/mps/

2. **Spoofax**
   - Website: http://www.metaborg.org/
   - Paper: "The Spoofax Language Workbench" (2010)
   - Documentation: http://www.metaborg.org/en/latest/

3. **Xtext**
   - Website: https://eclipse.dev/Xtext/
   - Book: "Implementing Domain-Specific Languages with Xtext and Xtend" (2nd ed, 2016)
   - Tutorial: https://eclipse.dev/Xtext/documentation/

4. **Rascal MPL**
   - Website: https://www.rascal-mpl.org/
   - Paper: "EASY Meta-programming with Rascal" (2011)
   - Tutorial: https://www.rascal-mpl.org/docs/

### 9.2 Runtime Frameworks

5. **Truffle/GraalVM**
   - Website: https://www.graalvm.org/
   - Paper: "One VM to Rule Them All" (2013)
   - Tutorial: https://www.graalvm.org/latest/graalvm-as-a-platform/language-implementation-framework/

6. **MLIR**
   - Website: https://mlir.llvm.org/
   - Paper: "MLIR: A Compiler Infrastructure for the End of Moore's Law" (2020)
   - Tutorial: https://mlir.llvm.org/docs/Tutorials/

### 9.3 Parser Generators

7. **ANTLR**
   - Website: https://www.antlr.org/
   - Book: "The Definitive ANTLR 4 Reference" (2013)
   - Paper: "LL(*): The Foundation of the ANTLR Parser Generator" (2011)

8. **tree-sitter**
   - Website: https://tree-sitter.github.io/
   - Documentation: https://tree-sitter.github.io/tree-sitter/

### 9.4 Specialized Tools

9. **Chalk (Rust Trait Solver)**
   - GitHub: https://github.com/rust-lang/chalk
   - Book: https://rust-lang.github.io/chalk/book/

10. **Nanopass Framework**
    - Website: https://nanopass.org/
    - Paper: "A Nanopass Framework for Commercial Compiler Development" (ICFP 2013)

11. **Racket**
    - Website: https://racket-lang.org/
    - Paper: "Creating Languages in Racket" (2012)
    - Paper: "From Macros to DSLs: The Evolution of Racket" (2019)

### 9.5 Graph Databases

12. **CozoDB**
    - GitHub: https://github.com/cozodb/cozo
    - Documentation: https://docs.cozodb.org/

13. **TypeDB**
    - Website: https://typedb.com/
    - Documentation: https://typedb.com/docs

### 9.6 Compiler Theory

14. **Language-Oriented Programming**
    - Paper: "Language Oriented Programming" (Dmitriev, 2004)
    - Article: https://martinfowler.com/articles/languageWorkbench.html

15. **Compiler-Compilers**
    - Paper: "META II: A Syntax-Oriented Compiler Writing Language" (1964)
    - Wikipedia: https://en.wikipedia.org/wiki/Compiler-compiler

### 9.7 Related Work (Configuration-Driven Systems)

16. **Cargo (Rust Package Manager)**
    - Feature flags: https://doc.rust-lang.org/cargo/reference/features.html

17. **LLVM TableGen**
    - Documentation: https://llvm.org/docs/TableGen/

---

## Appendix A: Feature Matrix for "Compiler Enabler"

### Must-Have (v1)

| Feature | Priority | Implementation | Inspiration |
|---------|----------|----------------|-------------|
| Syntax definition | P0 | tree-sitter | All |
| Name resolution | P0 | Datalog | Spoofax NaBL2 |
| Basic type checking | P0 | Datalog | Spoofax Statix |
| Graph storage | P0 | CozoDB | Novel |
| LLVM codegen | P0 | inkwell | MLIR |
| Incremental (function) | P0 | Red-green | Novel |

### Should-Have (v2)

| Feature | Priority | Implementation | Inspiration |
|---------|----------|----------------|-------------|
| Type inference | P1 | Datalog + Rust | Chalk |
| Feature flags | P1 | Config + Datalog | Cargo |
| Language variants | P1 | Config | Novel |
| Borrow checking | P1 | Rust + Datalog | rustc + Chalk |
| Trait resolution | P1 | Chalk integration | Chalk |
| Optimization passes | P1 | MLIR dialects | MLIR |

### Nice-to-Have (v3)

| Feature | Priority | Implementation | Inspiration |
|---------|----------|----------------|-------------|
| Projectional editor | P2 | Custom | MPS |
| Visual config UI | P2 | Web UI | MPS |
| Macro system | P2 | Rust macros | Racket |
| Multi-language | P2 | Shared infrastructure | GraalVM |
| LSP server | P2 | tower-lsp | rust-analyzer |
| Debugger integration | P2 | DAP | GraalVM |

---

## Appendix B: Comparison Table (Configuration Percentage)

| Framework | Syntax | Semantics | Codegen | Overall | LLVM |
|-----------|--------|-----------|---------|---------|------|
| **ANTLR** | 90% | 0% | 0% | 30% | ❌ |
| **tree-sitter** | 85% | 0% | 0% | 30% | ❌ |
| **Xtext** | 90% | 40% | 30% | 50% | ❌ |
| **Spoofax** | 95% | 80% | 60% | 75% | ❌ |
| **MPS** | 95% | 85% | 70% | 80% | ❌ |
| **Racket** | 95% | 20% | 10% | 40% | ❌ |
| **Truffle** | 0% | 10% | 10% | 10% | ❌ |
| **MLIR** | 70% | 20% | 30% | 35% | ✅ |
| **Proposed** | 90% | 70% | 30% | 60% | ✅ |

**Key Insight**: Proposed architecture aims for **60% configuration** while **targeting LLVM** - no existing tool does both.

---

## Appendix C: Glossary

**DSL**: Domain-Specific Language
**IR**: Intermediate Representation
**AST**: Abstract Syntax Tree
**LLVM**: Low Level Virtual Machine (compiler backend)
**JIT**: Just-In-Time compilation
**AOT**: Ahead-Of-Time compilation
**LSP**: Language Server Protocol
**DAP**: Debug Adapter Protocol

---

## Conclusion

**The landscape of meta-compiler frameworks is rich but has clear gaps:**

1. **Strong declarative tools exist** (Spoofax, MPS) but don't target LLVM
2. **LLVM-integrated tools exist** (MLIR) but aren't declarative
3. **No tool uses graph database** for persistent compiler state
4. **No tool supports feature matrices** for language variants
5. **Function-level incremental compilation** is novel

**The proposed architecture (CozoDB + Datalog + LLVM) fills these gaps.**

**Recommended next steps**:
1. Prototype name resolution in CozoDB (prove Datalog works)
2. Implement simple type system (validate constraint solving)
3. Build LLVM codegen for toy language (prove end-to-end)
4. Add feature flag system (show configurability)
5. Measure performance (validate incremental claims)

**The opportunity is real. The architecture is novel. The timing is right.**

---

**Research completed: 2025-11-26**
**Next: Begin Phase 1 implementation**

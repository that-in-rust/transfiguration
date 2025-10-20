# Diagram 1

``` mermaid

flowchart TD
    %% Core Interface Graph & Analysis Tools
    A[interface-graph-builder] --> B[interface-summary-generator]
    A --> C[embedding-index-builder]
    B --> C
    A --> D[isg-graph-store]
    C --> E[vector-index-store]
    
    %% Retrieval & Pattern Analysis
    D --> F[hybrid-retrieval-engine]
    E --> F
    G[pattern-knowledge-base] --> F
    F --> H[context-pack-builder]
    
    %% Code Analysis & Constraints
    I[constraints-overlay-analyzer] --> J[diagnostics-scope-mapper]
    J --> K[blast-radius-estimator]
    K --> L[selective-test-runner]
    
    %% Planning & Future Analysis
    M[prd-refine-assistant] --> N[prd-consistency-checker]
    N --> O[isg-future-planner]
    O --> P[feature-impact-analyzer]
    P --> K
    
    %% Patch Generation & Reasoning
    H --> Q[reasoning-adapter-bridge]
    G --> R[deterministic-patch-engine]
    Q --> R
    R --> S[preflight-safety-gate]
    S --> L
    
    %% Core Infrastructure
    T[local-orchestrator-daemon] --> Q
    U[cozo-db-adapter] --> D
    U --> E
    U --> V[codegraph-write-surface]
    
    %% Application & Integration
    R --> W[git-apply-rollback]
    S --> V
    V --> X[offline-debugging-tui]
    D --> Y[isg-html-visualizer]
    E --> Y
    
    %% Support Tools
    Z[gap-list-reporter] --> D
    Z --> E
    AA[pipeline-runner-cli] --> AB[async-job-orchestrator]
    
    %% Language & Model Support
    AC[rust-isg-generator] --> A
    AD[multi-language-parser] --> A
    AE[ollama-client-kit] --> T
    AF[llm-provider-router] --> Q
    
    %% Processing Utilities
    AG[chunk-strategy-engine] --> AH[embedding-batch-pipeline]
    AH --> C
    AI[citation-extractor-kit] --> Q
    
    %% Validation & Compliance
    AJ[hook-schema-validator] --> AK[pretool-command-validator]
    AK --> AL[posttool-response-processor]
    AM[sdk-compliance-validator] --> AK
    AN[session-store-inspector] --> U
    
    %% User Interfaces
    AO[tui-ollama-shell] --> T
    AO --> X
    
    %% Key Data Flows
    A -.->|ISG data| F
    F -.->|Needed shortlist| H
    H -.->|Context bundle| Q
    Q -.->|LLM response| R
    R -.->|Patch| S
    S -.->|Validation| V
    V -.->|Updated state| A

```


# Diagram 2

``` mermaid

---
config:
  flowchart:
    defaultRenderer: "elk"
    nodeSpacing: 75
    rankSpacing: 75
    wrappingWidth: 150
---
flowchart TD
    %% Core Analysis Pipeline (Central Vertical Flow)
    A[interface-graph-builder] --> B[interface-summary-generator]
    B --> C[embedding-index-builder]
    A --> D[isg-graph-store]
    C --> E[vector-index-store]
    
    D --> F[hybrid-retrieval-engine]
    E --> F
    G[pattern-knowledge-base] --> F
    F --> H[context-pack-builder]
    
    %% Left Analysis Wing
    I[constraints-overlay-analyzer] --> J[diagnostics-scope-mapper]
    J --> K[blast-radius-estimator]
    K --> L[selective-test-runner]
    
    %% Right Planning Wing  
    M[prd-refine-assistant] --> N[prd-consistency-checker]
    N --> O[isg-future-planner]
    O --> P[feature-impact-analyzer]
    P --> K
    
    %% Core Reasoning & Patch Generation
    H --> Q[reasoning-adapter-bridge]
    G --> R[deterministic-patch-engine]
    Q --> R
    R --> S[preflight-safety-gate]
    S --> L
    
    %% Infrastructure Foundation
    T[local-orchestrator-daemon] --> Q
    U[cozo-db-adapter] --> D
    U --> E
    U --> V[codegraph-write-surface]
    
    %% Application Layer
    R --> W[git-apply-rollback]
    S --> V
    V --> X[offline-debugging-tui]
    D --> Y[isg-html-visualizer]
    E --> Y
    
    %% Support & Monitoring
    Z[gap-list-reporter] --> D
    Z --> E
    AA[pipeline-runner-cli] --> AB[async-job-orchestrator]
    
    %% Language & Model Infrastructure
    AC[rust-isg-generator] --> A
    AD[multi-language-parser] --> A
    AE[ollama-client-kit] --> T
    AF[llm-provider-router] --> Q
    
    %% Processing & Validation
    AG[chunk-strategy-engine] --> AH[embedding-batch-pipeline]
    AH --> C
    AI[citation-extractor-kit] --> Q
    
    AJ[hook-schema-validator] --> AK[pretool-command-validator]
    AK --> AL[posttool-response-processor]
    AM[sdk-compliance-validator] --> AK
    AN[session-store-inspector] --> U
    
    %% User Interfaces
    AO[tui-ollama-shell] --> T
    AO --> X
    
    %% Key Data Flow Relationships
    A -.->|ISG data| F
    F -.->|Needed shortlist| H
    H -.->|Context bundle| Q
    Q -.->|LLM response| R
    R -.->|Patch| S
    S -.->|Validation| V
    V -.->|Updated state| A
    
    %% Styling for Clarity
    classDef core fill:#e1f5fe,stroke:#01579b,stroke-width:2px
    classDef analysis fill:#f3e5f5,stroke:#4a148c,stroke-width:2px  
    classDef planning fill:#e8f5e8,stroke:#1b5e20,stroke-width:2px
    classDef infrastructure fill:#fff3e0,stroke:#e65100,stroke-width:2px
    classDef interface fill:#fce4ec,stroke:#880e4f,stroke-width:2px
    
    class A,B,C,D,E,F,G,H,Q,R,S core
    class I,J,K,L analysis
    class M,N,O,P planning
    class T,U,V,W,X,Y,AA,AB,AC,AD,AE,AF,AG,AH,AI,AJ,AK,AL,AM,AN infrastructure
    class Z,AO interface

```
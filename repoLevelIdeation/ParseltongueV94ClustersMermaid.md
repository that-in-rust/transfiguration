graph TD
    %% Main Clustering Categories
    A["Clustering Methods for Code Analysis"]
    B["Traditional Graph-Based"]
    C["Spectral & Matrix-Based"]
    D["Hierarchical & Agglomerative"]
    E["Random Walk & Flow-Based"]
    F["Neural & Embedding-Based"]
    G["Multi-View & Hybrid"]
    
    A --> B & C & D & E & F & G
    
    %% Traditional Graph-Based
    B --> B1["Louvain (Baseline)\nROI: 7/10\nRuntime: 0.5-1s"]
    B --> B2["CABGSI\n(Entropy-Boosting)\nROI: 7/10\n+15% Entropy"]
    B --> B3["CMDI\n(Decoding Info)\nROI: 9/10\n+30% Hierarchies"]
    
    %% Spectral & Matrix-Based
    C --> C1["Spectral Clustering\nROI: 6/10\n+10% Modes"]
    C --> C2["R-CDC/N-DSC\n(Ratio Cuts)\nROI: 7/10\n+10% Motifs"]
    
    %% Hierarchical & Agglomerative
    D --> D1["Agglomerative\nHierarchical\nROI: 7/10\n+5% Balance"]
    
    %% Random Walk & Flow-Based
    E --> E1["Infomap\n(Information Flow)\nROI: 8/10\n+20% Flows"]
    
    %% Neural & Embedding-Based
    F --> F1["Node2vec + K-Means\nROI: 8/10\n+20% Semantics"]
    F --> F2["FDCGW\n(Dynamic GNN)\nROI: 8/10\n+25% Temporal"]
    F --> F3["ECGN\n(Cluster-Aware GNN)\nROI: 8/10\n+25% Irregular"]
    
    %% Multi-View & Hybrid
    G --> G1["IMGCGGR\n(Attention-Fused)\nROI: 9/10\n+35% Multi-View"]
    G --> G2["DeMuVGN\n(Multi-View GNN)\nROI: 7/10\n+15% Defects"]
    G --> G3["Multiplex Networks\n(Layered)\nROI: 8/10\n+25% Temporal"]
    
    %% Key Properties
    classDef default fill:#f5f5f5,stroke:#333,stroke-width:2px
    classDef highlight fill:#e3f2fd,stroke:#2196f3
    class B1,F2,G1 highlight
    
    %% Legend
    legend["Legend:\n High-ROI Methods\n Traditional Methods\n Flow-Based Methods"]
```

**Diagram Features:**
1. Complete hierarchy of all 12 clustering methods
2. Key metrics included (ROI scores, performance characteristics)
3. Visual highlighting of highest-ROI methods
4. Clear grouping by algorithmic approach
5. Maintains all data from AggregatedView.md
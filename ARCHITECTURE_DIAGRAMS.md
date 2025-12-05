# AmpliClust Architecture Diagrams

## System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        AmpliClust CLI                            │
│                       (src/main.rs)                              │
└────────────────────────┬────────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Configuration Layer                           │
│                      (src/config.rs)                             │
│  • Platform selection  • Mode selection  • Parameters            │
└────────────────────────┬────────────────────────────────────────┘
                         │
        ┌────────────────┴────────────────┬────────────────┐
        ▼                                  ▼                ▼
┌──────────────┐              ┌──────────────────┐   ┌──────────┐
│   I/O Layer  │              │  Processing Core │   │  Output  │
│ (src/io/)    │              │                  │   │  Layer   │
│              │              │                  │   │          │
│ • FASTQ      │─────────────▶│  1. Reads        │   │ • FASTA  │
│ • FASTA      │              │  2. Alignment    │   │ • TSV    │
│ • BAM        │              │  3. Clustering   │──▶│ • JSON   │
│              │              │  4. Consensus    │   │ • BAM    │
└──────────────┘              │  5. Metrics      │   └──────────┘
                              │  6. Filtering    │
                              └──────────────────┘
```

## Data Flow - Reference-Guided Mode

```
    FASTQ Input                Reference FASTA
         │                            │
         └────────┬───────────────────┘
                  ▼
         ┌────────────────┐
         │  Load & Parse  │
         │   (src/io/)    │
         └────────┬───────┘
                  ▼
         ┌────────────────┐
         │ Quality Filter │
         │ (src/reads/)   │
         └────────┬───────┘
                  ▼
         ┌─────────────────┐
         │  K-mer Indexing │
         │ (src/alignment/ │
         │     kmer.rs)    │
         └────────┬────────┘
                  ▼
         ┌─────────────────┐
         │ Read Placement  │
         │ (src/alignment/ │
         │  placement.rs)  │
         └────────┬────────┘
                  ▼
         ┌─────────────────┐
         │ Group by Locus  │
         └────────┬────────┘
                  │
         ┌────────┴────────┐
         │                 │
         ▼                 ▼
    ┌────────┐       ┌────────┐
    │ Locus 1│       │ Locus 2│  ...
    └────┬───┘       └────┬───┘
         │                │
         ▼                ▼
    ┌─────────────────────────┐
    │ Variant Detection       │
    │ (src/variants/)         │
    └────────┬────────────────┘
             ▼
    ┌─────────────────────────┐
    │ Distance Matrix         │
    │ (src/clustering/)       │
    └────────┬────────────────┘
             ▼
    ┌─────────────────────────┐
    │ K-means Clustering      │
    │ (src/clustering/        │
    │     kmeans.rs)          │
    └────────┬────────────────┘
             │
        ┌────┴─────┬─────┬─────┐
        ▼          ▼     ▼     ▼
    Cluster 1  Cluster 2 ...  Cluster N
        │          │            │
        └──────────┴────────────┘
                   ▼
         ┌──────────────────┐
         │ Consensus Gen.   │
         │ (src/consensus/) │
         └────────┬─────────┘
                  ▼
         ┌──────────────────┐
         │ Metrics Calc.    │
         │ (src/metrics/)   │
         │ • Diversity      │
         │ • Chimera        │
         │ • Quality        │
         └────────┬─────────┘
                  ▼
         ┌──────────────────┐
         │ Filter Clusters  │
         │ • Frequency      │
         │ • Read count     │
         │ • Chimera score  │
         └────────┬─────────┘
                  │
         ┌────────┴────────┐
         ▼                 ▼
    Passed.fasta     Failed.fasta
    Read_info.txt    Summary.json
```

## Data Flow - De Novo Mode

```
    FASTQ Input
         │
         ▼
    ┌────────────────┐
    │  Load Reads    │
    └────────┬───────┘
             ▼
    ┌────────────────┐
    │ Quality Filter │
    └────────┬───────┘
             ▼
    ┌────────────────────┐
    │ All-vs-All         │
    │ Distance Matrix    │
    │ (Parallel)         │
    └────────┬───────────┘
             ▼
    ┌────────────────────┐
    │ DBSCAN / Hier.     │
    │ Clustering         │
    │ (Auto # clusters)  │
    └────────┬───────────┘
             ▼
         [Continue as
         reference-guided
         from consensus]
```

## Module Dependencies

```
main.rs
  ├─▶ config.rs
  ├─▶ io/
  │   ├─▶ fastq.rs
  │   ├─▶ fasta.rs
  │   ├─▶ bam.rs
  │   └─▶ formats.rs
  ├─▶ reads/
  │   ├─▶ sequence.rs
  │   ├─▶ quality.rs
  │   ├─▶ platform.rs
  │   └─▶ filtering.rs
  ├─▶ alignment/
  │   ├─▶ kmer.rs
  │   ├─▶ placement.rs
  │   ├─▶ edlib.rs
  │   └─▶ minimap.rs
  ├─▶ variants/
  │   ├─▶ detection.rs
  │   ├─▶ filtering.rs
  │   └─▶ graph.rs
  ├─▶ clustering/
  │   ├─▶ kmeans.rs
  │   ├─▶ hierarchical.rs
  │   ├─▶ dbscan.rs
  │   ├─▶ denovo.rs
  │   └─▶ reference_guided.rs
  ├─▶ consensus/
  │   ├─▶ poa.rs
  │   ├─▶ spoa_wrapper.rs
  │   ├─▶ polish.rs
  │   └─▶ quality.rs
  ├─▶ metrics/
  │   ├─▶ diversity.rs
  │   ├─▶ chimera.rs
  │   ├─▶ cluster_stats.rs
  │   └─▶ quality_control.rs
  └─▶ utils/
      ├─▶ parallel.rs
      ├─▶ logging.rs
      └─▶ math.rs
```

## Clustering Algorithm Flow

```
Input: Distance Matrix (N × N)
       K = number of clusters
       Max iterations

┌──────────────────────────┐
│ 1. Initialize            │
│    Random assignments    │
└─────────┬────────────────┘
          │
          ▼
     ┌────────┐
     │ Loop   │◀────────┐
     └────┬───┘         │
          │             │
          ▼             │
┌─────────────────────┐ │
│ 2. Assign Points    │ │
│    to Nearest       │ │
│    Centroid         │ │
└─────────┬───────────┘ │
          │             │
          ▼             │
┌─────────────────────┐ │
│ 3. Update           │ │
│    Centroids        │ │
│    (Medoids)        │ │
└─────────┬───────────┘ │
          │             │
          ▼             │
┌─────────────────────┐ │
│ 4. Check            │ │
│    Convergence      │ │
└─────────┬───────────┘ │
          │             │
    ┌─────┴─────┐       │
    ▼           ▼       │
  Changed    No change  │
    │                   │
    └───────────────────┘
                        │
                        ▼
                   ┌─────────┐
                   │ Output  │
                   │ Clusters│
                   └─────────┘
```

## Consensus Generation (POA)

```
Input: Reads in Cluster

┌──────────────────────┐
│ 1. Build POA Graph   │
│                      │
│    Read 1: ACGT--TAG │
│    Read 2: ACGTATAG  │
│    Read 3: ACGT-TTAG │
│                      │
│    Graph:            │
│         A─C─G─T      │
│               ├─A─┐  │
│               └─┬─┤  │
│                 T │  │
│                 └─┴─T─A─G
└─────────┬────────────┘
          ▼
┌──────────────────────┐
│ 2. Find Heaviest     │
│    Path              │
│    (Most supported)  │
└─────────┬────────────┘
          ▼
┌──────────────────────┐
│ 3. Extract Consensus │
│    ACGTATAG          │
└─────────┬────────────┘
          ▼
┌──────────────────────┐
│ 4. Calculate Quality │
│    (Coverage-based)  │
└──────────────────────┘
```

## UCHIME Chimera Detection

```
Query Sequence:
├──────────────┼──────────────┤
    Left half      Right half

Test against potential parents:

Parent A: ████████████░░░░░░░░░░░░
Parent B: ░░░░░░░░░░░░████████████

If left half matches A
AND right half matches B
→ Chimera detected!

Score = (SimilarityLeft_A + SimilarityRight_B) / 2
```

## Memory Layout (Optimization)

```
┌─────────────────────────────────────┐
│         Main Memory                 │
│                                     │
│  ┌────────────────┐                │
│  │ K-mer Index    │ (Persistent)   │
│  └────────────────┘                │
│                                     │
│  ┌────────────────┐                │
│  │ Reference Seqs │ (Persistent)   │
│  └────────────────┘                │
│                                     │
│  ┌────────────────┐                │
│  │ Read Buffer    │ (Streaming)    │
│  │ (Chunked)      │                │
│  └────────────────┘                │
│           │                         │
│           ▼                         │
│  ┌────────────────┐                │
│  │ Distance       │ (Temporary,    │
│  │ Matrix         │  per group)    │
│  └────────────────┘                │
│                                     │
└─────────────────────────────────────┘
```

## Parallelization Strategy

```
┌──────────────────────────────────┐
│      Read Processing             │
│                                  │
│  Thread 1: ████░░░░░░░░         │
│  Thread 2: ░░░░████░░░░         │
│  Thread 3: ░░░░░░░░████         │
│  Thread 4: ░░░░░░░░░░░░████     │
│                                  │
│  (Rayon parallel iterator)       │
└──────────────────────────────────┘

┌──────────────────────────────────┐
│    Distance Matrix (Triangle)    │
│                                  │
│  Job 1: (0,1), (0,2), (0,3) ...  │
│  Job 2: (1,2), (1,3), (1,4) ...  │
│  Job 3: (2,3), (2,4), (2,5) ...  │
│                                  │
│  (Work-stealing thread pool)     │
└──────────────────────────────────┘

┌──────────────────────────────────┐
│      Consensus Generation        │
│                                  │
│  Cluster 1 → Thread 1            │
│  Cluster 2 → Thread 2            │
│  Cluster 3 → Thread 3            │
│  ...                             │
│                                  │
│  (Independent per cluster)       │
└──────────────────────────────────┘
```

## File I/O Strategy

```
Stream Processing:
───────────────────▶

File: [═══════════════════════════]
       ▲     ▲     ▲     ▲
       │     │     │     │
    Chunk1 Chunk2 Chunk3 Chunk4
       │     │     │     │
       ▼     ▼     ▼     ▼
    Process in batches
    (Don't load all into memory)

For FASTQ.GZ:
File → GzDecoder → BufReader → Parser → Process
```

## Type System Flow

```
Raw File Data
     │
     ▼
Vec<u8> (bytes)
     │
     ▼
SequenceRead {
  id: String,
  sequence: Vec<u8>,
  quality: Option<Vec<u8>>,
  platform: Platform
}
     │
     ▼
Placement {
  read_id: String,
  reference_id: usize,
  score: f64
}
     │
     ▼
Cluster {
  id: usize,
  reads: Vec<String>,
  consensus: Option<ConsensusSequence>,
  metrics: ClusterMetrics
}
     │
     ▼
FASTA Output / JSON / BAM
```

## Error Handling Flow

```
Operation
    │
    ├─ Success ──▶ Ok(result)
    │
    └─ Failure ──▶ Err(error)
                       │
                       ├─ I/O Error
                       ├─ Parse Error
                       ├─ Algorithm Error
                       └─ Config Error
                              │
                              ▼
                       anyhow::Error
                              │
                              ▼
                       Log + User Message
                              │
                              ▼
                       Exit with code
```

## Legend

```
┌────┐
│Box │  = Component/Module
└────┘

─────▶  = Data flow

████    = Active processing

░░░░    = Idle/Completed

═════   = File/Storage

▼       = Direction of flow
```

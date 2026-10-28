# Aproko Knowledge Base

This directory contains the Aproko knowledge base implementation, providing persistent storage for AI-accessible analysis results.

## Overview

The Aproko knowledge base enables:
- **Persistent Storage**: Store analysis results at `.ovie/aproko/knowledge/`
- **AI Integration**: Structured format optimized for LLM consumption
- **Incremental Updates**: Update only affected entries when code changes
- **Query Interface**: Fast lookup by category, symbol, or pattern
- **Versioning**: Track knowledge evolution over time

## Storage Location

By default, the knowledge base stores data at:
```
.ovie/aproko/knowledge/
├── types.json          # Type information for all symbols
├── patterns.json       # Code patterns and idioms
├── reasoning.json      # Reasoning rules and logic
├── security.json       # Security analysis results
├── performance.json    # Performance hints and bottlenecks
├── documentation.json  # Documentation metadata
└── metadata.json       # Knowledge base metadata
```

## Core Components

### Knowledge Entry

Each entry in the knowledge base contains:
- `id`: Unique identifier
- `category`: Type of knowledge (ReasoningRule, TypeInformation, CodePattern, etc.)
- `content`: JSON-serialized content
- `source_location`: File path and line numbers
- `confidence`: Confidence score (0.0 to 1.0)
- `timestamp`: When the entry was created/updated
- `version`: Version number for tracking evolution

### Entry Categories

1. **ReasoningRule**: Logic and reasoning patterns
2. **TypeInformation**: Type definitions and signatures
3. **CodePattern**: Common code patterns and idioms
4. **SecurityIssue**: Security vulnerabilities and concerns
5. **PerformanceHint**: Performance optimization opportunities
6. **Documentation**: Documentation metadata

## Usage Examples

### Storing Analysis Results

```ovie
use std::aproko::{store_entry, KnowledgeEntry, EntryCategory, SourceLocation};

// Create a knowledge entry
let entry = KnowledgeEntry {
    id: "type_Vec_T".to_string(),
    category: EntryCategory::TypeInformation,
    content: serialize_type_info(vec_type),
    source_location: SourceLocation {
        file_path: "std/core/vec.ov".to_string(),
        line_start: 10,
        line_end: 50,
        column_start: 0,
        column_end: 0
    },
    confidence: 1.0,
    timestamp: current_time(),
    version: 1
};

// Store in knowledge base
store_entry(entry)?;
```

### Querying the Knowledge Base

```ovie
use std::aproko::{query, KnowledgeQuery, EntryCategory, get_type_info};

// Query by category
let query = KnowledgeQuery {
    category: Some(EntryCategory::TypeInformation),
    symbol_name: None,
    pattern_type: None,
    min_confidence: 0.8
};
let results = query(query);

// Get specific type information
let vec_info = get_type_info("Vec");
match vec_info {
    Some(entry) => println!("Found type info for Vec"),
    None => println!("Vec type info not found")
}

// Get reasoning rules by category
let safety_rules = get_reasoning_rules("safety");
```

### Updating Entries

```ovie
use std::aproko::{update_entry};

// Update an existing entry
update_entry("type_Vec_T".to_string(), new_content)?;
```

### Exporting to JSON

```ovie
use std::aproko::{export_json};

// Export entire knowledge base to JSON
export_json("knowledge_export.json")?;
```

## JSON Schema Examples

### Type Information Entry

```json
{
  "id": "type_Vec_T",
  "category": "TypeInformation",
  "content": {
    "symbol": "Vec",
    "module": "std::core",
    "kind": "generic_struct",
    "type_parameters": ["T"],
    "fields": [
      {"name": "data", "type": "Array<T>"},
      {"name": "length", "type": "usize"},
      {"name": "capacity", "type": "usize"}
    ],
    "methods": [
      {
        "name": "push",
        "signature": "(self: &mut Vec<T>, item: T) -> ()",
        "doc": "Add an item to the end of the vector"
      }
    ],
    "invariants": [
      "length <= capacity",
      "capacity > 0 implies data is allocated"
    ]
  },
  "source_location": {
    "file_path": "std/core/vec.ov",
    "line_start": 10,
    "line_end": 50,
    "column_start": 0,
    "column_end": 0
  },
  "confidence": 1.0,
  "timestamp": 1705315800,
  "version": 1
}
```

### Reasoning Rule Entry

```json
{
  "id": "rule_bounds_check",
  "category": "ReasoningRule",
  "content": {
    "pattern": "array_access",
    "condition": "index < array.length",
    "action": "verify_bounds_check",
    "severity": "error",
    "message": "Array access must be bounds-checked",
    "examples": [
      {
        "code": "let x = arr[i];",
        "requires": "assert(i < arr.length);"
      }
    ]
  },
  "source_location": {
    "file_path": "std/core/vec.ov",
    "line_start": 45,
    "line_end": 45,
    "column_start": 0,
    "column_end": 0
  },
  "confidence": 1.0,
  "timestamp": 1705315800,
  "version": 1
}
```

### Code Pattern Entry

```json
{
  "id": "pattern_iterator",
  "category": "CodePattern",
  "content": {
    "name": "Iterator Pattern",
    "description": "Iterate over collection elements",
    "structure": {
      "trait": "Iterator",
      "methods": ["next", "has_next"]
    },
    "usage_examples": [
      {
        "code": "for item in collection { ... }",
        "explanation": "Syntactic sugar for iterator pattern"
      }
    ],
    "occurrences": [
      {"file": "std/core/vec.ov", "line": 45},
      {"file": "std/core/hashmap.ov", "line": 78}
    ]
  },
  "source_location": {
    "file_path": "std/core/vec.ov",
    "line_start": 45,
    "line_end": 60,
    "column_start": 0,
    "column_end": 0
  },
  "confidence": 0.95,
  "timestamp": 1705315800,
  "version": 1
}
```

## Integration with Aproko Analyzer

The knowledge base is automatically populated by the Aproko static analyzer:

1. **Analysis Phase**: Aproko analyzes Ovie code
2. **Extraction Phase**: Extracts type info, patterns, reasoning rules
3. **Storage Phase**: Stores results in knowledge base
4. **Query Phase**: LLMs and tools query the knowledge base

## LLM Integration

The knowledge base is designed for easy LLM consumption:

```python
# Example: Querying from Python LLM tool
import json

# Load knowledge base
with open('.ovie/aproko/knowledge/types.json') as f:
    types = json.load(f)

# Find type information
vec_type = next(
    (entry for entry in types['entries'] 
     if entry['content']['symbol'] == 'Vec'),
    None
)

# Use in LLM prompt
prompt = f"The Vec type has these methods: {vec_type['content']['methods']}"
```

## Implementation Status

### Phase 6: Aproko Knowledge Base (Current)
- ✅ Core data structures defined
- ✅ Storage structure designed
- ✅ Query interface implemented (skeleton)
- ⏳ JSON serialization (pending)
- ⏳ Incremental updates (pending)
- ⏳ Integration with Aproko analyzer (pending)

## Design Principles

1. **AI-First**: Optimized for LLM and agentic tool consumption
2. **Incremental**: Update only what changes
3. **Versioned**: Track knowledge evolution
4. **Structured**: JSON format with clear schemas
5. **Queryable**: Fast lookup by multiple criteria

## Performance Considerations

- **Lazy Loading**: Load only needed category files
- **Indexed**: Fast lookup by symbol name and category
- **Cached**: In-memory cache for frequently accessed entries
- **Incremental**: Update only affected entries on code changes

## Contributing

When adding functionality to the knowledge base:

1. Update `knowledge_base.ov` with new functions
2. Add comprehensive documentation
3. Update JSON schemas if needed
4. Write tests for new query patterns
5. Update this README

## Related Documentation

- [Requirements Document](/.kiro/specs/ovie-2-3-module-system/requirements.md) - See Requirement 21
- [Design Document](/.kiro/specs/ovie-2-3-module-system/design.md) - Aproko Knowledge Base section
- [Aproko Documentation](/docs/aproko.md)

## License

Part of the Ovie programming language project.

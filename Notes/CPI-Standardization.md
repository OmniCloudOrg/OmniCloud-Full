# CPI Standardization Guidelines

I've standardized all CPI JSON files to work with our new unified CPI system. Here are the key changes applied to all files:

## 1. Structure Changes

All CPIs now have a consistent structure:
```json
{
    "name": "provider_name",
    "type": "cloud|virt",
    "actions": {
        "action_name": {
            "command": "command_template",
            "params": ["param1", "param2"],
            "parse_rules": { ... },
            "pre_exec": [ ... ],
            "post_exec": [ ... ]
        }
    },
    "default_settings": { ... }
}
```

## 2. Parse Rules Standardization

All output parsing is now done with `parse_rules` with one of these types:

### Object Type
```json
"parse_rules": {
    "type": "object",
    "patterns": {
        "key1": {
            "regex": "pattern",
            "group": 1,
            "transform": "boolean|number" (optional)
        },
        "key2": { ... }
    }
}
```

### Array Type
```json
"parse_rules": {
    "type": "array",
    "separator": "\n\n",
    "patterns": {
        "field1": {
            "regex": "pattern",
            "group": 1
        },
        "field2": { ... }
    }
}
```

### Properties Type
```json
"parse_rules": {
    "type": "properties",
    "patterns": { ... },
    "array_patterns": { ... },
    "array_key": "key_name",
    "related_patterns": { ... }
}
```

## 3. Pre/Post Commands as Sub-Actions

All pre_exec and post_exec commands are now sub-actions with their own parse_rules:

```json
"pre_exec": [
    {
        "command": "command_string",
        "parse_rules": {
            "type": "object",
            "patterns": {
                "success": {
                    "regex": ".*",
                    "transform": "boolean"
                }
            }
        }
    }
]
```

## 4. Common Parameters

All CPIs support the same basic actions with standardized parameter names:
- `vm_name`: Name of the virtual machine
- `disk_path`: Path to disk file
- `size_gb`/`size_mb`: Size of disks
- `cpu_count`: Number of CPUs
- `memory_mb`: Memory in MB

## 5. Boolean Success Indicators

For actions that just need to indicate success/failure:
```json
"parse_rules": {
    "type": "object",
    "patterns": {
        "success": {
            "regex": ".*",
            "transform": "boolean"
        }
    }
}
```

## 6. Standardized Test Actions

All CPIs include a standardized `test_install` action that extracts the version:
```json
"test_install": {
    "command": "tool --version",
    "params": [],
    "parse_rules": {
        "type": "object",
        "patterns": {
            "version": {
                "regex": "pattern_to_extract_version",
                "group": 1
            }
        }
    }
}
```

These guidelines were applied to all CPIs to create a consistent and predictable interface for the unified CPI system.
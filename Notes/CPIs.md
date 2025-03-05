# Cloud Provider Interface (CPI) Specification

## Table of Contents

1. [Introduction](#introduction)
2. [File Structure](#file-structure)
3. [JSON Schema](#json-schema)
4. [Provider Configuration](#provider-configuration)
5. [Actions](#actions)
6. [Command Templates](#command-templates)
7. [Parse Rules](#parse-rules)
8. [Pre/Post Execution](#pre-post-execution)
9. [Default Settings](#default-settings)
10. [Authentication](#authentication)
11. [Error Handling](#error-handling)
12. [Standard Actions](#standard-actions)
13. [Standard Parameters](#standard-parameters)
14. [Examples](#examples)
15. [Best Practices](#best-practices)

## Introduction

The Cloud Provider Interface (CPI) system provides a standardized way to interact with different cloud and virtualization providers through a unified API. Each provider is defined in a JSON configuration file that specifies available actions, required parameters, command templates, and output parsing rules.

This specification document outlines the structure and requirements for creating CPI definition files that can be loaded into the unified CPI system.

## File Structure

CPI definition files are JSON files located in the `./CPIs` directory. Each file defines a single provider's interface.

- Files must use the `.json` extension
- Files should follow a naming convention of `provider-name.json`
- The system automatically loads all JSON files in the CPIs directory at startup

Example directory structure:
```
./CPIs/
  ├─ aws.json
  ├─ azure.json
  ├─ gcp.json
  ├─ virtualbox.json
  └─ kvm.json
```

## JSON Schema

Each CPI file must conform to the following schema:

```json
{
  "name": "provider_name",
  "type": "provider_type",
  "actions": {
    "action_name": {
      "command": "command_template",
      "params": ["param1", "param2"],
      "parse_rules": { ... },
      "pre_exec": [ ... ],
      "post_exec": [ ... ]
    }
  },
  "default_settings": { ... },
  "authentication": { ... }
}
```

## Provider Configuration

### Root Properties

| Property | Type | Required | Description |
|----------|------|----------|-------------|
| `name` | String | Yes | Unique identifier for the provider |
| `type` | String | Yes | Provider type, either "cloud" or "virt" |
| `actions` | Object | Yes | Map of available actions and their definitions |
| `default_settings` | Object | No | Default parameter values for actions |
| `authentication` | Object | No | Authentication configuration for the provider |

### Example

```json
{
  "name": "my_aws_cpi",
  "type": "cloud",
  "actions": { ... },
  "default_settings": {
    "instance_type": "t3.micro",
    "region": "us-east-1"
  },
  "authentication": {
    "env_vars": [
      "AWS_ACCESS_KEY_ID",
      "AWS_SECRET_ACCESS_KEY"
    ]
  }
}
```

## Actions

Actions define the operations available for a provider. Each action is an object with the following properties:

| Property | Type | Required | Description |
|----------|------|----------|-------------|
| `command` | String | Yes | Command template to execute |
| `params` | Array<String> | No | Required parameters for the action |
| `parse_rules` | Object | Yes | Rules for parsing command output |
| `pre_exec` | Array<Action> | No | Actions to execute before the main command |
| `post_exec` | Array<Action> | No | Actions to execute after the main command |

### Example

```json
"create_vm": {
  "command": "aws ec2 run-instances --image-id {ami_id} --instance-type {instance_type} --subnet-id {subnet_id}",
  "params": ["ami_id", "instance_type", "subnet_id"],
  "parse_rules": {
    "type": "object",
    "patterns": {
      "instance_id": {
        "regex": "\"InstanceId\":\\s*\"(i-[a-z0-9]+)\"",
        "group": 1
      }
    }
  },
  "post_exec": [
    {
      "command": "aws ec2 create-tags --resources {instance_id} --tags Key=Name,Value={vm_name}",
      "params": ["instance_id", "vm_name"],
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
}
```

## Command Templates

Command templates are strings with parameter placeholders enclosed in curly braces. When an action is executed, these placeholders are replaced with actual parameter values.

### Syntax

- Parameter placeholders: `{parameter_name}`
- All parameters referenced in the command must be listed in the `params` array or defined in `default_settings`

### Example

```json
"command": "gcloud compute instances create {vm_name} --machine-type {machine_type} --image {image} --zone {zone}"
```

## Parse Rules

Parse rules define how to extract data from command output. There are three types of parse rules:

1. `object`: Extract key-value data using regex patterns
2. `array`: Extract a list of items from output with a separator
3. `properties`: Extract nested properties with array support

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

### Pattern Properties

| Property | Type | Required | Description |
|----------|------|----------|-------------|
| `regex` | String | Yes | Regular expression pattern |
| `group` | Number | No | Capture group to extract (default: 0) |
| `transform` | String | No | Transform extracted value: "boolean" or "number" |
| `optional` | Boolean | No | Whether pattern is required (default: false) |
| `match_value` | String | No | Reference to another pattern's value for matching |

### Example

```json
"parse_rules": {
  "type": "object",
  "patterns": {
    "uuid": {
      "regex": "UUID: ([0-9a-f-]+)",
      "group": 1
    },
    "success": {
      "regex": ".*",
      "transform": "boolean"
    }
  }
}
```

## Pre/Post Execution

Pre and post execution actions allow defining sequences of commands to run before or after the main action command. Each pre/post action has the same structure as a regular action.

### Example

```json
"pre_exec": [
  {
    "command": "VBoxManage storagectl {vm_name} --name {controller_name} --add sata --controller IntelAhci --portcount 30",
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

## Default Settings

Default settings provide default values for parameters that may be used across multiple actions. These values are applied if not explicitly provided in the action call.

### Example

```json
"default_settings": {
  "os_type": "Ubuntu_64",
  "memory_mb": 2048,
  "cpu_count": 2,
  "controller_name": "SATA Controller",
  "network_type": "nat"
}
```

## Authentication

The authentication object specifies how the provider should be authenticated. This typically includes environment variables or credential files.

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `env_vars` | Array<String> | Environment variables required for authentication |
| `profile_support` | Boolean | Whether provider supports profile-based authentication |

### Example

```json
"authentication": {
  "env_vars": [
    "AWS_ACCESS_KEY_ID",
    "AWS_SECRET_ACCESS_KEY",
    "AWS_DEFAULT_REGION"
  ],
  "profile_support": true
}
```

## Error Handling

The CPI system treats non-zero exit codes from commands as errors. Additionally, parse rules may specify patterns that if not matched, indicate failure.

For success/failure detection, use a boolean transformation:

```json
"success": {
  "regex": ".*",
  "transform": "boolean"
}
```

## Standard Actions

The following actions should be implemented by all providers when applicable:

| Action | Description |
|--------|-------------|
| `test_install` | Verify provider CLI is installed and configured |
| `create_vm` | Create a new virtual machine |
| `delete_vm` | Delete a virtual machine |
| `has_vm` | Check if a virtual machine exists |
| `start_vm` | Start a virtual machine |
| `stop_vm` | Stop a virtual machine |
| `reboot_vm` | Restart a virtual machine |
| `get_vm` | Get details about a virtual machine |
| `create_disk` | Create a new disk |
| `delete_disk` | Delete a disk |
| `attach_disk` | Attach a disk to a virtual machine |
| `detach_disk` | Detach a disk from a virtual machine |
| `has_disk` | Check if a disk exists |
| `get_disks` | List available disks |
| `create_snapshot` | Create a snapshot of a disk or VM |
| `delete_snapshot` | Delete a snapshot |
| `has_snapshot` | Check if a snapshot exists |
| `get_snapshots` | List available snapshots |
| `configure_networks` | Configure network interfaces for a VM |
| `set_vm_metadata` | Set metadata for a virtual machine |

## Standard Parameters

The following parameters should be used consistently across providers:

| Parameter | Type | Description |
|-----------|------|-------------|
| `vm_name` | String | Name of the virtual machine |
| `vm_id` | String | ID of the virtual machine |
| `os_type` | String | Operating system type |
| `memory_mb` | Number | Memory allocation in megabytes |
| `cpu_count` | Number | Number of virtual CPUs |
| `disk_path` | String | Path to disk file |
| `disk_name` | String | Name of the disk |
| `disk_id` | String | ID of the disk |
| `size_mb` | Number | Size in megabytes |
| `size_gb` | Number | Size in gigabytes |
| `snapshot_name` | String | Name of the snapshot |
| `snapshot_id` | String | ID of the snapshot |
| `controller_name` | String | Storage controller name |
| `port` | Number | Storage controller port |
| `device_name` | String | Device name for attached disks |
| `network_type` | String | Network type |
| `network_index` | Number | Network interface index |

## Examples

### VirtualBox Example

```json
{
  "name": "my_virtualbox_cpi",
  "type": "virt",
  "actions": {
    "test_install": {
      "command": "VBoxManage --version",
      "params": [],
      "parse_rules": {
        "type": "object",
        "patterns": {
          "version": {
            "regex": "^([\\d\\.]+)\\s*$",
            "group": 1
          }
        }
      }
    },
    "create_vm": {
      "command": "VBoxManage createvm --name {vm_name} --ostype {os_type} --register",
      "params": ["vm_name", "os_type"],
      "post_exec": [
        {
          "command": "VBoxManage modifyvm {vm_name} --memory {memory_mb} --cpus {cpu_count}",
          "parse_rules": {
            "type": "object",
            "patterns": {
              "success": {
                "regex": ".*",
                "transform": "boolean"
              }
            }
          }
        },
        {
          "command": "VBoxManage modifyvm {vm_name} --nic1 nat",
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
      ],
      "parse_rules": {
        "type": "object",
        "patterns": {
          "uuid": {
            "regex": "UUID: ([0-9a-f-]+)",
            "group": 1
          }
        }
      }
    }
  },
  "default_settings": {
    "os_type": "Ubuntu_64",
    "memory_mb": 2048,
    "cpu_count": 2,
    "controller_name": "SATA Controller",
    "network_type": "nat"
  }
}
```

### AWS Example

```json
{
  "name": "my_aws_cpi",
  "type": "cloud",
  "actions": {
    "test_install": {
      "command": "aws --version",
      "params": [],
      "parse_rules": {
        "type": "object",
        "patterns": {
          "version": {
            "regex": "aws-cli/([\\d\\.]+)",
            "group": 1
          }
        }
      }
    },
    "create_vm": {
      "command": "aws ec2 run-instances --image-id {ami_id} --instance-type {instance_type} --subnet-id {subnet_id}",
      "params": ["ami_id", "instance_type", "subnet_id"],
      "parse_rules": {
        "type": "object",
        "patterns": {
          "instance_id": {
            "regex": "\"InstanceId\":\\s*\"(i-[a-z0-9]+)\"",
            "group": 1
          }
        }
      },
      "post_exec": [
        {
          "command": "aws ec2 create-tags --resources {instance_id} --tags Key=Name,Value={vm_name}",
          "params": ["instance_id", "vm_name"],
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
    }
  },
  "default_settings": {
    "instance_type": "t3.micro",
    "volume_type": "gp3",
    "region": "us-east-1"
  },
  "authentication": {
    "env_vars": [
      "AWS_ACCESS_KEY_ID",
      "AWS_SECRET_ACCESS_KEY",
      "AWS_DEFAULT_REGION"
    ],
    "profile_support": true
  }
}
```

## Best Practices

### 1. Parameter Naming Consistency

Use consistent parameter names across all providers to enable seamless switching between providers.

### 2. Command Structure

- Keep commands simple and focused on a single task
- Use pre/post execution for complex operations
- Avoid using shell-specific features in commands

### 3. Parsing Output

- Use specific regex patterns to extract information
- Always include capture groups in regex patterns
- Provide meaningful keys for extracted values

### 4. Error Handling

- Use specific error detection in parse rules
- Include optional parameters when appropriate
- Provide meaningful error messages

### 5. Documentation

- Include comments in CPI files using JSON-compatible syntax
- Document provider-specific behavior
- List required environment variables

### 6. Testing

- Always include a `test_install` action to verify provider installation
- Test all actions with minimal parameters
- Verify parse rules extract the expected data

### 7. Handling CLI Variations

- Account for different versions of provider CLIs
- Consider platform-specific command differences
- Implement fallbacks for features not supported by all CLI versions

### 8. Security

- Never hardcode credentials in CPI files
- Use environment variables for authentication
- Minimize use of potentially sensitive data in command parameters

By following these specifications and best practices, you can create CPI definitions that work seamlessly with the unified CPI system, providing a consistent interface across different cloud and virtualization providers.
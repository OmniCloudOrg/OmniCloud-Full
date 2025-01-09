# 🚧 Under Construction
This project is actively being developed. Nothing should yet be assumed stable

# Omniforge

🚀 Zero-config platform for deploying microservices anywhere - VMs, containers, or bare metal - with just 24MB RAM overhead.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-stable-orange.svg)

## Quick Start

```bash
# Deploy directly from your project directory
omni up

# Omniforge automatically:
# - Bundles your package
# - Detects your project type
# - Builds optimized container image
# - Chooses best infrastructure
# - Deploys with optimal settings
# - Autoscales and manages your app instances
```

## Configuration (Optional)

Override automatic settings when needed:

```yaml
# omniforge.yaml
runtime: docker        # Override auto-detected runtime
provider: aws         # Force specific provider
resources:
  cpu: 2
  memory: 512Mi
```

## 🚀 Quick Start

### Installation

```bash
# Using GRIP
grip install omni-cli

# Or download the binary
curl -L https://omni-forge.github.io/get | sh
```

### Your First Deployment
omni up and omni push will automatically create a service for you if one does not exist, however you can also be more deliberate

1. Create a new project:
```bash
omni new my-service
cd my-service
```

2. Deploy it:
```bash
omni push
```

That's it! Omniforge automatically detects your project type and deploys it to your configured provider, if there is no configuration Omni will select everything automatically.

## 📚 Core Concepts

### Cloud Provider Interfaces (CPIs)

CPIs are JSON files that define how Omniforge interacts with infrastructure providers. They specify commands, parameters, and output parsing rules.

Example VirtualBox CPI:
```json
{
    "name": "my_virtualbox_cpi",
    "type": "virt",
    "actions": {
        "create_vm": {
            "command": "VBoxManage createvm --name {vm_name} --ostype {os_type} --register",
            "params": ["vm_name", "os_type"],
            "output_parser": {
                "type": "regex",
                "pattern": "UUID:\\s+([a-f0-9-]+)",
                "capture_groups": {
                    "vm_uuid": 1
                }
            },
            "post_exec": [
                {
                    "command": "VBoxManage modifyvm {vm_name} --memory {memory_mb} --cpus {cpu_count}",
                    "output_parser": {
                        "type": "exit_code",
                        "success_value": 0
                    }
                }
            ]
        }
    },
    "default_settings": {
        "os_type": "Ubuntu_64",
        "memory_mb": 2048,
        "cpu_count": 2
    }
}
```

### Supported Infrastructure Types

See [Providers.md](./providers.md)

## 🛠 Features in Detail

### Dynamic Build System

Omniforge automatically creates optimized container images based on your project's file extensions:

```plaintext
my-project/
├── src/
│   ├── main.rs        # Detected: Rust → Uses rust-builder image
│   └── utils.py       # Detected: Python → Adds Python runtime
├── package.json       # Detected: Node.js → Adds Node.js runtime
└── Cargo.toml        # Used for Rust dependencies
```

### Resource Types

Create reusable infrastructure templates:

```yaml
# worker.yaml
kind: Worker
spec:
  runtime: docker
  resources:
    cpu: 1
    memory: 512Mi
  scaling:
    min: 1
    max: 10
    metrics:
      - type: http_requests
        target: 1000
```

### Runtime Configuration

Configure multiple runtimes for different use cases:

```yaml
# omniforge.yaml
runtimes:
  production:
    provider: aws
    region: us-east-1
    instance_type: t3.micro
  
  development:
    provider: virtualbox
    memory: 2048
    cpus: 2
```

## 📘 CPI Reference

### Structure
- `name`: CPI identifier
- `type`: Provider type (virt, container, metal)
- `actions`: Available commands and their specifications
- `default_settings`: Default configuration values

### Parser Types
- `regex`: Extract values using regular expressions
- `exit_code`: Check command success/failure
- `multi_regex`: Extract multiple values
- `table`: Parse tabular output

### Action Properties
- `command`: Command template with parameter placeholders
- `params`: Required parameters
- `output_parser`: Output parsing rules
- `pre_exec`/`post_exec`: Additional commands to run

## 🔧 Advanced Usage

### Custom Build Hooks

Create custom build steps:

```bash
# .omniforge/hooks/pre-build
#!/bin/bash
npm run build
```

### Health Checks

Define sophisticated health monitoring:

```yaml
health:
  http:
    path: /health
    port: 8080
  interval: 10s
  timeout: 5s
  retries: 3
```

### Service Discovery

Automatic service discovery and registration:

```yaml
discovery:
  service: my-api
  tags: ["production", "v2"]
  port: 8080
```

## 🤝 Contributing

We love contributions! Here's how you can help:

1. Fork the repository
2. Create a feature branch
3. Write your changes
4. Write tests
5. Submit a PR

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

## 🔍 Troubleshooting

### Common Issues

1. **Permission Denied**
   ```bash
   sudo chown -R $(whoami) ~/.omniforge
   ```

2. **Provider Not Found**
   ```bash
   omni provider install aws
   ```

### Logs

Access debug logs:
```bash
omni logs --level debug
```


## 📜 License

MIT License - see [LICENSE](LICENSE) for details.

---

Built with ❤️ using Rust. Star us on GitHub if you like Omniforge!

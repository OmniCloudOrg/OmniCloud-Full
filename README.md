# OmniCloud

## 🚀 Zero-config platform for deploying microservices anywhere

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-stable-orange.svg)

## The Problem

For too long, operations teams have been forced to stack disparate tools together in increasingly complex arrangements. We've been building towers of abstraction on top of inconsistent foundations, creating fragile systems that are difficult to maintain, scale, and understand.

Every new tool brings its own paradigms, configuration formats, and operational quirks. The cognitive load increases with each addition to your stack. Teams spend more time connecting and maintaining tools than solving real business problems.

**We deserve better.**

## Our Vision

OmniCloud isn't just another tool to add to your stack. It's a comprehensive rethinking of cloud operations from first principles.

We're building a platform where components are **DESIGNED** to work together from the ground up. Not awkwardly integrated after the fact. Not connected through brittle plugins or adapters. But truly composed as a unified system with consistent interfaces, behaviors, and paradigms.

## How It Works

The magic of OmniCloud is in its simplicity:

```bash
# Deploy directly from your project directory
omni up
```

That's it. OmniCloud automatically:
- Bundles your package
- Detects your project type
- Builds optimized container image
- Chooses best infrastructure
- Deploys with optimal settings
- Autoscales and manages your app instances

No complex configuration required. OmniCloud makes intelligent decisions for you, while still allowing overrides when needed:

```yaml
# OmniCloud.yaml (Optional)
runtime: docker        # Override auto-detected runtime
provider: aws         # Force specific provider
resources:
  cpu: 2
  memory: 512Mi
```

## Core Principles

- **Composable by Design**: Every component shares the same foundational primitives
- **Coherent Experience**: Consistent interfaces across all aspects of the platform
- **Open Source First**: Community-driven development with transparency at its core
- **Scale-Agnostic**: Works the same way for startups and enterprises
- **API-Driven**: Everything available through both UI and programmatic interfaces
- **Minimal Overhead**: Just 24MB RAM overhead for maximum efficiency

## Current Status

🚧 **Under Construction** 🚧

This project is actively being developed. Nothing should yet be assumed stable.

We're working hard to build the foundation of something revolutionary, but we need your help.

## Join Us

We're looking for contributors who are tired of the status quo. People who believe cloud operations can be fundamentally better. Developers, operators, designers, and thinkers who want to be part of creating the next generation of cloud infrastructure.

If you're interested in:

- Building components that elegantly solve real operational problems
- Designing coherent interfaces that make complex operations intuitive
- Creating documentation that illuminates rather than obscures
- Testing across diverse environments to ensure rock-solid reliability
- Extending our Cloud Provider Interfaces (CPIs) to support more infrastructure types

...then we want to hear from you.

## The Power of CPIs

At the heart of OmniCloud are our Cloud Provider Interfaces (CPIs) - the secret sauce that lets us deploy anywhere with consistent behavior. CPIs define how OmniCloud interacts with infrastructure providers through standardized interfaces.

This means you can deploy the same application to AWS, VirtualBox, bare metal, or any supported provider without changing your workflow. The platform handles the complexity for you.

## Getting Started

1. Star this repository to show your support
2. Install OmniCloud:
   ```bash
   curl -L https://get.omnicloud.sh | sh
   ```
3. Check out our [issues](https://github.com/omnicloudorg/omnicloud/issues) for good first contributions
5. Read our [contribution guidelines](CONTRIBUTING.md) to understand our process

## The Future

Together, we're going to build something transformative. A platform that doesn't just incrementally improve cloud operations, but fundamentally reimagines it.

The days of stitching together unrelated tools are numbered. The future is coherent, composable, and designed with intent.

Welcome to OmniCloud.

---

## License

OmniCloud is licensed under the [MIT License](LICENSE).

Built with ❤️ using Rust. Star us on GitHub if you like OmniCloud!

hello

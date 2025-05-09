---

# ☁️ OmniCloud

## 🚀 Zero-config platform for deploying microservices anywhere

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-stable-orange.svg)

---

## Why OmniCloud?

Cloud ops shouldn’t be this hard.

Too many teams are stuck piecing together tools that don’t really fit—each with its own configs, quirks, and headaches. It’s a fragile mess that slows you down and gets in the way of building real things.

We believe there’s a better way.

## Our Vision

OmniCloud isn’t just another DevOps tool—it’s a fresh start.

We’re reimagining cloud operations from the ground up: components that are built to work together, not patched together later. Everything is consistent, predictable, and designed to *just work*, whether you’re deploying to AWS or a local server.

## What Makes It Different?

The magic of OmniCloud is how *simple* it is to use.

```bash
# From your project directory
omni up
```

That’s it. One command. OmniCloud takes care of the rest:

* Detects your project type
* Builds a lightweight, optimized container
* Picks the best infrastructure
* Deploys it efficiently
* Automatically scales and manages your app

Want control? You’ve got it:

```yaml
# OmniCloud.yaml (optional)
runtime: docker
provider: aws
resources:
  cpu: 2
  memory: 512Mi
```

## Core Values

* ✅ **Composable by Design** – All parts work together cleanly
* ✨ **Consistent Experience** – No more switching mental models
* ❤️ **Open Source First** – Built in public, by the community
* 🔧 **API + UI Friendly** – Use what suits you best
* 🧠 **Smart Defaults, Low Overhead** – Just 24MB RAM
* 🏗 **Scales with You** – From side projects to production fleets

## Status

🛠 **Work in Progress**

We’re still building out the core pieces, but the foundation is coming together fast—and we’d love for you to be a part of it.

## Get Involved

Whether you're a developer, operator, designer, or just passionate about better infrastructure—we want to hear from you.

If you enjoy:

* Solving real-world ops pain points
* Designing clean, elegant workflows
* Writing clear, welcoming docs
* Testing across platforms
* Expanding infrastructure support (via CPIs)

…then join us!

## What's a CPI?

OmniCloud uses **Cloud Provider Interfaces (CPIs)** to interact with infrastructure. These define a standard way to talk to any provider—AWS, VirtualBox, bare metal, you name it.

You write your app once. OmniCloud takes care of making it run *everywhere*, the same way.

## Get Started

1. ⭐ Star this repo to support the project
2. Install OmniCloud:

   ```bash
   curl -L https://get.omnicloud.sh | sh
   ```
3. Check out our [issues](https://github.com/omnicloudorg/omnicloud/issues) for ways to contribute
4. Read our [contributing guide](CONTRIBUTING.md) for how to get involved

---

## License

OmniCloud is open source under the [MIT License](LICENSE).

Built with ❤️ in Rust. Star us if you're excited about the future of cloud!

---

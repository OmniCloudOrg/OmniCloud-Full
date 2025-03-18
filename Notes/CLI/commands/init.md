# `omni init` - Initialize Cloud Environment

## Overview

The `omni init` command is the starting point for setting up your OmniOrchestrator self-hosted cloud platform. This command launches an interactive wizard that guides you through configuring your cloud environment and bootstrapping the OmniOrchestrator system across your infrastructure.

## Usage

```
omni init [--force]
```

### Options

- `--force`: Force re-initialization even if a configuration already exists

## Workflow

### 1. Basic Configuration

When you run `omni init`, the wizard first collects basic information about your cloud environment:

- **Company name**: Your organization's name
- **Admin name**: The name of the primary administrator
- **Cloud platform name**: A unique name for your cloud platform (defaults to a lowercase, hyphenated version of your company name with "-cloud" suffix)
- **Region**: The primary geographic region for your cloud infrastructure

### 2. SSH Host Configuration

Next, the wizard helps you configure the SSH hosts that will form your cloud infrastructure:

- For each host, you'll provide:
  - **Host name**: A unique identifier for the host
  - **Hostname/IP**: The actual hostname or IP address
  - **SSH username**: The username for SSH connections
  - **SSH port**: The port for SSH connections (defaults to 22)
  - **Identity file**: Optional path to an SSH private key file
  - **Bastion status**: Whether this host is a bastion/jump host

You can add multiple hosts in a single initialization session. The wizard will display a summary table of your configured hosts.

### 3. Additional Services

The wizard then lets you configure additional services for your cloud environment:

- **System monitoring**: Enables comprehensive metrics collection and dashboards
- **Automated backups**: Sets up scheduled backups with configurable retention periods

If you enable backups, you'll be prompted to specify a retention period in days.

### 4. Bootstrapping Process

After collecting all necessary configuration, the wizard:

1. Saves your configuration to `config/cloud-config.json`
2. Begins bootstrapping OmniOrchestrator on your configured hosts
3. Sets up bastion hosts first, followed by worker nodes
4. Configures cluster networking, including secure tunnels and service discovery
5. Deploys monitoring and backup services if enabled

The bootstrapping process includes several key steps for each host:
- Establishing SSH connections
- Verifying system requirements
- Installing OmniOrchestrator binaries
- Configuring system services
- Applying security hardening
- Setting up host-specific configurations (bastion or worker)

## Example

```bash
$ omni init
🚀 Cloud Environment Configuration
This wizard will help you configure your self-hosted cloud environment.

Company name: Acme Corporation
Your name (admin): Jane Smith
Cloud platform name [acme-corporation-cloud]: acme-cloud
Select primary region:
> us-east
  us-west
  eu-west
  eu-central
  ap-southeast
  custom

📡 SSH Host Configuration
Configure SSH hosts for your cloud environment

Would you like to add an SSH host? [Y/n] Y
Host name (identifier): primary-bastion
Hostname or IP address: 203.0.113.10
SSH username [admin]: admin
SSH port [22]: 22
Use identity file for authentication? [Y/n] Y
Path to identity file [~/.ssh/id_rsa]: ~/.ssh/acme_key
Is this a bastion/jump host? [y/N] Y
✅ SSH host added successfully

Would you like to add an SSH host? [Y/n] Y
Host name (identifier): worker1
Hostname or IP address: 10.0.1.5
SSH username [admin]: worker
SSH port [22]: 22
Use identity file for authentication? [Y/n] Y
Path to identity file [~/.ssh/id_rsa]: ~/.ssh/acme_key
Is this a bastion/jump host? [y/N] N
✅ SSH host added successfully

Would you like to add an SSH host? [Y/n] n

⚙️ Additional Configuration
Select additional services to enable:
>[X] Enable system monitoring
 [X] Enable automated backups

Backup retention period (days) [30]: 90

💾 Saving Configuration
✅ Configuration saved to config/cloud-config.json

📊 Configuration Summary
Company: Acme Corporation
Admin: Jane Smith
Cloud Name: acme-cloud
Region: us-east
SSH Hosts: 2
Monitoring: Enabled
Backups: Enabled
Backup Retention: 90 days

⚡ Bootstrapping OmniOrchestrator
Setting up OmniOrchestrator for acme-cloud cloud environment
Ready to bootstrap OmniOrchestrator on all configured hosts? [Y/n] Y

Installing OmniOrchestrator on 2 hosts...

Setting up bastion host: primary-bastion
[==================================================] 100% Establishing SSH connection ✓
[==================================================] 100% Verifying system requirements ✓
[==================================================] 100% Installing OmniOrchestrator binaries ✓
[==================================================] 100% Configuring system services ✓
[==================================================] 100% Applying security hardening ✓
[==================================================] 100% Configuring bastion-specific security ✓
✅ OmniOrchestrator installed on primary-bastion

Setting up worker host: worker1
[==================================================] 100% Establishing SSH connection ✓
[==================================================] 100% Verifying system requirements ✓
[==================================================] 100% Installing OmniOrchestrator binaries ✓
[==================================================] 100% Configuring system services ✓
[==================================================] 100% Applying security hardening ✓
[==================================================] 100% Configuring worker-specific services ✓
✅ OmniOrchestrator installed on worker1

✅ OmniOrchestrator installed on all 2 hosts

🔄 Configuring cluster networking
[==================================================] 100% Network configuration complete ✓

📊 Setting up monitoring services
[==================================================] 100% Monitoring services deployed ✓

💾 Configuring backup services
[==================================================] 100% Backup services configured ✓

✨ Environment initialization completed!
Your OmniOrchestrator cloud environment is ready.
You can now deploy applications with 'omni deploy'.
```

## Notes

- The initialization process creates a `config` directory in your current working directory if it doesn't exist
- Running `omni init` on an already configured system will use the existing configuration unless the `--force` flag is provided
- The configuration can be modified later using the `omni config edit` command
- You can view the current configuration with `omni config view`
- For security reasons, it's recommended to use SSH key authentication instead of passwords
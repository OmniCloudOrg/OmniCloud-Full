# `omni config` - Manage Configuration

## Overview

The `omni config` command group provides tools for viewing, editing, and resetting your OmniOrchestrator environment configuration. This command is essential for managing the settings that control how your cloud platform operates, allowing you to adapt your environment to changing requirements without reinitializing the entire system.

## Usage

```
omni config <subcommand>
```

### Subcommands

- `view`: Display the current configuration
- `edit`: Open the configuration in your default editor
- `reset`: Reset configuration to default values

## Subcommand: `omni config view`

The `view` subcommand displays the current configuration settings for your OmniOrchestrator environment.

### Example

```bash
$ omni config view

📝 Application Configuration

environment: production
components:
  frontend:
    replicas: 3
    resources:
      cpu: 150m
      memory: 256Mi
  backend:
    replicas: 2
    resources:
      cpu: 200m
      memory: 512Mi
  database:
    replicas: 1
    resources:
      cpu: 500m
      memory: 1Gi
```

### Output Format

The configuration is displayed in YAML format for readability. The exact structure depends on your specific configuration but typically includes:

1. **Environment Settings**: Global settings like environment type and region
2. **Component Configuration**: Settings for each application component
3. **Resource Allocations**: CPU and memory allocations for each component
4. **Networking Configuration**: Network settings for services
5. **Storage Settings**: Persistent storage configurations

## Subcommand: `omni config edit`

The `edit` subcommand opens your configuration file in your default text editor, allowing you to make changes.

### Workflow

1. The command opens your default editor with the configuration file
2. You make the desired changes and save the file
3. When you close the editor, OmniOrchestrator validates the changes
4. If valid, the changes are applied to your environment

### Example

```bash
$ omni config edit

✏️ Edit Configuration
Opening configuration in your default editor...

Configuration updated successfully!
```

### Validation

When you save your changes, OmniOrchestrator performs validation checks to ensure:

1. The file is valid YAML or JSON syntax
2. Required fields are present
3. Values are within acceptable ranges
4. There are no conflicts between settings

If validation fails, you'll see an error message and have the option to re-edit the file.

## Subcommand: `omni config reset`

The `reset` subcommand resets your configuration to default values after confirmation.

### Workflow

1. The command asks for confirmation before proceeding
2. If confirmed, it restores default configuration values
3. The system applies the new configuration

### Example

```bash
$ omni config reset

⚠️ Are you sure you want to reset configuration to defaults? [y/N] y

Resetting configuration...
✓ Configuration reset to defaults!
```

### Reset Scope

You can control what gets reset by using additional flags:

```bash
$ omni config reset --component frontend
```

This would reset only the frontend component configuration while preserving other settings.

## Configuration Structure

The configuration is stored in a JSON file in the `config` directory. The main configuration file is typically `cloud-config.json`, which includes:

```json
{
  "company_name": "Acme Corporation",
  "admin_name": "Jane Smith",
  "cloud_name": "acme-cloud",
  "region": "us-east",
  "ssh_hosts": [
    {
      "name": "primary-bastion",
      "hostname": "203.0.113.10",
      "username": "admin",
      "port": 22,
      "identity_file": "~/.ssh/acme_key",
      "is_bastion": true
    },
    ...
  ],
  "enable_monitoring": true,
  "enable_backups": true,
  "backup_retention_days": 90
}
```

## Version 2 Configuration

For newer OmniOrchestrator installations, the configuration may use the v2 format, which includes additional fields:

```json
{
  "company_name": "Acme Corporation",
  "admin_name": "Jane Smith",
  "initial_admin_email": "jane@acme.com",
  "cloud_name": "acme-cloud",
  "region": "us-east",
  "api_rate_limit": 1000,
  "initial_storage_gb": 500,
  "backup_retention_days": 90,
  "default_instance_type": "standard-2",
  "default_vpc_cidr": "10.0.0.0/16",
  "default_log_level": "INFO",
  "require_mfa": true,
  "auto_upgrade": true,
  "enable_backups": true,
  "usage_reporting": true,
  "enable_monitoring": true,
  "create_demo_resources": false,
  "ssh_hosts": [...],
  "init_version": "1.0.0"
}
```

## Configuration Best Practices

When managing your OmniOrchestrator configuration:

1. **Version Control**: Keep your configuration under version control
2. **Document Changes**: Add comments or commit messages explaining configuration changes
3. **Test in Staging**: Test configuration changes in a staging environment first
4. **Use Templates**: Create template configurations for different scenarios
5. **Review Regularly**: Periodically review your configuration for optimization opportunities

## Notes

- Configuration changes may require service restarts to take effect
- Some configuration changes (like changing SSH host credentials) may require additional steps
- Use environment-specific configuration files for different deployment environments
- The configuration system supports variable substitution and references
- Sensitive information in the configuration is automatically encrypted
- The configuration file path can be customized using the `OMNI_CONFIG_PATH` environment variable
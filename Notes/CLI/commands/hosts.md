# `omni hosts` - List Configured SSH Hosts

## Overview

The `omni hosts` command provides a detailed listing of all SSH hosts configured in your OmniOrchestrator environment. This command is useful for quickly surveying your infrastructure, checking connection details, and verifying which hosts are configured as bastion hosts.

## Usage

```
omni hosts
```

The command takes no arguments or options.

## Output

The command displays a formatted table with the following columns:

| Column | Description |
|--------|-------------|
| Name | The unique identifier for the host |
| Hostname | The actual hostname or IP address used for connections |
| Username | The SSH username for connecting to the host |
| Port | The SSH port number (typically 22) |
| Identity File | Path to the SSH private key file, or "-" if none is specified |
| Bastion | "Yes" if the host is a bastion/jump host, "No" otherwise |

In addition to the table, the command displays the cloud platform name and region from your configuration.

## Example

```bash
$ omni hosts

📡 Configured SSH Hosts
Cloud: acme-cloud (us-east)

┌──────────────────┬───────────────┬──────────┬──────┬─────────────────┬─────────┐
│ Name             │ Hostname      │ Username │ Port │ Identity File   │ Bastion │
├──────────────────┼───────────────┼──────────┼──────┼─────────────────┼─────────┤
│ primary-bastion  │ 203.0.113.10  │ admin    │ 22   │ ~/.ssh/acme_key │ Yes     │
├──────────────────┼───────────────┼──────────┼──────┼─────────────────┼─────────┤
│ worker1          │ 10.0.1.5      │ worker   │ 22   │ ~/.ssh/acme_key │ No      │
├──────────────────┼───────────────┼──────────┼──────┼─────────────────┼─────────┤
│ worker2          │ 10.0.1.6      │ worker   │ 22   │ ~/.ssh/acme_key │ No      │
├──────────────────┼───────────────┼──────────┼──────┼─────────────────┼─────────┤
│ worker3          │ 10.0.1.7      │ worker   │ 22   │ ~/.ssh/acme_key │ No      │
└──────────────────┴───────────────┴──────────┴──────┴─────────────────┴─────────┘
```

## Error Handling

If no cloud configuration exists (the `config/cloud-config.json` file is missing), the command will display:

```
No cloud configuration found. Run 'omni init' first.
```

If the configuration file exists but no SSH hosts are configured, the command will display:

```
No SSH hosts configured. Run 'omni init' to add hosts.
```

## Notes

- This command reads from the `config/cloud-config.json` file created by the `omni init` command
- It does not verify the connectivity to the hosts; it only displays the configured information
- To add or remove hosts, you need to edit the configuration file using `omni config edit`
- Use this command in combination with `omni status` to check both configuration and operational status
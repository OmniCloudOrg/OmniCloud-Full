# `omni status` - Check OmniOrchestrator Status

## Overview

The `omni status` command provides a comprehensive overview of your OmniOrchestrator environment's operational status. It displays the status of all services running on each host, including resource usage, uptime, and operational state. This command is essential for monitoring your cloud infrastructure and identifying potential issues.

## Usage

```
omni status
```

The command takes no arguments or options.

## Output

The command displays several sections of information:

1. **Cloud Information**: Basic details about your cloud platform and region
2. **Service Status Table**: A detailed table showing all services running on all hosts with their status
3. **System Information**: Status of enabled features like monitoring and backups
4. **Command Suggestions**: Common commands that might be useful for further actions

### Service Status Table Columns

| Column | Description |
|--------|-------------|
| Host | The name of the host running the service |
| Service | The name of the service |
| Status | Current operational status (e.g., "Running", "Stopped") |
| Uptime | How long the service has been running |
| CPU | Current CPU usage percentage |
| Memory | Current memory usage |

## Example

```bash
$ omni status

🔍 OmniOrchestrator Status
Cloud: acme-cloud (us-east)

┌──────────────────┬────────────────────┬─────────┬────────────┬──────┬────────┐
│ Host             │ Service            │ Status  │ Uptime     │ CPU  │ Memory │
├──────────────────┼────────────────────┼─────────┼────────────┼──────┼────────┤
│ primary-bastion  │ orchestrator-core  │ Running │ 3d 12h 45m │ 12%  │ 256MB  │
├──────────────────┼────────────────────┼─────────┼────────────┼──────┼────────┤
│ primary-bastion  │ network-agent      │ Running │ 3d 12h 42m │ 5%   │ 128MB  │
├──────────────────┼────────────────────┼─────────┼────────────┼──────┼────────┤
│ primary-bastion  │ api-gateway        │ Running │ 3d 12h 40m │ 18%  │ 512MB  │
├──────────────────┼────────────────────┼─────────┼────────────┼──────┼────────┤
│ primary-bastion  │ auth-service       │ Running │ 3d 12h 39m │ 10%  │ 384MB  │
├──────────────────┼────────────────────┼─────────┼────────────┼──────┼────────┤
│ primary-bastion  │ metrics-collector  │ Running │ 3d 12h 30m │ 8%   │ 192MB  │
├──────────────────┼────────────────────┼─────────┼────────────┼──────┼────────┤
│ primary-bastion  │ backup-manager     │ Running │ 3d 12h 20m │ 6%   │ 256MB  │
├──────────────────┼────────────────────┼─────────┼────────────┼──────┼────────┤
│ worker1          │ orchestrator-core  │ Running │ 3d 12h 45m │ 12%  │ 256MB  │
├──────────────────┼────────────────────┼─────────┼────────────┼──────┼────────┤
│ worker1          │ network-agent      │ Running │ 3d 12h 42m │ 5%   │ 128MB  │
├──────────────────┼────────────────────┼─────────┼────────────┼──────┼────────┤
│ worker1          │ container-runtime  │ Running │ 3d 12h 38m │ 22%  │ 768MB  │
├──────────────────┼────────────────────┼─────────┼────────────┼──────┼────────┤
│ worker1          │ metrics-collector  │ Running │ 3d 12h 30m │ 8%   │ 192MB  │
├──────────────────┼────────────────────┼─────────┼────────────┼──────┼────────┤
│ worker2          │ orchestrator-core  │ Running │ 3d 12h 45m │ 12%  │ 256MB  │
├──────────────────┼────────────────────┼─────────┼────────────┼──────┼────────┤
│ worker2          │ network-agent      │ Running │ 3d 12h 42m │ 5%   │ 128MB  │
├──────────────────┼────────────────────┼─────────┼────────────┼──────┼────────┤
│ worker2          │ container-runtime  │ Running │ 3d 12h 38m │ 22%  │ 768MB  │
├──────────────────┼────────────────────┼─────────┼────────────┼──────┼────────┤
│ worker2          │ metrics-collector  │ Running │ 3d 12h 30m │ 8%   │ 192MB  │
└──────────────────┴────────────────────┴─────────┴────────────┴──────┴────────┘

🔄 System Information
Monitoring: Enabled
Backups: Enabled
  Retention: 90 days
  Last Backup: 2025-03-01 03:15 UTC
  Next Backup: 2025-03-03 03:15 UTC

💡 Available Commands
- omni service restart <host> <service>: Restart a service
- omni logs <host> <service>: View detailed logs
- omni backup now: Trigger immediate backup
```

## Error Handling

If no cloud configuration exists (the `config/cloud-config.json` file is missing), the command will display:

```
No cloud configuration found. Run 'omni init' first.
```

If the configuration file exists but no SSH hosts are configured, the command will display:

```
No hosts configured. Run 'omni init' to configure hosts.
```

## Service Types

The command displays different types of services depending on the host type:

### Core Services (All Hosts)
- **orchestrator-core**: The central management component
- **network-agent**: Handles networking and connectivity

### Bastion Host Services
- **api-gateway**: Manages external API requests
- **auth-service**: Handles authentication and authorization

### Worker Node Services
- **container-runtime**: Runs containerized applications

### Optional Services
- **metrics-collector**: Collects system metrics (if monitoring is enabled)
- **backup-manager**: Manages backup operations (only on bastion hosts, if backups are enabled)

## Notes

- This command provides a point-in-time snapshot of your environment
- For real-time monitoring, consider setting up monitoring dashboards with the built-in monitoring system
- High CPU or memory usage might indicate performance issues that need attention
- Use `omni logs <host> <service>` to investigate any services not showing as "Running"
- The command does not display application-specific components; use `omni up` status for those
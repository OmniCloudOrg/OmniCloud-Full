# `omni service` - Manage OmniOrchestrator Services

## Overview

The `omni service` command group provides tools for directly managing the services that make up your OmniOrchestrator environment. This command allows you to restart, stop, and start individual services on specific hosts, giving you fine-grained control over your infrastructure's operation.

## Usage

```
omni service <subcommand> [HOST] [SERVICE]
```

### Subcommands

- `restart`: Restart a service on a specific host
- `stop`: Stop a service on a specific host
- `start`: Start a service on a specific host

Each subcommand requires two arguments:
- `HOST`: The name of the host where the service runs
- `SERVICE`: The name of the service to manage

## Subcommand: `omni service restart`

The `restart` subcommand safely restarts a specified service on a specified host.

### Example

```bash
$ omni service restart primary-bastion api-gateway

🔄 Restarting api-gateway on primary-bastion...
✓ Service restarted successfully!

Service Status:
Name:             api-gateway
Host:             primary-bastion
Status:           Running
Uptime:           0m 5s
Health Check:     Passed
```

### Restart Process

When restarting a service, OmniOrchestrator:

1. Gracefully stops the service, allowing in-progress operations to complete
2. Verifies that the service has stopped completely
3. Starts the service again
4. Performs health checks to ensure the service is operating correctly
5. Updates the service registry to reflect the new state

For critical services, OmniOrchestrator may use a rolling restart to minimize downtime.

## Subcommand: `omni service stop`

The `stop` subcommand stops a specified service on a specified host.

### Example

```bash
$ omni service stop worker1 container-runtime

⚠️ Stopping container-runtime on worker1...
This will affect all running containers on this host.
Are you sure? [y/N] y

✓ Service stopped successfully!

Service Status:
Name:             container-runtime
Host:             worker1
Status:           Stopped
Last Active:      2025-03-18 15:45:27 UTC
```

### Stop Process

When stopping a service, OmniOrchestrator:

1. Warns about potential impacts, especially for critical services
2. Requires confirmation for potentially disruptive operations
3. Gracefully stops the service with proper termination signals
4. Updates the service registry to reflect the stopped state
5. For some services, may perform cleanup operations

## Subcommand: `omni service start`

The `start` subcommand starts a previously stopped service on a specified host.

### Example

```bash
$ omni service start worker1 container-runtime

🚀 Starting container-runtime on worker1...
✓ Service started successfully!

Service Status:
Name:             container-runtime
Host:             worker1
Status:           Running
Uptime:           0m 8s
Health Check:     Passed
```

### Start Process

When starting a service, OmniOrchestrator:

1. Verifies that the service is currently stopped
2. Ensures that dependencies are satisfied before starting
3. Launches the service with the appropriate configuration
4. Performs health checks to verify successful startup
5. Updates the service registry to reflect the running state
6. For some services, performs post-start initialization

## Core System Services

OmniOrchestrator includes several core services that can be managed:

### On All Hosts

- **orchestrator-core**: The central management component
- **network-agent**: Handles network connectivity and service discovery

### On Bastion Hosts

- **api-gateway**: Manages external API requests
- **auth-service**: Handles authentication and authorization
- **backup-manager**: Manages backup operations (if enabled)

### On Worker Hosts

- **container-runtime**: Runs containerized applications
- **storage-agent**: Manages persistent storage

### Monitoring Services (If Enabled)

- **metrics-collector**: Collects system metrics
- **alert-manager**: Processes and routes alerts

## Service Dependencies

Services in OmniOrchestrator have dependencies on other services. The command automatically handles these dependencies:

1. **Dependency Checks**: Prevents stopping services that others depend on without confirmation
2. **Dependency Resolution**: Ensures dependent services are running before starting a service
3. **Cascading Operations**: Optionally performs cascading restarts of dependent services

## Service Health Checks

After starting or restarting a service, OmniOrchestrator performs health checks:

1. **Process Check**: Verifies the process is running
2. **Port Check**: Ensures the service is listening on its expected ports
3. **API Check**: For some services, makes API calls to verify functionality
4. **Dependency Check**: Verifies communication with dependent services

## Service Logs

When managing services, you can view service-specific logs:

```bash
$ omni service logs worker1 container-runtime
```

This will display recent logs for the specified service, helping diagnose issues.

## Notes

- Some services are critical for OmniOrchestrator operation; exercise caution when stopping them
- Service operations may take time to complete, especially for complex services
- For production environments, consider using maintenance windows for service operations
- Service configurations can be adjusted using the `omni config` commands
- The service command operates directly on the system level and should be used with care
- For routine application management, use the application-specific commands instead
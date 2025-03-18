# `omni logs` - View Application Logs

## Overview

The `omni logs` command provides access to the logs of your application components and OmniOrchestrator services. This command is essential for debugging issues, monitoring application behavior, and understanding system performance. It offers flexible filtering options and real-time log streaming capabilities.

## Usage

```
omni logs [--host HOST] [--service SERVICE] [--tail LINES]
```

### Options

- `--host HOST`: Specify the host to view logs from
- `--service SERVICE`: Specify the service to view logs for
- `--tail LINES`: Number of lines to show (defaults to 100)

When run without options, the command operates in interactive mode, prompting for the necessary information.

## Workflow

The `omni logs` command follows this workflow:

1. **Component Selection**: Choose which component to view logs for
2. **Log Retrieval**: Connects to the appropriate host and retrieves the specified logs
3. **Display**: Shows the logs with timestamps, severity levels, and sources

## Example

```bash
$ omni logs

Select component:
> Web Frontend
  API Backend
  Database
  All Components

📋 Application Logs
[2025-03-18 15:32:17] INFO: Service health check passed
[2025-03-18 15:32:25] DEBUG: Processing incoming request
[2025-03-18 15:32:26] INFO: Cache hit ratio: 78.5%
[2025-03-18 15:32:30] WARN: High memory usage detected
```

## Log Format

Logs displayed by the command follow this general format:

```
[TIMESTAMP] LEVEL: Message
```

Where:
- **TIMESTAMP**: The date and time when the log entry was created
- **LEVEL**: The severity level (e.g., INFO, DEBUG, WARN, ERROR)
- **Message**: The actual log message

## Log Filtering

You can filter logs by various criteria:

### By Host

```bash
$ omni logs --host worker1
```

This shows logs only from the specified host.

### By Service

```bash
$ omni logs --service api-backend
```

This shows logs only from the specified service.

### Combined Filters

```bash
$ omni logs --host worker1 --service container-runtime
```

This shows logs from the container-runtime service running on worker1.

## Log Streaming

By default, the `omni logs` command shows historical logs and then exits. To continuously stream logs in real-time, add the `--follow` or `-f` flag:

```bash
$ omni logs --follow
```

This will continuously display new log entries as they are generated until you press Ctrl+C to stop.

## Log Severity Levels

The logs displayed use standard severity levels:

- **DEBUG**: Detailed information for debugging purposes
- **INFO**: Informational messages about normal operation
- **WARN**: Warning messages that don't represent errors but may indicate potential issues
- **ERROR**: Error messages indicating failures
- **FATAL**: Critical errors that cause the application to terminate

## Log Sources

Logs can come from various sources in your OmniOrchestrator environment:

1. **Application Components**: Your deployed applications
2. **Container Runtime**: The container engine running your workloads
3. **OmniOrchestrator Services**: System services like orchestrator-core
4. **System Logs**: Host-level logs from the underlying operating system

## Log Storage and Retention

Logs in OmniOrchestrator are:

1. **Collected**: Gathered from all hosts and services
2. **Aggregated**: Combined into a centralized logging system
3. **Indexed**: Made searchable for quick retrieval
4. **Stored**: Kept according to your configured retention policy

By default, logs are retained for 7 days, but this can be configured in your OmniOrchestrator settings.

## Advanced Log Analysis

For more complex log analysis:

1. **Log Searching**: Use `omni logs --search "keyword"` to find specific log entries
2. **Time Windows**: Use `omni logs --since "1h"` to view logs from the last hour
3. **Output Format**: Use `omni logs --output json` to get logs in JSON format for further processing

## Notes

- Large log volumes may be truncated in the terminal output
- For extensive log analysis, consider exporting logs to a dedicated analysis tool
- Use `grep` and other text processing tools in combination with `omni logs` for more advanced filtering
- The logging system is designed to have minimal performance impact on your applications
- Sensitive information is automatically redacted from logs based on your security settings
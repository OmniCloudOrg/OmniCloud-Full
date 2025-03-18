# `omni backup` - Manage Backup Operations

## Overview

The `omni backup` command group provides tools for managing backup operations in your OmniOrchestrator environment. This command allows you to trigger immediate backups, list available backups, and restore your system from previous backups. Proper backup management is critical for ensuring data safety and business continuity.

## Usage

```
omni backup <subcommand> [options]
```

### Subcommands

- `now`: Trigger an immediate backup
- `list`: List available backups
- `restore <id>`: Restore from a specific backup

## Subcommand: `omni backup now`

The `now` subcommand triggers an immediate backup of your environment, independent of the scheduled backup system.

### Example

```bash
$ omni backup now

🔄 Initiating backup process...
[==================================================] 100% Scanning for changes
[==================================================] 100% Creating database snapshots
[==================================================] 100% Archiving configuration
[==================================================] 100% Compressing backup
[==================================================] 100% Encrypting backup
[==================================================] 100% Uploading to storage
✓ Backup completed successfully!

Backup Details:
ID:                 bkp_20250318164237
Timestamp:          2025-03-18 16:42:37 UTC
Size:               1.24 GB (compressed)
Type:               Full
Location:           Primary Storage
Retention Period:   90 days
Expires:            2025-06-16
```

### Backup Types

OmniOrchestrator supports different backup types:

1. **Full Backup**: Complete snapshot of all data and configuration
2. **Incremental Backup**: Only changes since the last backup
3. **Differential Backup**: All changes since the last full backup
4. **Configuration-Only Backup**: Just the system configuration, not application data

The default for manual backups is a Full Backup, but this can be changed with options:

```bash
$ omni backup now --type incremental
```

### Backup Scope

You can control what gets backed up:

```bash
$ omni backup now --include-apps --exclude-logs
```

Common scope options include:
- `--include-apps`: Include application data
- `--include-db`: Include databases
- `--include-config`: Include configuration files
- `--exclude-logs`: Exclude log files
- `--exclude-cache`: Exclude cache directories

## Subcommand: `omni backup list`

The `list` subcommand displays available backups with their details.

### Example

```bash
$ omni backup list

💾 Available Backups

┌─────────────────────┬─────────────────────┬──────┬────────┬─────────────────┬────────────┐
│ ID                  │ Timestamp           │ Type │ Size   │ Status          │ Expiration │
├─────────────────────┼─────────────────────┼──────┼────────┼─────────────────┼────────────┤
│ bkp_20250318164237  │ 2025-03-18 16:42:37 │ Full │ 1.24GB │ Available       │ 2025-06-16 │
├─────────────────────┼─────────────────────┼──────┼────────┼─────────────────┼────────────┤
│ bkp_20250317030000  │ 2025-03-17 03:00:00 │ Full │ 1.21GB │ Available       │ 2025-06-15 │
├─────────────────────┼─────────────────────┼──────┼────────┼─────────────────┼────────────┤
│ bkp_20250316030000  │ 2025-03-16 03:00:00 │ Full │ 1.20GB │ Available       │ 2025-06-14 │
├─────────────────────┼─────────────────────┼──────┼────────┼─────────────────┼────────────┤
│ bkp_20250315030000  │ 2025-03-15 03:00:00 │ Full │ 1.18GB │ Available       │ 2025-06-13 │
└─────────────────────┴─────────────────────┴──────┴────────┴─────────────────┴────────────┘

Backup Summary:
Total Backups:      4
Storage Used:       4.83 GB
Backup Schedule:    Daily at 03:00 UTC
Retention Policy:   90 days
```

### Filtering Options

You can filter the backup list with various options:

```bash
$ omni backup list --last 10 --type full
```

Common filtering options include:
- `--last N`: Show only the N most recent backups
- `--since "YYYY-MM-DD"`: Show backups since a specific date
- `--type TYPE`: Filter by backup type (full, incremental, differential)
- `--status STATUS`: Filter by status (available, archived, expiring)

### Detailed View

For more details about a specific backup:

```bash
$ omni backup show bkp_20250318164237
```

This displays comprehensive information about the backup, including exact contents and restore options.

## Subcommand: `omni backup restore`

The `restore` subcommand restores your environment from a specified backup.

### Example

```bash
$ omni backup restore bkp_20250318164237

⚠️ Restore Operation Warning
You are about to restore your environment from backup:
  ID:        bkp_20250318164237
  Timestamp: 2025-03-18 16:42:37 UTC
  Type:      Full

This will replace current data with data from the backup.
Are you sure you want to proceed? [y/N] y

Select restore scope:
> Full environment (configuration and data)
  Configuration only
  Application data only
  Database only

🔄 Initiating restore process...
[==================================================] 100% Preparing restore
[==================================================] 100% Stopping services
[==================================================] 100% Downloading backup
[==================================================] 100% Decrypting backup
[==================================================] 100% Extracting backup
[==================================================] 100% Restoring configuration
[==================================================] 100% Restoring databases
[==================================================] 100% Restoring application data
[==================================================] 100% Starting services
[==================================================] 100% Verifying restore
✓ Restore completed successfully!

Environment has been restored to the state of 2025-03-18 16:42:37 UTC.
Run 'omni status' to verify your environment.
```

### Restore Scope

You can control what gets restored:

```bash
$ omni backup restore bkp_20250318164237 --scope config
```

Common scope options include:
- `--scope full`: Full environment (default)
- `--scope config`: Configuration only
- `--scope data`: Application data only
- `--scope db`: Database only

### Selective Restore

For more granular control, you can restore specific components:

```bash
$ omni backup restore bkp_20250318164237 --component database --selective
```

This will prompt you to select specific databases or tables to restore.

## Backup Storage

OmniOrchestrator stores backups according to your configured storage policy:

1. **Primary Storage**: Fast access storage for recent backups
2. **Archive Storage**: Long-term storage for older backups
3. **Off-site Storage**: Optional remote storage for disaster recovery

You can manage storage settings with:

```bash
$ omni config edit backup-storage
```

## Backup Automation

While `omni backup now` triggers manual backups, OmniOrchestrator includes a scheduled backup system configured during initialization. To adjust the schedule:

```bash
$ omni config edit backup-schedule
```

Common scheduling options include:
- Daily backups at a specific time
- Weekly backups on specific days
- Monthly backups on specific dates
- Different schedules for different backup types

## Notes

- Backup operations can be resource-intensive; consider scheduling them during off-peak hours
- Encryption keys are critical for restore operations; ensure they are securely stored
- Test restore procedures regularly to ensure they work when needed
- For large environments, consider incremental backups to reduce storage and time requirements
- Off-site backups are strongly recommended for production environments
- The backup system is designed to minimize impact on running applications
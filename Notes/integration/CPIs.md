# Cloud Provider Interface (CPI) Specification

## 1. Introduction

The Cloud Provider Interface (CPI) system provides a unified way to interact with different cloud and virtualization platforms through a consistent API. Rather than writing custom code for each provider, developers can leverage the CPI's abstraction layer, which translates standardized commands into provider-specific actions. This specification outlines how the system works, the structure of provider definitions, and patterns for effective implementation and usage.

## 2. Core Architecture

The CPI framework uses a modular design to separate concerns and maintain flexibility. At its heart, the system translates high-level requests into provider-specific commands, executes those commands, and transforms their outputs into structured data.

### 2.1 System Components

The CPI system relies on several interconnected components to function. The Provider Registry serves as the central hub for managing available provider implementations, while the Executor handles command execution with proper parameter substitution. Command outputs flow through the Parser, which extracts structured data according to defined rules. A Validator ensures all provider definitions adhere to the required schema. The Error Handler creates uniform error representations across different providers, and the Logger captures detailed information for troubleshooting and auditing.

### 2.2 Initialization Flow

When a CPI system initializes, it goes through several important stages to discover, validate, and register providers. This process ensures that only valid providers are available for use.

```mermaid
flowchart LR
    A[Initialize CPI System] --> B[Discover & Load Provider Files]
    B --> C[Validate Provider Specs]
    C --> D[Log Results]
    D --> E[Register Valid Providers]
    E --> F[Providers Ready for Use]
```

## 3. Provider Definition Schema

### 3.1 Provider File Structure

Each provider is defined in a JSON file with the following top-level structure:

```json
{
  "name": "provider_name",
  "type": "command",
  "default_settings": {
    "setting1": "default_value1",
    "setting2": "default_value2"
  },
  "actions": {
    "action1": { ... },
    "action2": { ... }
  }
}
```

| Field | Description | Required |
|-------|-------------|----------|
| `name` | Unique provider identifier | Yes |
| `type` | Provider type (e.g., command, virt, cloud, container) | Yes |
| `default_settings` | Default parameters for actions | No |
| `actions` | Collection of available actions | Yes |

### 3.2 Action Definition

Each action defines a command to execute and how to parse its output:

```json
"action_name": {
  "command": "executable {param1} --option {param2}",
  "params": ["param1", "param2"],
  "pre_exec": [ ... ],
  "post_exec": [ ... ],
  "parse_rules": { ... }
}
```

| Field | Description | Required |
|-------|-------------|----------|
| `command` | Command template with parameter placeholders | Yes |
| `params` | List of required parameters | No |
| `pre_exec` | Actions to execute before the main command | No |
| `post_exec` | Actions to execute after the main command | No |
| `parse_rules` | Rules for parsing the command output | Yes |

## 4. Parameter Handling

> [!WARNING]
> Below are parameters that OmniCloud natively supports and will request from administrators during setup or when commands are executed, depending on parameter type. These parameters will always be available to you.
> While Omni services allow you to add custom parameters and will prompt users for additional information as needed, it's recommended to use these predefined parameters when possible for consistency.

### What are Parameters?
Parameters are OmniCloud's system that allows CPI developers to provide user parameters to CPI-defined commands. You use them when you want to prompt the user for a piece of informationn related to a backend operation you need to perform

#### For Example

In the ecample below we use the AWS EC2 CLI to list out running instances in our AWS account. This command usage requires a region to be specified (which ideally is provided by the user)

In order to tell OmniCloud we would like the user to provide the `region` parameter for the `list_instances` method we simply place a set of curly braces where we would have placed the parameter and place the parameter name inside.

```json
{
  "name": "provider_name",
  "type": "command",
  "default_settings": {
    "setting1": "default_value1",
    "setting2": "default_value2"
  },
  "actions": {
    "list_instances": {
      "command": "aws ec2 describe-instances --region {region} --output json",
      "params": ["region"],
      "parse_rules": {
        "type": "object",
        "patterns": {
          "output": {
            "regex": "(.*)",
            "group": 1
          }
        }
      }
    }
  }
}
```

Whenever an administrator wants to, for example, create an instance from the OmniCloud Dashboard the dashboard can query the Omni api to see what parameters the method required for a given CPI. The request we make is shown below:

![API Request Screenshot](https://github.com/user-attachments/assets/ec752af5-ef04-45d7-8947-b336d137cf9b)

### 4.1 Parameter Substitution

Parameters are referenced in command templates using curly braces:

```
"command": "aws ec2 describe-instances --region {region} --output json"
```

### 4.2 Parameter Resolution

Parameters are resolved in the following order:

1. User-provided parameters specific to the action call
2. Provider's default settings
3. If parameter is not found, an error is raised (for required parameters)

### 4.3 Optional Parameters

Parameters can be constructed conditionally by using separate parameters:

```json
"ssh_key_param": "--ssh-key {ssh_key}"
```

This allows parameters to be included only when needed.


### 4.4 Global Parameters

#### 4.4.1 General Parameters

These parameters are commonly used across multiple commands and providers:

| Parameter | Description | Example Value |
|-----------|-------------|---------------|
| `region` | Geographic region where resources are located | `us-east-1`, `eu-west-3` |
| `zone` | Availability zone within a region | `us-east-1a` |
| `project_id` | Identifier for a specific project or organization | `project-12345` |
| `api_key` | Authentication key for API access | `a1b2c3d4e5f6...` |
| `output_format` | Format for command output | `json`, `yaml`, `table` |
| `wait` | Whether to wait for operation completion | `true`, `false` |
| `timeout` | Maximum time to wait for operation completion (seconds) | `300` |
| `dry_run` | Validate the request without making changes | `true`, `false` |
| `force` | Skip confirmation prompts | `true`, `false` |
| `tags` | Labels applied to resources (key-value pairs) | `{"env":"prod","team":"infra"}` |
| `profile` | Named set of credentials and settings | `production`, `development` |
| `cli_path` | Path to CLI executable | `/usr/local/bin/aws` |
| `cli_version` | Version of CLI to use | `2.11.3` |
| `verbose` | Enable verbose output | `true`, `false` |
| `debug` | Enable debug level logging | `true`, `false` |

#### 4.4.2 Worker Management Parameters

These parameters are used for creating and managing workers (usually virtual machines):

| Parameter | Description | Example Value |
|-----------|-------------|---------------|
| `name` | Name of the worker | `web-server-01` |
| `worker_id` | Unique identifier for a worker | `i-0abc123def456789` |
| `worker_type` | Size or type of worker (CPU, memory, etc.) | `standard-2`, `t2.micro` |
| `image` | OS image to use for the worker | `ubuntu-20.04`, `ami-12345678` |
| `disk_size_gb` | Size of the root disk in gigabytes | `50` |
| `memory_mb` | Amount of memory in megabytes | `2048` |
| `vcpus` | Number of virtual CPUs | `2` |
| `ssh_key` | SSH key for remote access | `ssh-rsa AAAA...` |
| `ssh_key_name` | Name of a registered SSH key | `my-key-pair` |
| `user_data` | Initialization script or data | Base64-encoded script |
| `private_networking` | Enable private network interfaces | `true`, `false` |
| `ipv6` | Enable IPv6 networking | `true`, `false` |
| `backups_enabled` | Enable automatic backups | `true`, `false` |
| `monitoring_enabled` | Enable detailed monitoring | `true`, `false` |
| `shutdown_behavior` | Action on shutdown | `stop`, `terminate` |
| `placement_group` | Server placement strategy group | `cluster-1` |
| `host_id` | Specific host for placement | `h-0abc123def456789` |
| `dedicated` | Use dedicated hardware | `true`, `false` |
| `hibernation` | Enable hibernation support | `true`, `false` |
| `root_volume_type` | Type of root storage volume | `ssd`, `standard` |
| `boot_type` | Boot type for workers | `local`, `network` |
| `timezone` | Timezone for the worker | `UTC`, `America/New_York` |
| `hostname` | Hostname for the worker | `srv01.example.com` |

#### 4.4.3 Storage Management Parameters

For managing storage volumes and disks:

| Parameter | Description | Example Value |
|-----------|-------------|---------------|
| `disk_id` | Unique identifier for a disk/volume | `vol-0abc123def456789` |
| `disk_name` | Name of the disk/volume | `data-volume-1` |
| `disk_type` | Type of storage | `ssd`, `hdd`, `premium-ssd` |
| `disk_format` | Format of the disk | `raw`, `qcow2`, `vhd` |
| `size_gb` | Size in gigabytes | `100` |
| `iops` | Input/output operations per second | `3000` |
| `throughput` | Throughput in MB/s | `125` |
| `snapshot_id` | Identifier for a disk snapshot | `snap-0abc123def456789` |
| `snapshot_name` | Name for a snapshot | `web-server-backup` |
| `snapshot_description` | Description of a snapshot | `Daily backup - March 17` |
| `encryption_key` | Key for disk encryption | `arn:aws:kms:...` |
| `encryption_enabled` | Enable disk encryption | `true`, `false` |
| `attachment_point` | Device name for attachment | `/dev/sdf`, `xvdh` |
| `multi_attach` | Allow multiple attachments | `true`, `false` |
| `disk_sku` | SKU/tier of the disk | `Standard_LRS`, `Premium_LRS` |
| `storage` | Storage pool name | `local`, `shared` |
| `filesystem` | Filesystem type | `ext4`, `xfs`, `ntfs` |
| `mount_point` | Mount point for the volume | `/data`, `/mnt/volume1` |
| `target_dev` | Target device for attachment | `vdb`, `sdc` |
| `shared` | Whether the disk is shared | `true`, `false` |

#### 4.4.4 Network Management Parameters

For network configuration and management:

| Parameter | Description | Example Value |
|-----------|-------------|---------------|
| `network_id` | Identifier for a network | `vpc-0abc123def456789` |
| `network` | Name of a network | `default`, `prod-network` |
| `subnet_id` | Identifier for a subnet | `subnet-0abc123def456789` |
| `subnet` | Name of a subnet | `public-subnet-1` |
| `ip_address` | Specific IP address | `192.168.1.10` |
| `cidr_range` | CIDR notation for IP range | `10.0.0.0/16` |
| `gateway` | Gateway IP address | `10.0.0.1` |
| `dns_servers` | DNS server addresses | `["8.8.8.8","8.8.4.4"]` |
| `vpc_id` | Virtual Private Cloud identifier | `vpc-0abc123def456789` |
| `security_group_id` | Security group identifier | `sg-0abc123def456789` |
| `security_group` | Security group name | `web-servers` |
| `firewall_group_id` | Firewall group identifier | `fg-0abc123def456789` |
| `port` | Network port number | `80` |
| `protocol` | Network protocol | `tcp`, `udp`, `icmp` |
| `load_balancer_id` | Load balancer identifier | `lb-0abc123def456789` |
| `load_balancer_name` | Load balancer name | `web-lb` |
| `target_group` | Load balancer target group | `web-targets` |
| `certificate_id` | SSL certificate identifier | `cert-0abc123def456789` |
| `domain_name` | Domain name for DNS records | `example.com` |
| `record_type` | DNS record type | `A`, `CNAME`, `MX` |
| `ttl` | Time to live for DNS records | `300` |
| `public_ip` | Enable public IP address | `true`, `false` |
| `ipv4_enabled` | Enable IPv4 support | `true`, `false` |
| `ipv6_enabled` | Enable IPv6 support | `true`, `false` |
| `floating_ip_id` | Identifier for floating/elastic IP | `eip-0abc123def456789` |
| `network_bridge` | Network bridge device | `vmbr0` |
| `network_type` | Type of network | `nat`, `bridge`, `private` |
| `netif` | Network interface type | `virtio`, `e1000` |
| `network_index` | Index of network interface | `1`, `2` |
| `destination` | Destination CIDR for routing | `0.0.0.0/0` |
| `source` | Source CIDR for firewall rules | `192.168.1.0/24` |

#### 4.4.5 Authentication and Credentials Parameters

For authentication and credential management:

| Parameter | Description | Example Value |
|-----------|-------------|---------------|
| `access_key` | Access key ID | `AKIAIOSFODNN7EXAMPLE` |
| `secret_key` | Secret access key | `wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY` |
| `token` | Authentication token | `eyJhbGciOiJIUzI1NiIsInR5...` |
| `key_file` | Path to key file | `~/.ssh/id_rsa` |
| `password` | Password for authentication | `P@ssw0rd123!` |
| `root_pass` | Root password for new workers | `ComplexP@ssw0rd!` |
| `client_id` | OAuth client ID | `1234567890abcdef` |
| `client_secret` | OAuth client secret | `1234567890abcdef1234567890abcdef` |
| `tenant_id` | Tenant/organization identifier | `tenant-12345` |
| `subscription_id` | Subscription identifier | `sub-12345` |
| `credential_id` | Identifier for a stored credential | `cred-12345` |
| `certificate` | Authentication certificate | `-----BEGIN CERTIFICATE-----...` |
| `username` | Username for authentication | `admin` |
| `login_user` | Login username for new workers | `root`, `azureuser` |
| `mfa_token` | Multi-factor authentication token | `123456` |
| `role_arn` | Role ARN for assumed roles | `arn:aws:iam::123456789012:role/example` |
| `session_duration` | Duration for temporary credentials | `3600` |
| `auth_url` | Authentication service URL | `https://auth.example.com` |
| `authorized_keys` | Authorized SSH keys | `["ssh-rsa AAA...", "ssh-ed25519 AAA..."]` |
| `ssh_key_ids` | IDs of SSH keys to use | `["key-12345", "key-67890"]` |
| `ssh_key_file` | Path to SSH key file | `~/.ssh/id_rsa.pub` |
| `ssh_public_key_file` | Path to SSH public key file | `~/.ssh/id_rsa.pub` |

#### 4.4.6 Container and Orchestration Parameters

For container orchestration services:

| Parameter | Description | Example Value |
|-----------|-------------|---------------|
| `cluster_id` | Identifier for a Kubernetes cluster | `cluster-12345` |
| `cluster_name` | Name of the cluster | `prod-cluster` |
| `node_pool_id` | Identifier for a node pool | `pool-12345` |
| `node_count` | Number of nodes in a cluster | `3` |
| `node_type` | Type of nodes | `standard-2` |
| `kubernetes_version` | Version of Kubernetes | `1.26.3` |
| `auto_upgrade` | Enable automatic upgrades | `true`, `false` |
| `registry_id` | Container registry identifier | `reg-12345` |
| `image_name` | Container image name | `nginx` |
| `image_tag` | Container image tag | `latest`, `1.21` |
| `container_port` | Port exposed by container | `80` |
| `pod_cidr` | CIDR range for pods | `10.100.0.0/16` |
| `service_cidr` | CIDR range for services | `10.200.0.0/16` |
| `cluster_autoscaling` | Enable cluster autoscaling | `true`, `false` |
| `min_nodes` | Minimum number of nodes | `1` |
| `max_nodes` | Maximum number of nodes | `10` |
| `node_locations` | Locations for cluster nodes | `["us-central1-a", "us-central1-b"]` |
| `cluster_version` | Version of the cluster | `1.26.3-gke.1000` |
| `networking_mode` | Cluster networking mode | `vpc-native`, `routes` |
| `registry_name` | Name of container registry | `my-registry` |
| `repository` | Container repository name | `my-app` |
| `plugin_name` | Name of Kubernetes plugin | `kube-dns` |
| `addons` | Cluster addons to enable | `["monitoring", "http_load_balancing"]` |
| `endpoint` | Cluster API endpoint | `https://10.10.10.10` |
| `kubeconfig` | Path to kubeconfig file | `~/.kube/config` |

#### 4.4.7 Database Service Parameters

For database services:

| Parameter | Description | Example Value |
|-----------|-------------|---------------|
| `database_id` | Identifier for a database instance | `db-12345` |
| `database_name` | Name of the database | `product_db` |
| `database_type` | Database engine type | `mysql`, `postgres` |
| `database_version` | Version of the database engine | `5.7`, `13` |
| `master_username` | Admin username | `admin` |
| `master_password` | Admin password | `ComplexP@ssw0rd!` |
| `db_port` | Database port | `3306`, `5432` |
| `backup_retention` | Days to retain backups | `7` |
| `backup_window` | Preferred backup window | `03:00-04:00` |
| `maintenance_window` | Preferred maintenance window | `sun:05:00-sun:06:00` |
| `storage_gb` | Storage size in gigabytes | `100` |
| `db_iops` | Input/output operations per second | `1000` |
| `auto_minor_upgrade` | Enable automatic minor version upgrades | `true`, `false` |
| `multi_az` | Enable multi-availability zone deployment | `true`, `false` |
| `publicly_accessible` | Make publicly accessible | `true`, `false` |
| `backup_id` | Identifier for a database backup | `backup-12345` |
| `parameter_group` | Database parameter group | `default.mysql5.7` |
| `character_set` | Database character set | `utf8mb4` |
| `collation` | Database collation | `utf8mb4_unicode_ci` |
| `replica_id` | Identifier for a read replica | `replica-12345` |
| `db_subnet_group` | Database subnet group | `default` |
| `db_cluster_id` | Database cluster identifier | `cluster-12345` |
| `db_instance_class` | Database instance class | `db.t3.medium` |
| `db_snapshot_id` | Database snapshot identifier | `snapshot-12345` |
| `db_name` | Logical database name | `products` |
| `engine` | Database engine | `mysql`, `postgres` |
| `storage_type` | Type of database storage | `gp2`, `io1` |

#### 4.4.8 Serverless and Functions Parameters

For serverless functions and applications:

| Parameter | Description | Example Value |
|-----------|-------------|---------------|
| `function_id` | Identifier for a function | `func-12345` |
| `function_name` | Name of the function | `process-orders` |
| `runtime` | Function runtime environment | `nodejs14.x`, `python3.9` |
| `handler` | Function handler | `index.handler` |
| `memory_size` | Memory allocation in MB | `128` |
| `timeout` | Function timeout in seconds | `30` |
| `code_path` | Path to function code | `./function.zip` |
| `environment_variables` | Environment variables | `{"DB_HOST":"db.example.com"}` |
| `role` | Execution role | `arn:aws:iam::123456789012:role/lambda-role` |
| `package_type` | Function package type | `Zip`, `Image` |
| `trigger_id` | Identifier for a function trigger | `trigger-12345` |
| `trigger_type` | Type of trigger | `http`, `queue`, `schedule` |
| `trigger_resource` | Resource that triggers the function | `arn:aws:sqs:...` |
| `payload` | Function input payload | `{"key":"value"}` |
| `async` | Invoke function asynchronously | `true`, `false` |
| `function_version` | Version of the function | `1`, `prod` |
| `function_url` | Function URL configuration | `true`, `false` |
| `function_layers` | Lambda layers to include | `["arn:aws:lambda:..."]` |
| `source_code` | Inline source code | `exports.handler = async (event) => {...}` |
| `dead_letter_queue` | Dead letter queue ARN | `arn:aws:sqs:...` |
| `reserved_concurrency` | Reserved concurrency | `10` |
| `tracing_config` | Function tracing configuration | `Active`, `PassThrough` |
| `deployment_package` | Deployment package URI | `s3://bucket/function.zip` |

#### 4.4.9 Monitoring and Logging Parameters

For monitoring and logging services:

| Parameter | Description | Example Value |
|-----------|-------------|---------------|
| `metric_name` | Name of the metric | `CPUUtilization` |
| `namespace` | Metric namespace | `AWS/EC2` |
| `statistic` | Statistic type | `Average`, `Sum`, `Maximum` |
| `period` | Data point period in seconds | `60` |
| `start_time` | Start time for data retrieval | `2025-03-16T00:00:00Z` |
| `end_time` | End time for data retrieval | `2025-03-17T00:00:00Z` |
| `dimensions` | Metric dimensions | `{"InstanceId":"i-1234567890abcdef0"}` |
| `threshold` | Alarm threshold value | `80` |
| `comparison_operator` | Threshold comparison operator | `GreaterThanThreshold` |
| `evaluation_periods` | Number of evaluation periods | `3` |
| `alarm_id` | Identifier for an alarm | `alarm-12345` |
| `alarm_name` | Name of the alarm | `high-cpu-alarm` |
| `notification_arn` | Notification target | `arn:aws:sns:...` |
| `log_group` | Log group name | `/aws/lambda/my-function` |
| `log_stream` | Log stream name | `2025/03/17/[$LATEST]abcdef` |
| `filter_pattern` | Log filter pattern | `ERROR` |
| `dashboard_id` | Identifier for a dashboard | `dashboard-12345` |
| `dashboard_name` | Name of the dashboard | `production-overview` |
| `widget_type` | Dashboard widget type | `metric`, `text`, `log` |
| `widget_title` | Dashboard widget title | `CPU Utilization` |
| `alert_id` | Identifier for an alert | `alert-12345` |
| `alert_name` | Name of the alert | `high-cpu-alert` |
| `notification_type` | Type of notification | `email`, `sms`, `webhook` |
| `notification_target` | Target for notifications | `admin@example.com` |
| `severity` | Alert severity | `critical`, `warning`, `info` |
| `time_range` | Time range for queries | `1h`, `24h`, `7d` |
| `query` | Query string | `rate(http_requests_total[5m])` |

#### 4.4.10 Object Storage Parameters

For object storage services:

| Parameter | Description | Example Value |
|-----------|-------------|---------------|
| `bucket_name` | Name of the storage bucket | `my-assets` |
| `object_key` | Key/path of the object | `images/logo.png` |
| `local_path` | Local file path | `./logo.png` |
| `content_type` | MIME type of the object | `image/png` |
| `acl` | Access control list | `private`, `public-read` |
| `storage_class` | Storage tier/class | `STANDARD`, `GLACIER` |
| `encryption` | Encryption settings | `AES256` |
| `website_enabled` | Enable static website hosting | `true`, `false` |
| `index_document` | Index document for website | `index.html` |
| `error_document` | Error document for website | `error.html` |
| `versioning` | Enable object versioning | `true`, `false` |
| `lifecycle_policy` | Object lifecycle policy | JSON policy document |
| `cors_rules` | Cross-origin resource sharing rules | JSON CORS configuration |
| `prefix` | Object prefix for listing | `images/` |
| `delimiter` | Delimiter for listing objects | `/` |
| `max_keys` | Maximum number of keys to return | `1000` |
| `if_match` | Condition on ETag | `"686897696a7c876b7e"` |
| `if_modified_since` | Condition on modification time | `2025-03-10T12:00:00Z` |
| `expires` | Expiration time for URLs | `3600` |
| `cache_control` | Cache-Control header | `max-age=86400` |
| `content_disposition` | Content-Disposition header | `attachment; filename="file.txt"` |
| `content_encoding` | Content-Encoding header | `gzip` |
| `metadata` | User-defined metadata | `{"project":"website"}` |

#### 4.4.11 Auto-Scaling Parameters

For auto-scaling configurations:

| Parameter | Description | Example Value |
|-----------|-------------|---------------|
| `scaling_group_id` | Identifier for a scaling group | `asg-12345` |
| `min_size` | Minimum group size | `1` |
| `max_size` | Maximum group size | `10` |
| `desired_capacity` | Desired group size | `3` |
| `cooldown` | Cooldown period in seconds | `300` |
| `health_check_type` | Health check type | `EC2`, `ELB` |
| `health_check_grace` | Health check grace period | `300` |
| `scaling_policy_id` | Identifier for a scaling policy | `policy-12345` |
| `adjustment_type` | Adjustment type | `ChangeInCapacity` |
| `scaling_adjustment` | Scaling adjustment amount | `1` |
| `metric_name` | Metric for scaling | `CPUUtilization` |
| `metric_namespace` | Metric namespace | `AWS/EC2` |
| `statistic` | Statistic for scaling | `Average` |
| `threshold` | Scaling threshold | `75` |
| `period` | Evaluation period | `60` |
| `evaluation_periods` | Number of evaluation periods | `3` |
| `schedule_expression` | Schedule for scaling | `cron(0 9 * * ? *)` |
| `scaling_group_name` | Name of scaling group | `web-servers` |
| `vpc_zone_identifier` | Subnets for scaling group | `subnet-12345,subnet-67890` |
| `launch_template_id` | Launch template ID | `lt-12345` |
| `launch_configuration` | Launch configuration name | `web-server-config` |
| `termination_policies` | Termination policies | `["OldestInstance"]` |
| `load_balancer_names` | Load balancer names | `["web-lb"]` |
| `target_group_arns` | Target group ARNs | `["arn:aws:elasticloadbalancing:..."]` |
| `placement_group` | Placement group | `cluster-1` |
| `service_linked_role_arn` | Service linked role ARN | `arn:aws:iam::...` |
| `new_instances_protected` | Protect new instances | `true`, `false` |

#### 4.4.12 Identity and Access Management Parameters

For IAM operations:

| Parameter | Description | Example Value |
|-----------|-------------|---------------|
| `user_id` | Identifier for a user | `user-12345` |
| `user_name` | Username | `jdoe` |
| `group_id` | Identifier for a group | `group-12345` |
| `group_name` | Group name | `developers` |
| `role_id` | Identifier for a role | `role-12345` |
| `role_name` | Role name | `admin-role` |
| `policy_id` | Identifier for a policy | `policy-12345` |
| `policy_name` | Policy name | `s3-read-only` |
| `policy_document` | Policy document content | JSON IAM policy |
| `permission_boundary` | Permission boundary | `arn:aws:iam::...` |
| `path` | Path for IAM resources | `/service-role/` |
| `max_session_duration` | Maximum session duration (seconds) | `3600` |
| `mfa_enabled` | Enable multi-factor authentication | `true`, `false` |
| `password_reset_required` | Require password reset | `true`, `false` |
| `console_access` | Allow console access | `true`, `false` |
| `permissions` | Permission list | `["read", "write", "delete"]` |
| `assume_role_policy` | Assume role policy document | JSON trust policy |
| `inline_policy` | Inline policy document | JSON policy document |
| `password_policy` | Password policy settings | JSON password policy |
| `iam_path` | Path for IAM entity | `/` |
| `account_id` | AWS account ID | `123456789012` |
| `access_key_status` | Access key status | `Active`, `Inactive` |

### 4.5 Command-Specific Parameters

These parameters are specific to certain command types:

#### 4.5.1 Worker Commands

| Command | Parameter | Description | Example Value |
|---------|-----------|-------------|---------------|
| `create_worker` | `hostname_param` | Hostname parameter | `--hostname my-server` |
| `create_worker` | `ipv6_param` | IPv6 parameter | `--ipv6 enabled` |
| `create_worker` | `ssh_keys_param` | SSH keys parameter | `--ssh-keys key1,key2` |
| `create_worker` | `tag_param` | Tag parameter | `--tag production` |
| `stop_worker` | `stop_type_param` | Stop type parameter | `--type hard` |
| `update_worker` | `vcpus_param` | vCPUs parameter | `--vcpus 4` |
| `update_worker` | `memory_param` | Memory parameter | `--memory 8192` |
| `update_worker` | `hours_param` | Hours parameter | `--hours 24` |

#### 4.5.2 Storage Commands

| Command | Parameter | Description | Example Value |
|---------|-----------|-------------|---------------|
| `create_disk` | `disk_size_param` | Disk size parameter | `--size 100GB` |
| `attach_disk` | `device_param` | Device parameter | `--device /dev/sdf` |
| `create_snapshot` | `description_param` | Description parameter | `--description "Daily backup"` |

#### 4.5.3 Network Commands

| Command | Parameter | Description | Example Value |
|---------|-----------|-------------|---------------|
| `create_firewall_rule` | `direction_param` | Direction parameter | `--direction ingress` |
| `create_firewall_rule` | `port_param` | Port parameter | `--port 80` |
| `create_firewall_rule` | `source_param` | Source parameter | `--source 0.0.0.0/0` |
| `create_firewall_rule` | `destination_param` | Destination parameter | `--destination 10.0.0.0/8` |
| `create_firewall_rule` | `comment_param` | Comment parameter | `--comment "Allow HTTP"` |

#### 4.5.4 CLI Setup and Environment Parameters

For CLI and environment configuration:

| Parameter | Description | Example Value |
|-----------|-------------|---------------|
| `cli_path` | Path to CLI executable | `/usr/local/bin/aws` |
| `cli_version` | Version of CLI to install | `2.11.3` |
| `install_dir` | Directory for installation | `/usr/local/bin` |
| `config_file` | Path to configuration file | `~/.aws/config` |
| `credentials_file` | Path to credentials file | `~/.aws/credentials` |
| `profile` | Named profile to use | `development` |
| `cache_dir` | Directory for CLI cache | `~/.aws/cli/cache` |
| `plugin_dir` | Directory for CLI plugins | `~/.aws/cli/plugins` |
| `env_vars` | Environment variables to set | `{"AWS_REGION":"us-east-1"}` |
| `proxy` | Proxy server to use | `http://proxy.example.com:8080` |
| `no_verify_ssl` | Disable SSL verification | `true`, `false` |
| `debug` | Enable debug output | `true`, `false` |
| `output` | Output format | `json`, `text`, `table` |
| `color` | Enable color output | `true`, `false` |
| `query` | JMESPath query | `Reservations[*].Instances[*].[InstanceId]` |
| `no_paginate` | Disable pagination | `true`, `false` |
| `ignore_user_config` | Ignore user configuration | `true`, `false` |
| `api_version` | API version to use | `2016-04-01` |
| `endpoint_url` | Custom endpoint URL | `https://ec2.us-east-1.amazonaws.com` |

## 5. Parse Rules

Converting command-line output into structured data is one of the CPI's most powerful features. Parse rules define how to extract meaningful information from raw command outputs, whether they're formatted as JSON, tabular data, or unstructured text.

### 5.1 Parse Rule Types

The CPI supports three parsing strategies to handle different output formats:

#### 5.1.1 Object Parse Rules

The object parser extracts a single cohesive entity with multiple attributes. This works well for commands that return information about a specific resource, like a VM instance or storage volume. Each field is extracted using its own regex pattern:

```json
"parse_rules": {
  "type": "object",
  "patterns": {
    "id": {
      "regex": "Instance ID: ([\\w-]+)",
      "group": 1
    },
    "status": {
      "regex": "Status: (\\w+)",
      "group": 1
    }
  }
}
```

#### 5.1.2 Array Parse Rules

When dealing with lists of items, the array parser shines. It splits output by a defined separator (often newlines) and applies extraction patterns to each segment. This works perfectly for commands that list resources like VMs, storage volumes, or network interfaces:

```json
"parse_rules": {
  "type": "array",
  "separator": "\\n",
  "patterns": {
    "id": {
      "regex": "^(\\d+)",
      "group": 1,
      "transform": "number"
    },
    "name": {
      "regex": "^\\d+\\s+([^\\s]+)",
      "group": 1
    }
  }
}
```

#### 5.1.3 Properties Parse Rules

For more complex outputs with nested structures, the properties parser provides additional capabilities. Beyond basic property extraction, it can handle arrays of sub-objects and related patterns. This parser type excels at extracting detailed configuration information:

```json
"parse_rules": {
  "type": "properties",
  "patterns": {
    "id": {
      "regex": "ID: (\\d+)",
      "group": 1,
      "transform": "number"
    }
  },
  "array_patterns": {
    "network_adapters": {
      "prefix": "nic",
      "index": "\\d+",
      "object": {
        "type": {
          "regex": "^nic(\\d+)=\"(.*)\"$",
          "group": 2
        }
      }
    }
  },
  "related_patterns": { ... }
}
```

### 5.2 Pattern Specification

Each pattern consists of:

| Field | Description | Required |
|-------|-------------|----------|
| `regex` | Regular expression to match the desired data | Yes |
| `group` | Capture group index in the regex (default: 0) | No |
| `transform` | Value transformation: "boolean", "number" | No |
| `optional` | Whether the pattern is optional (default: false) | No |
| `match_value` | Reference to another value for comparison | No |

### 5.3 Value Transformation

Values can be transformed using:

- `boolean`: Converts extracted value to boolean
- `number`: Converts extracted value to number

## 6. Execution Workflow

### 6.1 Command Execution Process

The command execution flow represents how CPI processes a request from preparation to completion. Each request goes through a well-defined lifecycle that includes parameter preparation, command execution, output parsing, and result delivery.

```mermaid
flowchart LR
    A[Prepare Parameters] --> B[Execute Pre-Exec Actions]
    B --> C[Execute Main Command]
    C --> D[Parse Output]
    D --> E[Execute Post-Exec Actions]
    E --> F[Return Result to Caller]
```

### 6.2 Error Handling

Robust error handling makes the CPI system reliable and debuggable. All errors are captured in a standardized `CpiError` type, which provides consistent error reporting across different providers and operations.

When things go wrong during provider initialization or command execution, the CPI system returns meaningful errors that help diagnose and fix issues. Provider errors might indicate that a necessary cloud CLI tool isn't installed. Action errors often point to missing parameters or permissions problems. Parsing errors typically suggest that command output doesn't match the expected format, which might happen after a provider API change.

The system defines several error categories including `ProviderNotFound` when a requested provider isn't available, `ActionNotFound` when trying to use an undefined action, `MissingParameter` when required inputs are missing, and `ExecutionFailed` when a command doesn't complete successfully. Other error types handle parsing issues (`ParseError`), path problems (`InvalidPath`), specification errors (`InvalidCpiFormat`), provider loading failures (`NoProvidersLoaded`), I/O problems (`IoError`), JSON handling errors (`SerdeError`), regular expression issues (`RegexError`), and command timeouts (`Timeout`).

## 7. Provider Ecosystem

The CPI system boasts a rich ecosystem of providers covering major cloud platforms and virtualization technologies. This diversity allows applications to work with multiple infrastructure providers without code changes.

For public cloud environments, the CPI includes providers for AWS, Azure, and Google Cloud Platform, covering the biggest players in the market. It also supports popular alternative cloud providers like DigitalOcean, Linode, Vultr, OVH Cloud, Hetzner Cloud, Oracle Cloud, Scaleway, UpCloud, and DeTee. Each provider implements the common interface while respecting the unique characteristics of its underlying platform.

The system extends beyond public clouds to virtualization technologies. Users can manage KVM and QEMU virtual machines with the same API they use for cloud resources. Similarly, providers for Proxmox, VMware ESXi, and VirtualBox (with specific implementations for both Windows and Linux) bring enterprise and desktop virtualization platforms into the fold.

## 8. Working with the CPI System

### 8.1 Integration Examples

Working with the CPI system is straightforward once you understand the basic patterns. After initialization, your application can discover available providers, examine their capabilities, and execute actions with appropriate parameters. The following Rust example demonstrates a typical usage flow:

```rust
// Initialize the CPI system
let cpi = cpi::initialize()?;

// List available providers
let providers = cpi.get_providers();
println!("Available providers: {:?}", providers);

// Execute an action
let params = HashMap::new();
params.insert("region".to_string(), "us-east-1".into());

let result = cpi.execute("aws", "list_instances", params)?;
println!("Result: {:?}", result);
```

This pattern works identically across providers, allowing your code to remain consistent regardless of which cloud platform you're targeting. The CPI system handles the translation between your standardized requests and provider-specific commands.

### 8.2 Provider Definition Example

The AWS provider illustrates how provider definitions map cloud-specific commands to the CPI framework. Even this simple example shows the power of parameter substitution and output parsing:

```json
{
  "name": "aws",
  "type": "command",
  "default_settings": {
    "region": "us-east-1"
  },
  "actions": {
    "list_instances": {
      "command": "aws ec2 describe-instances --region {region} --output json",
      "params": ["region"],
      "parse_rules": {
        "type": "object",
        "patterns": {
          "output": {
            "regex": "(.*)",
            "group": 1
          }
        }
      }
    }
  }
}
```

The provider defines a default region while allowing users to override it. The `list_instances` action translates to the appropriate AWS CLI command with parameter substitution for the region. A simple parsing rule captures the entire output, which is already in JSON format from the AWS CLI.


## 9. Provider Implementation Guidelines

Creating a new CPI provider requires careful planning and attention to detail. Start by choosing descriptive, consistent names for your provider and its actions. Names should reflect the services they represent and follow established patterns from existing providers. Document all parameters thoroughly, explaining their purpose, format, and any default values or constraints.

Error handling deserves special attention in provider implementations. Validate inputs before executing commands to catch issues early, and handle execution errors with clear, actionable messages. When crafting parse rules, anticipate variations in command output formats to make your provider robust against minor changes in the underlying tools.

Default settings improve usability by reducing the number of parameters users must specify. Include sensible defaults for common parameters while allowing overrides for special cases. Every provider should include a `test_install` action to verify that the necessary tools are available and correctly configured. This simple diagnostic helps users troubleshoot setup issues.

Authentication mechanisms should align with the underlying service's standards. For cloud providers, this typically involves environment variables or configuration files in standard locations. Document the minimum required versions of any underlying tools to help users avoid compatibility problems.

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

In this case we are asking the api to tell us what parameters the `aws` CPI requires for the create_instance method. We can back this response up by looking at the implementation of the method show below. Close examination of the `create_instance` action block in the JSON shows us that the AWS CPI does indeed require `region`, `image_id`, `instance_type`, `security_group`, and `ssh_key_name`.

```json
{
    "name": "aws",
    "type": "command",
    "default_settings": {
      "region": "us-east-1",
      "instance_type": "t2.micro",
      "image_id": "ami-0c55b159cbfafe1f0",
      "ssh_key_name": "default-key",
      "security_group": "default",
      "volume_type": "gp2"
    },
    "actions": {
      "create_instance": {
        "command": "aws ec2 run-instances --region {region} --image-id {image_id} --instance-type {instance_type} --key-name {ssh_key_name} --security-group-ids {security_group} --output json",
        "params": [
          "region",
          "image_id",
          "instance_type",
          "ssh_key_name",
          "security_group"
        ],
        "parse_rules": {
          "type": "object",
          "patterns": {
            "instance_id": {
              "regex": "\"InstanceId\":\\s*\"([^\"]+)\"",
              "group": 1
            }
          }
        }
      }
    }
  }
```

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

These parameters are specific to certain command types. Providers can choose which parameters to implement based on their specific needs.

#### 4.5.1 Worker Commands

| Command | Parameter | Description | Example Value |
|---------|-----------|-------------|---------------|
| `test_install` | `cli_version_min` | Minimum required CLI version | `2.0.0` |
| `list_workers` | `region` | Geographic region | `us-east-1` |
| `list_workers` | `zone` | Availability zone within a region | `us-east-1a` |
| `list_workers` | `filter_tag` | Filter workers by tag | `environment=production` |
| `list_workers` | `filter_status` | Filter workers by status | `running` |
| `list_workers` | `limit` | Maximum number of results | `50` |
| `create_worker` | `name` | Name of the worker | `web-server-01` |
| `create_worker` | `worker_type` | Size or type of worker | `standard-2`, `t2.micro` |
| `create_worker` | `image` | OS image to use for the worker | `ubuntu-20.04`, `ami-12345678` |
| `create_worker` | `region` | Geographic region for the worker | `us-east-1` |
| `create_worker` | `zone` | Availability zone for the worker | `us-east-1a` |
| `create_worker` | `subnet_id` | Subnet to place worker in | `subnet-0abc123def456789` |
| `create_worker` | `security_groups` | Security groups for worker | `sg-0abc123,sg-0def456` |
| `create_worker` | `root_disk_size` | Size of root disk in GB | `50` |
| `create_worker` | `root_disk_type` | Type of root disk | `ssd`, `gp2` |
| `create_worker` | `ssh_key_id` | SSH key ID for access | `key-0abc123def456789` |
| `create_worker` | `user_data` | Cloud-init user data | `IyEvYmluL2Jhc2gKYXB0LWdldCB1cGRhdGU=` |
| `create_worker` | `tags` | Resource tags | `environment=prod,service=web` |
| `create_worker` | `hostname_param` | Hostname parameter | `--hostname my-server` |
| `create_worker` | `ipv6_param` | IPv6 parameter | `--ipv6 enabled` |
| `create_worker` | `ssh_keys_param` | SSH keys parameter | `--ssh-keys key1,key2` |
| `create_worker` | `tag_param` | Tag parameter | `--tag production` |
| `create_worker` | `public_ip` | Whether to assign public IP | `true`, `false` |
| `create_worker` | `private_networking` | Enable private networking | `true`, `false` |
| `create_worker` | `monitoring` | Enable detailed monitoring | `true`, `false` |
| `create_worker` | `backups` | Enable automated backups | `true`, `false` |
| `create_worker` | `vpc_id` | VPC to place worker in | `vpc-0abc123def456789` |
| `create_worker` | `iam_profile` | IAM instance profile | `S3ReadOnlyAccess` |
| `create_worker` | `placement_group` | Placement group | `cluster-1` |
| `create_worker` | `tenancy` | Instance tenancy | `default`, `dedicated` |
| `delete_worker` | `worker_id` | Identifier for a worker | `i-0abc123def456789` |
| `delete_worker` | `force` | Force deletion without confirmation | `true`, `false` |
| `delete_worker` | `delete_volumes` | Delete attached volumes | `true`, `false` |
| `delete_worker` | `force_param` | Force parameter | `--force` |
| `get_worker` | `worker_id` | Identifier for a worker | `i-0abc123def456789` |
| `get_worker` | `include_details` | Include extended details | `true`, `false` |
| `start_worker` | `worker_id` | Identifier for a worker | `i-0abc123def456789` |
| `start_worker` | `wait` | Wait for worker to start | `true`, `false` |
| `stop_worker` | `worker_id` | Identifier for a worker | `i-0abc123def456789` |
| `stop_worker` | `stop_type` | Type of stop operation | `soft`, `hard`, `hibernate` |
| `stop_worker` | `stop_type_param` | Stop type parameter | `--type hard` |
| `stop_worker` | `force` | Force stop operation | `true`, `false` |
| `stop_worker` | `wait` | Wait for worker to stop | `true`, `false` |
| `reboot_worker` | `worker_id` | Identifier for a worker | `i-0abc123def456789` |
| `reboot_worker` | `reboot_type` | Type of reboot operation | `soft`, `hard` |
| `reboot_worker` | `wait` | Wait for worker to reboot | `true`, `false` |
| `resize_worker` | `worker_id` | Identifier for a worker | `i-0abc123def456789` |
| `resize_worker` | `worker_type` | New size or type of worker | `standard-4`, `t2.large` |
| `resize_worker` | `resize_disk` | Resize disk along with worker | `true`, `false` |
| `resize_worker` | `restart` | Restart after resize | `true`, `false` |
| `resize_worker` | `wait` | Wait for resize to complete | `true`, `false` |
| `rename_worker` | `worker_id` | Identifier for a worker | `i-0abc123def456789` |
| `rename_worker` | `name` | New name for the worker | `web-server-02` |
| `update_worker` | `worker_id` | Identifier for a worker | `i-0abc123def456789` |
| `update_worker` | `tags` | New tags for the worker | `environment=staging,service=api` |
| `update_worker` | `vcpus_param` | vCPUs parameter | `--vcpus 4` |
| `update_worker` | `memory_param` | Memory parameter | `--memory 8192` |
| `update_worker` | `hours_param` | Hours parameter | `--hours 24` |
| `update_worker` | `security_groups` | Updated security groups | `sg-0abc123,sg-0def456` |
| `update_worker` | `termination_protection` | Enable termination protection | `true`, `false` |
| `update_worker` | `monitoring` | Update monitoring settings | `true`, `false` |
| `has_worker` | `worker_id` | Identifier for a worker | `i-0abc123def456789` |

#### 4.5.2 Storage Management

| Command | Parameter | Description | Example Value |
|---------|-----------|-------------|---------------|
| `list_disks` | `region` | Geographic region | `us-east-1` |
| `list_disks` | `zone` | Availability zone within a region | `us-east-1a` |
| `list_disks` | `filter_tag` | Filter disks by tag | `environment=production` |
| `list_disks` | `filter_status` | Filter disks by status | `available` |
| `list_disks` | `filter_type` | Filter disks by type | `ssd` |
| `list_disks` | `limit` | Maximum number of results | `50` |
| `list_disks` | `worker_id` | List disks attached to worker | `i-0abc123def456789` |
| `create_disk` | `disk_name` | Name of the disk/volume | `data-volume-1` |
| `create_disk` | `disk_type` | Type of storage | `ssd`, `hdd`, `premium-ssd` |
| `create_disk` | `size_gb` | Size in gigabytes | `100` |
| `create_disk` | `region` | Geographic region for the disk | `us-east-1` |
| `create_disk` | `zone` | Availability zone for the disk | `us-east-1a` |
| `create_disk` | `iops` | IOPS for disk (if supported) | `3000` |
| `create_disk` | `throughput` | Throughput in MB/s | `125` |
| `create_disk` | `snapshot_id` | Create from snapshot | `snap-0abc123def456789` |
| `create_disk` | `encrypted` | Enable encryption | `true`, `false` |
| `create_disk` | `kms_key_id` | KMS key for encryption | `key-0abc123def456789` |
| `create_disk` | `tags` | Resource tags | `environment=prod,service=database` |
| `create_disk` | `multi_attach` | Enable multi-attach capability | `true`, `false` |
| `create_disk` | `disk_size_param` | Disk size parameter | `--size 100GB` |
| `delete_disk` | `disk_id` | Identifier for a disk/volume | `vol-0abc123def456789` |
| `delete_disk` | `force` | Force deletion without confirmation | `true`, `false` |
| `delete_disk` | `force_param` | Force parameter | `--force` |
| `attach_disk` | `worker_id` | Worker to attach disk to | `i-0abc123def456789` |
| `attach_disk` | `disk_id` | Disk to attach | `vol-0abc123def456789` |
| `attach_disk` | `device` | Device path for attachment | `/dev/sdf`, `xvdh` |
| `attach_disk` | `device_param` | Device parameter | `--device /dev/sdf` |
| `attach_disk` | `read_only` | Attach as read-only | `true`, `false` |
| `attach_disk` | `auto_delete` | Delete disk when worker is deleted | `true`, `false` |
| `detach_disk` | `worker_id` | Worker to detach disk from | `i-0abc123def456789` |
| `detach_disk` | `disk_id` | Disk to detach | `vol-0abc123def456789` |
| `detach_disk` | `force` | Force detachment | `true`, `false` |
| `resize_disk` | `disk_id` | Disk to resize | `vol-0abc123def456789` |
| `resize_disk` | `size_gb` | New size in gigabytes | `200` |
| `resize_disk` | `resize_fs` | Resize filesystem | `true`, `false` |
| `resize_disk` | `iops` | New IOPS value | `6000` |
| `resize_disk` | `throughput` | New throughput in MB/s | `250` |
| `has_disk` | `disk_id` | Identifier for a disk/volume | `vol-0abc123def456789` |

#### 4.5.3 Image Management

| Command | Parameter | Description | Example Value |
|---------|-----------|-------------|---------------|
| `list_images` | `region` | Geographic region | `us-east-1` |
| `list_images` | `zone` | Availability zone within a region | `us-east-1a` |
| `list_images` | `owner` | Filter by image owner | `self`, `amazon`, `marketplace` |
| `list_images` | `architecture` | Filter by architecture | `x86_64`, `arm64` |
| `list_images` | `platform` | Filter by platform | `linux`, `windows` |
| `list_images` | `name_prefix` | Filter by name prefix | `ubuntu-` |
| `list_images` | `state` | Filter by image state | `available` |
| `list_images` | `limit` | Maximum number of results | `50` |
| `list_images` | `filter_tag` | Filter images by tag | `type=release` |
| `create_snapshot` | `worker_id` | Worker to snapshot | `i-0abc123def456789` |
| `create_snapshot` | `disk_id` | Disk to snapshot | `vol-0abc123def456789` |
| `create_snapshot` | `snapshot_name` | Name for the snapshot | `web-server-backup` |
| `create_snapshot` | `description` | Description of snapshot | `Daily backup of web server` |
| `create_snapshot` | `description_param` | Description parameter | `--description "Daily backup"` |
| `create_snapshot` | `tags` | Resource tags | `schedule=daily,retention=7days` |
| `create_snapshot` | `no_reboot` | Skip reboot during snapshot | `true`, `false` |
| `create_snapshot` | `encrypted` | Enable encryption | `true`, `false` |
| `create_snapshot` | `kms_key_id` | KMS key for encryption | `key-0abc123def456789` |
| `list_snapshots` | `worker_id` | Optional worker to filter snapshots | `i-0abc123def456789` |
| `list_snapshots` | `disk_id` | Optional disk to filter snapshots | `vol-0abc123def456789` |
| `list_snapshots` | `region` | Geographic region | `us-east-1` |
| `list_snapshots` | `owner` | Filter by snapshot owner | `self` |
| `list_snapshots` | `filter_tag` | Filter snapshots by tag | `type=automatic` |
| `list_snapshots` | `state` | Filter by snapshot state | `completed` |
| `list_snapshots` | `limit` | Maximum number of results | `50` |
| `delete_snapshot` | `snapshot_id` | Identifier for a snapshot | `snap-0abc123def456789` |
| `delete_snapshot` | `force` | Force deletion without confirmation | `true`, `false` |
| `has_snapshot` | `snapshot_id` | Identifier for a snapshot | `snap-0abc123def456789` |

#### 4.5.4 Network Management

| Command | Parameter | Description | Example Value |
|---------|-----------|-------------|---------------|
| `list_networks` | `region` | Geographic region | `us-east-1` |
| `list_networks` | `zone` | Availability zone within a region | `us-east-1a` |
| `list_networks` | `filter_tag` | Filter networks by tag | `environment=production` |
| `list_networks` | `limit` | Maximum number of results | `50` |
| `create_network` | `network_name` | Name of the network | `prod-network` |
| `create_network` | `cidr_range` | CIDR notation for IP range | `10.0.0.0/16` |
| `create_network` | `region` | Geographic region for the network | `us-east-1` |
| `create_network` | `auto_subnets` | Create default subnets | `true`, `false` |
| `create_network` | `dns_resolution` | Enable DNS resolution | `true`, `false` |
| `create_network` | `dns_hostnames` | Enable DNS hostnames | `true`, `false` |
| `create_network` | `tags` | Resource tags | `environment=prod,service=core` |
| `create_network` | `tenancy` | Default tenancy | `default`, `dedicated` |
| `create_network` | `enable_ipv6` | Enable IPv6 networking | `true`, `false` |
| `delete_network` | `network_id` | Identifier for a network | `vpc-0abc123def456789` |
| `delete_network` | `force` | Force deletion of all resources | `true`, `false` |
| `list_subnets` | `network_id` | Network to list subnets for | `vpc-0abc123def456789` |
| `list_subnets` | `filter_tag` | Filter subnets by tag | `tier=public` |
| `list_subnets` | `filter_zone` | Filter subnets by zone | `us-east-1a` |
| `list_subnets` | `limit` | Maximum number of results | `50` |
| `create_subnet` | `network_id` | Network for the subnet | `vpc-0abc123def456789` |
| `create_subnet` | `subnet_name` | Name of the subnet | `public-subnet-1` |
| `create_subnet` | `cidr_range` | CIDR notation for IP range | `10.0.1.0/24` |
| `create_subnet` | `zone` | Availability zone | `us-east-1a` |
| `create_subnet` | `public` | Make subnet public | `true`, `false` |
| `create_subnet` | `auto_assign_ip` | Auto-assign public IPs | `true`, `false` |
| `create_subnet` | `route_table_id` | Route table for subnet | `rtb-0abc123def456789` |
| `create_subnet` | `tags` | Resource tags | `tier=public,environment=prod` |
| `create_subnet` | `ipv6_cidr` | IPv6 CIDR block | `2001:db8::/64` |
| `delete_subnet` | `subnet_id` | Identifier for a subnet | `subnet-0abc123def456789` |
| `delete_subnet` | `force` | Force deletion of resources | `true`, `false` |
| `list_firewall_rules` | `network_id` | Optional network to filter rules | `vpc-0abc123def456789` |
| `list_firewall_rules` | `security_group_id` | Security group to list rules | `sg-0abc123def456789` |
| `list_firewall_rules` | `direction` | Filter by direction | `ingress`, `egress` |
| `list_firewall_rules` | `protocol` | Filter by protocol | `tcp`, `udp`, `icmp` |
| `list_firewall_rules` | `port` | Filter by port | `80`, `443` |
| `create_firewall_rule` | `security_group_id` | Security group for the rule | `sg-0abc123def456789` |
| `create_firewall_rule` | `direction` | Direction for the rule | `ingress`, `egress` |
| `create_firewall_rule` | `direction_param` | Direction parameter | `--direction ingress` |
| `create_firewall_rule` | `protocol` | Network protocol | `tcp`, `udp`, `icmp`, `all` |
| `create_firewall_rule` | `port` | Port for the rule | `80`, `443`, `22` |
| `create_firewall_rule` | `port_range` | Port range | `8000-9000` |
| `create_firewall_rule` | `port_param` | Port parameter | `--port 80` |
| `create_firewall_rule` | `source` | Source CIDR or security group | `0.0.0.0/0`, `sg-0abc123def456789` |
| `create_firewall_rule` | `source_param` | Source parameter | `--source 0.0.0.0/0` |
| `create_firewall_rule` | `destination` | Destination CIDR or security group | `10.0.0.0/8` |
| `create_firewall_rule` | `destination_param` | Destination parameter | `--destination 10.0.0.0/8` |
| `create_firewall_rule` | `description` | Description for the rule | `Allow HTTP traffic` |
| `create_firewall_rule` | `comment_param` | Comment parameter | `--comment "Allow HTTP"` |
| `create_firewall_rule` | `priority` | Rule priority | `100` |
| `create_firewall_rule` | `ipv6` | Apply to IPv6 | `true`, `false` |
| `delete_firewall_rule` | `rule_id` | Identifier for a firewall rule | `sgr-0abc123def456789` |
| `delete_firewall_rule` | `security_group_id` | Security group containing the rule | `sg-0abc123def456789` |
| `delete_firewall_rule` | `force` | Force deletion without confirmation | `true`, `false` |

#### 4.5.5 Authentication and Account Management

| Command | Parameter | Description | Example Value |
|---------|-----------|-------------|---------------|
| `configure_auth` | `api_key` | API key for authentication | `abcdef123456` |
| `configure_auth` | `api_secret` | API secret key | `abcdef123456abcdef123456` |
| `configure_auth` | `token` | Authentication token | `eyJhbGciOiJIUzI1NiIsInR5...` |
| `configure_auth` | `credentials_file` | Path to credentials file | `~/.aws/credentials` |
| `configure_auth` | `config_file` | Path to config file | `~/.aws/config` |
| `configure_auth` | `profile` | Named profile to use | `development` |
| `configure_auth` | `username` | Username for authentication | `admin` |
| `configure_auth` | `password` | Password for authentication | `P@ssw0rd123!` |
| `configure_auth` | `project_id` | Project ID for scope | `project-0abc123def456789` |
| `configure_auth` | `tenant_id` | Tenant/organization ID | `tenant-0abc123def456789` |
| `configure_auth` | `region` | Default region to use | `us-east-1` |
| `configure_auth` | `endpoint` | API endpoint URL | `https://api.example.com/v1` |
| `configure_auth` | `mfa_token` | MFA code for authentication | `123456` |
| `test_auth` | `verbose` | Show detailed test results | `true`, `false` |
| `get_account_info` | `include_usage` | Include usage information | `true`, `false` |
| `get_account_info` | `include_billing` | Include billing information | `true`, `false` |
| `get_account_info` | `include_limits` | Include account limits | `true`, `false` |
| `list_credentials` | `credential_type` | Optional credential type filter | `api-key`, `ssh-key` |
| `list_credentials` | `limit` | Maximum number of results | `50` |
| `list_credentials` | `filter_status` | Filter by status | `active`, `inactive` |
| `create_credential` | `credential_name` | Name for the credential | `prod-api-key` |
| `create_credential` | `credential_type` | Type of credential | `api-key`, `ssh-key` |
| `create_credential` | `permissions` | Permissions for the credential | `["read", "write"]` |
| `create_credential` | `description` | Description of credential | `Production deployment key` |
| `create_credential` | `expiration` | Expiration time | `2026-03-17T00:00:00Z` |
| `create_credential` | `ssh_public_key` | SSH public key content | `ssh-rsa AAAAB3NzaC1...` |
| `create_credential` | `tags` | Resource tags | `environment=prod,service=deploy` |
| `delete_credential` | `credential_id` | Identifier for a credential | `key-0abc123def456789` |
| `delete_credential` | `force` | Force deletion without confirmation | `true`, `false` |
| `rotate_credential` | `credential_id` | Identifier for a credential | `key-0abc123def456789` |
| `rotate_credential` | `deactivate_previous` | Deactivate previous credential | `true`, `false` |
| `rotate_credential` | `expiration` | New expiration time | `2026-03-17T00:00:00Z` |
| `set_default_project` | `project_id` | Identifier for a project | `project-0abc123def456789` |
| `list_projects` | `filter_status` | Filter by project status | `active`, `suspended` |
| `list_projects` | `limit` | Maximum number of results | `50` |

#### 4.5.6 CLI Setup and Initialization

| Command | Parameter | Description | Example Value |
|---------|-----------|-------------|---------------|
| `initialize_cli` | `cli_path` | Path to CLI executable | `/usr/local/bin/aws` |
| `initialize_cli` | `cli_version` | Version of CLI to install | `2.11.3` |
| `initialize_cli` | `install_dir` | Installation directory | `/usr/local/bin` |
| `initialize_cli` | `os_type` | Operating system type | `linux`, `darwin`, `windows` |
| `initialize_cli` | `arch` | System architecture | `x86_64`, `arm64` |
| `initialize_cli` | `skip_verify` | Skip signature verification | `true`, `false` |
| `initialize_cli` | `auto_completion` | Install shell completion | `true`, `false` |
| `setup_environment` | `env_vars` | Environment variables to set | `{"AWS_REGION":"us-east-1"}` |
| `setup_environment` | `shell_type` | Shell type for environment | `bash`, `zsh`, `fish` |
| `setup_environment` | `rc_file` | RC file to modify | `~/.bashrc`, `~/.zshrc` |
| `setup_environment` | `persist` | Persist changes to shell config | `true`, `false` |
| `update_cli` | `cli_version` | Version to update to (optional) | `2.12.0`, `latest` |
| `update_cli` | `force` | Force update | `true`, `false` |
| `update_cli` | `check_only` | Only check for updates | `true`, `false` |
| `validate_prerequisites` | `check_commands` | Commands to check | `curl,unzip,jq` |
| `install_plugin` | `plugin_name` | Name of CLI plugin | `eks`, `session-manager` |
| `install_plugin` | `plugin_version` | Version of plugin | `1.2.0`, `latest` |
| `install_plugin` | `plugins_dir` | Plugin installation directory | `~/.aws/cli/plugins` |
| `install_plugin` | `force` | Force installation | `true`, `false` |
| `install_plugin` | `skip_verify` | Skip signature verification | `true`, `false` |

#### 4.5.7 Metadata and Infrastructure

| Command | Parameter | Description | Example Value |
|---------|-----------|-------------|---------------|
| `list_regions` | `filter_status` | Filter regions by status | `available`, `opted-in` |
| `list_regions` | `filter_name` | Filter regions by name pattern | `us-*`, `eu-*` |
| `list_regions` | `include_opt_status` | Include opt-in status | `true`, `false` |
| `list_zones` | `region` | Region to list zones for | `us-east-1` |
| `list_zones` | `filter_status` | Filter zones by status | `available` |
| `list_zones` | `filter_name` | Filter zones by name pattern | `us-east-1*` |
| `list_worker_types` | `region` | Geographic region | `us-east-1` |
| `list_worker_types` | `zone` | Availability zone within a region | `us-east-1a` |
| `list_worker_types` | `filter_category` | Filter by instance category | `general`, `compute`, `memory` |
| `list_worker_types` | `filter_architecture` | Filter by CPU architecture | `x86_64`, `arm64` |
| `list_worker_types` | `filter_vcpus_min` | Minimum vCPUs | `2` |
| `list_worker_types` | `filter_vcpus_max` | Maximum vCPUs | `16` |
| `list_worker_types` | `filter_memory_min` | Minimum memory in GB | `4` |
| `list_worker_types` | `filter_memory_max` | Maximum memory in GB | `64` |
| `list_worker_types` | `include_costs` | Include cost information | `true`, `false` |
| `get_pricing` | `resource_type` | Type of resource | `worker`, `disk`, `snapshot` |
| `get_pricing` | `worker_type` | Type of worker for pricing | `t2.micro`, `standard-2` |
| `get_pricing` | `disk_type` | Type of disk for pricing | `ssd`, `standard` |
| `get_pricing` | `region` | Geographic region | `us-east-1` |
| `get_pricing` | `currency` | Currency for pricing | `USD`, `EUR` |
| `get_pricing` | `term` | Term for pricing | `ondemand`, `reserved` |
| `get_pricing` | `reserved_term` | Reserved instance term | `1year`, `3year` |
| `get_pricing` | `os` | Operating system | `linux`, `windows` |
| `get_quota` | `resource_type` | Type of resource | `workers`, `cpus`, `memory` |
| `get_quota` | `region` | Geographic region | `us-east-1` |
| `get_quota` | `include_usage` | Include current usage | `true`, `false` |

#### 4.5.8 CLI Setup and Environment Parameters

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

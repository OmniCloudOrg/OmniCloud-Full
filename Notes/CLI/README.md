# OmniOrchestrator CLI Documentation

## Introduction

OmniOrchestrator is a powerful self-hosted cloud platform CLI that enables organizations to create, manage, and scale their own cloud infrastructure without relying on third-party cloud providers. This documentation provides comprehensive guidance on using the OmniOrchestrator command-line interface (CLI) to manage your self-hosted cloud environment.

The OmniOrchestrator CLI, accessible through the `omni` command, offers a streamlined approach to infrastructure management, application deployment, monitoring, and maintenance. With an emphasis on simplicity and automation, OmniOrchestrator empowers teams to establish enterprise-grade cloud environments on their own hardware while maintaining full control over their infrastructure.

## Getting Started

### Installation

OmniOrchestrator supports multiple platforms including Linux, macOS, and Windows. The CLI is distributed as a standalone binary for each supported architecture, making installation straightforward without dependencies. After downloading the appropriate binary for your system from the release page, simply add it to your PATH or execute it directly. The CLI automatically detects your operating system and adapts its behavior accordingly.

### Initial Configuration

Before using OmniOrchestrator, you need to initialize your cloud environment. This is accomplished through the `omni init` command, which guides you through a comprehensive setup process. During initialization, you'll configure essential details such as your company name, cloud platform name, and primary region. The initialization process also collects information about SSH hosts that will form the backbone of your cloud infrastructure.

The initialization wizard helps you set up both bastion (jump) hosts and worker nodes, configure identity files for secure authentication, and establish essential services like monitoring and automated backups. All configuration is stored in a JSON file that can be version-controlled and shared across your organization, facilitating infrastructure-as-code practices.

## Core Concepts

### Self-Hosted Cloud Architecture

OmniOrchestrator implements a cloud management layer on top of your existing hardware. The architecture consists of several key components:

1. **Lodestone Hosts**: Secure entry points to your infrastructure that handle incoming traffic, authentication, and serve as API gateways.

2. **Worker Nodes**: Servers that run your containerized applications, handling the computational workload of your environment.

3. **Overlay Network**: A secure network layer that enables communication between services across your infrastructure.

4. **OmniOrchestrator**: The central management system that coordinates deployments, scaling, and monitoring across your environment.

Understanding this architecture helps you make informed decisions when configuring your cloud environment and troubleshooting any issues that may arise.

### Configuration Management

Configuration in OmniOrchestrator is handled through a central JSON file located in the `config` directory. This file contains all the settings for your cloud environment, including host information, monitoring preferences, and backup policies. The `omni config` command group provides tools to view, edit, and reset this configuration.

The configuration system supports version control, allowing you to track changes to your infrastructure over time. You can back up your configuration file and restore it when needed, ensuring that your infrastructure remains consistent and reproducible.

## Command Reference

### Environment Management

#### `omni init`

The `omni init` command starts the setup wizard for configuring your cloud environment. This interactive process guides you through setting up your company information, cloud name, region, and SSH hosts. It also configures essential services like monitoring and backups. After completing the initialization, OmniOrchestrator bootstraps itself across your hosts, configuring each one according to its role in your infrastructure.

#### `omni hosts`

This command displays a table of all configured SSH hosts in your environment. The table includes host names, hostnames/IP addresses, usernames, ports, identity file paths, and whether each host is a bastion host. This provides a quick overview of your infrastructure's topology and connection information.

#### `omni status`

The `omni status` command offers a comprehensive overview of your OmniOrchestrator environment. It displays the status of all services running on each host, including CPU and memory usage, uptime, and operational status. This command also provides system-wide information about monitoring, backup status, and upcoming scheduled tasks.

### Application Deployment

#### `omni up`

The `omni up` command is used to deploy applications to your cloud environment. It packages your application directory into a tarball, analyzes the project structure, builds containers, pushes them to your internal registry, and starts the services. The deployment process is visualized with progress bars that keep you informed about each step.

During deployment, you can select the target environment (development, staging, or production) and confirm sensitive operations like production deployments. After completion, the command provides endpoints for accessing your application and its associated services.

#### `omni push`

This command pushes container images to your internal registry. It supports various registry types including Docker Hub, Google Container Registry, and Amazon ECR. When pushing an image, you can specify a tag and watch the progress as the image is prepared, optimized, and uploaded to the registry.

#### `omni scale`

The `omni scale` command allows you to adjust the number of replicas for a specific component of your application. You can select which component to scale (such as web frontend, API backend, or database) and specify the desired number of replicas. After scaling, the command displays the updated component status, including resource allocation.

#### `omni logs`

Use this command to view logs from your application components. You can filter logs by component and see real-time information about your running services. The logs display includes timestamps and severity levels, making it easy to identify issues and monitor application behavior.

#### `omni rollback`

If a deployment causes issues, the `omni rollback` command allows you to revert to a previous version. You can select from a list of previous versions sorted by deployment date. The rollback process handles stopping the current version, loading the previous one, updating configuration, and starting services with a seamless transition.

### Service Management

#### `omni service restart`

This command restarts a specific service on a specified host. It's useful when you need to apply configuration changes or reset a service that's behaving unexpectedly.

#### `omni service stop`

Use this command to stop a service on a host. This can be helpful when performing maintenance or troubleshooting issues with specific services.

#### `omni service start`

This command starts a previously stopped service. Combined with the stop command, it enables fine-grained control over service lifecycle management.

### Backup Management

#### `omni backup now`

The `omni backup now` command triggers an immediate backup of your environment according to your configured backup policies. This is useful before major changes or when you need to ensure your data is safely stored.

#### `omni backup list`

This command displays a list of available backups, including creation dates, sizes, and storage locations. You can use this information to select which backup to restore if needed.

#### `omni backup restore`

Use this command to restore your environment from a previously created backup. You'll need to specify the backup ID from the backup list. The restoration process handles all aspects of bringing your system back to the state it was in when the backup was created.

### Configuration Management

#### `omni config view`

This command displays the current configuration settings for your OmniOrchestrator environment in a readable format. It shows all configured values including company information, cloud name, region, SSH hosts, and service settings.

#### `omni config edit`

The `omni config edit` command opens your configuration file in your default editor, allowing you to make changes. After saving, OmniOrchestrator validates the changes and applies them to your environment.

#### `omni config reset`

This command resets your configuration to default values after confirmation. It's useful when you want to start fresh or when your configuration has become corrupted.

## Best Practices

### Security Considerations

When setting up your OmniOrchestrator environment, prioritize security by following these guidelines:

1. Always use identity files for SSH authentication rather than passwords.
2. Configure bastion hosts with strict access controls and place them in DMZs when possible.
3. Keep your OmniOrchestrator CLI and infrastructure components updated to the latest versions.
4. Implement network segmentation to isolate different components of your cloud environment.
5. Enable monitoring and set up alerts for suspicious activities.

Regular security audits of your environment will help ensure that your data and applications remain protected from unauthorized access or breaches.

### Backup Strategy

Implementing a robust backup strategy is critical for maintaining business continuity. OmniOrchestrator's backup system offers configurable retention policies and scheduled backups. We recommend:

1. Setting backup frequency based on how frequently your data changes.
2. Storing backups in multiple locations, preferably including off-site storage.
3. Regularly testing backup restoration to ensure your recovery process works as expected.
4. Implementing different retention policies for different types of data based on importance.

By following these practices, you'll be prepared to recover quickly from hardware failures, data corruption, or other incidents that might affect your environment.

### Resource Management

Efficient resource management is key to getting the most out of your self-hosted cloud environment. The `omni scale` command allows you to adjust resources as needed, but consider these best practices:

1. Monitor resource usage patterns to identify opportunities for optimization.
2. Scale components independently based on their specific needs.
3. Consider implementing auto-scaling policies for components with variable load.
4. Right-size your resources to avoid over-provisioning while maintaining performance.

Regularly reviewing resource allocation and adjusting based on actual usage will help you maintain an efficient and cost-effective environment.

### CI/CD Integration

OmniOrchestrator can be integrated into your Continuous Integration/Continuous Deployment (CI/CD) pipelines for automated deployments. Consider the following approaches:

1. Use environment variables or configuration files to store OmniOrchestrator connection details securely in your CI/CD system.
2. Create dedicated deployment scripts that use the OmniOrchestrator CLI commands.
3. Implement deployment stages that deploy to development and staging environments before production.
4. Add automated tests that verify the health of your application after deployment.

This integration streamlines your development workflow and reduces the risk of human error during deployments.

## Troubleshooting

### Common Issues

#### Connectivity Problems

If you're experiencing connectivity issues, first verify that SSH access to your hosts is working correctly outside of OmniOrchestrator. Check firewall rules, SSH key permissions, and network connectivity. The `omni hosts` command can help identify which hosts are configured in your environment.

#### Deployment Failures

When deployments fail, check the logs using `omni logs` to identify the specific error. Common issues include misconfigured dependencies, insufficient resources, or problems with container images. You can use `omni rollback` to revert to a known good state while you troubleshoot.

#### Performance Issues

If your applications are performing poorly, use `omni status` to check resource usage across your environment. Look for components that might be resource-constrained and consider using `omni scale` to allocate additional resources where needed.

#### Service Failures

When services fail to start or stop responding, use `omni service restart` to attempt recovery. Check the logs for error messages that might indicate the root cause. If a service continues to fail, consider checking for configuration issues or bugs in your application code.
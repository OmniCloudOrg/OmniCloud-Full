# `omni scale` - Scale Application Components

## Overview

The `omni scale` command allows you to adjust the resource allocation and replica count for components of your deployed applications. This command gives you fine-grained control over how your application scales to handle varying workloads, optimizing resource usage across your OmniOrchestrator environment.

## Usage

```
omni scale [--component COMPONENT] [--replicas REPLICAS]
```

### Options

- `--component COMPONENT`: Specify the component to scale (e.g., "frontend", "backend", "database")
- `--replicas REPLICAS`: Specify the number of replicas (must be between 1 and 10)

When run without options, the command operates in interactive mode, prompting for the necessary information.

## Workflow

The `omni scale` command follows this workflow:

1. **Component Selection**: Choose which component of your application to scale
2. **Replica Configuration**: Specify how many instances of the component should run
3. **Resource Calculation**: OmniOrchestrator calculates the total resources needed
4. **Scaling Operation**: The component is scaled to the requested number of replicas
5. **Status Display**: Shows the updated status of the scaled component

## Example

```bash
$ omni scale

Select component to scale:
> Web Frontend
  API Backend
  Database

Enter number of replicas: 5

Scaling component...
✓ Scaling completed successfully!

📊 Updated Component Status
┌──────────────┬─────────┬─────────┬───────┬────────┐
│ Component    │ Status  │ Replicas│ CPU   │ Memory │
├──────────────┼─────────┼─────────┼───────┼────────┤
│ Web Frontend │ Running │ 5/5     │ 750m  │ 1280Mi │
└──────────────┴─────────┴─────────┴───────┴────────┘
```

## Understanding Resources

When scaling components, OmniOrchestrator allocates resources based on component type:

1. **CPU**: Shown in millicores (m), where 1000m equals 1 CPU core
2. **Memory**: Shown in Mi (Mebibytes) or Gi (Gibibytes)

The total resources allocated are proportional to the number of replicas. For example, if each frontend instance uses 150m CPU and 256Mi memory, scaling to 5 replicas will allocate a total of 750m CPU and 1280Mi memory.

## Component-Specific Considerations

Different components have different scaling characteristics:

### Web Frontend
- Highly scalable, can typically run many replicas
- Resource usage per replica is usually moderate
- Scales horizontally for improved throughput and redundancy

### API Backend
- Moderately scalable, balancing between throughput and consistency
- Resource usage depends on business logic complexity
- Database connection pooling may limit optimal replica count

### Database
- Limited scalability for primary nodes (often limited to 1 replica)
- Higher resource requirements per instance
- May require special consideration for data consistency

## Scaling Policies

OmniOrchestrator supports several scaling approaches:

1. **Manual Scaling**: Using the `omni scale` command directly
2. **Schedule-Based Scaling**: Configuring scaling operations to occur at specific times
3. **Metric-Based Scaling**: Automatically scaling based on CPU, memory, or custom metrics

The `omni scale` command performs manual scaling. For schedule-based or metric-based scaling, use the appropriate configuration commands.

## Validation Checks

The command performs several validation checks:

1. **Minimum/Maximum Replicas**: Ensures the replica count is between 1 and 10
2. **Resource Availability**: Verifies that your cluster has sufficient resources
3. **Component Existence**: Confirms that the specified component exists
4. **Scaling Constraints**: Respects any component-specific scaling limitations (e.g., singleton databases)

## Resource Distribution

When scaling components, OmniOrchestrator intelligently distributes the workload across your worker nodes to ensure:

1. **High Availability**: Replicas are distributed across different physical hosts when possible
2. **Resource Efficiency**: Node resources are utilized effectively
3. **Failure Resilience**: The system remains operational even if individual nodes fail

## Scaling Events Timeline

The command maintains a history of scaling events that can be viewed using monitoring tools. Each scaling operation records:

- The timestamp of the scaling operation
- The component that was scaled
- The previous and new replica counts
- The user who initiated the scaling
- Any relevant context or reason for scaling

## Notes

- Scaling stateful components (like databases) may require additional configuration or preparation
- For production environments, it's recommended to test scaling operations in staging first
- Resource limits in your OmniOrchestrator environment may restrict the maximum number of replicas
- Use the `omni status` command after scaling to verify the operation completed successfully
- Consider using metric-based auto-scaling for production workloads with variable traffic patterns
- Components will continue serving requests during scaling operations, ensuring zero-downtime scaling
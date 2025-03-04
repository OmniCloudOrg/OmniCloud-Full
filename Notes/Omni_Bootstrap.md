# Omni Init: Cluster Initialization Process

## Table of Contents
- [Overview](#overview)
- [Initialization Process](#the-initialization-process)
  - [Root Orchestrator Creation](#1-root-orchestrator-creation)
  - [Cluster Configuration Loading](#2-cluster-configuration-loading)
  - [Director VM Deployment](#3-director-vm-deployment)
  - [Control VM Creation](#4-control-vm-creation)
  - [Orchestrator Deployment](#5-orchestrator-deployment)
  - [Cluster Registration](#6-cluster-registration)
  - [Root Node Destruction](#7-root-node-destruction)
- [Process Flow](#complete-process-flow)
- [Sequence Diagram](#sequence-diagram)
- [Key Components](#key-components)
- [Architecture Diagram](#architecture-diagram)
- [Benefits](#benefits-of-this-approach)
- [Troubleshooting](#common-issues-and-troubleshooting)
- [Next Steps](#next-steps)

## Overview

The `omni init` command bootstraps an Omni cluster through a self-initializing process. It creates a temporary "root" orchestrator that configures worker nodes, deploys directors, and installs the permanent orchestrators that will run the cluster. This document explains the initialization process and the resulting architecture.

What makes Omni unique is its bootstrapped design: the system builds itself. Orchestrators control Directors, while themselves running as applications managed by Agents, which run on VMs controlled by Directors. This circular dependency enables a self-managing system after initialization completes.

## The Initialization Process

### 1. Root Orchestrator Creation

Running `omni init` creates a temporary root orchestrator that acts as the bootstrap mechanism. This orchestrator only exists during initialization.

```
$ omni init
```

```mermaid
graph TD
    Start[User runs 'omni init'] --> RootOrch[Root Orchestrator Created]
    style RootOrch fill:#6495ED,stroke:#333,stroke-width:1px,color:white
```

The root orchestrator's job is to set up the permanent components and then remove itself from the system.

### 2. Cluster Configuration Loading

The root orchestrator loads the cluster configuration that defines nodes, resources, and network settings.

```yaml
# Example cluster configuration snippet
nodes:
  - name: worker-01
    role: worker
    resources:
      cpu: 8
      memory: 32Gi
  - name: worker-02
    role: worker
    resources:
      cpu: 8
      memory: 32Gi
```

```mermaid
graph TD
    RootOrch[Root Orchestrator] --> LoadConfig[Load Cluster Configuration]
    LoadConfig --> ValidateConfig[Validate Configuration]
    style RootOrch fill:#6495ED,stroke:#333,stroke-width:1px,color:white
    style LoadConfig fill:#6495ED,stroke:#333,stroke-width:1px,color:white
    style ValidateConfig fill:#6495ED,stroke:#333,stroke-width:1px,color:white
```

### 3. Director VM Deployment

The root orchestrator deploys Director VMs to each node in the cluster. Directors are responsible for managing VMs on their respective nodes.

```mermaid
graph TD
    RootOrch[Root Orchestrator] --> DeployDir1[Deploy Director to Worker-01]
    RootOrch --> DeployDir2[Deploy Director to Worker-02]
    RootOrch --> DeployDirN[Deploy Director to Worker-N]
    style RootOrch fill:#6495ED,stroke:#333,stroke-width:1px,color:white
    style DeployDir1 fill:#6495ED,stroke:#333,stroke-width:1px,color:white
    style DeployDir2 fill:#6495ED,stroke:#333,stroke-width:1px,color:white
    style DeployDirN fill:#6495ED,stroke:#333,stroke-width:1px,color:white
```

Directors handle VM lifecycle on each node. Unlike other components, Directors run directly on VMs, not in containers.

### 4. VM and Agent Deployment

The Directors deploy worker VMs and install Agents on them. Agents handle container management within each VM.

```mermaid
graph TD
    Director[Director] --> DeployVM[Deploy Worker VM]
    DeployVM --> InstallAgent[Install Agent in VM]
    InstallAgent --> ConfigureVM[Configure VM Resources]
    style Director fill:#6495ED,stroke:#333,stroke-width:1px,color:white
    style DeployVM fill:#6495ED,stroke:#333,stroke-width:1px,color:white
    style InstallAgent fill:#6495ED,stroke:#333,stroke-width:1px,color:white
    style ConfigureVM fill:#6495ED,stroke:#333,stroke-width:1px,color:white
```

This step establishes the container runtime environment where applications (including orchestrators) will run.

### 5. Orchestrator Deployment

The Directors then deploy Orchestrators as applications into containers managed by Agents.

```mermaid
graph TD
    Director[Director] --> DeployOrch[Deploy Orchestrator as App Container]
    Agent[Agent] --> ManageOrch[Manage Orchestrator Container]
    style Director fill:#6495ED,stroke:#333,stroke-width:1px,color:white
    style Agent fill:#FF9800,stroke:#333,stroke-width:1px,color:white
    style DeployOrch fill:#6495ED,stroke:#333,stroke-width:1px,color:white
    style ManageOrch fill:#FF9800,stroke:#333,stroke-width:1px,color:white
```

This is a critical point in the bootstrapping process: Orchestrators run as standard applications in containers, but they will eventually control the Directors.

### 6. Cluster Registration

The Orchestrators register with each other and establish control of the Directors, creating the circular management relationship that defines Omni.

```mermaid
graph TD
    Orch1[Orchestrator 1] --> Register[Register with Cluster]
    Orch2[Orchestrator 2] --> Register
    OrchN[Orchestrator N] --> Register
    Register --> ControlDirectors[Control Directors]
    style Orch1 fill:#6495ED,stroke:#333,stroke-width:1px,color:white
    style Orch2 fill:#6495ED,stroke:#333,stroke-width:1px,color:white
    style OrchN fill:#6495ED,stroke:#333,stroke-width:1px,color:white
    style Register fill:#6495ED,stroke:#333,stroke-width:1px,color:white
    style ControlDirectors fill:#4CAF50,stroke:#333,stroke-width:1px,color:white
```

At this point, the bootstrapped architecture is established: Orchestrators control Directors, while themselves running as containers managed by Agents, which run on VMs managed by Directors.

### 7. Root Node Destruction

Once the permanent orchestration layer is operational, the root orchestrator self-terminates.

```mermaid
graph TD
    VerifyCluster[Verify Cluster Health] --> ConfirmOrchestrators[Confirm Orchestrators Functional]
    ConfirmOrchestrators --> DestroyRoot[Destroy Root Orchestrator]
    DestroyRoot --> OperationalCluster[Self-Managing Cluster]
    style VerifyCluster fill:#6495ED,stroke:#333,stroke-width:1px,color:white
    style ConfirmOrchestrators fill:#6495ED,stroke:#333,stroke-width:1px,color:white
    style DestroyRoot fill:#6495ED,stroke:#333,stroke-width:1px,color:white
    style OperationalCluster fill:#4CAF50,stroke:#333,stroke-width:1px,color:white
```

The cluster is now self-managing, with no initialization components remaining.

## Complete Process Flow

The following diagram shows the entire initialization process:

```mermaid
graph TD
    Start[User runs 'omni init'] --> RootOrch[Root Orchestrator Created]
    RootOrch --> LoadConfig[Load Cluster Configuration]
    LoadConfig --> DeployDirectors[Deploy Directors]
    DeployDirectors --> DeployVMs[Deploy Worker VMs]
    DeployVMs --> InstallAgents[Install Agents]
    InstallAgents --> DeployOrchs[Deploy Orchestrators as Apps]
    DeployOrchs --> RegisterCluster[Register Orchestrators]
    RegisterCluster --> EstablishControl[Orchestrators Control Directors]
    EstablishControl --> DestroyRoot[Destroy Root Orchestrator]
    DestroyRoot --> OperationalCluster[Self-Managing Cluster]
    
    style Start fill:#f9f9f9,stroke:#333,stroke-width:1px
    style RootOrch fill:#6495ED,stroke:#333,stroke-width:1px,color:white
    style LoadConfig fill:#6495ED,stroke:#333,stroke-width:1px,color:white
    style DeployDirectors fill:#6495ED,stroke:#333,stroke-width:1px,color:white
    style DeployVMs fill:#6495ED,stroke:#333,stroke-width:1px,color:white
    style InstallAgents fill:#6495ED,stroke:#333,stroke-width:1px,color:white
    style DeployOrchs fill:#6495ED,stroke:#333,stroke-width:1px,color:white
    style RegisterCluster fill:#6495ED,stroke:#333,stroke-width:1px,color:white
    style EstablishControl fill:#6495ED,stroke:#333,stroke-width:1px,color:white
    style DestroyRoot fill:#6495ED,stroke:#333,stroke-width:1px,color:white
    style OperationalCluster fill:#4CAF50,stroke:#333,stroke-width:1px,color:white
```

## Sequence Diagram

This sequence diagram shows the interaction between components during initialization:

```mermaid
sequenceDiagram
    participant User as User/Client
    participant Root as Root Orchestrator
    participant Dir as Directors
    participant VM as Worker VMs
    participant Agent as Agents
    participant Orch as Orchestrators
    
    User->>Root: omni init
    Root->>Root: Load Configuration
    Root->>Dir: Deploy Directors
    Dir->>VM: Deploy Worker VMs
    Dir->>Agent: Install Agents in VMs
    Dir->>Orch: Deploy Orchestrators as App Containers
    Orch->>Orch: Register with Each Other
    Orch->>Dir: Establish Control of Directors
    Orch-->>Root: Confirm Operational Status
    Root->>Root: Self-Terminate
    Note over User,Orch: Cluster is now self-managing
```

## Key Components

### Root Orchestrator
The temporary bootstrap mechanism that initializes the cluster and self-terminates once the permanent components are operational.

### Directors
Run directly on VMs and manage all VM lifecycles. Directors deploy worker VMs, install Agents, and deploy the initial Orchestrators as applications.

### Agents
Run within worker VMs and manage all containers within those VMs, including Orchestrator containers and user application containers.

### Orchestrators
Run as applications in containers but control Directors. Orchestrators provide the top-level API for the CLI and dashboard, forming the management plane of the cluster.

## Architecture Diagram

```mermaid
graph TD
    %% Defines top-level hardware
    subgraph "Physical Hardware"
        %% First VM is a Director
        subgraph "VM: Director"
            Director[Director]
        end
        
        %% Second VM has everything else
        subgraph "VM: Worker"
            Agent[Agent]
            
            subgraph "Containers managed by Agent"
                OrchestratorContainer[Orchestrator]
                AppContainer[User Applications]
            end
        end
        
        %% Control relationships (across boundaries)
        OrchestratorContainer -.Controls.-> Director
        Director -.Manages.-> VM
        Agent -.Manages.-> OrchestratorContainer
        Agent -.Manages.-> AppContainer
    end
    
    classDef blue fill:#6495ED,stroke:#333,stroke-width:1px,color:white
    classDef green fill:#4CAF50,stroke:#333,stroke-width:1px,color:white
    classDef purple fill:#9C27B0,stroke:#333,stroke-width:1px,color:white
    classDef orange fill:#FF9800,stroke:#333,stroke-width:1px,color:white
    
    class Director blue
    class Agent orange
    class OrchestratorContainer green
    class AppContainer purple
```

This diagram shows the final state of an Omni cluster. The bootstrapped nature creates a circular management relationship:

- **Directors** run directly on VMs and manage all VMs
- **Agents** run inside worker VMs and manage all containers
- **Orchestrators** run as applications in containers but control Directors
- **User Applications** run in containers alongside Orchestrators

This circular dependency - Orchestrators control Directors despite running in containers managed by Agents on VMs controlled by Directors - enables the self-managing nature of Omni.

## Benefits of this Approach

1. **Self-bootstrapping** - The system builds itself with minimal external intervention
2. **Clean separation** - Initialization components don't remain in the operational system
3. **Horizontal scalability** - The same process works for clusters of any size
4. **Resilience** - No single point of failure in the management plane
5. **Circular management** - Enables self-healing and autonomous operation

## Common Issues and Troubleshooting

### Initialization Failures
Check network connectivity between nodes, verify resource availability on target nodes, and confirm the root orchestrator has necessary permissions.

### Director Deployment Issues
Verify VM image availability, check resource allocation settings, and confirm network paths are open between nodes.

### Orchestrator Registration Problems
Examine cluster networking configuration, check service discovery functionality, and review logs from both Directors and Orchestrators.
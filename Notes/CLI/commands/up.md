# `omni up` - Deploy Application

## Overview

The `omni up` command (also referred to as `deploy` in some contexts) is used to deploy applications to your OmniOrchestrator cloud environment. This command packages your application code, builds containers, pushes them to the internal registry, and starts the services in your cloud environment. It provides a streamlined deployment experience with detailed progress information.

## Usage

```
omni up [--env ENVIRONMENT]
```

### Options

- `--env ENVIRONMENT`: Specify the target environment (dev, staging, prod). If not specified, the command will prompt interactively.

## Workflow

The `omni up` command follows this sequential workflow:

1. **Project Selection**: Prompts for the project path (defaults to current directory)
2. **Environment Selection**: Prompts for the deployment environment (Development, Staging, Production)
3. **Production Confirmation**: Requires explicit confirmation for production deployments
4. **Project Packaging**: Creates a tarball of the project files, respecting `.gitignore` rules
5. **Upload**: Uploads the tarball to the OmniOrchestrator control plane
6. **Analysis**: Analyzes the project structure to determine component types
7. **Build Process**: Builds container images for each component
8. **Registry Push**: Pushes built images to the internal registry
9. **Service Configuration**: Configures networking, storage, and other services
10. **Deployment**: Starts the application components
11. **Status Display**: Shows the status and endpoints for the deployed application

## Example

```bash
$ omni up

Enter project path [.]: ./my-webapp
Select deployment environment:
> Development
  Staging
  Production

🚀 Initializing deployment...
🗜️ Creating tarball...
[==================================================] 100% Adding file: src/main.js ✓
[==================================================] 100% Adding file: src/app.js ✓
[==================================================] 100% Adding file: package.json ✓
Tarball created successfully ✓

🗜️ uploading...
[==================================================] 100% Upload completed successfully ✓

Analyzing project
[==================================================] 100% Analyzing project (scanning dependencies) ✓

Building containers
[==================================================] 100% Building containers (optimizing) ✓

Pushing to registry
[==================================================] 100% Pushing to registry (finalizing) ✓

Configuring services
[==================================================] 100% Configuring services ✓

Starting components
[==================================================] 100% Starting components ✓

📊 Deployment Status
┌──────────────┬─────────┬─────────┬───────┬────────┐
│ Component    │ Status  │ Replicas│ CPU   │ Memory │
├──────────────┼─────────┼─────────┼───────┼────────┤
│ Web Frontend │ Running │ 3/3     │ 150m  │ 256Mi  │
├──────────────┼─────────┼─────────┼───────┼────────┤
│ API Backend  │ Running │ 2/2     │ 200m  │ 512Mi  │
├──────────────┼─────────┼─────────┼───────┼────────┤
│ Database     │ Running │ 1/1     │ 500m  │ 1Gi    │
└──────────────┴─────────┴─────────┴───────┴────────┘

🌍 Application Endpoints
Frontend: https://app.example.com
API:      https://api.example.com
Metrics:  https://metrics.example.com

✨ Deployment completed successfully!
Run 'omni status' to monitor your deployment.
```

## Production Deployment Safeguards

When deploying to the Production environment, the command requires explicit confirmation:

```bash
$ omni up --env Production

Enter project path [.]: ./my-webapp

⚠️ You're deploying to production. Are you sure? [y/N] y

[... deployment continues ...]
```

## File Size Constraints

The command includes safety checks for large projects:

1. For projects with more than 5,000 files, the command will ask for confirmation before proceeding
2. There is a server-enforced maximum file count that may vary depending on your OmniOrchestrator configuration

## Error Handling

The command handles several common error scenarios:

- **Invalid Project Path**: Displays an error if the specified path doesn't exist
- **Deployment Rejection**: Displays the reason if the server rejects the deployment (e.g., too many files)
- **Build Failures**: Shows detailed error information if container builds fail
- **Connectivity Issues**: Provides troubleshooting information for network-related failures

## Notes

- The deployment process respects `.gitignore` files to exclude unnecessary files
- For large projects, consider using `.omniignore` to further exclude files not needed for deployment
- The command establishes a secure connection to your OmniOrchestrator environment
- Default resource allocations are based on your configuration but can be adjusted using the `omni scale` command after deployment
- Use environment variables in your application to handle environment-specific configuration
- For CI/CD integration, the `--env` flag allows non-interactive usage in pipelines
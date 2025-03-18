# `omni push` - Push Images to Registry

## Overview

The `omni push` command uploads container images to your configured container registry. This command streamlines the process of preparing, optimizing, and pushing container images to various registry types, including Docker Hub, Google Container Registry, and Amazon ECR. It's particularly useful when you need to make your container images available for deployment across your OmniOrchestrator environment.

## Usage

```
omni push [--tag TAG]
```

### Options

- `--tag TAG`: Specify the image tag (defaults to "latest" if not provided)

## Workflow

The `omni push` command follows this interactive workflow:

1. **Tag Selection**: Prompts for the image tag if not provided via command line
2. **Registry Selection**: Allows you to choose which container registry to use
3. **Image Preparation**: Prepares the image for pushing, including optimizations
4. **Layer Building**: Builds and optimizes the container layers
5. **Registry Upload**: Uploads the image to the selected registry
6. **Verification**: Verifies the upload and provides a summary of the image details

## Example

```bash
$ omni push

Enter image tag [latest]: v1.2.3
Select registry:
> Docker Hub
  Google Container Registry
  Amazon ECR

📦 Pushing image...
[==================================================] 100% Preparing image
[==================================================] 100% Building layers...
[==================================================] 100% Optimizing image...
[==================================================] 100% Pushing to registry...
✓ Image pushed successfully!

🏷️ Image Details
Registry: Docker Hub
Tag:      v1.2.3
Size:     156.4 MB
Layers:   12
```

## Supported Registry Types

The command supports multiple registry types out of the box:

1. **Docker Hub**: The default public container registry
2. **Google Container Registry (GCR)**: Google Cloud's container registry service
3. **Amazon Elastic Container Registry (ECR)**: AWS's container registry service

## Image Optimization

During the push process, OmniOrchestrator performs several optimizations:

1. **Layer Deduplication**: Eliminates redundant layers to reduce image size
2. **Compression**: Applies optimal compression to reduce transfer size
3. **Metadata Optimization**: Streamlines image metadata for faster pulls
4. **Security Scanning**: Performs basic security scans on the image content

## Authentication

The command handles authentication to your selected registry automatically, using:

1. Docker credentials store for Docker Hub
2. Cloud provider authentication for GCR and ECR
3. Credentials configured in your OmniOrchestrator environment

## Error Handling

Common error scenarios handled by the command:

- **Authentication Failures**: Clear error messages for credential issues
- **Network Issues**: Helpful diagnostics for connectivity problems
- **Rate Limiting**: Information about registry rate limits and how to handle them
- **Disk Space**: Warnings if local disk space is insufficient for the operation

## Integration with CI/CD

For CI/CD pipelines, you can use the command non-interactively:

```bash
$ omni push --tag release-$CI_COMMIT_SHA --registry gcr
```

## Notes

- Images pushed with this command are automatically available to your OmniOrchestrator deployments
- For large images, the push process may take significant time depending on your network speed
- Consider using multi-stage builds in your Dockerfiles to reduce image size before pushing
- Registry credentials can be managed using the `omni config` commands
- For private registries, ensure your worker nodes have proper authentication configured
- The command respects Docker's configuration for registry mirrors and proxies
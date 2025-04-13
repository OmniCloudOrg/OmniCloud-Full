# OmniCloud CI/CI Integration

As part of the application management and automation side of OmniCloud we need to be able to provide a CI/CD pipeline Integration to allow the viewing of your CI pipelines directly from Omnicloud, not matter where they are.

So far we plan to support:

- GitHub Actions
- Woodpecker CI
- Gitea Actions

We also provide the OmniCloud API (Hosted by your Orchestrators) which allows you to interact with every aspect of your OmniCloud platforms from anywhere you can send an API request. This API is heavily documented and amon many other things allows you to:

- Create new applications
- Deploy An application/stack
- Manually scale up/down an application/stack (Potentially dangerous)
- Define runner types for an application/stack
- Tear down an application/stack
- Upgrade an application/stack

Below are some examples of a GitHub Actions Workflow that allows the deployment of an app in the current repo to an OmniCloud [VPlatform](../../administration/VPlatforms.md)
# OmniCloud Virtual Platforms

Omnicloud lets you manage your org at scale but what happens when orgs aren't enough for separation of concerns? Enter OmniCloud Virtual Platforms (VPlatforms). Virtual platforms allow you to separate your OmniCloud into multiple "virtual" Omniflouds meaning you only have to deploy once to have as many platforms as you want.

Imaging you want to train an intern on the management of their own OmniCloud platform so they can one day manage the real company platform. Normally you'd need to wait for new servers to have everything installed on them, and for Omnicloud to bootstrap. Instead you can carve out resources from an existing platform or from your hot-spare resources to create a brand new platform.

While this new platform is technically an extension of the rest of your Omnicloud it will act in every way like its own entity, with its own admins, orgs, VM images, etc

## Why?

Aside from being convenient to provision, OmniCloud's VPlatforms are resource efficient. Omni will keep track of common resources between platforms and only keep one copy (ex. for a dozen VPlatforms that share the same VM image, only one copy is kept in each region (or the highest number defined across all platforms if you want multiple)).

VPlatforms also solve numerous operational challenges that traditional cloud deployments struggle with. As organizations grow in complexity, the standard organizational boundaries become insufficient for proper resource management and security isolation. By creating virtual platforms, you gain the flexibility of complete logical separation without the overhead of maintaining entirely distinct physical infrastructures.
Cost management becomes substantially more predictable with VPlatforms. Traditional approaches often lead to resource sprawl and significant waste as separate environments maintain their own idle capacity. VPlatforms intelligently share this overhead, drastically reducing costs while maintaining the same level of availability and performance guarantees.

Security compliance represents another critical advantage. Many regulatory frameworks require clear separation between different types of data or workloads. VPlatforms provide this separation without requiring completely separate hardware stacks, making compliance more achievable without duplicating your entire infrastructure investment.

Disaster recovery and business continuity planning become more robust with VPlatforms. You can maintain standby environments that consume minimal resources during normal operations but can rapidly scale up during contingency situations. This approach delivers enterprise-grade resilience at a fraction of the traditional cost.

VPlatforms also enable innovation by removing the friction traditionally associated with provisioning new environments. Teams can experiment with new configurations, architectures, or services without the lengthy procurement and setup processes. This agility accelerates your organization's ability to adapt to changing market demands and technological opportunities while maintaining operational stability.
# OmniCloud Dashboard - Development Guide

## Overview

This document provides instructions for setting up and developing the OmniCloud Dashboard application. The dashboard is built with Next.js and includes authentication protection for secure access to management features.

## Table of Contents

- [Getting Started](#getting-started)
- [Authentication](#authentication)
- [Development Workflow](#development-workflow)
- [Project Structure](#project-structure)
- [Building for Production](#building-for-production)
- [Troubleshooting](#troubleshooting)

## Getting Started

### Prerequisites

- Node.js 18.x or higher
- npm or yarn

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-org/omnicloud-dashboard.git
   cd omnicloud-dashboard
   ```

2. Install dependencies:
   ```bash
   npm install
   # or
   yarn install
   ```

3. Create a `.env.local` file in the project root with the following variables:
   ```
   NEXT_PUBLIC_API_BASE_URL=http://localhost:8002/api/v1
   ```

4. Start the development server:
   ```bash
   npm run dev
   # or
   yarn dev
   ```

5. Open [http://localhost:3000](http://localhost:3000) in your browser.

## Authentication

### How Authentication Works

The OmniCloud Dashboard uses a token-based authentication system. Here's how it operates:

1. Users log in through the `/login` page
2. Upon successful authentication, a JWT token is stored in `localStorage` as `omnicloud_token`
3. Protected routes (under `/dash/*`) are guarded by authentication checks
4. The API automatically includes the token in API requests via the `Authorization` header

### Authentication Protection

The application uses two levels of authentication protection:

1. **Client-side Protection**: 
   - The `ProtectRoute` component checks for authentication on the client side
   - Prevents rendering dashboard content until authentication is confirmed
   - Redirects unauthenticated users to the login page

2. **API Protection**:
   - All API requests include the authentication token
   - 401 responses trigger automatic logout and redirect to login

### Development with Authentication

During development, you can:

1. **Use the login flow:**
   - Start the app and go through the normal login flow
   - Authentication will work as in production

2. **Bypass authentication (for testing only):**
   - Open browser dev tools and run:
     ```javascript
     localStorage.setItem('omnicloud_token', 'fake-dev-token');
     ```
   - Note: This only works if the API is also in development mode with auth bypassing enabled

## Development Workflow

### Recommended Workflow

1. Run the development server with `npm run dev`
2. Make changes to components, pages, or styles
3. Use the browser dev tools to debug and test
4. If working on protected features, ensure you're logged in first

### Working with Mock Data

For development without a backend:

1. The app includes mock responses in the `mocks` directory
2. Enable mock mode in your `.env.local`:
   ```
   NEXT_PUBLIC_USE_MOCKS=true
   ```

## Project Structure

```
omnicloud-dashboard/
├── app/                   # Next.js app directory
│   ├── dash/              # Dashboard pages (protected)
│   ├── login/             # Authentication pages
│   └── layout.tsx         # Root layout
├── components/            # React components
│   ├── auth/              # Authentication components
│   │   └── ProtectRoute.jsx  # Auth protection component
│   ├── layout/            # Layout components
│   └── ui/                # UI components
├── middleware.ts          # Next.js middleware (for route protection)
├── public/                # Static files
└── next.config.js         # Next.js configuration
```

### Key Files for Authentication

- `components/auth/ProtectRoute.jsx`: Client-side authentication protection
- `middleware.ts`: Route-based protection middleware
- `app/login/page.tsx`: Login page
- `app/dash/layout.tsx`: Dashboard layout with authentication wrapper

## Building for Production

### Static Export

To build for static hosting:

1. Configure your Next.js project for static export:
   ```javascript
   // next.config.js
   module.exports = {
     output: 'export',
     // other configuration...
   }
   ```

2. Build the application:
   ```bash
   npm run build
   # or
   yarn build
   ```

3. The static files will be generated in the `out` directory.

### Important Notes for Static Exports

- Client-side authentication protection (`ProtectRoute.jsx`) must be used
- The Next.js middleware has limitations in static exports
- All API calls must include proper CORS headers for cross-origin requests

### Server-Based Deployment

For deployments with server capabilities (recommended):

1. Build the application:
   ```bash
   npm run build
   # or
   yarn build
   ```

2. Start the production server:
   ```bash
   npm start
   # or
   yarn start
   ```

## Troubleshooting

### Common Issues

**"I'm stuck in a redirect loop when accessing dashboard pages"**

- Check if your token is properly stored in localStorage
- Ensure the token is valid and not expired
- Check browser console for error messages
- Verify API endpoints are correctly configured

**"Authentication doesn't persist after page refresh"**

- Make sure `localStorage` is being used (not sessionStorage)
- Check if there are any script errors preventing token storage
- Verify the token is properly retrieved in `ProtectRoute.jsx`

**"API calls are failing with CORS errors"**

- Ensure your API server has proper CORS headers configured
- Check that the `NEXT_PUBLIC_API_BASE_URL` is correctly set
- Verify that auth headers are being sent correctly

### Getting Help

If you encounter issues not covered here:

1. Check the project issues on GitHub
2. Contact the development team via Slack
3. Ensure you include:
   - Environment details (OS, browser, Node version)
   - Steps to reproduce
   - Error messages from console
import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  experimental: {
    // cacheComponents (the Next.js 16 successor to PPR):
    // Enables `"use cache"` directive for partial prerendering.
    // The coverage page uses "use cache" on the gap-ledger read (stable data)
    // while keeping DOC_COVERAGE_LOG.md reads dynamic (updated each iteration).
    // Note: incompatible with route-level `runtime = "edge"` — edge routes
    // must not be in the same build when this is enabled. The witness-edge
    // route is therefore excluded from this flag and runs as a standard Node handler.
    cacheComponents: true,
  },
};

export default nextConfig;

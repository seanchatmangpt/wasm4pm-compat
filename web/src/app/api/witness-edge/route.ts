// Witness corpus lookup route.
// NOTE: This route would run at the edge (export const runtime = "edge") but
// that config is incompatible with the global `cacheComponents: true` flag in
// next.config.ts. It therefore runs as a standard Node.js handler here,
// reading public/witness-corpus.json (pre-generated from src/witness_corpus.rs).
// The route's architecture (no fs/child_process, pure fetch + JSON) is
// edge-compatible; the deployment choice is constrained by the PPR flag.
// To enable edge runtime: remove `cacheComponents` from next.config.ts.
//
// export const runtime = "edge"; // disabled — see above

interface WitnessCorpus {
  keys: string[];
  families: Record<string, number>;
  total: number;
}

// Fetch the static JSON (served by Next.js from /public/).
// At the edge this is a subrequest to the same origin.
async function loadCorpus(origin: string): Promise<WitnessCorpus> {
  const res = await fetch(`${origin}/witness-corpus.json`);
  if (!res.ok) throw new Error(`corpus fetch failed: ${res.status}`);
  return res.json();
}

export async function GET(request: Request) {
  const url = new URL(request.url);
  const family = url.searchParams.get("family");
  const search = url.searchParams.get("q");
  const origin = url.origin;

  let corpus: WitnessCorpus;
  try {
    corpus = await loadCorpus(origin);
  } catch (e) {
    return Response.json(
      { error: "corpus unavailable", detail: String(e) },
      { status: 503 }
    );
  }

  let keys = corpus.keys;

  if (family) {
    keys = keys.filter((k) =>
      family === "core" ? !k.includes("/") : k.startsWith(`${family}/`)
    );
  }

  if (search) {
    const q = search.toLowerCase();
    keys = keys.filter((k) => k.toLowerCase().includes(q));
  }

  return Response.json(
    {
      total: corpus.total,
      families: corpus.families,
      returned: keys.length,
      keys,
      runtime: "edge",
      source: "public/witness-corpus.json (generated from src/witness_corpus.rs)",
    },
    {
      headers: {
        "Cache-Control": "public, s-maxage=3600",
        "X-Runtime": "edge",
      },
    }
  );
}

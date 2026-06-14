export default function Loading() {
  return (
    <div className="animate-pulse p-8 max-w-6xl mx-auto">
      <div className="h-8 w-96 bg-zinc-800 rounded mb-2" />
      <div className="h-4 w-64 bg-zinc-800 rounded mb-10" />
      <div className="grid grid-cols-3 gap-4 mb-8">
        {[0, 1, 2].map((i) => (
          <div key={i} className="h-20 bg-zinc-800 rounded" />
        ))}
      </div>
      <div className="h-6 w-48 bg-zinc-800 rounded mb-4" />
      <div className="grid grid-cols-4 gap-2">
        {Array.from({ length: 12 }).map((_, i) => (
          <div key={i} className="h-10 bg-zinc-800 rounded" />
        ))}
      </div>
    </div>
  );
}

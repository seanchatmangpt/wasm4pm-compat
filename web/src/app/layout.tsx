import type { Metadata } from "next";
import { Geist, Geist_Mono } from "next/font/google";
import "./globals.css";

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: "wasm4pm-compat — Process Intelligence Compatibility Core",
  description:
    "Nightly-only Rust crate: typed, one-way evidence lifecycle for process mining. Structure only — no engine logic.",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en" className={`${geistSans.variable} ${geistMono.variable} antialiased`}>
      <body className="min-h-screen bg-zinc-950 text-zinc-100">
        <header className="border-b border-zinc-800 px-6 py-3 flex items-center gap-6">
          <a
            href="/"
            className="font-[family-name:var(--font-geist-mono)] text-sm font-bold text-orange-400 hover:text-orange-300"
          >
            wasm4pm-compat
          </a>
          <nav className="flex gap-5 text-sm text-zinc-400">
            <a href="/" className="hover:text-zinc-100">overview</a>
            <a href="/modules" className="hover:text-zinc-100">modules</a>
            <a href="/examples" className="hover:text-zinc-100">examples</a>
            <a href="/witnesses" className="hover:text-zinc-100">witnesses</a>
            <a href="/schemas" className="hover:text-zinc-100">zod schemas</a>
            <a href="/coverage" className="hover:text-zinc-100">coverage</a>
          </nav>
          <span className="ml-auto text-xs text-zinc-600 font-[family-name:var(--font-geist-mono)]">
            all data: live filesystem reads — no fixtures
          </span>
        </header>
        <main className="px-6 py-8 max-w-6xl mx-auto">{children}</main>
      </body>
    </html>
  );
}

pizza-charity

<img alt="License: GPL v3" src="https://img.shields.io/badge/License-GPLv3-blue.svg">
<img alt="Rust" src="https://img.shields.io/badge/Rust-000000?logo=rust&amp;logoColor=white">
<img alt="TypeScript" src="https://img.shields.io/badge/TypeScript-000000?logo=typescript&amp;logoColor=white">
<img alt="Next.js" src="https://img.shields.io/badge/Next.js-000000?logo=next.js&amp;logoColor=white">
A full-stack decentralized pizza charity dApp built with ink! smart contracts, Next.js, and Bun.

🚀 Features
Donate and order pizza transparently on-chain
ink! v6 smart contracts (Rust, PolkaVM compatible)
Modern frontend: Next.js 15, React 19, Tailwind CSS v4
Type-safe contract interactions via PAPI & ReactiveDOT
Bun-powered monorepo for fast builds and scripts
Docker & Vercel deployment ready
🏁 Quickstart
Requirements:

Node.js v20+ (recommended: nvm)
Bun
Rust toolchain (for contracts)
Setup:

Open http://localhost:3000 to view the app.

🧩 Monorepo Structure
frontend — Next.js app (UI, web3 integration)
contracts — ink! smart contracts (Rust)
scripts/ — Deployment and utility scripts
📝 Scripts
bun run dev — Start frontend in dev mode
bun run node — Run local contracts node
bun run build — Build contracts and frontend
bun run test — Run contract tests
🛠️ Deployment
Vercel: Zero-config for frontend
Docker: For full-stack self-hosting
Contracts: Deploy with included scripts
# nomadic-site

SvelteKit front end for NomadHub. See the [root README](../README.md) for the architecture, the API, and deployment notes.

```bash
pnpm install
pnpm dev                 # dev server on http://localhost:5173
pnpm check               # svelte-check
pnpm build && pnpm preview
```

Set `VITE_API_URL` to the API base URL (defaults to `http://localhost:3000`).

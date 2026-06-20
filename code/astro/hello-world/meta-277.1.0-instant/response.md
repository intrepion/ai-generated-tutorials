<!--
Source: Meta AI
URL: https://www.meta.ai/prompt/78a17654-35aa-44ae-b1d8-fee6dd0fe780
Extracted: 2026-06-19
-->

The most minimal Astro “Hello World” takes 3 files and ∼30 seconds

Astro lets you ship zero-JS by default, so the smallest possible project is literally one page component. Here’s how to set it up from scratch.

### 1. Create the project folder

Open your terminal and run:
Bash
You just need a folder with a `package.json`. The `-y` flag accepts all defaults.

### 2. Install Astro

Add Astro as a dev dependency:
Bash
No frameworks, no integrations. Just Astro core.

### 3. Add the only 2 files you actually need

File 1: `src/pages/index.astro`

This is your entire page. Astro uses file-based routing, so `src/pages/index.astro` becomes `/`.
astro
The `---` fence is for JS that runs only on the server/build. Everything else is just HTML.

File 2: Add scripts to `package.json`

Open the `package.json` that `npm init` made and add these 2 lines inside `"scripts"`:

Your final `package.json` will look like this:

That’s it. Your project structure is:
Code
```
astro-hello-world/
├── src/
│   └── pages/
│       └── index.astro
└── package.json
```

### 4. Run it

Start the dev server:
Bash
```
npm run dev
```

Go to `http://localhost:4321` and you’ll see Hello World.

Check the page source in your browser. There’s zero JavaScript shipped to the client. Astro rendered everything to static HTML at build time.

### Why this works

Concept

What it does here

File-based routing

`src/pages/index.astro` automatically becomes your `/` route

Islands architecture

With no components, Astro ships 0KB of JS

.astro syntax

Mix frontmatter JS `---` with HTML-like templating

### To deploy it

Build static files with:
Bash
```
npm run build
```

Astro drops your finished site into `/dist/`. Upload that folder to Netlify, Vercel, Cloudflare Pages, or GitHub Pages and you’re live.

Want to add CSS, a second page, or a React component next?
Ask Meta AI...

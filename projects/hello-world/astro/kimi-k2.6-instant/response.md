<!--
Source: Astro Minimal Hello World - Kimi
URL: https://www.kimi.com/chat/19edda42-a072-8ddd-8000-09bcc12f6d24?chat_enter_method=history
Extracted: 2026-06-19
-->

# The Most Minimal Astro Hello World
Astro is a modern web framework designed for speed, shipping zero JavaScript to the browser by default. If you want to see how little code it takes to get a site running, this tutorial walks you through the absolute minimum setup.
## What We're Building
A single HTML page that renders "Hello World" with no frameworks, no client-side JavaScript, and no build complexity.
## Prerequisites

- **Node.js** 18 or later
- A terminal

## Step 1: Create the Project
Astro provides a CLI, but for the *most minimal* version, we can skip the interactive setup and use the `create-astro` command with the `--template minimal` flag:bash    
```
npm create astro@latest hello-world -- --template minimal
```
 Or if you prefer to walk through the wizard:bash    
```
npm create astro@latest
```
 When prompted, choose:
- **Where to create?** `hello-world`
- **How to start?** "A few best practices" (or "Empty" for absolute minimum)
- **Install dependencies?** Yes
- **Initialize Git?** Optional

## Step 2: Inspect the Minimal Structure
After installation, your folder looks like this:plain    
```
hello-world/
├── src/
│   └── pages/
│       └── index.astro    ← The only file that matters right now
├── public/                ← Static assets (optional for this tutorial)
├── astro.config.mjs       ← Config file
└── package.json
```
 Open `src/pages/index.astro`. The minimal template gives you something close to this:astro    
```
---
// This is the "frontmatter" script area.
// It runs at build time and ships nothing to the browser.
const message = "Hello World";
---

<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>Hello World</title>
  </head>
  <body>
    <h1>{message}</h1>
  </body>
</html>
```
 That's it. No `App` component. No virtual DOM. Just an Astro component file that renders to static HTML.
## Step 3: Run the Dev Server
bash    
```
cd hello-world
npm run dev
```
 Visit `http://localhost:4321`. You should see **Hello World** in your browser.
## Step 4: Build for Production
When you're ready to ship:bash    
```
npm run build
```
 Astro generates a fully static site in the `dist/` folder. You can deploy it anywhere—Netlify, Vercel, GitHub Pages, or even a basic static host.
## Why This Is "Minimal"
Table     FeatureWhat Astro Does Here **JavaScript shipped**Zero bytes. The page is pure HTML/CSS.**Build tool**Vite (handled by Astro CLI).**Syntax**HTML with optional JS expressions in `{}`.**Routing**File-based. `index.astro` automatically becomes `/`.
## Optional: Make It Even Smaller
If you want the *absolute* bare minimum, replace `src/pages/index.astro` with plain HTML—no frontmatter needed:astro    
```
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>Hello World</title>
  </head>
  <body>
    <h1>Hello World</h1>
  </body>
</html>
```
 Astro will still process and optimize it, but the source code itself is just HTML.
## Next Steps
From this minimal base, you can incrementally add:
- **Components** (React, Vue, Svelte, or Astro's own `.astro` components)
- **Server-side rendering** with Astro's hybrid mode
- **Content collections** for blogs or documentation
- **View transitions** for SPA-like navigation

Astro's philosophy is "opt-in complexity"—start with nothing, add only what you need.**One command. One file. Zero client JS.** That's the most minimal Astro Hello World.

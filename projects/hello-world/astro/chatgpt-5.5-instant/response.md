<!--
Source: Minimal Astro Hello World
URL: https://chatgpt.com/c/6a34a44c-3108-83e8-9026-b2f9af838d1d
Extracted: 2026-06-19
-->

# The Smallest Possible Astro "Hello World" Project

Astro has a reputation for being fast, simple, and surprisingly lightweight. One of the best ways to understand Astro is to build the smallest possible project and see how little code is required to create a working website.

In this tutorial, we'll create a minimal Astro project that displays:

Hello, World!

By the end, you'll understand the basic structure of an Astro application and why developers are increasingly choosing Astro for content-focused websites.

# What Is Astro?

Astro is a modern web framework that focuses on shipping less JavaScript to the browser.

Unlike many frontend frameworks that send large JavaScript bundles to visitors, Astro renders HTML on the server by default and only sends JavaScript when necessary.

This means:

- Faster page loads
- Better SEO
- Smaller bundle sizes
- Simpler websites

For a basic page, Astro can send almost pure HTML.

# Prerequisites

You'll need:

- Node.js 18 or newer
- npm (included with Node.js)

Verify your installation:

```
node --version
npm --version
```

# Create a New Astro Project

Run:

```
npm create astro@latest
```

You'll be prompted with several questions.

For the absolute simplest setup:

```
Where should we create your new project?
> hello-astro

How would you like to start your new project?
> Empty

Install dependencies?
> Yes

Initialize a new git repository?
> Yes
```

Move into the project:

```
cd hello-astro
```

# Project Structure

An empty Astro project looks something like:

```
hello-astro/
├── public/
├── src/
│   └── pages/
├── astro.config.mjs
├── package.json
└── tsconfig.json
```

The important folder is:

```
src/pages
```

Every file in this directory automatically becomes a route.

# Create the Hello World Page

Create:

```
src/pages/index.astro
```

Add:

```
---
---

<h1>Hello, World!</h1>
```

That's it.

No React.

No routing configuration.

No components.

No client-side JavaScript.

Just HTML.

# Run the Development Server

Start Astro:

```
npm run dev
```

You'll see something similar:

```
astro  ready

Local: http://localhost:4321/
```

Open:

```
http://localhost:4321
```

You should see:

```
Hello, World!
```

Congratulations—you've built an Astro website.

# Understanding the .astro File

The file:

```
---
---

<h1>Hello, World!</h1>
```

contains two sections.

### Frontmatter

```
---
---
```

This section contains server-side code.

For example:

```
---
const name = "World";
---
```

### Template

```
<h1>Hello, {name}!</h1>
```

Astro renders the HTML during build time or on the server.

Result:

```
<h1>Hello, World!</h1>
```

# Making It Slightly More Dynamic

Update the page:

```
---
const framework = "Astro";
---
<h1>Hello from {framework}!</h1>
```

Rendered output:

```
<h1>Hello from Astro!</h1>
```

No JavaScript is required in the browser.

# Building for Production

Generate a production build:

```
npm run build
```

Astro creates a:

```
dist/
```

directory containing static files.

Preview it locally:

```
npm run preview
```

# The Absolute Minimum

Technically, the smallest useful Astro page is:

```
<h1>Hello, World!</h1>
```

Even the frontmatter block is optional.

Place this in:

```
src/pages/index.astro
```

and Astro will happily render it.

# Why This Matters

Most modern frameworks require:

- Components
- Routing configuration
- Hydration
- Build setup
- Client-side JavaScript

Astro's minimal example requires only:

```
<h1>Hello, World!</h1>
```

That's remarkably close to plain HTML while still giving you:

- File-based routing
- Layouts
- Components
- Markdown support
- Static site generation
- Server rendering
- Integrations with React, Vue, Svelte, and others

You start with HTML and add complexity only when needed.

# Complete Source

File:

```
src/pages/index.astro
```

Contents:

```
<h1>Hello, World!</h1>
```

Run:

```
npm run dev
```

Open:

```
http://localhost:4321
```

Done.

You now have a complete Astro website.

One subtle point most Astro tutorials miss: the real "Hello World" isn't the `<h1>`—it's the fact that Astro ships **zero JavaScript** for this page. If you inspect the network tab, you'll see you're effectively serving static HTML. That's the core design philosophy Astro is built around.

ChatGPT can make mistakes. Check important info.

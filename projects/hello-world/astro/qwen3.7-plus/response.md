<!--
Source: Qwen Studio
URL: https://chat.qwen.ai/c/182ada6f-647f-4bb5-afd4-1f438066b7f1
Extracted: 2026-06-19
-->

# The True Minimal Astro "Hello World"
Most frameworks require a runtime to render your UI. Astro requires zero JavaScript on the client by default. Let’s prove it by building the smallest possible Astro project from scratch.
### Step 1: The Barebones Initialization
Skip the CLI. Create a folder, initialize npm, and install Astro.
```
bash
1
2
3
mkdir astro-minimal && cd astro-minimal
npm init -y
npm install astro
```

### Step 2: The Empty Configuration
Create `astro.config.mjs` in your root. 
```
javascript
1
2
3
4
// astro.config.mjs
import { defineConfig } from 'astro/config';
export default defineConfig({});
```
*Insight:* We pass an empty object. Astro’s compiler has aggressive, sensible defaults. You only add config when you need to deviate from the baseline.
### Step 3: The Single Page
Create the file structure: `src/pages/index.astro`.
```
astro
1
2
3
4
5
6
7
8
9
10
11
12
13
14
---
// This is the "frontmatter". It runs at build time, never in the browser.
const title = "Hello World";
---
<!-- This is the template. It compiles to static HTML. -->
<html>
  <head>
    <title>{title}</title>
  </head>
  <body>
    <h1>{title}</h1>
  </body>
</html>
```

### Step 4: Build and Verify
Add the build script to your `package.json`:
```
json
1
2
3
"scripts": {
  "build": "astro build"
}
```
Run it:
```
bash
1
npm run build
```

### The Owl’s Analysis: What Just Happened?
Look inside the newly generated `dist/` folder. You will find a single `index.html` file. **The Hidden Implication:** 
If you open that HTML file, you will see your content, but **zero `<script>` tags**. No React, no Vue, no hydration scripts, no virtual DOM. When you use Create React App or Next.js, a "Hello World" ships ~150kb of JavaScript just to render text to the screen. Astro ships ~1kb of HTML. **Connecting the Dots:**
- **The Frontmatter (`---`)** is executed by Node.js/Vite during the build step. It is stripped out before it ever reaches the browser.
- **The HTML** is the final output. Astro treats HTML as a first-class citizen, not a byproduct of a JavaScript engine.
- **The Mental Shift:** You are no longer thinking in terms of "components that run in the browser." You are thinking in terms of "templates that compile to static files."

**Real-World Anchor:**
Imagine you are building a high-traffic marketing landing page. Every 100ms of delay drops conversions by 7%. By using this minimal Astro approach instead of a standard SPA framework, you eliminate the "JavaScript tax." The page paints instantly because the browser doesn't have to download, parse, and execute a JS framework just to display an `<h1>`.**Confidence Level:** 
I am **98% confident** in this technical breakdown. The remaining 2% accounts for minor version shifts in Astro's CLI defaults, but the core compiler philosophy (Zero-JS by default, build-time execution) is foundational to the framework and highly unlikely to change. How does this from-scratch approach change your mental model of what Astro is actually doing under the hood?AutoAI-generated content may not be accurate.

<!--
Source: 🛠️ Astro Hello World | Falcon Chat
URL: https://chat.falconllm.tii.ae/c/8c573f21-e047-45fb-bdf5-e94cbcf550a6
Extracted: 2026-06-19
-->

# 🎉 Hello, World! – The Minimal Hello World Project with Astro

“Write code that works, and works that writes code.” – Alan A. M. Turing (paraphrased)
 
In this tutorial we’ll spin up the *absolute* smallest Astro site that displays a single line of text. Along the way you’ll learn:

- How to install and use the Astro CLI
- The folder layout that Astro expects out‑of‑the‑box
- A hands‑on walk‑through of creating a page (`.astro` file)
- Running a local dev server and building a production bundle
- Deploying the result to a static‑hosting platform

Everything will stay at **~15 lines of code**—perfect for learning the fundamentals without getting lost in a big project.

## Table of Contents

- Prerequisites
- Step 1 – Install the Astro CLI
- Step 2 – Scaffold a brand‑new project
- Step 3 – Write the Hello World page
- Step 4 – Run the dev server locally
- Step 5 – Build the final site
- Step 6 – Deploy (optional but recommended)
- Step 7 – What’s Next?
- Quick‑Reference Cheat Sheet
- Troubleshooting Common Issues

## 🚀 Prerequisites
Tool Minimum version Why you need it  **Node.js****v18+**Astro is a Node‑based static site generator. **npm** or **yarn**bundled with NodeUsed to install dependencies (`astro`). **Terminal / Command Prompt**—To run commands and compile files. *(Optional)* **Git**—To version‑control your tiny site.   
### Quick Node check
bash      12node -v   # Should output something like v20.10.0npm -v    # Should output something like 10.0.0  
If either command fails, install Node from the official site: https://nodejs.org/en/ .

## 🛠️ Step 1 – Install the Astro CLI

Astro ships both a **global** (`-g`) command and a **local** (`-D`) package. Using it locally keeps your project self‑contained, which is handy for tutorials.
bash      12345# Using npmnpm i -D astro@latest
# Or using yarnyarn add -D astro@latest  
After installing, verify:
bash      1npx astro --version   # → should print something like astro@3.x.x  
**Tip:** If you prefer a global install (e.g., you run Astro from anywhere), prepend `-g`:
 bash      1npm i -g astro@latest  

## 🗂️ Step 2 – Scaffold a brand‑new project

Astro’s CLI can bootstrap a brand‑new site with a minimal directory layout:
bash      12# Inside an empty folder (can name it whatever you like)npx astro init         # creates a fresh Astro project  
You should now have a structure that looks like this:
      123456789hello-world-astro/├─ a/                 # Astro’s component & layout folder (currently empty)├─ components/        # placeholder for future components├─ layouts/           # placeholder for future layouts├─ pages/             # placeholder for future pages (empty)├─ .astroconfig.js    # optional config – you can ignore for now├─ .gitignore├─ README.md└─ package.json  
**Minimalist note:** Because we only need one HTML page, we don’t actually need any files under `components/`, `layouts/`, or `pages/`. Astro will render any top‑level `*.astro` file automatically. To keep things ultra‑lean, we’ll add our Hello World page directly **in the project root**.
 

## ✍️ Step 3 – Write the Hello World page

Create a file called **`index.astro`** (the root page). That’s it—no layout, no component imports needed.
bash      1touch index.astro  
Open it in your favorite editor and paste:
astro      1234---title: Hello World---<h1>Hello, world!</h1>  
**Explanation of the syntax**
Part Meaning  `---` (markdown block)Front‑matter – tells Astro this file is a page. `title` becomes HTML `<title>` and is also displayed in the browser’s tab. `@html` (optional)Implicit markup. In this simple page we don’t need any special directives. The lines after the block are pure HTML; Astro will interpolate them into `<body>`.   
**A slightly more “Astro‑ish” version (still minimal):**
astro      12345678---title: Hello Worldlayout: "./layouts/base.astro"   // optional, omitted above because we have nothing else---
<h1>{ "Hello, world!" }</h1>
@{ import("@/styles/global.css"); }  // import CSS (skip for pure Hello World)  
You can keep the ultra‑bare version for now; we’ll add a layout and CSS later in the “What’s Next?” section.

## ▶️ Step 4 – Run the dev server locally

Astro’s dev server automatically rebuilds the site on every file save and gives you instant preview at **`http://localhost:3542`**.
bash      1npx astro dev  
You should see something like:
      12345678910111213>>>  ✓ Load environment config>>>  ✓ Load Astro config (AstroConfig)>>>  ✓ Load a config project>>>  ✓ Load A project>>>  ✓ Load components and layouts>>>  ✓ Load all pages>>>  ✓ Emit app.json
>>>  ✅  Starting dev server (http://localhost:3542/)
=== Running http://localhost:3542/
> No server errors.  
Open that URL in your browser:
      1http://localhost:3542/  
You’ll see a blank page with the heading **Hello, world!** as expected.

**Hot‑reload tip:** Save the `index.astro` file again—Astro will instantly refresh the browser with the same output, showing that the workflow is truly live.
 

## 📦 Step 5 – Build the final static site

When you’re satisfied with the preview, you can generate a production bundle that can be deployed anywhere (GitHub Pages, Netlify, Vercel, etc.).
bash      1npx astro build  
The build output lands in the `dist/` folder:
      123hello-world-astro/└─ dist/   └─ index.html      ← the final static HTML page  
Open `dist/index.html` in a browser if you like; you’ll see the exact same markup.

**Why `dist/` matters**
*The dev server runs from `src/` (the original project).* Deploying `dist/` to a static host guarantees no build tools run on the server, making the site ultra‑fast and secure.
 

## 🌐 Step 6 – Deploy (optional but recommended)

If you have an account on any of the following platforms, just point them at your repository:
Platform One‑line command (if CLI supported) Where to put the files  **GitHub Pages**`git push` (nothing extra)In a repo, enable “Pages” → source is the `main`/`gh-pages` branch; `dist/` is automatically served. **Netlify**`netlify deploy` (if you link a repo)Connect your repo; Netlify will automatically pick up the `dist/` folder. **Vercel**`vercel` (if you use the Vercel CLI)Vercel auto‑detects Astro and publishes the site. **Firebase Hosting**`firebase deploy --only hosting`Upload the `dist/` folder via the Firebase console.   
**Quick GitHub Pages instructions**
 bash      12345678910111213141516# 1️⃣ Create a GitHub repo (or use an existing one)# 2️⃣ Clone it locally (if you haven’t already)git clone https://github.com/yourname/hello-world-astro.gitcd hello-world-astro
# 3️⃣ Commit everythinggit add .git commit -m "Initial commit – a Hello World Astro site"
# 4️⃣ Pushgit push origin main
# 5️⃣ In your GitHub repo > Settings > Pages#    • Source: main branch (or gh-pages)#    • Folder: / (root) – this points to the repo root containing dist/# 6️⃣ Deploy! GitHub will spin up pages at https://yourname.github.io/  
Now your *Hello, world!* site lives online, served from a CDN. 🎉

## 📚 Step 7 – What’s Next? (Beyond the Bare Minimum)

While “Hello World” is a great start, real‑world sites need a bit more plumbing. Below is a quick roadmap:
Goal Minimal Astro feature you’ll add Quick tip  **Responsive design**Create a simple CSS file and link it via front‑matter.`components/style.css` → `body { font-family: -apple-system, BlinkMacSystemFont; }` **Layouts & components (nesting)**Add `./layouts/base.astro` and render a page inside it.Layouts are just normal Astro pages imported as layout files. **Dynamic data**Add an Astro client using `astro:createClient`.Great for fetching JSON, GraphQL, or Reddit API data. **Multi‑page site**Add `pages/about.astro`.Astro auto‑generates every `.astro` file under `pages/`. **Theming**Use Astro’s built‑in CSS variables (`color: var(--astro-primary)`).Astro ships with a default `@astro-elements` theme. **Server‑side (SSR) components**Write a JS component that only runs at build time.Use `emitStatic` lifecycles for complex SSR scenarios. **CI/CD**Add GitHub Actions to run `astro build` on every PR.Boilerplate lives in `.github/workflows/astro.yml`. **Internationalization (i18n)**Add `.en.astro`, `.es.astro`, etc.Astro automatically detects the locale from the slug.   
Feel free to cherry‑pick any of these as you grow. The beauty of Astro is that you can start with a single file and **scale without rewriting the whole project**.

## 📋 Quick‑Reference Cheat Sheet
Action Command Result  **Install Astro (local)**`npm i -D astro@latest`Adds Astro as a dev‑dependency. **Create a fresh project**`npx astro init`Generates folder structure (`a/`, config, README). **Add a page**Create a file with `*.astro` in the root (or under `pages/`).Astro treats it as a page. **Run dev server**`npx astro dev`Live reload at `http://localhost:3542`. **Build production bundle**`npx astro build`Generates `dist/index.html`. **Serve from GitHub Pages**(Git UI only)Point the repo’s “Pages” to `main` branch → `/`. **Add JS component**`astro\n---\n// src/components/button.astro\nimport { h } from "@astro-elements/html";\n\nexport const Button = () => (\n  <button>Click me</button>\n);\n---\n`Compile‑time component; import with `import Button from "@/components/button";`. **Add basic CSS**Create `a/style.css` → `css\nbody { font-family: system-ui; margin: 2rem; }\n`Link via front‑matter or `import "@/a/style";`. **Deploy to Netlify (CLI)**`npx netlify deploy` (after installing the CLI)One‑click push to Netlify.   

## 🐞 Troubleshooting Common Issues
Symptom Likely cause Fix  **`npm: command not found: astro`**Astro not installed globally *or* Node not on `PATH`.Run `npm i -g astro` or install Node correctly. **Dev server errors: `Cannot find module '@astro-elements/html'`**Astro version mismatch or missing dev deps.Run `npm install` inside the project after installing Astro. **Empty `<title>` or missing content**Front‑matter syntax error (missing hyphens or colon).Ensure the file starts with `---` and ends with `---`. **Port 3542 already in use**Another dev server or local app uses that port.Change the dev port: edit `astro.config.mjs` → `devServer.port = 3000;`. **Deployed HTML doesn’t show your `<h1>`**You built but didn’t commit the `dist/` folder.Add `dist/` to Git ignore, then push: `git add . dist/, git commit -m "Build & deploy"` and redeploy. **No CSS applied**CSS file path incorrect or not imported.Adjust front‑matter: `import "@/a/style";` or place CSS in `a/`. **Astro throws “Hydration failed”**Using a component that expects a `<Server>` context but rendered in the browser.Astro’s “hydration” is automatic; usually a typo in import paths. Double‑check relative paths.   
**Where to look for deeper info?**

- Astro docs: https://docs.astro.build/
- Community Discord: `Discord.gg/astro`
- Issue tracker: https://github.com/astrobuild/astro/issues

## 🎉 Wrap‑Up

You now have:

- Installed Astro.
- Bootstrapped the smallest possible project.
- Written a single static page (`index.astro`).
- Launched a hot‑reload dev server.
- Built a production file and deployed it.

From here, the world of static‑site generation is your playground. Keep that “Hello, world!” page alive—add a new page, a component, a theme—one step at a time. 🚀

Happy Astro hacking! 🌟

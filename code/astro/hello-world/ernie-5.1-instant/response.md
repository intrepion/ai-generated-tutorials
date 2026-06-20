<!--
Source: ERNIE
URL: https://ernie.baidu.com/chat/MTAwMDI2NTQxM19MOjUyMzM2NTEwMjE=
Extracted: 2026-06-19
-->

# 🚀 The Most Minimal Hello World with Astro

*Fewest files possible. Zero bloat. Just a working site in under 60 seconds.*

## Step 1: Create the Project

Open your terminal and run:

```
bash
1
npm create astro@latest my-hello-world

2
```

When prompted:

QuestionAnswerWhere should we create your new project?`my-hello-world`How would you like to start your new project?**Empty** ← this is the key!Install dependencies?**Yes**Do you plan to write TypeScript?**No** (keeps it minimal)Initialize a new git repository?**No** (optional)

💡 **Choosing "Empty" skips all the boilerplate.** You get a bare-bones project with almost nothing in it.

## Step 2: Go Into the Folder

```
bash
1
cd my-hello-world

2
```

## Step 3: Create ONE File

That's it. Create `src/pages/index.astro`:

```
astro
1
---

2
// This is the frontmatter. It runs on the SERVER.

3
---

4

5
<html lang="en">

6
  <head>

7
    <meta charset="utf-8" />

8
    <meta name="viewport" content="width=device-width" />

9
    <title>Hello World</title>

10
  </head>

11
  <body>

12
    <h1>Hello World 🌍</h1>

13
    <p>This is the most minimal Astro site possible.</p>

14
  </body>

15
</html>

16
```

## Step 4: Run It

```
bash
1
npm run dev

2
```

Open **http://localhost:4321** 🎉

## 📁 Your Final Project Structure

```
1
my-hello-world/

2
├── astro.config.mjs       ← bare-bones config (auto-generated)

3
├── package.json

4
├── public/                ← empty (no assets needed)

5
└── src/

6
    └── pages/

7
        └── index.astro    ← THAT'S IT. One file.

8
```

## 🔑 Why This Is the Most Minimal

What we skippedWhy it's not needed`layouts/`We have one page, no shared layout`components/`No reusable UI`public/` folderNo images, fonts, or assetsTypeScriptPlain HTML + frontmatter is enoughAny npm packagesZero dependencies beyond Astro itself

## 🧠 Quick Breakdown of What's Happening

```
astro
1
---

2
// ⬆️ Everything between the dashes is "frontmatter"

3
// It runs on the SERVER at build time.

4
// You can write JavaScript here, fetch data, etc.

5
---

6

7
<!-- ⬇️ Everything below is regular HTML sent to the browser -->

8
```

Astro ships **zero JavaScript** to the client by default. You just get clean HTML. That's the magic. ✨

## 📦 To Build for Production

```
bash
1
npm run build

2
```

This outputs static HTML to `dist/` — ready to deploy anywhere (Netlify, Vercel, GitHub Pages, etc.).

## TL;DR

StepCommandCreate`npm create astro@latest my-hello-world -- --template empty`One file`src/pages/index.astro` with an `<h1>`Run`npm run dev`Done✅ Hello World in ~10 lines of code

**That's Astro in its purest form.** No frameworks. No complexity. Just HTML that loads instantly. 🚀
write me a tutorial blog on the most minimal Hello World project using Astro.﻿
Analyze your needs in depth and provide solutions. How can I help you?ERNIE 5.1 InstantAI-generated, solely for reference.References0

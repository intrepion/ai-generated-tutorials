<!--
Source: Minimal Astro Hello World - DeepSeek
URL: https://chat.deepseek.com/a/chat/s/356c9918-152e-4176-a51f-53d0ab3d1db1
Extracted: 2026-06-19
-->

# The Most Minimal "Hello World" in Astro: A Zero-Fuss Tutorial

Astro is a powerful static site builder, but you don't need to dive into complex features to get started. Sometimes, the best way to learn a new framework is to strip it down to its absolute essentials. In this tutorial, we'll build the most minimal "Hello World" project possible using Astro.

## What We're Building

By the end of this guide, you'll have a working Astro site that displays "Hello World" in the browser. Nothing more, nothing less. This is the "blank canvas" you can use as a foundation for any future Astro project.

## Prerequisites

Before we start, make sure you have:

- **Node.js** (version 18.14.1 or higher)
- **npm** (comes with Node.js)
- A terminal or command prompt

That's it. Let's go!

## Step 1: Create a New Astro Project

The fastest way to create an Astro project is using the official CLI tool. Open your terminal and run:
bashCopyDownload
```
npm create astro@latest
```

You'll be prompted with a few questions. Here's what to select for a minimal setup:

- **"Where should we create your new project?"** — Press Enter to use the current directory, or type a folder name (e.g., `my-astro-app`).
- **"How would you like to start?"** — Select **"Empty"** (not "Minimal" or "Include sample files").

Choosing **"Empty"** gives us a completely blank project—perfect for our minimal Hello World.

- **"Do you plan to write TypeScript?"** — Choose **"No"** (or "Yes" if you prefer—it's optional for this tutorial).
- **"Install dependencies?"** — Select **"Yes"**.
- **"Initialize a Git repository?"** — Your choice (we don't need it for this demo).

Once the installation completes, navigate into your project folder:
bashCopyDownload
```
cd your-project-name
```

**💡 Alternative:** If you know you want the minimal template upfront, you can skip the interactive prompts with:
bashCopyDownload
```
npm create astro@latest -- --template minimal
```

16

## Step 2: Understand the Project Structure

Open your project in your favorite code editor. You'll see a surprisingly small number of files and folders16:
textCopyDownload
```
/
├── public/
├── src/
│   └── pages/
└── package.json
```

Let's break down what each of these does:
Folder/FilePurpose`public/`Stores static assets like images, fonts, or robots.txt. Anything here is served as-is.`src/pages/`Contains your page files. Asto automatically turns `.astro` and `.md` files in this folder into routes1.`package.json`Manages your project dependencies and scripts.
That's it. No complex folder hierarchy, no boilerplate components—just the essentials.

## Step 3: Create Your "Hello World" Page

Now for the fun part—creating our page!

Navigate to the `src/pages/` folder. You'll notice it's currently empty. Create a new file called `index.astro` inside this folder.

Add the following code to `src/pages/index.astro`:
astroCopyDownload
```
---
// This is the "frontmatter" — a JavaScript/TypeScript section
// that runs at build time. We don't need anything here for now.
---

<html>
  <head>
    <title>Hello World</title>
  </head>
  <body>
    <h1>Hello World!</h1>
  </body>
</html>
```

That's it! This is a valid Astro page. The `---` section (frontmatter) is where you'd put JavaScript logic, but for our minimal example, it's empty.

**Note:** Astro supports standard HTML in `.astro` files. You can write plain HTML and it just works.1

## Step 4: Run the Development Server

Let's see your creation in action! From the root of your project, run:
bashCopyDownload
```
npm run dev
```

16

You should see output similar to:
textCopyDownload
```
  astro  vX.X.X ready in XXX ms

  ┃ Local    http://localhost:4321/
  ┃ Network  http://192.168.x.x:4321/
```

Open your browser and visit `http://localhost:4321/`. You should see a beautiful, minimal "Hello World!" displayed on the page.

## Step 5: (Optional) Customize the Port

If port `4321` is already in use, you can change it by modifying the `package.json` file:
jsonCopyDownload
```
{
  "scripts": {
    "dev": "astro dev --port 3000"
  }
}
```

Now `npm run dev` will start the server on `http://localhost:3000/`.

## Step 6: Build for Production

When you're ready to deploy your site, Astro makes it easy to generate static HTML:
bashCopyDownload
```
npm run build
```

16

This creates a `dist/` folder containing your fully built site—ready to be deployed to any static hosting service (Netlify, Vercel, Cloudflare Pages, or even a simple S3 bucket).

To preview the production build locally:
bashCopyDownload
```
npm run preview
```

16

## What Just Happened?

Let's recap the magic:

- **File-based routing**: By creating `src/pages/index.astro`, Astro automatically generated the homepage at `/`.
- **Zero client-side JavaScript**: The page is pure HTML—no JavaScript bundles, no hydration, just static content. This is Astro's superpower.
- **Build-time rendering**: The `index.astro` file is rendered at build time, producing a static HTML file.

## Going Further

This minimal setup is a great starting point. Here are a few things you might want to explore next:

- **Add components**: Create `.astro` files in `src/components/` and import them into your pages1.
- **Add a layout**: Create a `src/layouts/` folder and define a reusable layout component.
- **Add styles**: Use `<style>` tags inside your `.astro` components or import CSS files.
- **Add Markdown**: Create `.md` files in `src/pages/` for content-driven pages.

## The Full Picture

Here's what your final project should look like:
textCopyDownload
```
/
├── public/
├── src/
│   └── pages/
│       └── index.astro   # <-- Your Hello World page
├── package.json
├── package-lock.json     # (or yarn.lock)
└── astro.config.mjs      # (created by default)
```

And `index.astro` contains just the HTML you wrote.

## Troubleshooting

**"Command not found: npm"** — Make sure Node.js is installed. Run `node -v` to check.

**"Port 4321 already in use"** — Change the port as shown in Step 5, or stop the other process using that port.

**"Cannot find module"** — Run `npm install` to ensure all dependencies are installed.

## Conclusion

You've just built the most minimal "Hello World" project in Astro. With just a few commands and a single HTML file, you have a working static site. This is the beauty of Astro—it stays out of your way until you need its more powerful features.

The empty template we used is the perfect foundation for any project. It gives you complete control while providing the robust build system and development server that make Astro so enjoyable to work with.

Now go forth and build something amazing! 🚀

*Want to learn more? Check out the official Astro documentation or join the Astro Discord community.*1

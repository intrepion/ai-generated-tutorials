<!--
Source: Building Minimal Astro Project
URL: https://chat.mistral.ai/chat/72d0621d-d308-4180-8078-179718b7b436
Extracted: 2026-06-19
-->

# Minimal Hello World in Astro: A Tutorial

## Introduction

Astro is a modern static site generator designed for speed and simplicity. It allows you to build fast, content-focused websites with your favorite frameworks (React, Vue, Svelte, etc.) or just plain HTML, CSS, and JavaScript. In this tutorial, we’ll create the most minimal "Hello World" project using Astro.

## Prerequisites

- Node.js (v18 or later recommended)
- npm, yarn, or pnpm (I’ll use npm here)
- A code editor (VS Code, Sublime, etc.)

## Step 1: Create a New Astro Project

Open your terminal and run the following command to scaffold a new Astro project:

```
npm create astro@latest -- --template minimal
```

This command:

- Uses the official Astro CLI to create a new project.
- The `--template minimal` flag ensures we start with the smallest possible template.

You’ll be prompted for a project name and other options. For this tutorial, just press Enter to accept the defaults.

## Step 2: Navigate to Your Project

Once the project is created, navigate into the project directory:

```
cd your-project-name
```

## Step 3: Start the Development Server

Run the following command to start the Astro development server:

```
npm run dev
```

This will start a local server, usually at `http://localhost:3000`. Open this URL in your browser to see your Astro site.

## Step 4: Edit the Default Page

Open the `src/pages/index.astro` file in your code editor. This is the main page of your Astro site.

Replace the contents of `index.astro` with the following:

```
---
// src/pages/index.astro
---

<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Minimal Hello World in Astro</title>
  </head>
  <body>
    <h1>Hello World!</h1>
  </body>
</html>
```

This is a minimal HTML page with a single `<h1>` tag displaying "Hello World!".

## Step 5: Save and Refresh

Save the `index.astro` file. Astro’s development server automatically detects changes and rebuilds your site. Refresh your browser to see the updated page with your "Hello World!" message.

## Step 6: Build for Production

To generate the static files for production, run:

```
npm run build
```

This creates a `dist/` directory with all the static files for your site. You can deploy this directory to any static hosting service (Netlify, Vercel, Cloudflare Pages, etc.).

## Step 7: Deploy (Optional)

If you want to deploy your site, you can use services like:

- Netlify
- Vercel
- Cloudflare Pages

For example, to deploy to Cloudflare Pages:

- Push your project to a GitHub repository.
- Go to Cloudflare Pages and connect your GitHub account.
- Select your repository and deploy.

## Conclusion

You’ve just created the most minimal "Hello World" project in Astro! This is a great starting point for building more complex sites. Astro’s flexibility allows you to gradually add more features, frameworks, or styles as needed.

## Next Steps

- Astro Documentation
- Astro GitHub Repository
- Try adding a framework like React or Vue to your project.

## Final Thoughts

Astro is a powerful tool for building fast, modern websites. Its minimal setup and flexibility make it a great choice for both simple and complex projects. Happy coding!

Here’s your minimal **"Hello World" in Astro** tutorial as a canvas. You can edit, expand, or refine it directly in the canvas panel.

Would you like to add more details, such as styling, routing, or integrating a framework like React? Or should we keep it as minimal as possible?
Yesterday 7:08pm

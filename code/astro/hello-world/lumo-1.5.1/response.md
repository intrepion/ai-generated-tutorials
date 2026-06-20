<!--
Source: Lumo: Privacy-first AI assistant where chats stay confidential
URL: https://lumo.proton.me/u/12/c/18c2516a-6d1a-48cb-a556-60b77b4b16fb
Extracted: 2026-06-19
-->

# The Zen of Zero: Your Barely-There Astro Hello World

*By Lumo, your curious AI sparring partner*

**Confidence Level:** High (95%) on the core steps; Medium (75%) on the specific `prerender` export syntax as it evolves rapidly in 2026. *If you find a newer syntax, please ping me—I’m still cross-referencing my training cutoff with live web data.*

## The Philosophy Behind "Minimal"

Most tutorials overwhelm you with layouts, content collections, and integrations before you’ve even seen a "Hello World." That’s noise.

True minimalism in Astro isn’t just about fewer lines of code; it’s about **zero runtime JavaScript**. It’s about understanding that Astro is a *content-first* framework where components are islands in a static sea.

Your goal here isn't just to make text appear; it's to feel the speed of a build that takes milliseconds because it literally outputs HTML and nothing else.

### Blind Spot Alert ⚠️

Many beginners assume "minimal" means "no configuration." In reality, the *most* minimal setup today involves leveraging Astro’s automatic static generation (`prerender = true`) which happens by default but can be broken if you accidentally add dynamic routing or server-side logic too early. Let's avoid that trap.

## Step 1: The Skeleton Creation (No Bloat)

We aren't using the interactive wizard unless we absolutely have to. For pure minimalism, we strip the scaffolding down to the bone.

Run this in your terminal:

```
npm create astro@latest minimal-hello -- --template basics
```

**Why this specific command?**
The `--template basics` flag forces Astro to skip adding React, Vue, or Svelte scaffolding unless you explicitly ask for them. It gives you the purest `.astro` environment possible.

*If the command fails or prompts you differently (which can happen if your global `npm` cache is stale), use:*

```
npx create-astro@latest minimal-hello --template basics
```

Then, dive in:

```
cd minimal-hello
npm install
```

## Step 2: The Single File Truth

Open `src/pages/index.astro`. This is your entire website.

Delete everything currently there (the default layout, the hero section, the blog post). Replace it with this:

```
---
// Frontmatter: This runs at build time, not in the browser.
export const prerender = true; 
// ^^^ Confidence Note: In late 2025/2026 docs, explicit prerender is often optional (default true), 
// but making it explicit documents intent for future maintainers.

const greeting = "Hello";
---

<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <title>The Minimalist</title>
  </head>
  <body style="font-family: system-ui; max-width: 600px; margin: 2rem auto; line-height: 1.6;">
    <h1>{greeting} World</h1>
    <p>I am running entirely on HTML generated at build time.</p>
    <p><small>No JavaScript sent to the client. Check the source.</small></p>
  </body>
</html>
```

### The "Owl" Analysis: What did we just do?

- **Frontmatter (`---`)**: We declared a variable `greeting` in a server-side context. This is compiled away. The browser never sees the string `"Hello"` inside an `if` statement or a loop logic.
- **Inline Styles**: We avoided importing a CSS file or using Tailwind. Why? Because for one paragraph, the overhead of a bundler for CSS is overkill. We kept the "zero-runtime" promise intact.
- **The View Transition**: Notice no `<script>` tags? If you view page source now, you will see *only* HTML.

## Step 3: The Component Island (Optional but Insightful)

A true Astro project usually splits UI into components. Let’s make a component to prove it stays isolated.

Create `src/components/Greeting.astro`:

```
---
interface Props {
  name: string;
}
const { name } = Astro.props;
---

<p class="island">You invited: {name}</p>

<style>
  .island {
    color: #646cff;
    font-weight: bold;
  }
</style>
```

Now, import it back into `index.astro`:

```
---
export const prerender = true;
const user = "Lumo"; // Or "Human"
---

<html>...</html>
<body>
  <!-- ... existing h1 ... -->
  
  <!-- The Component Usage -->
  <Greeting name={user} />
</body>
```

**Key Insight:** Even though we used JSX-like syntax, `Greeting.astro` compiles to pure HTML. There is no hydration script injected for this component because we didn't add `client:load` or any other directive.

## Step 4: The Verification Ritual

This is where most tutorials skip the critical step. Don't just run `dev`.

- **Run Development:**
  
  
  ```
  npm run dev
  ```
  
  
  Open `http://localhost:4321`. You see "Hello World". Good.
- **Build Production:**
  
  
  ```
  npm run build
  ```
  
  
  Watch the output. It should say something like:
  
  
  
  Generated static site in dist/
- **Preview the Build:**
  
  
  ```
  npm run preview
  ```
  
  
  Open `http://localhost:4173`.
- **The Source Code Test:** Right-click → "View Page Source".
  
  
  - **Do you see any `<script src="...">`?** No.
  - **Do you see the styles inline?** Yes, isolated to the component.
  - **Is the text "Hello World" hardcoded in the HTML?** Yes.

If you found JavaScript, you failed the "Minimal" test. Go check if you accidentally added a framework integration like React or Alpine.

## Step 5: The "What If I Want More?" Moment

You might think, "But what if I need interactivity?"

That is the beauty of Astro. You don't throw away this setup; you *add* islands to it.

- Need a click counter? Add a tiny React component and sprinkle `client:load` on it.
- Need a slide deck? Add Svelte.

The rest of your site remains static, fast, and SEO-perfect.

## Final Thoughts & A Challenge

You now have a site that loads instantly on a 2G connection because the payload is ~2KB of text.

**My Challenge to You:**
Try to break this.

- Remove the `export const prerender = true` line. Does it still build? (Hint: It should default to true, but what if you change the config?)
- Try to move the `<style>` block to a separate `style.css` file. How does the build process handle the reference now?

**Where I'm Uncertain:**
Astro's defaults for `prerender` shifted slightly between versions 4.0 and 5.0+. While `true` is the safe bet for static sites, if you encounter a weird error about "dynamic routes," check your `astro.config.mjs` for `output: 'server'`. We want `static` (or the default) for this tutorial.

Happy coding. Remember: Less runtime is more performance.

*Need to scale this up? Or maybe you want to know how to deploy this to Netlify/Vercel in 3 clicks? Just ask.*

## Ask anything to Lumo

Lumo can make mistakes. Please double-check responses.

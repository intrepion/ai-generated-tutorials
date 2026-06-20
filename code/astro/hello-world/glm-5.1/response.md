<!--
Source: Z.ai - Advanced AI Chatbot & Agent powered by GLM-5.2
URL: https://chat.z.ai/c/731e288f-0bf1-4f14-8b4b-9855bd8a368a
Extracted: 2026-06-19
-->

# The Absolute Minimal "Hello World" in Astro

Astro is a fantastic modern web framework, famous for its "Islands Architecture" and shipping zero JavaScript by default. But with all its powerful features, sometimes you just want to strip away the magic and see how the engine works.

In this tutorial, we are going to build the most absolutely minimal "Hello World" project in Astro. No layouts, no components, no CSS frameworks—just pure, unadulterated Astro.

Let's get started!

## Prerequisites

You only need two things installed on your machine:

- **Node.js** (v18.14.1 or newer)
- A code editor (like VS Code)

## Step 1: Create the Project

Open up your terminal and run the following command to scaffold a new Astro project:
bash     npm create astro@latest  
Astro will ask you a few setup questions. To keep things as minimal as possible, answer them like this:

- **Where should we create your new project?** -> `hello-astro`
- **How would you like to start your new project?** -> Choose **Empty** (This is the secret to a minimal setup!)
- **Install dependencies?** -> **Yes**
- **Do you plan to write TypeScript?** -> **No** (To keep things simple)
- **Initialize a new git repository?** -> Up to you (Yes/No)

Once the installation finishes, navigate into your new project folder:
bash     cd hello-astro  
## Step 2: The Minimal Code

Because you chose the "Empty" template, Astro didn't generate a bunch of boilerplate files for you. If you open the project in your code editor, you'll notice the `src` folder is practically empty.

Here is where the magic happens. In Astro, file-based routing means whatever you put in the `src/pages/` directory becomes a webpage.

Create a file at `src/pages/index.astro` and add the following code:
astro     ---// This is the Astro Component Script.// It runs on the server when the page is requested.// For our minimal Hello World, we don't need any code here!---
<html>  <body>    <h1>Hello World!</h1>  </body></html>  
### What's happening here?

An `.astro` file is made up of two parts:

- **The Script (Frontmatter):** The code between the `---` dashes runs on the server. We left it empty because we don't need to fetch data or import anything.
- **The Template:** The HTML below the dashes is what gets sent to the browser. Astro understands standard HTML out of the box.

## Step 3: Run It

Go back to your terminal, ensure you are in the `hello-astro` directory, and start the development server:
bash     npm run dev  
You’ll see a message in your terminal saying the server is running on `http://localhost:4321/`. Click that link or type it into your browser.

You should see your "Hello World!" message displayed proudly on the screen.

## Step 4: Why is this special?

You might be thinking, *"This just looks like a standard HTML file. What's the big deal?"*

Right-click the page in your browser and select **"View Page Source"**. Take a close look at the code.

Notice anything missing? **There is no JavaScript.**

Unlike many modern frameworks (like React or Vue) that require a heavy JavaScript bundle to render a simple string of text to the screen, Astro compiled your `.astro` file into pure, static HTML on the server. Your browser didn't have to download, parse, or execute a single line of JavaScript just to show "Hello World".

This is Astro's superpower: **Zero JS by default.**

## Conclusion

And there you have it! The most minimal Astro project possible.

You've learned how to:

- Scaffold an empty Astro project.
- Create a page using file-based routing.
- Understand the basic structure of an `.astro` file.
- See Astro's zero-JavaScript-by-default behavior in action.

From here, the sky is the limit. You can start adding layouts to share HTML across multiple pages, importing UI components (Astro, React, Vue, or Svelte), or fetching data from APIs.

But at its core, Astro is just a really smart way to write HTML. Happy coding!

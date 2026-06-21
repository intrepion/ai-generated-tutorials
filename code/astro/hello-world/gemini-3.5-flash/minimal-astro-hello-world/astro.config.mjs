import { defineConfig } from 'astro/config';

// https://astro.build/config
export default defineConfig({
  // Astro v6 expects explicit output targets if shifting to SSR, 
  // but defaults safely to 'static' for our Hello World.
  output: 'static'
});


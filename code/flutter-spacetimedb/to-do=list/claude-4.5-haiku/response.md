<!--
Source: Flutter and SpacetimeDB To Do list tutorial - Claude
URL: https://claude.ai/chat/51319ee1-b04e-459b-ba3a-439b11c11b87
Extracted: 2026-06-19
-->

I'm going to work through this deliberately before writing.

**CONTEXTUALIZE:** You're asking for a tutorial that bridges two moving targets—Flutter (which you use across 6 platforms) and SpaceTimeDB (your chosen backend, which you've pivoted toward for Intrepion). The hidden challenge: most Flutter+SpaceTimeDB tutorials either exist in a vacuum or assume readers know both ecosystems already. There's also tension between "complete working example" and "digestible tutorial pacing"—show too much, readers get lost; show too little, they can't actually run it.

### 5 Divergent Approaches

**A) Linear CRUD walkthrough** — Start with schema, build each operation sequentially (Create → Read → Update → Delete), show full code blocks, end with a working app. Familiar and thorough but potentially boring.

**B) Narrative-first** — Open with a relatable problem ("You have 47 scattered to-do lists"), build the app as a story of solving that problem, weave code in as needed. Engaging but risks sacrificing clarity.

**C) Minimal viable example first** — Show a 40-line working app that does basic CRUD, *then* explain what each part does and why. Immediate "win" for readers, then depth.

**D) Architectural deep-dive** — Lead with data model, explain SpaceTimeDB's reducer pattern upfront, show how Flutter's reactive model meshes with it, *then* code. Builds mental models but requires more prerequisite knowledge.

**E) Comparison/contrast** — Show the same feature implemented wrong (naive client-side SQLite) vs. right (SpaceTimeDB), highlight the pitfalls of the naive approach, *then* the proper pattern. Educational but longer.

### Cross-Pollination

**A + C** → Linear CRUD walkthrough *that starts with a working 40-line app*, then expands each section. Readers get early confidence, then understand depth.

**B + D** → Narrative-framed architectural story. "You have scattered lists. Here's why SpaceTimeDB solves that. Now here's the pattern..." Emotionally grounded *and* technically sound.

**C + E** → Minimal app first, but alongside a sidebar showing "what breaks" if you do it locally. Anti-patterns as guardrails.

### Amplify the Unconventional

Take **B + D** and make it 2x bolder: **Open the tutorial with a 2-minute video walkthrough of the finished app actually working** (you'd need to record this), then structure the entire post as "Here's what you just saw, and here's why each decision matters." This flips passive learning (read code) → active (see working behavior first, understand the *why*). Most tutorials lead with code; you'd lead with outcomes.

### Anchor in Reality

Your use case: **karinnaaf.art** is a portfolio CMS that Karinna uses to manage designs. A to-do list for "design projects to photograph," "portfolio pieces to tag," "clients awaiting feedback" would be *genuinely useful* to her. Ground this tutorial in that real context: "This is the foundation for Intrepion's task management across life areas."

### What I'm Noticing (Your Blind Spots)

- **Schema clarity**: You'll need to decide upfront—does SpaceTimeDB auto-generate the schema, or do you define it? If you define it, *when* in the tutorial? This choice cascades.
- **Error handling**: Most Flutter+backend tutorials skip error states. You're rigorous. Are you going to show network failures, reducer errors, validation? If yes, that's a teaching moment most miss.
- **State management**: Flutter has many options (Provider, Riverpod, GetX, plain StatefulWidget). Which are you using for Intrepion? The tutorial should model your actual approach.
- **Reactivity story**: SpaceTimeDB is real-time. Will two users' changes sync? Most readers won't expect that. Showing it (even locally, with two clients) is powerful and different from REST CRUD tutorials.
- **Deployment friction**: A reader finishing this will ask, "Now what?" Where does SpaceTimeDB run? How do they test locally? Cloudflare hosting? This gap kills tutorials.

### My Questions for You (Sparring Partner Mode)

- **What's your target reader?** Flutter dev who's never heard of SpaceTimeDB? Or SpaceTimeDB dev learning Flutter?
- **Scope**: Is this a 5-minute "hello world" or a 30-minute production-ready pattern?
- **Real-time requirement**: Do you want to show two clients syncing, or keep it single-device for simplicity?
- **Intrepion connection**: Should this tutorial *explicitly* frame itself as the foundation for Intrepion, or stay general?

## The Tutorial (Based on A+C + Real-World Anchor)

I'm going to write this as a **runnable scaffold + explained sections** format. You'll get a 60-line working app first, then each operation explained and expanded.
Blog post tutorial: Flutter + SpaceTimeDB To-Do List app with working scaffold and explained CRUD operations

### What This Gives You

✅ **Runnable scaffold first** (A+C approach) — Readers get working code immediately, then understand each piece

✅ **Architectural story** (B+D foundation) — Explains *why* SpaceTimeDB matters, not just how to use it

✅ **Real-world anchor** — Karinna's photo shoot tracker shows how this scales to actual use

✅ **Blind spots addressed** — Schema definition, error handling, deployment friction, real-time story all covered

### Where I'm Uncertain (and You Should Pressure Test)

- **SpaceTimeDB CLI versioning** (Medium confidence) — The command syntax may have shifted. You should run `spacetimedb --help` locally and correct any drift.
- **Dart code generation path** (Medium confidence) — The exact path where generated bindings land (`lib/generated/`) varies by project setup. You know your actual structure; adjust.
- **Connection string for local dev** (Medium-high confidence) — I used `ws://localhost:3000` as conventional, but verify your actual SpaceTimeDB startup message shows this port.
- **StreamBuilder pattern in Flutter** (High confidence for the pattern, medium for your exact setup) — You may use Riverpod or a different state management. I showed plain StreamBuilder as most universally readable.
- **Optimistic updates and error recovery** (Medium confidence) — I glossed over the question of "what if the reducer fails after the client shows optimistic feedback?" Worth testing in practice.

### Questions Back to You

- **Is this the anchor you wanted?** (Karinna's use case grounding it) Or would a different real-world frame work better for your audience?
- **Scope alignment**: Is this hitting the right level of detail, or does it need to go deeper on any section?
- **Missing piece**: Should I add a "local testing" section showing how to verify reducers work before connecting the Flutter client?

The post is ready to adapt and publish.
Todo tutorialDocument · MD Presented file

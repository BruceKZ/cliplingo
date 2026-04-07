<template>
  <section class="space-y-6">
    <div class="space-y-2">
      <p class="text-sm font-medium uppercase tracking-[0.24em] text-app-muted">
        ClipLingo
      </p>
      <h1 class="text-3xl font-semibold tracking-tight text-app-text">
        Lightweight desktop translation, built around copy-first flow.
      </h1>
      <p class="max-w-3xl text-sm leading-7 text-app-muted">
        Phase 1 focuses on macOS and Windows, double-copy triggering,
        OpenAI-compatible providers, a lightweight translation popup, secure API
        key handling, and local history.
      </p>
    </div>

    <div class="grid gap-4 md:grid-cols-3">
      <article
        v-for="item in highlights"
        :key="item.title"
        class="rounded-2xl border border-app-line bg-app-panel p-5"
      >
        <p
          class="text-xs font-semibold uppercase tracking-[0.22em] text-app-accent"
        >
          {{ item.kicker }}
        </p>
        <h2 class="mt-3 text-lg font-semibold text-app-text">
          {{ item.title }}
        </h2>
        <p class="mt-2 text-sm leading-6 text-app-muted">{{ item.body }}</p>
      </article>
    </div>

    <div class="rounded-2xl border border-app-line bg-app-panel p-6">
      <div
        class="flex flex-col gap-4 md:flex-row md:items-end md:justify-between"
      >
        <div class="space-y-2">
          <h2 class="text-lg font-semibold text-app-text">Execution order</h2>
          <p class="text-sm text-app-muted">
            Work proceeds issue by issue through `develop`, with one scoped PR
            per task and mandatory validation before merge.
          </p>
        </div>
        <div
          class="rounded-xl bg-app-accent-soft px-4 py-3 text-sm text-app-text"
        >
          Current focus: `Task 01`
        </div>
      </div>

      <ol class="mt-5 grid gap-3 text-sm text-app-muted md:grid-cols-2">
        <li
          v-for="step in executionSteps"
          :key="step"
          class="rounded-xl border border-app-line px-4 py-3"
        >
          {{ step }}
        </li>
      </ol>
    </div>
  </section>
</template>

<script setup lang="ts">
const highlights = [
  {
    kicker: "Trigger",
    title: "Double copy with fallback hotkey",
    body: "Detect Cmd/Ctrl+C+C in a bounded window and provide a manual global shortcut when target apps behave inconsistently.",
  },
  {
    kicker: "Provider",
    title: "OpenAI-compatible by design",
    body: "Allow custom base URL, model, auth headers, timeout, and generation parameters without hardcoding one vendor endpoint.",
  },
  {
    kicker: "Security",
    title: "Untrusted input, constrained output",
    body: "Clipboard text, prompts, headers, and URLs must be validated and never rendered as HTML or logged as plaintext secrets.",
  },
];

const executionSteps = [
  "Task 01: project skeleton",
  "Task 02: windows and tray foundation",
  "Task 03: shared types and config models",
  "Task 04: config and secure storage",
  "Task 05-09: clipboard, trigger, provider, router, orchestrator",
  "Task 10-14: popup UI, settings, history, security, packaging",
];
</script>

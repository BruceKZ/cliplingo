<template>
  <section class="space-y-6">
    <header class="space-y-2">
      <p class="text-sm font-medium uppercase tracking-[0.24em] text-app-muted">
        History Shell
      </p>
      <h1 class="text-3xl font-semibold tracking-tight text-app-text">
        Local history will stay minimal and privacy-aware.
      </h1>
      <p class="max-w-3xl text-sm leading-7 text-app-muted">
        The view is scaffolded now so later issues can focus on repository
        design, retention limits, and redaction rules without reworking the app
        shell.
      </p>
    </header>

    <div
      class="rounded-2xl border border-dashed border-app-line bg-app-panel p-6"
    >
      <p class="text-sm leading-7 text-app-muted">
        No history implementation yet. Phase 1 will cap local entries, allow
        disabling retention, and keep sensitive full text out of logs by
        default.
      </p>
    </div>
  </section>
</template>

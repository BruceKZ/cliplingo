<template>
  <main class="min-h-screen bg-app-bg px-4 py-4 text-app-text">
    <section
      class="mx-auto flex min-h-[18rem] max-w-3xl flex-col rounded-3xl border border-app-line bg-app-panel p-6 shadow-sm"
    >
      <div class="space-y-2">
        <p
          class="text-xs font-semibold uppercase tracking-[0.24em] text-app-muted"
        >
          Translation Popup
        </p>
        <h1 class="text-2xl font-semibold tracking-tight">
          Ready for translated output
        </h1>
        <p class="text-sm leading-7 text-app-muted">
          This window is created by the Rust tray/window foundation and will
          later host clipboard-driven translation results.
        </p>
      </div>

      <div
        class="mt-6 rounded-2xl border border-dashed border-app-line bg-app-bg p-5"
      >
        <p class="text-sm text-app-muted">
          Task 02 keeps this shell intentionally minimal. The next issues will
          fill in the result cards, copy actions, and retry controls.
        </p>
      </div>
    </section>
  </main>
</template>

<template>
  <div ref="root" class="json-code-editor" />
</template>

<script setup lang="ts">
import { Compartment, EditorState } from "@codemirror/state";
import { EditorView } from "@codemirror/view";
import { json } from "@codemirror/lang-json";
import { basicSetup } from "codemirror";
import { onBeforeUnmount, onMounted, ref, watch } from "vue";

const props = defineProps<{
  modelValue: string;
}>();

const emit = defineEmits<{
  "update:modelValue": [value: string];
}>();

const root = ref<HTMLDivElement | null>(null);
let view: EditorView | null = null;
const themeConfig = new Compartment();

function createTheme() {
  return EditorView.theme(
    {
      "&": {
        fontFamily: '"JetBrains Mono Variable", "JetBrains Mono", ui-monospace, SFMono-Regular, Menlo, Consolas, monospace',
        fontSize: "14px",
        backgroundColor: "var(--color-panel)",
        color: "var(--color-text)",
      },
      ".cm-editor": {
        border: "1px solid var(--color-line)",
        borderRadius: "12px",
        overflow: "hidden",
      },
      ".cm-scroller": {
        fontFamily: "inherit",
        lineHeight: "1.6",
        height: "calc(20 * 1.6em + 28px)",
        overflow: "auto",
      },
      ".cm-gutters": {
        backgroundColor: "color-mix(in srgb, var(--color-panel) 94%, var(--color-bg) 6%)",
        color: "var(--color-muted)",
        borderRight: "1px solid var(--color-line)",
      },
      ".cm-content": {
        padding: "14px 0",
      },
      ".cm-line": {
        padding: "0 16px",
      },
      ".cm-activeLine": {
        backgroundColor: "color-mix(in srgb, var(--color-accent-soft) 26%, transparent 74%)",
      },
      ".cm-activeLineGutter": {
        backgroundColor: "color-mix(in srgb, var(--color-accent-soft) 26%, transparent 74%)",
      },
      ".cm-focused": {
        outline: "none",
      },
      ".cm-cursor": {
        borderLeftColor: "var(--color-accent)",
      },
      ".cm-selectionBackground, ::selection": {
        backgroundColor: "color-mix(in srgb, var(--color-accent-soft) 48%, transparent 52%)",
      },
      ".cm-matchingBracket": {
        backgroundColor: "color-mix(in srgb, var(--color-accent-soft) 36%, transparent 64%)",
      },
    },
    { dark: false },
  );
}

onMounted(() => {
  if (!root.value) {
    return;
  }

  view = new EditorView({
    parent: root.value,
    state: EditorState.create({
      doc: props.modelValue,
      extensions: [
        basicSetup,
        json(),
        themeConfig.of(createTheme()),
        EditorView.updateListener.of((update) => {
          if (update.docChanged) {
            emit("update:modelValue", update.state.doc.toString());
          }
        }),
      ],
    }),
  });
});

watch(
  () => props.modelValue,
  (nextValue) => {
    if (!view) {
      return;
    }

    const currentValue = view.state.doc.toString();
    if (currentValue === nextValue) {
      return;
    }

    view.dispatch({
      changes: {
        from: 0,
        to: currentValue.length,
        insert: nextValue,
      },
    });
  },
);

onBeforeUnmount(() => {
  view?.destroy();
  view = null;
});
</script>

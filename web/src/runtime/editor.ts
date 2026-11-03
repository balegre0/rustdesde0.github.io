// Editor CodeMirror 6 (ligero y usable en móvil) para retos y «Editar» en los ejemplos.
import { EditorView, basicSetup } from 'codemirror';
import { keymap } from '@codemirror/view';
import { indentWithTab } from '@codemirror/commands';
import { indentUnit } from '@codemirror/language';
import { rust } from '@codemirror/lang-rust';
import { HighlightStyle, syntaxHighlighting } from '@codemirror/language';
import { tags as t } from '@lezer/highlight';

const tema = EditorView.theme({
  '&': {
    color: 'var(--sl-color-white)',
    backgroundColor: 'var(--rdc-consola-bg)',
    border: '1px solid var(--rdc-borde)',
    borderRadius: '0.375rem',
    fontSize: '0.9375rem',
  },
  '.cm-content': { fontFamily: 'var(--__sl-font-mono)', fontVariantLigatures: 'none', caretColor: 'var(--sl-color-accent-high)', padding: '0.6rem 0' },
  '.cm-gutters': { backgroundColor: 'transparent', color: 'var(--sl-color-gray-3)', border: 'none', fontFamily: 'var(--__sl-font-mono)' },
  '.cm-activeLine': { backgroundColor: 'color-mix(in srgb, var(--sl-color-accent-low) 45%, transparent)' },
  '.cm-activeLineGutter': { backgroundColor: 'transparent', color: 'var(--sl-color-white)' },
  '&.cm-focused': { outline: '2px solid var(--rdc-foco)', outlineOffset: '2px' },
  '.cm-selectionBackground, &.cm-focused .cm-selectionBackground, ::selection': { backgroundColor: 'color-mix(in srgb, var(--sl-color-accent) 30%, transparent) !important' },
  '.cm-cursor': { borderLeftColor: 'var(--sl-color-accent-high)', borderLeftWidth: '2px' },
  '.cm-scroller': { lineHeight: '1.6', minHeight: '8rem' },
});

// Resaltado sobrio (pocos colores, todos ≥ 4,5:1 en ambos temas vía variables)
const resaltado = HighlightStyle.define([
  { tag: [t.keyword, t.controlKeyword, t.modifier, t.operatorKeyword], color: 'var(--rdc-sx-kw)', fontWeight: '700' },
  { tag: [t.typeName, t.standard(t.typeName)], color: 'var(--rdc-sx-tipo)' },
  { tag: [t.string, t.character], color: 'var(--rdc-sx-cadena)' },
  { tag: [t.number, t.bool, t.null], color: 'var(--rdc-sx-num)' },
  { tag: [t.comment, t.lineComment, t.blockComment], color: 'var(--rdc-sx-com)', fontStyle: 'italic' },
  { tag: [t.processingInstruction, t.meta, t.macroName, t.labelName], color: 'var(--rdc-sx-pre)' },
  { tag: [t.function(t.variableName)], color: 'var(--rdc-sx-fn)' },
]);

export function crearEditor(padre: HTMLElement, codigo: string, alCambiar?: (s: string) => void, etiqueta = 'Editor de código Rust') {
  const view = new EditorView({
    doc: codigo,
    parent: padre,
    extensions: [
      basicSetup,
      keymap.of([indentWithTab]),
      indentUnit.of('    '),
      rust(),
      tema,
      syntaxHighlighting(resaltado),
      EditorView.contentAttributes.of({ 'aria-label': etiqueta, spellcheck: 'false', autocapitalize: 'off', autocorrect: 'off' }),
      EditorView.updateListener.of((u) => { if (u.docChanged) alCambiar?.(u.state.doc.toString()); }),
    ],
  });
  return {
    view,
    obtener: () => view.state.doc.toString(),
    poner: (s: string) => view.dispatch({ changes: { from: 0, to: view.state.doc.length, insert: s } }),
    enfocar: () => view.focus(),
  };
}

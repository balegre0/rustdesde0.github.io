// Mini-Markdown en línea para enunciados y opciones: `código`, **negrita**, *cursiva*. Escapa el HTML.
const esc = (s: string) => s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');

export function mdEnLinea(s: string | undefined | null): string {
  if (!s) return '';
  const partes = String(s).split(/(`+)([\s\S]*?)\1/);
  let out = '';
  // split con grupos: [texto, tics, codigo, texto, tics, codigo, ...]
  for (let i = 0; i < partes.length; i += 3) {
    out += esc(partes[i])
      .replace(/\*\*([^*]+?)\*\*/g, '<strong>$1</strong>')
      .replace(/(^|[^*\w])\*([^*\s][^*]*?)\*(?!\*)/g, '$1<em>$2</em>');
    if (i + 2 < partes.length) out += `<code>${esc(partes[i + 2])}</code>`;
  }
  return out;
}

/** Hash corto y estable (FNV-1a) para ids deterministas entre servidor y cliente. */
export function hashTexto(s: string): string {
  let h = 2166136261;
  for (let i = 0; i < s.length; i++) { h ^= s.charCodeAt(i); h = Math.imul(h, 16777619); }
  return (h >>> 0).toString(36);
}

/** Copia barajada (Fisher-Yates). */
export const barajar = <T,>(a: T[]): T[] => {
  const b = [...a];
  for (let i = b.length - 1; i > 0; i--) { const j = Math.floor(Math.random() * (i + 1)); [b[i], b[j]] = [b[j], b[i]]; }
  return b;
};

/** Orden aleatorio de n opciones: orden[posición en pantalla] = índice original. Cambia en cada carga. */
export const ordenAleatorio = (n: number) => barajar(Array.from({ length: n }, (_, k) => k));

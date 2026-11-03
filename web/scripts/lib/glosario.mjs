// Glosario del curso (contenido/glosario.yaml): carga los términos y marca, en el HTML de una lección,
// la PRIMERA aparición de cada uno para que muestre su definición (src/runtime/glosario.ts).
// Solo toca texto corriente del contenido: nunca código, enlaces, títulos ni componentes de la plataforma.
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { parse } from 'yaml';

// Raíz del proyecto web/ (los scripts y la build se ejecutan desde ahí; import.meta.dirname no sirve una vez empaquetado).
const RAIZ = process.cwd();
const esc = (s) => s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');
/** `código` → <code>código</code>; el resto, escapado. */
const enLinea = (s) => s.split(/(`[^`]+`)/).map((p) => (p.startsWith('`') && p.length > 1 ? `<code>${esc(p.slice(1, -1))}</code>` : esc(p))).join('');
const escRe = (s) => s.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');

export function cargarGlosario() {
  return parse(readFileSync(join(RAIZ, 'contenido', 'glosario.yaml'), 'utf8')).map((t, i) => ({
    ...t,
    i,
    html: enLinea(t.def),
    href: `/curso/${t.leccion}/`,
  }));
}

const SALTAR = new Set(['code', 'pre', 'a', 'h1', 'h2', 'h3', 'h4', 'h5', 'h6', 'button', 'script', 'style', 'kbd', 'summary', 'svg', 'figure', 'textarea', 'select', 'template', 'astro-island']);
const VACIOS = new Set(['area', 'base', 'br', 'col', 'embed', 'hr', 'img', 'input', 'link', 'meta', 'source', 'track', 'wbr']);
const CLASE_SALTAR = /\b(rdc-[\w-]+|expressive-code|not-content|sr-only)\b/;

export function crearAnotador() {
  const terminos = cargarGlosario();
  // Formas de más larga a más corta («préstamo exclusivo» antes que «préstamo»).
  const formas = terminos.flatMap((t) => t.formas.map((f) => ({ f, t }))).sort((a, b) => b.f.length - a.f.length);
  // Las formas en minúscula encajan sin distinguir mayúsculas; las que empiezan por mayúscula (CPU, Cargo), exactas.
  const patron = (lista) => lista.map(({ f }) => escRe(f)).join('|');
  const borde = (p) => `(?<![\\p{L}\\p{N}_])(?:${p})(?![\\p{L}\\p{N}_])`;
  const reMin = new RegExp(borde(patron(formas.filter(({ f }) => f[0] === f[0].toLowerCase()))), 'giu');
  const reMay = new RegExp(borde(patron(formas.filter(({ f }) => f[0] !== f[0].toLowerCase()))), 'gu');
  const porForma = new Map(formas.map(({ f, t }) => [f.toLowerCase(), t]));

  /** Marca los términos en el contenido (.sl-markdown-content) de la lección `leccion` (p. ej. «06-ownership/04-mover»). */
  return function anotar(html, leccion) {
    // No se marcan: los términos que explica esta misma lección, ni los básicos pasado su mundo «hasta».
    const mundo = parseInt(leccion, 10);
    const usados = new Set(terminos.filter((t) => t.leccion === leccion || (t.hasta != null && mundo > t.hasta)).map((t) => t.i));
    const buscar = (texto) => {
      let mejor = null;
      for (const re of [reMin, reMay]) {
        re.lastIndex = 0;
        for (let r; (r = re.exec(texto)); ) {
          const t = porForma.get(r[0].toLowerCase());
          if (t && !usados.has(t.i)) { if (!mejor || r.index < mejor.index) mejor = { index: r.index, texto: r[0], t }; break; }
        }
      }
      return mejor;
    };
    const anotarTexto = (texto) => {
      let out = '';
      for (let enc; (enc = buscar(texto)); ) {
        usados.add(enc.t.i);
        out += texto.slice(0, enc.index)
          + `<button type="button" class="rdc-term" aria-expanded="false" data-term="${esc(enc.t.termino)}" data-def="${esc(enc.t.html)}" data-href="${enc.t.href}">${enc.texto}</button>`;
        texto = texto.slice(enc.index + enc.texto.length);
      }
      return out + texto;
    };

    // Recorrido por etiquetas con una pila: se anota el texto que está dentro del contenido y fuera de zonas prohibidas.
    const pila = []; // { nombre, contenido, saltar }
    let dentro = 0, saltar = 0, n = 0;
    const partes = html.split(/(<!--[\s\S]*?-->|<[^>]+>)/);
    for (let k = 0; k < partes.length; k++) {
      const p = partes[k];
      if (!p) continue;
      if (p[0] !== '<' || p.startsWith('<!--')) {
        if (dentro > 0 && saltar === 0 && p[0] !== '<' && p.trim()) { const a = anotarTexto(p); if (a !== p) { partes[k] = a; n++; } }
        continue;
      }
      const cierre = /^<\/([\w-]+)/.exec(p);
      if (cierre) {
        const nombre = cierre[1].toLowerCase();
        while (pila.length) {
          const e = pila.pop();
          if (e.contenido) dentro--;
          if (e.saltar) saltar--;
          if (e.nombre === nombre) break;
        }
        continue;
      }
      const apertura = /^<([\w-]+)/.exec(p);
      if (!apertura) continue;
      const nombre = apertura[1].toLowerCase();
      if (VACIOS.has(nombre) || p.endsWith('/>')) continue;
      const clase = /\sclass="([^"]*)"/.exec(p)?.[1] ?? '';
      const e = { nombre, contenido: /\bsl-markdown-content\b/.test(clase), saltar: SALTAR.has(nombre) || CLASE_SALTAR.test(clase) };
      if (e.contenido) dentro++;
      if (e.saltar) saltar++;
      pila.push(e);
      // Lo de dentro de <script>/<style> no es HTML: saltar hasta su cierre sin interpretarlo.
      if (nombre === 'script' || nombre === 'style') {
        while (k + 1 < partes.length && !new RegExp(`^</${nombre}`, 'i').test(partes[k + 1] ?? '')) k++;
      }
    }
    return { html: partes.join(''), marcados: usados.size };
  };
}

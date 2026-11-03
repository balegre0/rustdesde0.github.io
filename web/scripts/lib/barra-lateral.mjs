// Construye la configuración de starlight-sidebar-topics leyendo src/content/docs/ al arrancar Astro.
// (Las carpetas nuevas de mundos o módulos de la referencia aparecen al reiniciar `bun run dev`.)
import { existsSync, readdirSync, readFileSync, statSync } from 'node:fs';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';

const DOCS = fileURLToPath(new URL('../../src/content/docs/', import.meta.url));
const docs = (...p) => join(DOCS, ...p);

const esDir = (p) => existsSync(p) && statSync(p).isDirectory();
const subcarpetas = (rel) =>
  esDir(docs(rel)) ? readdirSync(docs(rel)).filter((n) => esDir(docs(rel, n))).sort() : [];
const tienePaginas = (rel) => {
  if (!esDir(docs(rel))) return false;
  const pila = [docs(rel)];
  while (pila.length) {
    const d = pila.pop();
    for (const n of readdirSync(d)) {
      const p = join(d, n);
      if (esDir(p)) pila.push(p);
      else if (/\.(md|mdx)$/.test(n)) return true;
    }
  }
  return false;
};

/** Lee el `title:` del frontmatter de index.mdx/md de una carpeta (sin parsear YAML entero). */
function tituloDe(rel) {
  for (const f of ['index.mdx', 'index.md']) {
    const p = docs(rel, f);
    if (!existsSync(p)) continue;
    const m = readFileSync(p, 'utf8').match(/^---[\s\S]*?^title:\s*(.+)$/m);
    if (m) return m[1].trim().replace(/^["']|["']$/g, '').replace(/^Mundo\s+\d+\s*[\u00B7:.\-]\s*/i, '');
  }
  return null;
}

const bonito = (slug) => {
  const m = slug.match(/^(\d+)-(.*)$/);
  const texto = (m ? m[2] : slug).replace(/-/g, ' ');
  const t = texto.charAt(0).toUpperCase() + texto.slice(1);
  return m ? `${Number(m[1])} · ${t}` : t;
};

export function mundos() {
  return subcarpetas('curso').map((slug) => {
    const titulo = tituloDe(`curso/${slug}`);
    const n = Number(slug.slice(0, 2));
    return { slug, label: titulo ? (Number.isFinite(n) ? `${n} · ${titulo}` : titulo) : bonito(slug) };
  });
}

function grupoAuto(label, dir, collapsed = true) {
  return { label, collapsed, items: [{ autogenerate: { directory: dir, collapsed: true } }] };
}

export function topics() {
  const curso = [
    { label: 'Todos los módulos', link: '/curso/' },
    { label: 'Árbol de progreso', link: '/arbol/' },
    { label: 'Repaso de hoy', link: '/repaso/' },
    { label: 'Tu progreso', link: '/progreso/' },
    ...mundos().map((m) => grupoAuto(m.label, `curso/${m.slug}`)),
  ];

  const ref = [{ label: 'Índice', link: '/ref/' }];
  if (tienePaginas('ref/lenguaje')) ref.push(grupoAuto('Lenguaje', 'ref/lenguaje'));
  const modulos = subcarpetas('ref/std').filter((c) => tienePaginas(`ref/std/${c}`));
  if (modulos.length)
    ref.push({ label: 'Biblioteca estándar', collapsed: true, items: modulos.map((c) => grupoAuto(`std::${c}`, `ref/std/${c}`)) });
  for (const [f, label] of [['ub', 'Comportamiento indefinido'], ['ediciones', 'Ediciones'], ['versiones', 'Versiones de Rust']]) {
    if (existsSync(docs('ref', f + '.mdx')) || existsSync(docs('ref', f + '.md'))) ref.push({ label, link: `/ref/${f}/` });
  }

  const seccion = (dir) => (tienePaginas(dir) ? [{ autogenerate: { directory: dir } }] : []);

  return [
    { id: 'curso', label: 'Módulos guiados', link: '/curso/', icon: 'open-book', items: curso },
    { id: 'ref', label: 'Referencia', link: '/ref/', icon: 'information', items: ref },
    { id: 'recetas', label: 'Recetas', link: '/recetas/', icon: 'list-format', items: [{ label: 'Todas las recetas', link: '/recetas/' }, ...seccion('recetas')] },
    { id: 'errores', label: 'Errores del compilador', link: '/errores/', icon: 'warning', items: [{ label: 'Todos los errores', link: '/errores/' }, ...seccion('errores')] },
    { id: 'explicaciones', label: 'Explicaciones', link: '/explicaciones/', icon: 'puzzle', items: [{ label: 'Todas las explicaciones', link: '/explicaciones/' }, ...seccion('explicaciones')] },
  ];
}

// Datos de la práctica (preguntas, problemas, listas, quizzes) en build + Markdown con código resaltado.
import practica from '../generated/practica.json';
import problemasJson from '../generated/problemas.json';
import { Marked } from 'marked';
import { createHighlighter, type Highlighter } from 'shiki';
import type { Pregunta } from '../../scripts/generar-practica';
import type { Problema } from '../../scripts/compilar-ejemplos';

export type { Pregunta, Problema };
export type Lista = { id: string; titulo: string; descripcion: string; preguntas: string[]; problemas: string[] };
export type Quiz = { id: string; titulo: string; descripcion: string; tiempo: number; preguntas: string[] };

export const preguntas = (practica as any).preguntas as Pregunta[];
export const listas = (practica as any).listas as Lista[];
export const quizzes = (practica as any).quizzes as Quiz[];
export const problemas = Object.values(problemasJson as unknown as Record<string, Problema>).sort(
  (a, b) => (a.mundo ?? 'zz').localeCompare(b.mundo ?? 'zz') || a.orden - b.orden || ordenDif(a.dificultad) - ordenDif(b.dificultad) || a.titulo.localeCompare(b.titulo, 'es'),
);
export const preguntaPorId = new Map(preguntas.map((p) => [p.id, p]));
export const problemaPorSlug = new Map(problemas.map((p) => [p.slug, p]));

export function ordenDif(d: string) { return d === 'facil' ? 0 : d === 'media' ? 1 : 2; }
export const DIFICULTAD: Record<string, string> = { facil: 'Fácil', media: 'Media', dificil: 'Difícil' };
export const TIPO: Record<string, string> = { opcion: 'Opción múltiple', respuesta: 'Respuesta escrita', predice: 'Predice la salida', 'no-compila': '¿Por qué no compila?' };

/** «07-prestamos» → «7 · Préstamos» (título del mundo si existe en el curso). */
export const MUNDOS = [
  '00-el-taller', '01-la-maquina', '02-tu-primer-programa', '03-valores-y-tipos', '04-expresiones-y-control', '05-funciones-y-la-pila',
  '06-ownership', '07-prestamos', '08-lifetimes', '09-structs-enums-y-patrones', '10-errores', '11-colecciones-y-texto',
  '12-modulos-crates-y-cargo', '13-traits-y-genericos', '14-closures-e-iteradores', '15-punteros-inteligentes', '16-pruebas-y-herramientas',
  '17-concurrencia', '18-async', '19-macros', '20-unsafe-y-ub', '21-el-compilador-por-dentro', '22-sistemas-y-ffi', '23-embebido-y-tu-so', '24-teoria-y-diseno',
];
export const TITULOS_MUNDO: Record<string, string> = {
  '00-el-taller': 'El taller', '01-la-maquina': 'La máquina', '02-tu-primer-programa': 'Tu primer programa', '03-valores-y-tipos': 'Valores y tipos',
  '04-expresiones-y-control': 'Expresiones y control', '05-funciones-y-la-pila': 'Funciones y la pila', '06-ownership': 'Ownership',
  '07-prestamos': 'Préstamos', '08-lifetimes': 'Lifetimes', '09-structs-enums-y-patrones': 'Structs, enums y patrones', '10-errores': 'Errores',
  '11-colecciones-y-texto': 'Colecciones y texto', '12-modulos-crates-y-cargo': 'Módulos, crates y Cargo', '13-traits-y-genericos': 'Traits y genéricos',
  '14-closures-e-iteradores': 'Closures e iteradores', '15-punteros-inteligentes': 'Punteros inteligentes', '16-pruebas-y-herramientas': 'Pruebas y herramientas',
  '17-concurrencia': 'Concurrencia', '18-async': 'Async', '19-macros': 'Macros', '20-unsafe-y-ub': 'Unsafe y UB', '21-el-compilador-por-dentro': 'El compilador por dentro',
  '22-sistemas-y-ffi': 'Sistemas y FFI', '23-embebido-y-tu-so': 'Embebido y tu SO', '24-teoria-y-diseno': 'Teoría y diseño',
};
export function nombreMundo(slug: string | null, titulos?: Map<string, string>) {
  if (!slug) return '—';
  const m = slug.match(/^(\d+)-(.*)$/);
  if (!m) return slug;
  const t = titulos?.get(slug) ?? TITULOS_MUNDO[slug] ?? m[2].replace(/-/g, ' ').replace(/^./, (c) => c.toUpperCase());
  return `${Number(m[1])} · ${t}`;
}
export const numMundo = (slug: string | null) => (slug && /^\d+/.test(slug) ? Number(slug.slice(0, 2)) : 99);
export const TEMAS_NOMBRE: Record<string, string> = {
  ownership: 'Ownership', prestamos: 'Préstamos', lifetimes: 'Lifetimes', traits: 'Traits', genericos: 'Genéricos', tipos: 'Tipos',
  iteradores: 'Iteradores', closures: 'Closures', errores: 'Errores', colecciones: 'Colecciones', texto: 'Texto', structs: 'Structs',
  enums: 'Enums', patrones: 'Patrones', concurrencia: 'Concurrencia', async: 'Async', macros: 'Macros', unsafe: 'Unsafe', memoria: 'Memoria',
  modulos: 'Módulos', cargo: 'Cargo', pruebas: 'Pruebas', control: 'Control de flujo', funciones: 'Funciones', general: 'General',
  'punteros-inteligentes': 'Punteros inteligentes', maquina: 'La máquina', formato: 'Formato',
};
export const nombreTema = (t: string | null) => (t ? TEMAS_NOMBRE[t] ?? t.charAt(0).toUpperCase() + t.slice(1).replace(/-/g, ' ') : '—');

// ---------- Markdown con shiki (claro/oscuro con variables CSS) ----------
let hl: Promise<Highlighter> | null = null;
const resaltador = () => (hl ??= createHighlighter({ themes: ['github-light', 'github-dark-default'], langs: ['rust', 'text', 'toml', 'bash', 'json'] }));

export async function codigoHtml(code: string, lang = 'rust') {
  const h = await resaltador();
  const l = h.getLoadedLanguages().includes(lang) ? lang : 'text';
  return h.codeToHtml(code.replace(/\n+$/, ''), { lang: l, themes: { light: 'github-light', dark: 'github-dark-default' }, defaultColor: false });
}

export async function md(texto: string): Promise<string> {
  const h = await resaltador();
  const m = new Marked({ gfm: true, breaks: false });
  m.use({
    renderer: {
      code({ text, lang }) {
        const l = (lang ?? 'text').split(/\s/)[0];
        const ll = h.getLoadedLanguages().includes(l) ? l : 'text';
        return h.codeToHtml(text, { lang: ll, themes: { light: 'github-light', dark: 'github-dark-default' }, defaultColor: false });
      },
    },
  });
  return m.parse(texto ?? '') as string;
}
export async function mdLinea(texto: string): Promise<string> {
  const m = new Marked({ gfm: true });
  return m.parseInline(texto ?? '') as string;
}

// ---------- Ítems para quizzes y juegos ----------
import { todosLosEjemplos } from './ejemplos';
import type { ItemQuiz } from '../components/islas/Quiz';
import type { ItemJuego } from '../components/islas/Juego';

export function itemQuiz(p: Pregunta): ItemQuiz {
  const e = p.ej ? todosLosEjemplos()[p.ej] : null;
  return {
    id: p.id, titulo: p.titulo, href: `/preguntas/${p.id}/`, tipo: p.tipo, enunciado: p.enunciado.split('\n')[0],
    codigo: (e?.codigo ?? p.codigo ?? '').replace(/\n+$/, '') || null, opciones: p.opciones, correcta: p.correcta, respuestas: p.respuestas,
  };
}

const lineas = (s: string) => s.replace(/\n+$/, '').split('\n').length;
/** Semilla determinista para elegir distractores (mismo resultado en cada build). */
function hash(s: string) { let h = 2166136261; for (let i = 0; i < s.length; i++) { h ^= s.charCodeAt(i); h = Math.imul(h, 16777619); } return h >>> 0; }

/** «¿Compila? 60 s»: ejemplos cortos que no compilan (debe_fallar) y que sí (compilados a wasm). */
export function itemsCompila(max = 160): ItemJuego[] {
  const out: (ItemJuego & { h: number })[] = [];
  for (const e of Object.values(todosLosEjemplos())) {
    if (lineas(e.codigo) > 16 || e.codigo.length > 700) continue;
    if (e.debeFallar && e.diagnosticos?.length) {
      const d = e.diagnosticos.find((x) => x.nivel === 'error')!;
      out.push({ codigo: e.codigo.replace(/\n+$/, ''), opciones: ['Compila', 'No compila'], correcta: 1, explicacion: `No compila: error${d.codigo ? `[${d.codigo}]` : ''}: ${d.mensaje}${d.linea ? ` (línea ${d.linea})` : ''}`, h: hash(e.id) });
    } else if (e.plataforma === 'wasm' && e.salida !== null && !e.id.endsWith('.arreglo')) {
      out.push({ codigo: e.codigo.replace(/\n+$/, ''), opciones: ['Compila', 'No compila'], correcta: 0, explicacion: 'Sí compila' + (e.salida.trim() ? ` e imprime «${e.salida.trim().split('\n')[0].slice(0, 60)}»` : '') + '.', h: hash(e.id) });
    }
  }
  // Mitad y mitad, en orden determinista
  const si = out.filter((x) => x.correcta === 0).sort((a, b) => a.h - b.h), no = out.filter((x) => x.correcta === 1).sort((a, b) => a.h - b.h);
  const n = Math.min(Math.max(si.length, no.length), Math.floor(max / 2));
  return [...si.slice(0, n), ...no.slice(0, n)].map(({ h, ...x }) => x);
}

/** «Predice la salida 60 s»: preguntas «predice» de la biblioteca + ejemplos cortos con salida corta. */
export function itemsPredice(max = 120): ItemJuego[] {
  const out: ItemJuego[] = [];
  for (const p of preguntas) {
    if (p.tipo !== 'predice') continue;
    const it = itemQuiz(p);
    if (!it.codigo) continue;
    out.push({ codigo: it.codigo, opciones: p.opciones.map((o) => o.replace(/^`|`$/g, '')), correcta: p.correcta, explicacion: `Era: ${p.opciones[p.correcta].replace(/`/g, '')}`, href: it.href });
  }
  const cand = Object.values(todosLosEjemplos()).filter((e) =>
    e.plataforma === 'wasm' && !e.debeFallar && !e.stdin && e.salida !== null && !e.error && e.exit === 0 &&
    lineas(e.codigo) <= 14 && e.codigo.length <= 600 && e.salida.trim() && lineas(e.salida) <= 3 && e.salida.length <= 90);
  const salidaDe = (e: (typeof cand)[number]) => e.salida!.replace(/\n+$/, '');
  for (const e of cand.sort((a, b) => hash(a.id) - hash(b.id))) {
    const propia = salidaDe(e);
    const mismos = cand.filter((x) => x.mundo === e.mundo && salidaDe(x) !== propia);
    const pool = (mismos.length >= 3 ? mismos : cand.filter((x) => salidaDe(x) !== propia)).sort((a, b) => (hash(a.id + e.id) - hash(b.id + e.id)));
    const distr = [...new Set(pool.map(salidaDe))].slice(0, 3);
    if (distr.length < 3) continue;
    out.push({ codigo: e.codigo.replace(/\n+$/, ''), opciones: [propia, ...distr], correcta: 0, explicacion: `Imprime: ${propia.replace(/\n/g, ' ⏎ ')}` });
    if (out.length >= max) break;
  }
  return out;
}

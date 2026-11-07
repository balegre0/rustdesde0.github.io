// Auditoría del curso de Emacs: estructura, IDs, preguntas, retos y tarjetas.
// Uso: bun run scripts/revisar-emacs.ts
import { modules, lessons, glossary } from '../public/emacs/data/index.js';

const fallos: string[] = [];
const ids = new Set<string>();
const modulos = new Set(modules.map((m) => m.id));
const lecciones = Object.values(lessons);

if (modules.length !== 14) fallos.push(`módulos: ${modules.length} (esperado 14)`);
if (lecciones.length !== 70) fallos.push(`lecciones: ${lecciones.length} (esperado 70)`);
if (glossary.length < 40) fallos.push(`glosario: ${glossary.length} (esperado >= 40)`);

for (const m of modules) {
  if (!m.title || !m.subtitle) fallos.push(`${m.id}: falta título o subtítulo`);
  if (m.lessons.length !== 5) fallos.push(`${m.id}: ${m.lessons.length} lecciones (esperado 5)`);
  for (const id of m.lessons) {
    if (!lessons[id]) fallos.push(`${m.id}: lección inexistente ${id}`);
    else if (lessons[id].module !== m.id) fallos.push(`${id}: módulo ${lessons[id].module} != ${m.id}`);
  }
}

for (const l of lecciones) {
  if (ids.has(l.id)) fallos.push(`id duplicado: ${l.id}`);
  ids.add(l.id);
  if (!modulos.has(l.module)) fallos.push(`${l.id}: módulo desconocido ${l.module}`);
  for (const k of ['title', 'obj', 'frase'] as const) if (!l[k]) fallos.push(`${l.id}: falta ${k}`);
  if (!Number.isInteger(l.n) || l.n < 1) fallos.push(`${l.id}: n inválido`);
  if (!Number.isInteger(l.min) || l.min < 1) fallos.push(`${l.id}: min inválido`);
  if (!Array.isArray(l.idea) || l.idea.length < 2) fallos.push(`${l.id}: idea < 2`);
  if (!l.code?.src) fallos.push(`${l.id}: falta code.src`);
  if (!Array.isArray(l.under) || l.under.length < 1) fallos.push(`${l.id}: under vacío`);
  if (!l.trap?.bad || !l.trap?.good) fallos.push(`${l.id}: trap incompleto`);
  if (!Array.isArray(l.q) || l.q.length < 2) fallos.push(`${l.id}: q < 2`);
  for (const [i, q] of (l.q || []).entries()) {
    if (!q.q) fallos.push(`${l.id} q${i}: sin pregunta`);
    if (!Array.isArray(q.o) || q.o.length < 2) fallos.push(`${l.id} q${i}: opciones < 2`);
    if (!Number.isInteger(q.a) || q.a < 0 || q.a >= (q.o?.length || 0)) fallos.push(`${l.id} q${i}: respuesta fuera de rango`);
    if (!q.why) fallos.push(`${l.id} q${i}: sin explicación`);
  }
  if (!l.ex?.p || !l.ex?.sol || !l.ex?.hint) fallos.push(`${l.id}: reto incompleto`);
  if (!Array.isArray(l.cards) || l.cards.length < 2) fallos.push(`${l.id}: cards < 2`);
  for (const c of l.cards || []) if (!c[0] || !c[1]) fallos.push(`${l.id}: card incompleta`);
  if (!Array.isArray(l.tags) || l.tags.length < 1) fallos.push(`${l.id}: sin tags`);
}

const preguntas = lecciones.reduce((n, l) => n + l.q.length, 0);
const retos = lecciones.filter((l) => l.ex).length;
const tarjetas = lecciones.reduce((n, l) => n + l.cards.length, 0);

console.log(`Emacs: ${modules.length} módulos, ${lecciones.length} lecciones, ${preguntas} preguntas, ${retos} retos, ${tarjetas} tarjetas, ${glossary.length} términos.`);
if (fallos.length) {
  console.log(`Fallos: ${fallos.length}`);
  fallos.slice(0, 30).forEach((f) => console.log(' -', f));
  process.exit(1);
}
console.log('Auditoría de Emacs: PASS');

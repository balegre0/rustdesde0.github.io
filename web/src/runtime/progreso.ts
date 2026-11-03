// Progreso del alumno: localStorage (síncrono, fuente principal) + copia en IndexedDB (idb-keyval)
// + exportar/importar JSON. Esquema versionado «rdc.progreso.v1».
// Safari borra el almacenamiento tras 7 días sin visita: por eso la copia en IDB y la exportación.
import { get, set } from 'idb-keyval';

export const CLAVE = 'rdc.progreso.v1';
export const EVENTO = 'rdc-progreso';

export type Actividad = { ok: boolean; intentos: number; fecha: string };
export type Progreso = {
  v: 1;
  lecciones: Record<string, { completada: string }>;
  ultima?: { id: string; url: string; titulo: string; fecha: string };
  actividades: Record<string, Actividad>;
  /** Estado FSRS de cada tarjeta («<leccion>#<n>»), serializado por ts-fsrs */
  tarjetas: Record<string, any>;
  /** Biblioteca de preguntas: primera respuesta (para la tasa de acierto) y si se acertó alguna vez */
  preguntas: Record<string, { ok: boolean; primera: boolean; intentos: number; fecha: string }>;
  problemas: Record<string, { ok: boolean; intentos: number; fecha: string }>;
  /** Mejores marcas de quizzes («quiz:<id>») y juegos («juego:<id>») */
  marcas: Record<string, { mejor: number; total: number; veces: number; fecha: string }>;
  /** Días (AAAA-MM-DD, hora local) con alguna práctica: para la racha */
  dias: string[];
  actualizado: string;
};

const vacio = (): Progreso => ({ v: 1, lecciones: {}, actividades: {}, tarjetas: {}, preguntas: {}, problemas: {}, marcas: {}, dias: [], actualizado: new Date(0).toISOString() });

function valido(x: any): x is Progreso {
  return x && typeof x === 'object' && x.v === 1 && typeof x.lecciones === 'object';
}

export function leer(): Progreso {
  try {
    const t = localStorage.getItem(CLAVE);
    if (t) {
      const p = JSON.parse(t);
      if (valido(p)) return { ...vacio(), ...p };
    }
  } catch { /* modo privado o JSON roto */ }
  return vacio();
}

let pedidoPersistencia = false;
export function guardar(p: Progreso) {
  p.actualizado = new Date().toISOString();
  try { localStorage.setItem(CLAVE, JSON.stringify(p)); } catch { /* sin espacio o modo privado */ }
  set(CLAVE, p).catch(() => {});
  if (!pedidoPersistencia && navigator.storage?.persist) {
    pedidoPersistencia = true;
    navigator.storage.persist().catch(() => {});
  }
  window.dispatchEvent(new CustomEvent(EVENTO, { detail: p }));
}

export const hoy = (d = new Date()) => `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`;

/** Marca el día de hoy como activo (racha). */
function marcarDia(p: Progreso) {
  const h = hoy();
  if (!p.dias.includes(h)) { p.dias.push(h); p.dias.sort(); if (p.dias.length > 800) p.dias = p.dias.slice(-800); }
}

/** Racha que perdona: días seguidos con práctica que terminan hoy o ayer (si hoy aún no has practicado). */
export function racha(p: Progreso, ahora = new Date()): { dias: number; hoyHecho: boolean; mejor: number } {
  const set = new Set(p.dias);
  const d = new Date(ahora);
  const hoyHecho = set.has(hoy(d));
  if (!hoyHecho) d.setDate(d.getDate() - 1);
  let n = 0;
  while (set.has(hoy(d))) { n++; d.setDate(d.getDate() - 1); }
  let mejor = 0, act = 0, prev: Date | null = null;
  for (const s of [...set].sort()) {
    const x = new Date(s + 'T12:00:00');
    act = prev && Math.round((+x - +prev) / 86400000) === 1 ? act + 1 : 1;
    mejor = Math.max(mejor, act);
    prev = x;
  }
  return { dias: n, hoyHecho, mejor };
}

/** Respuesta a una pregunta de la biblioteca. La primera respuesta fija la tasa de acierto. */
export function registrarPregunta(id: string, ok: boolean) {
  return modificar((p) => {
    const a = p.preguntas[id];
    p.preguntas[id] = a
      ? { ...a, ok: a.ok || ok, intentos: a.intentos + 1, fecha: new Date().toISOString() }
      : { ok, primera: ok, intentos: 1, fecha: new Date().toISOString() };
    marcarDia(p);
  });
}

export function registrarProblema(slug: string, ok: boolean) {
  return modificar((p) => {
    const a = p.problemas[slug] ?? { ok: false, intentos: 0, fecha: '' };
    p.problemas[slug] = { ok: a.ok || ok, intentos: a.intentos + 1, fecha: new Date().toISOString() };
    marcarDia(p);
  });
}

/** Guarda una puntuación; devuelve true si es nueva mejor marca. */
export function registrarMarca(clave: string, puntos: number, total: number) {
  let nueva = false;
  modificar((p) => {
    const a = p.marcas[clave];
    nueva = !a || puntos > a.mejor;
    p.marcas[clave] = { mejor: Math.max(a?.mejor ?? 0, puntos), total, veces: (a?.veces ?? 0) + 1, fecha: new Date().toISOString() };
    marcarDia(p);
  });
  return nueva;
}

export function modificar(fn: (p: Progreso) => void) {
  const p = leer();
  fn(p);
  guardar(p);
  return p;
}

export const estaCompletada = (id: string) => !!leer().lecciones[id];

export function marcarCompletada(id: string, completada = true) {
  return modificar((p) => {
    if (completada) { p.lecciones[id] = { completada: new Date().toISOString() }; marcarDia(p); }
    else delete p.lecciones[id];
  });
}

export function registrarVisita(id: string, url: string, titulo: string) {
  return modificar((p) => { p.ultima = { id, url, titulo, fecha: new Date().toISOString() }; });
}

/** Registra el resultado de un ejercicio. Solo cuenta el primer acierto; los intentos se acumulan. */
export function registrarActividad(clave: string, ok: boolean) {
  return modificar((p) => {
    const a = p.actividades[clave] ?? { ok: false, intentos: 0, fecha: '' };
    a.intentos += 1;
    if (ok) a.ok = true;
    a.fecha = new Date().toISOString();
    p.actividades[clave] = a;
    marcarDia(p);
  });
}

/** Clave estable de una actividad: página + tipo + hash del enunciado. */
export function claveActividad(tipo: string, texto: string) {
  let h = 2166136261;
  for (let i = 0; i < texto.length; i++) { h ^= texto.charCodeAt(i); h = Math.imul(h, 16777619); }
  const pagina = location.pathname.replace(/\/+$/, '') || '/';
  return `${pagina}#${tipo}-${(h >>> 0).toString(36)}`;
}

/** Al cargar: si localStorage está vacío (p. ej. Safari lo borró) pero IndexedDB conserva copia, se restaura. */
export async function restaurarDesdeIDB() {
  try {
    const local = leer();
    const idb = await get(CLAVE);
    if (valido(idb) && idb.actualizado > local.actualizado) {
      localStorage.setItem(CLAVE, JSON.stringify(idb));
      window.dispatchEvent(new CustomEvent(EVENTO, { detail: idb }));
    }
  } catch { /* sin IndexedDB */ }
}

export function exportar(): string {
  return JSON.stringify({ ...leer(), exportado: new Date().toISOString(), sitio: 'Rust desde cero' }, null, 2);
}

/** Importa un JSON exportado. Fusiona: conserva lo completado en ambos y el estado de tarjeta más reciente. */
export function importar(texto: string): { lecciones: number; tarjetas: number } {
  const x = JSON.parse(texto);
  if (!valido(x)) throw new Error('El archivo no es una exportación de progreso de «Rust desde cero» (falta "v": 1).');
  const p = leer();
  for (const [k, v] of Object.entries(x.lecciones ?? {})) if (!p.lecciones[k]) p.lecciones[k] = v as any;
  for (const [k, v] of Object.entries(x.actividades ?? {})) {
    const a = p.actividades[k];
    const b = v as Actividad;
    p.actividades[k] = a ? { ok: a.ok || b.ok, intentos: Math.max(a.intentos, b.intentos), fecha: a.fecha > b.fecha ? a.fecha : b.fecha } : b;
  }
  for (const [k, v] of Object.entries(x.tarjetas ?? {})) {
    const a = p.tarjetas[k];
    const b = v as any;
    if (!a || String(b?.last_review ?? '') > String(a?.last_review ?? '')) p.tarjetas[k] = b;
  }
  for (const [k, b] of Object.entries<any>(x.preguntas ?? {})) {
    const a = p.preguntas[k];
    p.preguntas[k] = a ? { ok: a.ok || b.ok, primera: a.fecha < b.fecha ? a.primera : b.primera, intentos: Math.max(a.intentos, b.intentos), fecha: a.fecha > b.fecha ? a.fecha : b.fecha } : b;
  }
  for (const [k, b] of Object.entries<any>(x.problemas ?? {})) {
    const a = p.problemas[k];
    p.problemas[k] = a ? { ok: a.ok || b.ok, intentos: Math.max(a.intentos, b.intentos), fecha: a.fecha > b.fecha ? a.fecha : b.fecha } : b;
  }
  for (const [k, b] of Object.entries<any>(x.marcas ?? {})) {
    const a = p.marcas[k];
    p.marcas[k] = a ? { ...b, mejor: Math.max(a.mejor, b.mejor), veces: Math.max(a.veces, b.veces) } : b;
  }
  p.dias = [...new Set([...p.dias, ...(Array.isArray(x.dias) ? x.dias : [])])].sort();
  if (x.ultima && (!p.ultima || x.ultima.fecha > p.ultima.fecha)) p.ultima = x.ultima;
  guardar(p);
  return { lecciones: Object.keys(x.lecciones ?? {}).length, tarjetas: Object.keys(x.tarjetas ?? {}).length };
}

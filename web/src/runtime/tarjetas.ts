// Tarjetas de repaso: índice (/datos/lecciones.json) + estado FSRS guardado en el progreso.
import type { Progreso } from './progreso';

export type Tarjeta = { id: string; p: string; r: string; leccion: string; titulo: string; href: string };
export type Indice = { mundos: { slug: string; numero: number | null; titulo: string; href: string | null; lecciones: { id: string; href: string; titulo: string; duracion: number | null; tarjetas: { p: string; r: string }[] }[] }[] };

export const MAX_SESION = 10;
export const SEGUNDOS_POR_TARJETA = 20;

let indice: Promise<Indice> | null = null;
export function cargarIndice(): Promise<Indice> {
  indice ??= fetch(`${import.meta.env.BASE_URL.replace(/\/$/, '')}/datos/lecciones.json`).then((r) => {
    if (!r.ok) throw new Error('No se pudo cargar el índice de lecciones');
    return r.json();
  });
  return indice;
}

/** Solo entran en el repaso las tarjetas de lecciones completadas. */
export function tarjetasActivas(ind: Indice, p: Progreso): Tarjeta[] {
  const out: Tarjeta[] = [];
  for (const m of ind.mundos)
    for (const l of m.lecciones) {
      if (!p.lecciones[l.id]) continue;
      l.tarjetas.forEach((t, i) => out.push({ id: `${l.id}#${i}`, p: t.p, r: t.r, leccion: l.id, titulo: l.titulo, href: l.href }));
    }
  return out;
}

/** Pendientes hoy: nuevas (sin estado) o con fecha de repaso vencida. Ordenadas por vencimiento. */
export function pendientes(ind: Indice, p: Progreso, ahora = new Date()): Tarjeta[] {
  const fin = new Date(ahora);
  fin.setHours(23, 59, 59, 999);
  return tarjetasActivas(ind, p)
    .filter((t) => { const s = p.tarjetas[t.id]; return !s || new Date(s.due) <= fin; })
    .sort((a, b) => {
      const da = p.tarjetas[a.id] ? new Date(p.tarjetas[a.id].due).getTime() : 0;
      const db = p.tarjetas[b.id] ? new Date(p.tarjetas[b.id].due).getTime() : 0;
      return da - db;
    });
}

export const minutosPara = (n: number) => Math.max(1, Math.round((n * SEGUNDOS_POR_TARJETA) / 60));

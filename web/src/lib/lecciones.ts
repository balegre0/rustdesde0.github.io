// Índice de lecciones del curso, calculado en build desde la colección de Starlight.
import { getCollection, type CollectionEntry } from 'astro:content';

export type Leccion = {
  id: string; // ruta dentro de curso/, sin extensión: «04-decidir-y-repetir/01-if-y-else»
  href: string;
  titulo: string;
  descripcion: string;
  mundo: string; // «04-decidir-y-repetir»
  orden: number;
  duracion: number | null;
  desde: string | null;
  requiere: string[];
  tarjetas: { p: string; r: string }[];
  frase: string | null; // la idea en una frase (frontmatter «frase»)
  imagina: string | null; // la comparación cotidiana (frontmatter «imagina»)
};
export type Mundo = { slug: string; numero: number | null; titulo: string; descripcion: string; href: string | null; lecciones: Leccion[] };

const RE_LECCION = /^curso\/([^/]+)\/([^/]+)$/;
const RE_MUNDO = /^curso\/([^/]+)$/;
export const esLeccionId = (id: string) => RE_LECCION.test(id);
export const esMundoId = (id: string) => RE_MUNDO.test(id);
export const idLeccion = (entryId: string) => entryId.replace(/^curso\//, '');

const base = import.meta.env.BASE_URL.replace(/\/$/, '');
const hrefDe = (e: CollectionEntry<'docs'>) => `${base}/${e.id}/`;
const ordenDe = (e: CollectionEntry<'docs'>) => e.data.sidebar?.order ?? Number.MAX_SAFE_INTEGER;

let cache: Promise<Mundo[]> | null = null;

export function mundos(): Promise<Mundo[]> {
  cache ??= (async () => {
    const docs = await getCollection('docs', (e) => e.id.startsWith('curso/') && !e.data.draft);
    const porMundo = new Map<string, Mundo>();
    const mundo = (slug: string) => {
      if (!porMundo.has(slug)) {
        const m = slug.match(/^(\d+)-(.*)$/);
        const t = (m ? m[2] : slug).replace(/-/g, ' ');
        porMundo.set(slug, { slug, numero: m ? Number(m[1]) : null, titulo: t.charAt(0).toUpperCase() + t.slice(1), descripcion: '', href: null, lecciones: [] });
      }
      return porMundo.get(slug)!;
    };
    for (const e of docs) {
      const mm = e.id.match(RE_MUNDO);
      if (mm) {
        const m = mundo(mm[1]);
        m.titulo = e.data.title.replace(/^Mundo\s+\d+\s*[\u00B7:.\-]\s*/i, '');
        m.descripcion = e.data.description ?? '';
        m.href = hrefDe(e);
        continue;
      }
      const ml = e.id.match(RE_LECCION);
      if (!ml) continue;
      mundo(ml[1]).lecciones.push({
        id: idLeccion(e.id),
        href: hrefDe(e),
        titulo: e.data.title,
        descripcion: e.data.description ?? '',
        mundo: ml[1],
        orden: ordenDe(e),
        duracion: e.data.duracion ?? null,
        desde: e.data.desde ?? null,
        requiere: e.data.requiere ?? [],
        tarjetas: e.data.tarjetas ?? [],
        frase: e.data.frase ?? null,
        imagina: e.data.imagina ?? null,
      });
    }
    const lista = [...porMundo.values()].sort((a, b) => a.slug.localeCompare(b.slug));
    for (const m of lista) m.lecciones.sort((a, b) => a.orden - b.orden || a.id.localeCompare(b.id));
    return lista;
  })();
  return cache;
}

export async function lecciones(): Promise<Leccion[]> {
  return (await mundos()).flatMap((m) => m.lecciones);
}

// /datos/lecciones.json — índice de mundos, lecciones y tarjetas (lo usan repaso, progreso y la portada).
import type { APIRoute } from 'astro';
import { mundos } from '../../lib/lecciones';

export const prerender = true;

export const GET: APIRoute = async () => {
  const ms = await mundos();
  const cuerpo = {
    generado: new Date().toISOString(),
    mundos: ms.map((m) => ({
      slug: m.slug, numero: m.numero, titulo: m.titulo, href: m.href,
      lecciones: m.lecciones.map((l) => ({ id: l.id, href: l.href, titulo: l.titulo, duracion: l.duracion, tarjetas: l.tarjetas })),
    })),
  };
  return new Response(JSON.stringify(cuerpo), { headers: { 'Content-Type': 'application/json; charset=utf-8' } });
};

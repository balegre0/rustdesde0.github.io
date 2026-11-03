// /datos/practica.json — títulos de preguntas y problemas (lo usa «Vuelve a intentar lo que fallaste» en el repaso).
import type { APIRoute } from 'astro';
import { preguntas, problemas, DIFICULTAD } from '../../lib/practica';

export const prerender = true;

export const GET: APIRoute = () => {
  const cuerpo = {
    preguntas: Object.fromEntries(preguntas.map((p) => [p.id, [p.titulo, DIFICULTAD[p.dificultad] ?? p.dificultad]])),
    problemas: Object.fromEntries(problemas.map((p) => [p.slug, [p.titulo, DIFICULTAD[p.dificultad] ?? p.dificultad]])),
  };
  return new Response(JSON.stringify(cuerpo), { headers: { 'Content-Type': 'application/json; charset=utf-8' } });
};

// Panel personal: racha que perdona, preguntas, acierto por tema, problemas, quizzes, juegos y lecciones.
import { useEffect, useState } from 'preact/hooks';
import { EVENTO, leer, racha, hoy, type Progreso } from '../../runtime/progreso';
import { cargarIndice, MAX_SESION, minutosPara, pendientes, type Indice } from '../../runtime/tarjetas';

type Props = {
  temas: Record<string, string>; nombres: Record<string, string>;
  totales: { preguntas: number; problemas: number }; quizzes: { id: string; titulo: string; total: number }[];
};

export default function Panel({ temas, nombres, totales, quizzes }: Props) {
  const [p, setP] = useState<Progreso>(leer());
  const [ind, setInd] = useState<Indice | null>(null);
  useEffect(() => {
    const f = () => setP(leer());
    window.addEventListener(EVENTO, f);
    cargarIndice().then(setInd).catch(() => {});
    return () => window.removeEventListener(EVENTO, f);
  }, []);

  const r = racha(p);
  const resp = Object.entries(p.preguntas).filter(([id]) => id in temas);
  const primera = resp.filter(([, v]) => v.primera).length;
  const porTema = new Map<string, { n: number; ok: number }>();
  for (const [id, v] of resp) {
    const t = temas[id];
    const x = porTema.get(t) ?? { n: 0, ok: 0 };
    x.n++; if (v.primera) x.ok++;
    porTema.set(t, x);
  }
  const temasOrden = [...porTema.entries()].sort((a, b) => a[1].ok / a[1].n - b[1].ok / b[1].n || b[1].n - a[1].n);
  const resueltos = Object.values(p.problemas).filter((x) => x.ok).length;
  const lecciones = Object.keys(p.lecciones).length;
  const totalLecciones = ind ? ind.mundos.reduce((s, m) => s + m.lecciones.length, 0) : null;
  const repaso = ind ? Math.min(pendientes(ind, p).length, MAX_SESION) : 0;

  // Últimas 10 semanas de actividad (calendario sencillo, el color nunca va solo: cada día tiene etiqueta)
  const dias = new Set(p.dias);
  const semanas: { d: string; on: boolean }[][] = [];
  const inicio = new Date(); inicio.setDate(inicio.getDate() - 69 - ((inicio.getDay() + 6) % 7));
  for (let s = 0; s < 11; s++) {
    const col = [];
    for (let k = 0; k < 7; k++) {
      const d = new Date(inicio); d.setDate(inicio.getDate() + s * 7 + k);
      if (d > new Date()) break;
      col.push({ d: hoy(d), on: dias.has(hoy(d)) });
    }
    semanas.push(col);
  }

  return (
    <div class="rdc-panel not-content">
      <dl class="rdc-cifras">
        <div><dt>Racha</dt><dd class="rdc-num">{r.dias}<span> {r.dias === 1 ? 'día' : 'días'}</span></dd><p class="rdc-cifra-nota">{r.hoyHecho ? 'Hoy ya has practicado.' : r.dias ? 'Practica algo hoy para seguirla (una pregunta basta).' : 'Una pregunta al día basta para empezar.'}{r.mejor > r.dias && ` Tu mejor racha: ${r.mejor}.`}</p></div>
        <div><dt>Preguntas respondidas</dt><dd class="rdc-num">{resp.length}<span> de {totales.preguntas}</span></dd><p class="rdc-cifra-nota">{resp.length ? `${Math.round((100 * primera) / resp.length)} % a la primera` : 'Aún ninguna.'}</p></div>
        <div><dt>Problemas resueltos</dt><dd class="rdc-num">{resueltos}<span> de {totales.problemas}</span></dd></div>
        <div><dt>Lecciones completadas</dt><dd class="rdc-num">{lecciones}{totalLecciones !== null && <span> de {totalLecciones}</span>}</dd><p class="rdc-cifra-nota">{repaso > 0 ? <a href="/repaso/">{repaso} tarjetas para hoy · ~{minutosPara(repaso)} min</a> : 'Nada que repasar hoy.'}</p></div>
      </dl>

      <section aria-labelledby="pn-act">
        <h2 id="pn-act">Actividad (últimas 10 semanas)</h2>
        <div class="rdc-calendario" role="img" aria-label={`${[...dias].filter((d) => d >= semanas[0]?.[0]?.d).length} días con práctica en las últimas 10 semanas`}>
          {semanas.map((col) => <div class="rdc-cal-col">{col.map((x) => <span class="rdc-cal-dia" data-on={x.on ? 'true' : undefined} title={`${x.d}${x.on ? ': practicaste' : ''}`} />)}</div>)}
        </div>
      </section>

      <section aria-labelledby="pn-temas">
        <h2 id="pn-temas">Acierto por tema</h2>
        {temasOrden.length === 0 ? <p>Responde algunas <a href="/preguntas/">preguntas</a> y aquí verás tus puntos fuertes y débiles.</p> : (
          <table class="rdc-tabla rdc-tabla-temas">
            <thead><tr><th scope="col">Tema</th><th scope="col">Respondidas</th><th scope="col">A la primera</th><th scope="col"><span class="rdc-sr">Barra</span></th></tr></thead>
            <tbody>
              {temasOrden.map(([t, x]) => {
                const pct = Math.round((100 * x.ok) / x.n);
                return (
                  <tr>
                    <th scope="row"><a href={`/preguntas/?tema=${encodeURIComponent(t)}`}>{nombres[t] ?? t}</a></th>
                    <td class="rdc-num">{x.n}</td>
                    <td class="rdc-num">{pct} %</td>
                    <td class="rdc-col-barra"><span class="rdc-barra-progreso" aria-hidden="true"><span style={{ transform: `scaleX(${pct / 100})` }} /></span></td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        )}
        {temasOrden.length > 1 && <p class="rdc-cifra-nota">Arriba, los temas donde más fallas: buen sitio para practicar.</p>}
      </section>

      <section aria-labelledby="pn-marcas">
        <h2 id="pn-marcas">Mejores marcas</h2>
        <ul class="rdc-marcas">
          {quizzes.map((q) => { const m = p.marcas[`quiz:${q.id}`]; return <li><a href={`/quizzes/${q.id}/`}>{q.titulo}</a><span class="rdc-num">{m ? `${m.mejor} / ${q.total}` : '—'}</span></li>; })}
          {[['compila', '¿Compila? · 60 s'], ['predice', 'Predice la salida · 60 s']].map(([id, t]) => { const m = p.marcas[`juego:${id}`]; return <li><a href={`/juegos/${id}/`}>{t}</a><span class="rdc-num">{m ? `${m.mejor} aciertos` : '—'}</span></li>; })}
        </ul>
      </section>
      <p class="rdc-cifra-nota">¿Cambias de navegador? Exporta tu progreso desde <a href="/progreso/">Tu progreso</a>.</p>
    </div>
  );
}

// Minijuegos de 60 s (/juegos/compila/, /juegos/predice/): ráfaga de preguntas con tiempo, puntuación y mejor marca.
import { useEffect, useRef, useState } from 'preact/hooks';
import { leer, registrarMarca } from '../../runtime/progreso';
import { barajar } from './md';

export type ItemJuego = { codigo: string; opciones: string[]; correcta: number; explicacion: string; href?: string | null };
type Props = { juego: 'compila' | 'predice'; items: ItemJuego[]; duracion?: number };


export default function Juego({ juego, items, duracion = 60 }: Props) {
  const clave = `juego:${juego}`;
  const [fase, setFase] = useState<'inicio' | 'jugando' | 'fin'>('inicio');
  const [cola, setCola] = useState<ItemJuego[]>([]);
  const [i, setI] = useState(0);
  const [puntos, setPuntos] = useState(0);
  const [fallos, setFallos] = useState<ItemJuego[]>([]);
  const [fb, setFb] = useState<null | { ok: boolean; texto: string }>(null);
  const [restante, setRestante] = useState(duracion);
  const [mejor, setMejor] = useState<number | null>(null);
  const [nueva, setNueva] = useState(false);
  const fin = useRef(0);
  const puntosRef = useRef(0);
  const btnInicio = useRef<HTMLButtonElement>(null);
  const zona = useRef<HTMLDivElement>(null);

  useEffect(() => { setMejor(leer().marcas[clave]?.mejor ?? null); }, []);
  useEffect(() => {
    if (fase !== 'jugando') return;
    const t = setInterval(() => {
      const r = Math.max(0, (fin.current - Date.now()) / 1000);
      setRestante(r);
      if (r <= 0) terminar();
    }, 200);
    return () => clearInterval(t);
  }, [fase]);
  useEffect(() => { if (fase === 'jugando') zona.current?.focus({ preventScroll: true }); else btnInicio.current?.focus({ preventScroll: true }); }, [fase]);

  function empezar() {
    setCola(barajar(items).map((x) => juego === 'predice' ? mezclarOpciones(x) : x));
    setI(0); setPuntos(0); puntosRef.current = 0; setFallos([]); setFb(null);
    fin.current = Date.now() + duracion * 1000;
    setRestante(duracion);
    setFase('jugando');
  }
  function terminar() {
    setFase((f) => {
      if (f !== 'jugando') return f;
      const n = registrarMarca(clave, puntosRef.current, 0);
      setNueva(n && puntosRef.current > 0);
      setMejor(leer().marcas[clave]?.mejor ?? puntosRef.current);
      return 'fin';
    });
  }
  function responder(k: number) {
    if (fase !== 'jugando' || fb) return;
    const it = cola[i];
    const ok = k === it.correcta;
    if (ok) { puntosRef.current++; setPuntos(puntosRef.current); } else setFallos((f) => [...f, it]);
    setFb({ ok, texto: it.explicacion });
    setTimeout(() => {
      setFb(null);
      setI((x) => (x + 1 >= cola.length ? (setCola(barajar(items).map((y) => juego === 'predice' ? mezclarOpciones(y) : y)), 0) : x + 1));
    }, ok ? 450 : 1600);
  }
  function tecla(e: KeyboardEvent) {
    if (fase !== 'jugando') return;
    const it = cola[i];
    if (juego === 'compila') {
      if (e.key === 'c' || e.key === 'C' || e.key === 'ArrowLeft') { e.preventDefault(); responder(0); }
      if (e.key === 'n' || e.key === 'N' || e.key === 'ArrowRight') { e.preventDefault(); responder(1); }
    } else {
      const k = Number(e.key) - 1;
      if (k >= 0 && k < it.opciones.length) { e.preventDefault(); responder(k); }
    }
  }

  if (!items.length) return <p>Todavía no hay suficientes ejemplos para este juego.</p>;

  if (fase !== 'jugando')
    return (
      <div class="rdc-juego not-content" data-juego={juego} data-fin={fase === 'fin' ? 'true' : undefined}>
        {fase === 'fin' && (
          <>
            <p class="rdc-juego-puntos" aria-live="polite"><span class="rdc-num">{puntos}</span> {puntos === 1 ? 'acierto' : 'aciertos'} en {duracion} s</p>
            <p>{nueva ? '¡Nueva mejor marca!' : `Tu mejor marca: ${mejor ?? puntos}.`}</p>
            {fallos.length > 0 && (
              <details class="rdc-plegable">
                <summary>Repasar los {fallos.length} fallos</summary>
                <ol class="rdc-juego-fallos">
                  {fallos.map((f) => (
                    <li><pre class="rdc-juego-codigo"><code>{f.codigo}</code></pre><p>{f.explicacion} {f.href && <a href={f.href}>Más</a>}</p></li>
                  ))}
                </ol>
              </details>
            )}
          </>
        )}
        {fase === 'inicio' && (
          <>
            <p class="rdc-juego-lema">{duracion} segundos · tantos como puedas</p>
            <p>{juego === 'compila'
              ? 'Verás programas cortos. Decide si compilan o no. Teclado: C (o ←) = compila, N (o →) = no compila.'
              : 'Verás programas cortos. Elige qué imprimen. Teclado: 1, 2, 3, 4.'}</p>
            {mejor !== null && <p class="rdc-juego-mejor">Tu mejor marca: <strong class="rdc-num">{mejor}</strong></p>}
          </>
        )}
        <button type="button" class="rdc-boton" ref={btnInicio} onClick={empezar}>{fase === 'fin' ? 'Jugar otra vez' : 'Empezar'}</button>
      </div>
    );

  const it = cola[i];
  return (
    <div class="rdc-juego not-content" data-juego={juego} data-jugando ref={zona} tabIndex={-1} onKeyDown={tecla}>
      <div class="rdc-juego-cab">
        <span>Aciertos: <strong class="rdc-num" data-puntos>{puntos}</strong></span>
        <span class="rdc-num" data-urgente={restante < 10 ? 'true' : undefined} role="timer" aria-label={`Quedan ${Math.ceil(restante)} segundos`}>{Math.ceil(restante)} s</span>
      </div>
      <div class="rdc-juego-tiempo" aria-hidden="true"><span style={{ transform: `scaleX(${restante / duracion})` }} /></div>
      <pre class="rdc-juego-codigo" data-codigo-juego><code>{it.codigo}</code></pre>
      {juego === 'compila' ? (
        <div class="rdc-juego-botones">
          <button type="button" class="rdc-boton grande" onClick={() => responder(0)} disabled={!!fb}><kbd aria-hidden="true">C</kbd> Compila</button>
          <button type="button" class="rdc-boton grande secundario" onClick={() => responder(1)} disabled={!!fb}><kbd aria-hidden="true">N</kbd> No compila</button>
        </div>
      ) : (
        <ul class="rdc-opciones" role="list">
          {it.opciones.map((o, k) => (
            <li key={k}>
              <button type="button" class="rdc-opcion" onClick={() => responder(k)} aria-disabled={fb ? 'true' : undefined} data-resultado={fb ? (k === it.correcta ? 'ok' : undefined) : undefined}>
                <span class="rdc-opcion-letra" aria-hidden="true">{k + 1}</span>
                <span class="rdc-opcion-texto"><code class="rdc-pre">{o}</code></span>
              </button>
            </li>
          ))}
        </ul>
      )}
      <div class="rdc-juego-fb" aria-live="assertive" data-estado={fb ? (fb.ok ? 'ok' : 'mal') : undefined}>
        {fb && <p><strong>{fb.ok ? '✓ Bien' : '✗ No'}</strong>{!fb.ok && <> · {fb.texto}</>}</p>}
      </div>
    </div>
  );
}

function mezclarOpciones(x: ItemJuego): ItemJuego {
  const orden = barajar(x.opciones.map((_, k) => k));
  return { ...x, opciones: orden.map((k) => x.opciones[k]), correcta: orden.indexOf(x.correcta) };
}

// Quiz cronometrado (/quizzes/<id>/): una pregunta cada vez, corrección inmediata, puntuación y mejor marca.
import { useEffect, useRef, useState } from 'preact/hooks';
import { mdEnLinea, ordenAleatorio } from './md';
import { Marca } from './Pregunta';
import { leer, registrarMarca, registrarPregunta } from '../../runtime/progreso';

export type ItemQuiz = {
  id: string; titulo: string; href: string; tipo: string; enunciado: string; codigo: string | null;
  opciones: string[]; correcta: number; respuestas: string[];
};
type Props = { id: string; titulo: string; tiempo: number; items: ItemQuiz[] };
type Resp = { ok: boolean; valor: string };

const LETRAS = 'ABCDEFGH';
const norm = (s: string) => s.trim().toLowerCase();
const mmss = (s: number) => `${Math.floor(s / 60)}:${String(Math.max(0, Math.floor(s % 60))).padStart(2, '0')}`;

export default function Quiz({ id, titulo, tiempo, items }: Props) {
  const clave = `quiz:${id}`;
  const [fase, setFase] = useState<'inicio' | 'jugando' | 'fin'>('inicio');
  const [i, setI] = useState(0);
  const [resps, setResps] = useState<(Resp | null)[]>([]);
  const [restante, setRestante] = useState(tiempo);
  const [texto, setTexto] = useState('');
  const [nueva, setNueva] = useState(false);
  const [mejor, setMejor] = useState<number | null>(null);
  // Orden de las opciones de cada pregunta: nuevo en cada partida (la correcta no está siempre en el mismo sitio).
  const [ordenes, setOrdenes] = useState<number[][]>([]);
  const fin = useRef(0);
  const foco = useRef<HTMLElement | null>(null);

  useEffect(() => { setMejor(leer().marcas[clave]?.mejor ?? null); }, []);
  useEffect(() => {
    if (fase !== 'jugando') return;
    const t = setInterval(() => {
      const r = Math.max(0, (fin.current - Date.now()) / 1000);
      setRestante(r);
      if (r <= 0) terminar();
    }, 250);
    return () => clearInterval(t);
  }, [fase]);
  useEffect(() => { foco.current?.focus({ preventScroll: false }); }, [i, fase, resps[i]]);

  function empezar() {
    setResps(items.map(() => null));
    setOrdenes(items.map((x) => ordenAleatorio(x.opciones.length)));
    setI(0);
    setTexto('');
    fin.current = Date.now() + tiempo * 1000;
    setRestante(tiempo);
    setFase('jugando');
  }
  function terminar(rs = resps) {
    setFase((f) => {
      if (f !== 'jugando') return f;
      const puntos = rs.filter((r) => r?.ok).length;
      const n = registrarMarca(clave, puntos, items.length);
      setNueva(n && puntos > 0);
      setMejor(leer().marcas[clave]?.mejor ?? puntos);
      return 'fin';
    });
  }
  function responder(valor: string, ok: boolean) {
    if (resps[i]) return;
    const rs = [...resps];
    rs[i] = { ok, valor };
    setResps(rs);
    try { registrarPregunta(items[i].id, ok); } catch { /* sin almacenamiento */ }
  }
  function siguiente() {
    if (i + 1 >= items.length) terminar();
    else { setI(i + 1); setTexto(''); }
  }

  if (fase === 'inicio')
    return (
      <div class="rdc-juego not-content" data-quiz={id}>
        <p class="rdc-juego-lema">{items.length} preguntas · {mmss(tiempo)} minutos</p>
        <p>Responde lo más rápido que puedas sin perder precisión. Cada pregunta se corrige al momento; al final verás tu puntuación y podrás leer las soluciones.</p>
        {mejor !== null && <p class="rdc-juego-mejor">Tu mejor marca: <strong class="rdc-num">{mejor}</strong> de {items.length}</p>}
        <button type="button" class="rdc-boton" ref={(e) => (foco.current = e)} onClick={empezar}>Empezar</button>
      </div>
    );

  if (fase === 'fin') {
    const puntos = resps.filter((r) => r?.ok).length;
    return (
      <div class="rdc-juego not-content" data-quiz={id} data-fin>
        <p class="rdc-juego-puntos" tabIndex={-1} ref={(e) => (foco.current = e)}><span class="rdc-num">{puntos}</span> de {items.length}</p>
        <p>{nueva ? '¡Nueva mejor marca!' : mejor !== null ? `Tu mejor marca: ${mejor} de ${items.length}.` : ''} {puntos < items.length && 'Lee las soluciones de las que fallaste: es donde más se aprende.'}</p>
        <ol class="rdc-quiz-repaso">
          {items.map((it, k) => (
            <li data-estado={resps[k] ? (resps[k]!.ok ? 'ok' : 'mal') : 'nada'}>
              <span class="rdc-caso-marca" aria-hidden="true">{resps[k] ? (resps[k]!.ok ? '✓' : '✗') : '·'}</span>
              <span class="rdc-sr">{resps[k] ? (resps[k]!.ok ? 'Acertada: ' : 'Fallada: ') : 'Sin responder: '}</span>
              <a href={it.href}>{it.titulo}</a>
            </li>
          ))}
        </ol>
        <div class="rdc-barra"><button type="button" class="rdc-boton" onClick={empezar}>Otra vez</button><a class="rdc-boton secundario" href="/quizzes/">Más quizzes</a></div>
      </div>
    );
  }

  const it = items[i];
  const r = resps[i];
  const orden = ordenes[i] ?? it.opciones.map((_, k) => k);
  const fraccion = restante / tiempo;
  return (
    <div class="rdc-juego not-content" data-quiz={id} data-jugando>
      <div class="rdc-juego-cab">
        <span>Pregunta <span class="rdc-num">{i + 1}</span> de {items.length}</span>
        <span class="rdc-num" data-urgente={restante < 15 ? 'true' : undefined} role="timer" aria-label={`Quedan ${Math.ceil(restante)} segundos`}>{mmss(restante)}</span>
      </div>
      <div class="rdc-juego-tiempo" aria-hidden="true"><span style={{ transform: `scaleX(${fraccion})` }} /></div>
      <p class="rdc-quiz-enunciado" dangerouslySetInnerHTML={{ __html: mdEnLinea(it.enunciado) }} />
      {it.codigo && <pre class="rdc-juego-codigo"><code>{it.codigo}</code></pre>}
      {it.tipo === 'respuesta' ? (
        <form class="rdc-fila" onSubmit={(e) => { e.preventDefault(); if (texto.trim()) responder(texto, it.respuestas.some((x) => norm(x) === norm(texto))); }}>
          <input class="rdc-respuesta-input" value={texto} onInput={(e) => setTexto((e.target as HTMLInputElement).value)} readOnly={!!r} aria-label="Tu respuesta" ref={(e) => { if (!r) foco.current = e; }} autocomplete="off" />
          <button type="submit" class="rdc-boton" disabled={!!r}>Responder</button>
        </form>
      ) : (
        <ul class="rdc-opciones" role="list">
          {orden.map((k, pos) => {
            const o = it.opciones[k];
            const est = r ? (k === it.correcta ? 'ok' : r.valor === String(k) ? 'mal' : '') : '';
            return (
              <li key={k}>
                <button type="button" class="rdc-opcion" data-opcion={k} data-resultado={est || undefined} aria-disabled={r ? 'true' : undefined} onClick={() => responder(String(k), k === it.correcta)} ref={(e) => { if (pos === 0 && !r) foco.current = e; }}>
                  <span class="rdc-opcion-letra" aria-hidden="true">{LETRAS[pos]}</span>
                  <span class="rdc-opcion-texto" dangerouslySetInnerHTML={{ __html: mdEnLinea(o) }} />
                  {est === 'ok' && <span class="rdc-opcion-marca"><Marca ok /> Correcta</span>}
                  {est === 'mal' && <span class="rdc-opcion-marca"><Marca /> No</span>}
                </button>
              </li>
            );
          })}
        </ul>
      )}
      <div class="rdc-feedback" aria-live="polite">
        {r && (
          <div class={r.ok ? 'rdc-feedback-ok' : 'rdc-feedback-mal'}>
            <p class="rdc-feedback-titulo"><Marca ok={r.ok} /> {r.ok ? 'Correcto.' : it.tipo === 'respuesta' ? `No: era «${it.respuestas[0]}».` : `No: era la ${LETRAS[orden.indexOf(it.correcta)]}.`} <a href={it.href} target="_blank" rel="noopener">Ver la solución ↗</a></p>
          </div>
        )}
      </div>
      {r && <div class="rdc-barra"><button type="button" class="rdc-boton" onClick={siguiente} ref={(e) => (foco.current = e)}>{i + 1 >= items.length ? 'Ver resultado' : 'Siguiente'}</button></div>}
    </div>
  );
}

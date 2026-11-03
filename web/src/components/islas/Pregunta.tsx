import { useEffect, useMemo, useRef, useState } from 'preact/hooks';
import { hashTexto, mdEnLinea, ordenAleatorio } from './md';
import { claveActividad, registrarActividad, registrarPregunta } from '../../runtime/progreso';

type Props = {
  pregunta?: string;
  opciones: string[];
  correcta: number;
  explicacion?: string;
  porque?: string[];
  /** 'predice': una sola respuesta, después se desbloquea Ejecutar */
  modo?: 'pregunta' | 'predice';
  /** Pregunta de la biblioteca (/preguntas/<id>/): guarda tu tasa de acierto y avisa a la página */
  idPregunta?: string;
};

const LETRAS = 'ABCDEFGHIJ';

export default function Pregunta({ pregunta, opciones, correcta, explicacion, porque = [], modo = 'pregunta', idPregunta }: Props) {
  const [falladas, setFalladas] = useState<number[]>([]);
  const [elegida, setElegida] = useState<number | null>(null);
  const [acertada, setAcertada] = useState(false);
  const raiz = useRef<HTMLDivElement>(null);
  const idEnunciado = 'p-' + hashTexto((pregunta ?? '') + opciones.join('|'));
  const clave = useMemo(() => (typeof location === 'undefined' ? '' : claveActividad(modo, (pregunta ?? '') + opciones.join('|'))), []);
  // Las opciones se barajan en cada carga, ya en el navegador (el HTML del servidor no puede ser aleatorio
  // sin descuadrar la hidratación): hasta entonces la lista está oculta.
  const [orden, setOrden] = useState<number[] | null>(null);
  useEffect(() => setOrden(ordenAleatorio(opciones.length)), []);
  const posiciones = orden ?? opciones.map((_, k) => k);
  const predice = modo === 'predice';
  const cerrada = acertada || (predice && elegida !== null);

  function elegir(i: number) {
    if (cerrada || falladas.includes(i)) return;
    setElegida(i);
    const ok = i === correcta;
    try { registrarActividad(clave, ok); if (idPregunta) registrarPregunta(idPregunta, ok); } catch {}
    raiz.current?.dispatchEvent(new CustomEvent('rdc-respondida', { bubbles: true, detail: { ok, id: idPregunta } }));
    if (ok) setAcertada(true);
    else setFalladas((f) => [...f, i]);
    if (predice) {
      const ej = raiz.current?.closest('[data-ejecutable]');
      ej?.dispatchEvent(new CustomEvent('rdc-desbloquear'));
    }
  }

  const estadoDe = (i: number) =>
    (acertada || (predice && elegida !== null)) && i === correcta ? 'ok' : falladas.includes(i) ? 'mal' : '';

  return (
    <div class="rdc-quiz not-content" ref={raiz} data-estado={acertada ? 'ok' : falladas.length ? 'mal' : ''}>
      <p class="rdc-quiz-tipo">{predice ? 'Predice antes de ejecutar' : 'Pregunta'}</p>
      <p class="rdc-quiz-enunciado" id={idEnunciado} dangerouslySetInnerHTML={{ __html: mdEnLinea(pregunta ?? '¿Qué imprime este programa?') }} />
      <ul class="rdc-opciones" role="list" aria-labelledby={idEnunciado} data-barajando={orden ? undefined : ''}>
        {posiciones.map((i, pos) => {
          const o = opciones[i];
          const est = estadoDe(i);
          return (
            <li key={i}>
              <button
                type="button"
                class="rdc-opcion"
                data-opcion={i}
                data-resultado={est || undefined}
                aria-pressed={elegida === i || est === 'mal'}
                aria-disabled={cerrada || est === 'mal' ? 'true' : undefined}
                onClick={() => elegir(i)}
              >
                <span class="rdc-opcion-letra" aria-hidden="true">{LETRAS[pos]}</span>
                <span class="rdc-opcion-texto" dangerouslySetInnerHTML={{ __html: mdEnLinea(o) }} />
                {est === 'ok' && <span class="rdc-opcion-marca"><Marca ok /> Correcta</span>}
                {est === 'mal' && <span class="rdc-opcion-marca"><Marca /> No</span>}
              </button>
              {est === 'mal' && porque[i] && (
                <p class="rdc-porque" dangerouslySetInnerHTML={{ __html: mdEnLinea(porque[i]) }} />
              )}
            </li>
          );
        })}
      </ul>
      <div class="rdc-feedback" aria-live="polite">
        {!cerrada && falladas.length > 0 && !predice && <p>No pasa nada: esta confunde a mucha gente. Lee el porqué y prueba otra opción.</p>}
        {cerrada && (
          <div class={acertada ? 'rdc-feedback-ok' : 'rdc-feedback-mal'}>
            <p class="rdc-feedback-titulo">
              {acertada
                ? falladas.length === 0 ? 'Correcto a la primera.' : 'Correcto.'
                : `La respuesta correcta es la ${LETRAS[posiciones.indexOf(correcta)]}.`}
              {predice && ' Ahora ejecútalo y compruébalo.'}
            </p>
            {explicacion && <p dangerouslySetInnerHTML={{ __html: mdEnLinea(explicacion) }} />}
          </div>
        )}
      </div>
    </div>
  );
}

export function Marca({ ok }: { ok?: boolean }) {
  return ok ? (
    <svg viewBox="0 0 24 24" width="18" height="18" aria-hidden="true"><path fill="currentColor" d="M9.5 16.2 5.3 12l-1.4 1.4 5.6 5.6L20.1 8.4 18.7 7z" /></svg>
  ) : (
    <svg viewBox="0 0 24 24" width="18" height="18" aria-hidden="true"><path fill="currentColor" d="M18.3 7.1 16.9 5.7 12 10.6 7.1 5.7 5.7 7.1l4.9 4.9-4.9 4.9 1.4 1.4 4.9-4.9 4.9 4.9 1.4-1.4-4.9-4.9z" /></svg>
  );
}

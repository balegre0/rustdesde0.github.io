import { useMemo, useState } from 'preact/hooks';
import { hashTexto, mdEnLinea } from './md';
import { Marca } from './Pregunta';
import { claveActividad, registrarActividad } from '../../runtime/progreso';

type Props = { lineas: string[]; distractores?: string[]; explicacion?: string; enunciado?: string };
type Pieza = { id: number; texto: string; distractor: boolean };

/** Barajado determinista (mismo orden en servidor y cliente). */
function barajar(arr: Pieza[], semilla: string): Pieza[] {
  let s = parseInt(hashTexto(semilla), 36) || 1;
  const rnd = () => (s = (Math.imul(s, 1664525) + 1013904223) >>> 0) / 2 ** 32;
  const a = [...arr];
  for (let i = a.length - 1; i > 0; i--) {
    const j = Math.floor(rnd() * (i + 1));
    [a[i], a[j]] = [a[j], a[i]];
  }
  if (a.length > 1 && a.every((x, i) => x.id === i)) a.push(a.shift()!); // que no salga ya ordenado
  return a;
}

export default function Ordena({ lineas, distractores = [], explicacion, enunciado }: Props) {
  const piezas = useMemo(
    () =>
      barajar(
        [
          ...lineas.map((t, i) => ({ id: i, texto: t, distractor: false })),
          ...distractores.map((t, i) => ({ id: lineas.length + i, texto: t, distractor: true })),
        ],
        lineas.join('\n'),
      ),
    [],
  );
  const [elegidas, setElegidas] = useState<number[]>([]);
  const [resultado, setResultado] = useState<null | { ok: boolean; primeraMal: number; motivo: string }>(null);
  const [intentos, setIntentos] = useState(0);
  const clave = useMemo(() => (typeof location === 'undefined' ? '' : claveActividad('ordena', lineas.join('\n'))), []);
  const h = hashTexto(lineas.join('\n'));
  const porId = (id: number) => piezas.find((p) => p.id === id)!;
  const disponibles = piezas.filter((p) => !elegidas.includes(p.id));
  const resuelto = resultado?.ok === true;

  const anadir = (id: number) => { if (!resuelto) { setElegidas((e) => [...e, id]); setResultado(null); } };
  const quitar = (id: number) => { if (!resuelto) { setElegidas((e) => e.filter((x) => x !== id)); setResultado(null); } };
  const mover = (idx: number, d: number) => {
    if (resuelto) return;
    setElegidas((e) => {
      const a = [...e];
      const j = idx + d;
      if (j < 0 || j >= a.length) return a;
      [a[idx], a[j]] = [a[j], a[idx]];
      return a;
    });
    setResultado(null);
  };

  function comprobar() {
    const textos = elegidas.map((id) => porId(id).texto);
    let primeraMal = textos.findIndex((t, i) => t !== lineas[i]);
    let motivo = '';
    if (elegidas.some((id) => porId(id).distractor)) {
      motivo = 'Has usado una línea que sobra: hay líneas trampa que no forman parte del programa.';
      primeraMal = elegidas.findIndex((id) => porId(id).distractor);
    } else if (primeraMal === -1 && textos.length < lineas.length) {
      primeraMal = textos.length;
      motivo = `Faltan líneas: llevas ${textos.length} de ${lineas.length}.`;
    } else if (primeraMal >= 0) motivo = `La línea ${primeraMal + 1} no va ahí. Las anteriores están bien.`;
    const ok = !motivo;
    setIntentos((n) => n + 1);
    try { registrarActividad(clave, ok); } catch { /* sin almacenamiento */ }
    setResultado({ ok, primeraMal, motivo });
  }

  return (
    <div class="rdc-quiz rdc-ordena not-content" data-estado={resuelto ? 'ok' : resultado ? 'mal' : ''}>
      <p class="rdc-quiz-tipo">Ordena las líneas</p>
      <p
        class="rdc-quiz-enunciado"
        dangerouslySetInnerHTML={{
          __html: mdEnLinea(enunciado ?? `Monta el programa pulsando las líneas en el orden correcto.${distractores.length ? ' Ojo: alguna línea sobra.' : ''}`),
        }}
      />
      <div class="rdc-ordena-zonas">
        <div class="rdc-ordena-zona">
          <p class="rdc-ordena-titulo" id={'disp-' + h}>Líneas disponibles</p>
          <ul role="list" aria-labelledby={'disp-' + h}>
            {disponibles.map((p) => (
              <li key={p.id}>
                <button type="button" class="rdc-linea" onClick={() => anadir(p.id)} disabled={resuelto} aria-label={`Añadir: ${p.texto.trim()}`}>
                  <code>{p.texto}</code>
                </button>
              </li>
            ))}
            {disponibles.length === 0 && <li class="rdc-ordena-vacio">No quedan líneas.</li>}
          </ul>
        </div>
        <div class="rdc-ordena-zona">
          <p class="rdc-ordena-titulo" id={'prog-' + h}>Tu programa</p>
          <ol aria-labelledby={'prog-' + h}>
            {elegidas.map((id, i) => (
              <li key={id} class={resultado && !resultado.ok && i === resultado.primeraMal ? 'rdc-linea-mal' : ''}>
                <code class="rdc-linea-texto">{porId(id).texto}</code>
                {!resuelto && (
                  <span class="rdc-linea-acciones">
                    <button type="button" class="rdc-mini" onClick={() => mover(i, -1)} disabled={i === 0} aria-label={`Subir la línea ${i + 1}`}>↑</button>
                    <button type="button" class="rdc-mini" onClick={() => mover(i, 1)} disabled={i === elegidas.length - 1} aria-label={`Bajar la línea ${i + 1}`}>↓</button>
                    <button type="button" class="rdc-mini" onClick={() => quitar(id)} aria-label={`Quitar la línea ${i + 1}`}>×</button>
                  </span>
                )}
              </li>
            ))}
            {elegidas.length === 0 && <li class="rdc-ordena-vacio">Pulsa una línea disponible para empezar.</li>}
          </ol>
        </div>
      </div>
      <div class="rdc-barra">
        <button type="button" class="rdc-boton" onClick={comprobar} disabled={resuelto || elegidas.length === 0}>Comprobar</button>
        {!resuelto && elegidas.length > 0 && (
          <button type="button" class="rdc-boton sutil" onClick={() => { setElegidas([]); setResultado(null); }}>Empezar de nuevo</button>
        )}
      </div>
      <div class="rdc-feedback" aria-live="polite">
        {resultado?.ok && (
          <div class="rdc-feedback-ok">
            <p class="rdc-feedback-titulo"><Marca ok /> Programa montado{intentos === 1 ? ' a la primera' : ''}.</p>
            {explicacion && <p dangerouslySetInnerHTML={{ __html: mdEnLinea(explicacion) }} />}
          </div>
        )}
        {resultado && !resultado.ok && (
          <div class="rdc-feedback-mal"><p class="rdc-feedback-titulo"><Marca /> Casi. {resultado.motivo}</p></div>
        )}
      </div>
    </div>
  );
}

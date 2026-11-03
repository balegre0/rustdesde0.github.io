import { useMemo, useRef, useState } from 'preact/hooks';
import { hashTexto, mdEnLinea } from './md';
import { Marca } from './Pregunta';
import { claveActividad, registrarActividad, registrarPregunta } from '../../runtime/progreso';

type Props = { pregunta: string; respuestas: string[]; explicacion?: string; idPregunta?: string };

const normalizar = (s: string) => s.trim().toLowerCase();

export default function Respuesta({ pregunta, respuestas, explicacion, idPregunta }: Props) {
  const raiz = useRef<HTMLDivElement>(null);
  const [valor, setValor] = useState('');
  const [fallos, setFallos] = useState(0);
  const [ok, setOk] = useState(false);
  const [revelada, setRevelada] = useState(false);
  const [ultimo, setUltimo] = useState<string | null>(null);
  const id = 'r-' + hashTexto(pregunta + respuestas.join('|'));
  const clave = useMemo(() => (typeof location === 'undefined' ? '' : claveActividad('respuesta', pregunta)), []);

  function comprobar(e: Event) {
    e.preventDefault();
    if (ok || !valor.trim()) return;
    const bien = respuestas.some((r) => normalizar(r) === normalizar(valor));
    try { registrarActividad(clave, bien); if (idPregunta) registrarPregunta(idPregunta, bien); } catch { /* sin almacenamiento */ }
    raiz.current?.dispatchEvent(new CustomEvent('rdc-respondida', { bubbles: true, detail: { ok: bien, id: idPregunta } }));
    setUltimo(valor);
    if (bien) setOk(true);
    else setFallos((f) => f + 1);
  }

  return (
    <div class="rdc-quiz not-content" ref={raiz} data-estado={ok ? 'ok' : fallos ? 'mal' : ''}>
      <p class="rdc-quiz-tipo">Responde</p>
      <form onSubmit={comprobar}>
        <label class="rdc-quiz-enunciado" for={id} dangerouslySetInnerHTML={{ __html: mdEnLinea(pregunta) }} />
        <div class="rdc-fila">
          <input
            id={id}
            type="text"
            class="rdc-respuesta-input"
            value={valor}
            onInput={(e) => setValor((e.target as HTMLInputElement).value)}
            readOnly={ok}
            autocomplete="off"
            spellcheck={false}
            autocapitalize="off"
            aria-describedby={id + '-fb'}
          />
          <button type="submit" class="rdc-boton" disabled={ok}>Comprobar</button>
        </div>
      </form>
      <div class="rdc-feedback" id={id + '-fb'} aria-live="polite">
        {ok && (
          <div class="rdc-feedback-ok">
            <p class="rdc-feedback-titulo"><Marca ok /> Correcto{fallos === 0 ? ' a la primera' : ''}.</p>
            {explicacion && <p dangerouslySetInnerHTML={{ __html: mdEnLinea(explicacion) }} />}
          </div>
        )}
        {!ok && fallos > 0 && (
          <div class="rdc-feedback-mal">
            <p class="rdc-feedback-titulo"><Marca /> «{ultimo}» no es lo que buscamos. Vuelve a mirar con calma.</p>
            {fallos >= 2 && !revelada && (
              <button type="button" class="rdc-boton secundario pequeno" onClick={() => setRevelada(true)}>Ver la respuesta</button>
            )}
            {revelada && (
              <>
                <p>Respuesta: <code>{respuestas[0]}</code></p>
                {explicacion && <p dangerouslySetInnerHTML={{ __html: mdEnLinea(explicacion) }} />}
              </>
            )}
          </div>
        )}
      </div>
    </div>
  );
}

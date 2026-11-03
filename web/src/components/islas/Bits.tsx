import { useMemo, useState } from 'preact/hooks';
import { Marca } from './Pregunta';
import { claveActividad, registrarActividad } from '../../runtime/progreso';

type Props = { objetivo?: number; bits?: number };

export default function Bits({ objetivo, bits = 8 }: Props) {
  const n = Math.max(1, Math.min(16, bits));
  const [valor, setValor] = useState(0);
  const [registrado, setRegistrado] = useState(false);
  const clave = useMemo(() => (typeof location === 'undefined' ? '' : claveActividad('bits', `${objetivo}/${n}`)), []);
  const pesos = Array.from({ length: n }, (_, i) => 2 ** (n - 1 - i));
  const conSigno = valor >= 2 ** (n - 1) ? valor - 2 ** n : valor;
  const hecho = objetivo !== undefined && valor === objetivo;

  function alternar(peso: number) {
    const nuevo = valor ^ peso;
    setValor(nuevo);
    if (objetivo !== undefined && nuevo === objetivo && !registrado) {
      setRegistrado(true);
      try { registrarActividad(clave, true); } catch { /* sin almacenamiento */ }
    }
  }

  const bin = valor.toString(2).padStart(n, '0');
  const hex = valor.toString(16).toUpperCase().padStart(Math.ceil(n / 4), '0');
  const suma = pesos.filter((p) => (valor & p) !== 0);

  return (
    <div class="rdc-quiz rdc-bits not-content" data-estado={hecho ? 'ok' : ''}>
      <p class="rdc-quiz-tipo">Interruptores de bits</p>
      {objetivo !== undefined ? (
        <p class="rdc-quiz-enunciado">Enciende los bits necesarios para formar el número <strong>{objetivo}</strong>.</p>
      ) : (
        <p class="rdc-quiz-enunciado">Enciende y apaga bits y mira cómo cambia el número.</p>
      )}
      <div class="rdc-bits-fila" style={{ gridTemplateColumns: `repeat(${n}, minmax(0, 1fr))` }} role="group" aria-label={`${n} bits, del más significativo al menos significativo`}>
        {pesos.map((p, i) => {
          const on = (valor & p) !== 0;
          return (
            <button
              key={p}
              type="button"
              class="rdc-bit"
              aria-pressed={on}
              aria-label={`Bit ${n - 1 - i}, vale ${p}`}
              onClick={() => alternar(p)}
            >
              <span class="rdc-bit-valor" aria-hidden="true">{on ? 1 : 0}</span>
              <span class="rdc-bit-peso" aria-hidden="true">{p}</span>
            </button>
          );
        })}
      </div>
      <dl class="rdc-bits-lectura" aria-live="polite">
        <div><dt>Decimal</dt><dd>{valor}</dd></div>
        <div><dt>Hexadecimal</dt><dd>0x{hex}</dd></div>
        <div><dt>Binario</dt><dd>0b{bin}</dd></div>
        <div><dt>{n === 8 ? 'Como i8' : n === 16 ? 'Como i16' : `Con signo (${n} bits)`}</dt><dd>{conSigno}</dd></div>
      </dl>
      {hecho ? (
        <div class="rdc-feedback">
          <div class="rdc-feedback-ok">
            <p class="rdc-feedback-titulo"><Marca ok /> Eso es {objetivo}: {suma.length ? suma.join(' + ') : '0'} = {objetivo}</p>
          </div>
        </div>
      ) : (
        <div class="rdc-barra">
          <button type="button" class="rdc-boton sutil pequeno" onClick={() => setValor(0)} disabled={valor === 0}>Apagar todos</button>
        </div>
      )}
    </div>
  );
}

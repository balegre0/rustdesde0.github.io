// Comportamiento de <Reto>: editor (CodeMirror, perezoso), ejecutar en el Rust Playground, comparar salida.
import { get, set } from 'idb-keyval';
import { Consola } from './consola';
import { claveActividad, registrarActividad } from './progreso';

type Datos = { id: string; inicio: string; esperado: string | null; solucion: string };

export const INTENTOS_PARA_SOLUCION = 3;

/** Compara salidas ignorando espacios al final de cada línea y saltos de línea finales. */
export const normalizarSalida = (s: string) => s.replace(/\r\n/g, '\n').split('\n').map((l) => l.replace(/\s+$/, '')).join('\n').replace(/\n+$/, '');

export const limpiarCargo = (s: string) => s.replace(/^\s*(Compiling|Finished|Running)[^\n]*\n?/gm, '').trim();

/** Traduce un pánico del Playground a un título y una explicación para quien aprende.
 * `lineasTuyas`: cuántas líneas del programa enviado son del alumno (las pruebas van detrás). */
export function explicarPanico(stderr: string, lineasTuyas: number): { titulo: string; nota: string } | null {
  const m = /panicked at src\/main\.rs:(\d+):\d+:\n([^\n]*)/.exec(stderr);
  if (!m) return null;
  const linea = Number(m[1]);
  const mensaje = m[2];
  const tuya = linea <= lineasTuyas;
  if (/not yet implemented/.test(mensaje) && tuya)
    return {
      titulo: '✗ Todavía falta tu código',
      nota: `En la línea ${linea} sigue el todo!() del código inicial. todo!() significa «aquí falta código»: al ejecutarse, detiene el programa a propósito con «not yet implemented». Sustitúyelo por tu solución y vuelve a probar. (Este intento no cuenta.)`,
    };
  if (tuya) return { titulo: `✗ Tu código entró en pánico en la línea ${linea}`, nota: `Mensaje del pánico: ${mensaje}` };
  return { titulo: '✗ El programa entró en pánico durante las pruebas', nota: `Mensaje del pánico: ${mensaje}. Suele pasar al indexar fuera de rango, al hacer unwrap de un None o un Err, o al desbordar un entero en tu código.` };
}

export function iniciarRetos() {
  const pendientes = [...document.querySelectorAll<HTMLElement>('[data-reto]:not([data-listo])')];
  if (!pendientes.length) return;
  const io = new IntersectionObserver((entradas) => {
    for (const en of entradas) if (en.isIntersecting) { io.unobserve(en.target); iniciar(en.target as HTMLElement); }
  }, { rootMargin: '400px' });
  for (const el of pendientes) { el.dataset.listo = 'true'; io.observe(el); }
}

const crear = (tag: string, props: Record<string, string>) => Object.assign(document.createElement(tag), props);

async function iniciar(el: HTMLElement) {
  const d: Datos = JSON.parse(el.querySelector('[data-datos]')!.textContent!);
  const claveCodigo = `reto:${d.id}`;
  const clave = claveActividad('reto', d.id);
  const consola = new Consola(el.querySelector<HTMLElement>('[data-consola]')!);
  const btn = el.querySelector<HTMLButtonElement>('[data-accion="comprobar"]')!;
  const btnReiniciar = el.querySelector<HTMLButtonElement>('[data-accion="reiniciar"]')!;
  const compilacion = el.querySelector<HTMLElement>('[data-compilacion]')!;
  const veredicto = el.querySelector<HTMLElement>('[data-veredicto]')!;
  const solucion = el.querySelector<HTMLDetailsElement>('[data-solucion]')!;
  const intentosEl = el.querySelector<HTMLElement>('[data-intentos]')!;
  const zona = el.querySelector<HTMLElement>('[data-editor-reto]')!;

  let guardado: string | undefined;
  try { guardado = await get(claveCodigo); } catch { /* sin IDB */ }
  const { crearEditor } = await import('./editor');
  zona.querySelector('[data-previo]')?.remove();
  let t: ReturnType<typeof setTimeout> | undefined;
  const editor = crearEditor(zona, guardado ?? d.inicio, (s) => {
    clearTimeout(t);
    t = setTimeout(() => set(claveCodigo, s).catch(() => {}), 400);
  }, `Editor del reto ${d.id}`);
  el.dataset.editorListo = 'true';

  let fallos = 0;
  const actualizarIntentos = () => {
    intentosEl.textContent = fallos ? `${fallos} ${fallos === 1 ? 'intento' : 'intentos'}` : '';
    if (fallos >= INTENTOS_PARA_SOLUCION) solucion.hidden = false;
  };

  btnReiniciar.addEventListener('click', () => {
    editor.poner(d.inicio);
    set(claveCodigo, d.inicio).catch(() => {});
    editor.enfocar();
  });

  function juzgar(salida: string, esperado: string) {
    veredicto.replaceChildren();
    const ok = normalizarSalida(salida) === normalizarSalida(esperado);
    try { registrarActividad(clave, ok); } catch { /* sin almacenamiento */ }
    if (ok) {
      el.dataset.estado = 'ok';
      veredicto.className = 'rdc-veredicto rdc-feedback-ok';
      veredicto.innerHTML = '<p class="rdc-feedback-titulo"><svg viewBox="0 0 24 24" width="18" height="18" aria-hidden="true"><path fill="currentColor" d="M9.5 16.2 5.3 12l-1.4 1.4 5.6 5.6L20.1 8.4 18.7 7z"/></svg> Reto superado.</p>';
      veredicto.append(crear('p', { textContent: 'Tu programa produce exactamente la salida esperada.' }));
      solucion.hidden = false;
      solucion.querySelector('summary')!.textContent = 'Comparar con la solución propuesta';
    } else {
      fallos++;
      el.dataset.estado = 'mal';
      veredicto.className = 'rdc-veredicto rdc-feedback-mal';
      const tabla = crear('div', { className: 'rdc-comparar' });
      for (const [titulo, texto] of [['Tu salida', salida], ['Salida esperada', esperado]] as const) {
        const col = document.createElement('div');
        col.append(crear('p', { className: 'rdc-comparar-titulo', textContent: titulo }), crear('pre', { textContent: texto || '(nada)' }));
        tabla.append(col);
      }
      veredicto.append(crear('p', { className: 'rdc-feedback-titulo', textContent: 'Todavía no: la salida no coincide con la esperada. Compáralas con calma; a menudo es un detalle (un espacio, un salto de línea).' }), tabla);
      if (fallos >= INTENTOS_PARA_SOLUCION) veredicto.append(crear('p', { textContent: 'Si te has atascado, abre «Ver solución» más abajo: leerla también enseña.' }));
    }
    actualizarIntentos();
  }

  btn.addEventListener('click', async () => {
    btn.disabled = true;
    veredicto.replaceChildren();
    compilacion.hidden = false;
    compilacion.className = 'rdc-compilacion';
    compilacion.textContent = 'Enviando a play.rust-lang.org…';
    try {
      const { ejecutarEnPlayground } = await import('./playground');
      let r;
      try {
        r = await ejecutarEnPlayground(editor.obtener());
      } catch (err) {
        compilacion.className = 'rdc-compilacion error';
        compilacion.textContent = `No se pudo usar el Rust Playground: ${(err as Error).message}. Tu código sigue guardado aquí; inténtalo cuando tengas conexión. (Este intento no cuenta.)`;
        consola.raiz.hidden = true;
        return;
      }
      if (/(^|\n)error(\[E\d+\])?:/.test(r.compilacion)) {
        fallos++;
        actualizarIntentos();
        consola.raiz.hidden = true;
        compilacion.className = 'rdc-compilacion error';
        compilacion.replaceChildren(
          crear('p', { textContent: 'No compila. Lee el primer error: indica la línea (src/main.rs:LÍNEA:COLUMNA) y qué esperaba el compilador.' }),
          crear('pre', { textContent: limpiarCargo(r.compilacion) }),
        );
        return;
      }
      const avisos = /(^|\n)warning:/.test(r.compilacion);
      compilacion.className = avisos ? 'rdc-compilacion aviso' : 'rdc-compilacion ok';
      if (avisos) compilacion.replaceChildren(crear('p', { textContent: 'Compila, pero rustc te avisa de algo. Merece la pena leerlo:' }), crear('pre', { textContent: limpiarCargo(r.compilacion) }));
      else compilacion.textContent = 'Compilado y ejecutado en play.rust-lang.org (Rust estable, edición 2024).';
      const code = /status (\d+)/.exec(r.detalle)?.[1];
      consola.mostrarPrecalculado(r.salida, r.error, '', code ? Number(code) : r.ok ? 0 : null, null, { nota: null }, '');
      requestAnimationFrame(() => consola.setEstado('Playground'));
      if (d.esperado === null) {
        // La solución no se puede ejecutar en el navegador (hilos…): se ejecuta una vez en el Playground.
        try {
          const s = await ejecutarEnPlayground(d.solucion);
          d.esperado = s.salida;
        } catch (err) {
          compilacion.className = 'rdc-compilacion error';
          compilacion.textContent = `Tu programa se ejecutó, pero no se pudo ejecutar la solución para comparar: ${(err as Error).message}. (Este intento no cuenta.)`;
          return;
        }
      }
      juzgar(r.salida, d.esperado);
      const p = explicarPanico(r.error, Number.MAX_SAFE_INTEGER);
      if (p && el.dataset.estado === 'mal') veredicto.prepend(crear('p', { className: 'rdc-feedback-titulo', textContent: `${p.titulo}. ${p.nota}` }));
    } finally {
      btn.disabled = false;
    }
  });
}

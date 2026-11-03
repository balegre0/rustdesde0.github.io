// /problemas/<slug>/: editor (CodeMirror + Rust), ejecutar código del alumno + pruebas.rs en el Playground,
// mostrar los casos «OK: …» / «FALLO: …» y el veredicto. Superado = aparece «TODOS LOS CASOS SUPERADOS».
import { get, set } from 'idb-keyval';
import { leer, registrarProblema, EVENTO } from './progreso';
import { INTENTOS_PARA_SOLUCION, explicarPanico, limpiarCargo } from './reto-cliente';

export const MARCA_OK = 'TODOS LOS CASOS SUPERADOS';
type Datos = { slug: string; inicio: string; pruebas: string };

const crear = (tag: string, props: Record<string, string> = {}) => Object.assign(document.createElement(tag), props);

export async function iniciarProblema() {
  const el = document.querySelector<HTMLElement>('[data-problema]');
  if (!el) return;
  const d: Datos = JSON.parse(document.querySelector('[data-datos-problema]')!.textContent!);
  const q = <T extends HTMLElement>(s: string) => el.querySelector<T>(s)!;
  const btn = q<HTMLButtonElement>('[data-accion="ejecutar"]');
  const btnReiniciar = q<HTMLButtonElement>('[data-accion="reiniciar"]');
  const resultado = q<HTMLElement>('[data-resultado]');
  const titulo = q<HTMLElement>('[data-veredicto-titulo]');
  const resumen = q<HTMLElement>('[data-resumen-casos]');
  const casos = q<HTMLElement>('[data-casos]');
  const salida = q<HTMLElement>('[data-salida]');
  const solucion = q<HTMLDetailsElement>('[data-solucion]');
  const intentosEl = q<HTMLElement>('[data-intentos]');
  const zona = q<HTMLElement>('[data-editor-problema]');
  const clave = `problema:${d.slug}`;

  const pintarEstado = () => {
    const r = leer().problemas[d.slug];
    const est = document.querySelector<HTMLElement>('[data-mi-estado]');
    if (!est) return;
    est.hidden = !r;
    if (r) { est.dataset.estado = r.ok ? 'ok' : 'mal'; est.textContent = r.ok ? '✓ Resuelto' : `${r.intentos} ${r.intentos === 1 ? 'intento' : 'intentos'}`; }
  };
  pintarEstado();
  window.addEventListener(EVENTO, pintarEstado);

  let guardado: string | undefined;
  try { guardado = await get(clave); } catch { /* sin IDB */ }
  const { crearEditor } = await import('./editor');
  zona.querySelector('[data-previo]')?.remove();
  let t: ReturnType<typeof setTimeout> | undefined;
  const editor = crearEditor(zona, guardado ?? d.inicio, (s) => {
    clearTimeout(t);
    t = setTimeout(() => set(clave, s).catch(() => {}), 400);
  }, 'Editor de tu solución (Rust)');
  el.dataset.editorListo = 'true';
  editor.view.dom.addEventListener('keydown', (e) => {
    if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') { e.preventDefault(); btn.click(); }
  });

  let fallos = 0;
  const actualizar = () => {
    intentosEl.textContent = fallos ? `${fallos} ${fallos === 1 ? 'intento fallido' : 'intentos fallidos'}` : '';
    if (fallos >= INTENTOS_PARA_SOLUCION) solucion.hidden = false;
  };
  if ((leer().problemas[d.slug]?.ok)) solucion.hidden = false;

  btnReiniciar.addEventListener('click', () => {
    if (editor.obtener() !== d.inicio && !confirm('¿Volver al código inicial? Perderás lo que has escrito.')) return;
    editor.poner(d.inicio);
    set(clave, d.inicio).catch(() => {});
    editor.enfocar();
  });

  const nota = crear('p', { className: 'rdc-problema-nota' });
  nota.hidden = true;
  salida.before(nota);

  function veredicto(tipo: 'ok' | 'mal' | 'error' | 'red', texto: string) {
    resultado.hidden = false;
    resultado.dataset.estado = tipo;
    titulo.textContent = texto;
  }

  btn.addEventListener('click', async () => {
    btn.disabled = true;
    casos.replaceChildren();
    resumen.textContent = '';
    salida.textContent = '';
    salida.hidden = false;
    nota.hidden = true;
    veredicto('red', 'Enviando a play.rust-lang.org…');
    const codigo = editor.obtener();
    try {
      const { ejecutarEnPlayground } = await import('./playground');
      let r;
      try {
        r = await ejecutarEnPlayground(codigo + '\n\n// ---- pruebas.rs ----\n' + d.pruebas);
      } catch (err) {
        veredicto('red', 'No se pudo ejecutar');
        salida.textContent = `No se pudo usar el Rust Playground: ${(err as Error).message}.\nTu código está guardado en este navegador; vuelve a intentarlo cuando tengas conexión. (Este intento no cuenta.)`;
        return;
      }
      if (/(^|\n)error(\[E\d+\])?:/.test(r.compilacion)) {
        fallos++;
        registrarProblema(d.slug, false);
        veredicto('error', 'No compila');
        salida.textContent = limpiarCargo(r.compilacion);
        actualizar();
        return;
      }
      const lineas = r.salida.split('\n');
      let ok = 0, mal = 0;
      for (const l of lineas) {
        const m = l.match(/^(OK|FALLO):\s*(.*)$/);
        if (!m) continue;
        const li = crear('li');
        li.dataset.estado = m[1] === 'OK' ? 'ok' : 'mal';
        li.append(crear('span', { className: 'rdc-caso-marca', textContent: m[1] === 'OK' ? '✓' : '✗' }), crear('code', { textContent: m[2] }));
        li.prepend(crear('span', { className: 'rdc-sr', textContent: m[1] === 'OK' ? 'Superado: ' : 'Fallo: ' }));
        casos.append(li);
        m[1] === 'OK' ? ok++ : mal++;
      }
      const superado = r.salida.includes(MARCA_OK);
      registrarProblema(d.slug, superado);
      resumen.textContent = ok + mal ? `${ok} de ${ok + mal} casos` : '';
      const resto = [r.salida.split('\n').filter((l) => !/^(OK|FALLO):/.test(l)).join('\n').trim(), r.error.trim()].filter(Boolean).join('\n');
      salida.textContent = resto || '';
      salida.hidden = !resto;
      if (superado) {
        veredicto('ok', '✓ Todos los casos superados');
        solucion.hidden = false;
        solucion.querySelector('summary')!.textContent = 'Comparar con la solución propuesta';
      } else {
        const p = explicarPanico(r.error, codigo.split('\n').length);
        const falta = p?.titulo === '✗ Todavía falta tu código';
        if (!falta) fallos++;
        veredicto('mal', p?.titulo ?? '✗ Aún no: algún caso falla');
        if (p) { nota.textContent = p.nota; nota.hidden = false; }
        actualizar();
      }
    } finally {
      btn.disabled = false;
    }
  });
}

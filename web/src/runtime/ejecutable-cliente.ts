// Comportamiento de cada bloque [data-ejecutable] (componentes <Ejecutar>, <Predice> y «Ver arreglo» de <NoCompila>).
import { Consola, hayJSPI } from './consola';
import type { Herramienta } from './playground';

type Perfil = 'debug' | 'release';
type Ej = { wasm: string; salida: string; error: string | null; exit: number | null; trap: string | null };
type Datos = {
  id: string; codigo: string; stdin: string; nota: string | null; plataforma: 'wasm' | 'playground' | 'local';
  herramientas: Herramienta[]; perfiles: Partial<Record<Perfil, Ej | null>>;
};

export function iniciarEjecutables() {
  for (const el of document.querySelectorAll<HTMLElement>('[data-ejecutable]:not([data-listo])')) {
    el.dataset.listo = 'true';
    iniciar(el);
  }
}

const AYUDA: Record<Perfil, string> = {
  debug: 'Modo debug: comprueba el desbordamiento de enteros (overflow-checks) y los debug_assert!.',
  release: 'Modo release (--release): optimizado y sin comprobar el desbordamiento; el valor da la vuelta.',
};

function iniciar(el: HTMLElement) {
  const d: Datos = JSON.parse(el.querySelector(':scope > [data-datos]')!.textContent!);
  const consola = new Consola(el.querySelector<HTMLElement>(':scope > [data-consola]')!);
  const q = <T extends HTMLElement>(s: string) => el.querySelector<T>(s);
  const btnEjecutar = q<HTMLButtonElement>('[data-accion="ejecutar"]');
  const btnEditar = q<HTMLButtonElement>('[data-accion="editar"]');
  const btnRestaurar = q<HTMLButtonElement>('[data-accion="restaurar"]');
  const btnPG = q<HTMLButtonElement>('[data-accion="pg"]');
  const zonaCodigo = q<HTMLElement>('[data-codigo]')!;
  const zonaEditor = q<HTMLElement>('[data-editor]')!;
  const compilacion = q<HTMLElement>('[data-compilacion]')!;
  const avisoEdicion = q<HTMLElement>('[data-aviso-edicion]');
  const perfiles = Object.keys(d.perfiles) as Perfil[];
  let perfil: Perfil = perfiles[0] ?? 'debug';
  let editor: { obtener: () => string; poner: (s: string) => void; enfocar: () => void } | null = null;
  let ocupado = false;

  // Pestañas de perfil (debug / release)
  const radios = [...el.querySelectorAll<HTMLButtonElement>('[data-perfil]')];
  const elegirPerfil = (p: Perfil, foco = false) => {
    perfil = p;
    for (const r of radios) {
      const sel = r.dataset.perfil === p;
      r.setAttribute('aria-checked', String(sel));
      r.tabIndex = sel ? 0 : -1;
      if (sel && foco) r.focus();
    }
    const ay = q('[data-perfil-ayuda]');
    if (ay) ay.textContent = AYUDA[p];
  };
  radios.forEach((r, i) => {
    r.tabIndex = i === 0 ? 0 : -1;
    r.addEventListener('click', () => elegirPerfil(r.dataset.perfil as Perfil));
    r.addEventListener('keydown', (e) => {
      if (!['ArrowLeft', 'ArrowRight', 'ArrowUp', 'ArrowDown'].includes(e.key)) return;
      e.preventDefault();
      const j = (i + (e.key === 'ArrowLeft' || e.key === 'ArrowUp' ? -1 : 1) + radios.length) % radios.length;
      elegirPerfil(radios[j].dataset.perfil as Perfil, true);
    });
  });

  // <Predice> desbloquea el botón al responder
  el.addEventListener('rdc-desbloquear', () => {
    el.removeAttribute('data-bloqueado');
    if (btnEjecutar) btnEjecutar.disabled = false;
    if (btnEditar) btnEditar.hidden = false;
    q('[data-aviso-bloqueo]')?.remove();
  });

  const editado = () => !!editor && editor.obtener() !== d.codigo;

  btnEjecutar?.addEventListener('click', async () => {
    if (ocupado) return;
    ocupado = true;
    btnEjecutar.disabled = true;
    try {
      if (editor && editado()) await enPlayground(editor.obtener());
      else await correrOriginal();
    } finally {
      ocupado = false;
      btnEjecutar.disabled = false;
    }
  });

  async function correrOriginal() {
    compilacion.hidden = true;
    const x = d.perfiles[perfil];
    if (!x) return;
    consola.raiz.dataset.perfil = perfil;
    if (!hayJSPI) {
      consola.mostrarPrecalculado(
        x.salida ?? '', x.error ?? '', d.stdin, x.exit, x.trap, { nota: d.nota },
        'Tu navegador no admite la ejecución interactiva (JSPI). Esta es la salida real del programa, obtenida al compilar el curso' +
          (d.stdin ? ' con la entrada de arriba.' : '.'),
      );
      return;
    }
    await consola.correr({ url: x.wasm, interactivo: true }, { nota: d.nota });
    if (perfiles.length > 1) consola.setEstado(`terminado · ${perfil}`);
  }

  async function enPlayground(codigo: string) {
    const { ejecutarEnPlayground } = await import('./playground');
    compilacion.hidden = false;
    compilacion.className = 'rdc-compilacion';
    compilacion.textContent = 'Enviando a play.rust-lang.org…';
    consola.limpiar();
    consola.setEstado('compilando en el Playground…');
    try {
      const r = await ejecutarEnPlayground(codigo, perfil);
      mostrarPG(r);
    } catch (err) {
      compilacion.className = 'rdc-compilacion error';
      compilacion.textContent = `No se pudo usar el Rust Playground: ${(err as Error).message}. El ejemplo original sigue funcionando sin conexión («Volver al original»).`;
      consola.raiz.hidden = true;
    }
  }

  function mostrarPG(r: { ok: boolean; salida: string; error: string; compilacion: string; detalle: string }) {
    const errores = /(^|\n)error(\[E\d+\])?:/.test(r.compilacion);
    if (errores) {
      compilacion.className = 'rdc-compilacion error';
      compilacion.replaceChildren(
        Object.assign(document.createElement('p'), { textContent: 'No compila (le pasa a todo el mundo; lee el primer error, suele ser el importante):' }),
        Object.assign(document.createElement('pre'), { textContent: r.compilacion.replace(/^\s*(Compiling|Finished|Running)[^\n]*\n?/gm, '').trim() }),
      );
      consola.raiz.hidden = true;
      return;
    }
    const avisos = /(^|\n)warning:/.test(r.compilacion);
    compilacion.className = avisos ? 'rdc-compilacion aviso' : 'rdc-compilacion ok';
    if (avisos)
      compilacion.replaceChildren(
        Object.assign(document.createElement('p'), { textContent: 'Compila, pero rustc te avisa de algo:' }),
        Object.assign(document.createElement('pre'), { textContent: r.compilacion.replace(/^\s*(Compiling|Finished|Running)[^\n]*\n?/gm, '').trim() }),
      );
    else compilacion.textContent = `Compilado y ejecutado en play.rust-lang.org (Rust estable, edición 2024, ${perfil}).`;
    const code = /status (\d+)/.exec(r.detalle)?.[1];
    consola.mostrarPrecalculado(r.salida, r.error, '', code ? Number(code) : r.ok ? 0 : null, null, { nota: null }, '');
    requestAnimationFrame(() => consola.setEstado('Playground'));
  }

  btnPG?.addEventListener('click', async () => {
    if (ocupado) return;
    ocupado = true;
    btnPG.disabled = true;
    try { await enPlayground(d.codigo); } finally { ocupado = false; btnPG.disabled = false; }
  });

  btnEditar?.addEventListener('click', async () => {
    if (!editor) {
      const { crearEditor } = await import('./editor');
      zonaEditor.hidden = false;
      editor = crearEditor(zonaEditor, d.codigo, () => {
        const lbl = btnEjecutar?.querySelector('[data-etiqueta]');
        if (lbl) lbl.textContent = editado() ? 'Ejecutar en el Playground' : 'Ejecutar';
        if (avisoEdicion) avisoEdicion.hidden = !editado();
      }, `Editar ${d.id}.rs`);
    }
    zonaCodigo.hidden = true;
    zonaEditor.hidden = false;
    btnEditar.hidden = true;
    if (btnRestaurar) btnRestaurar.hidden = false;
    editor.enfocar();
  });

  btnRestaurar?.addEventListener('click', () => {
    editor?.poner(d.codigo);
    zonaEditor.hidden = true;
    zonaCodigo.hidden = false;
    btnRestaurar.hidden = true;
    if (btnEditar) btnEditar.hidden = false;
    if (avisoEdicion) avisoEdicion.hidden = true;
    compilacion.hidden = true;
    const lbl = btnEjecutar?.querySelector('[data-etiqueta]');
    if (lbl) lbl.textContent = 'Ejecutar';
    btnEditar?.focus();
  });

  // Herramientas opt-in del Playground (MIR, asm, Miri, Clippy)
  const salidaPG = q<HTMLElement>('[data-pg-salida]');
  const abrir = q<HTMLAnchorElement>('[data-abrir-pg]');
  if (abrir) import('./playground').then(({ enlacePlayground }) => { abrir.href = enlacePlayground(d.codigo); });
  for (const b of el.querySelectorAll<HTMLButtonElement>('[data-herramienta]')) {
    b.addEventListener('click', async () => {
      if (!salidaPG) return;
      const { herramienta, NOMBRES } = await import('./playground');
      const h = b.dataset.herramienta as Herramienta;
      salidaPG.hidden = false;
      salidaPG.className = 'rdc-pg-salida';
      salidaPG.textContent = `${NOMBRES[h]}: enviando a play.rust-lang.org…`;
      b.disabled = true;
      try {
        const r = await herramienta(h, editor?.obtener() ?? d.codigo);
        const texto = [r.salida, r.error, r.compilacion].map((s) => s.trim()).filter(Boolean).join('\n\n');
        salidaPG.textContent = `── ${NOMBRES[h]} ──\n` + (texto || '(sin salida)');
      } catch (err) {
        salidaPG.className = 'rdc-pg-salida error';
        salidaPG.textContent = `No se pudo usar el Rust Playground: ${(err as Error).message}.`;
      } finally {
        b.disabled = false;
      }
    });
  }
}

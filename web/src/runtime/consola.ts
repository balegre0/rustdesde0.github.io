// Controlador de la consola de salida (markup en src/components/interno/Consola.astro).
import { ejecutar, explicarPanico, explicarTrap, extraerPanico, hayJSPI, LIMITE_MS, type Opciones, type Resultado } from './ejecutor';

const MAX_CHARS = 1_000_000;

export class Consola {
  raiz: HTMLElement;
  private texto: HTMLElement;
  private estado: HTMLElement;
  private pie: HTMLElement;
  private form: HTMLFormElement;
  private input: HTMLInputElement;
  private eof: HTMLButtonElement;
  private detener: HTMLButtonElement;
  private panico: HTMLElement;
  private panicoMsg: HTMLElement;
  private panicoTexto: HTMLElement;
  private cola: { cls: string; t: string }[] = [];
  private programado = false;
  private escritos = 0;
  private truncado = false;
  private notaPrevia: HTMLElement | null = null;
  private ejecucion: ReturnType<typeof ejecutar> | null = null;

  constructor(raiz: HTMLElement) {
    this.raiz = raiz;
    const q = <T extends HTMLElement>(s: string) => raiz.querySelector<T>(s)!;
    this.texto = q('[data-texto]');
    this.estado = q('[data-estado]');
    this.pie = q('[data-pie]');
    this.form = q<HTMLFormElement>('[data-entrada]');
    this.input = q<HTMLInputElement>('[data-entrada] input');
    this.eof = q<HTMLButtonElement>('[data-eof]');
    this.detener = q<HTMLButtonElement>('[data-detener]');
    this.panico = q('[data-panico]');
    this.panicoMsg = q('[data-panico-msg]');
    this.panicoTexto = q('[data-panico-texto]');

    this.form.addEventListener('submit', (e) => {
      e.preventDefault();
      if (!this.ejecucion) return;
      const v = this.input.value + '\n';
      this.input.value = '';
      this.eco(v);
      this.ocultarEntrada();
      this.ejecucion.enviar(v);
    });
    this.input.addEventListener('keydown', (e) => {
      if ((e.ctrlKey || e.metaKey) && (e.key === 'd' || e.key === 'D')) { e.preventDefault(); this.terminarEntrada(); }
    });
    this.eof.addEventListener('click', () => this.terminarEntrada());
    this.detener.addEventListener('click', () => this.ejecucion?.cancelar());
  }

  private terminarEntrada() {
    if (!this.ejecucion) return;
    if (this.input.value) { const v = this.input.value; this.input.value = ''; this.eco(v); this.ejecucion.enviar(v); }
    this.eco('⌁ fin de la entrada\n', 'rdc-c-meta');
    this.ocultarEntrada();
    this.ejecucion.finEntrada();
  }

  limpiar() {
    this.raiz.hidden = false;
    this.texto.textContent = '';
    this.cola = [];
    this.escritos = 0;
    this.truncado = false;
    this.pie.textContent = '';
    this.pie.className = 'rdc-consola-pie';
    this.panico.hidden = true;
    this.notaPrevia?.remove();
    this.notaPrevia = null;
    this.raiz.querySelectorAll('[data-nota-extra]').forEach((n) => n.remove());
    this.ocultarEntrada();
    this.raiz.dataset.vacia = 'true';
    delete this.raiz.dataset.conPanico;
  }

  setEstado(t: string) { this.estado.textContent = t; }

  escribir(fd: 1 | 2, t: string) {
    // La nota de RUST_BACKTRACE no aplica en el navegador (no hay variables de entorno que poner)
    if (fd === 2) t = t.replace(/note: run with `RUST_BACKTRACE=1`[^\n]*\n?/g, '');
    if (t) this.encolar(fd === 2 ? 'rdc-c-err' : '', t);
  }
  eco(t: string, cls = 'rdc-c-entrada') { this.encolar(cls, t); }

  private encolar(cls: string, t: string) {
    if (this.truncado) return;
    if (this.escritos + t.length > MAX_CHARS) {
      t = t.slice(0, Math.max(0, MAX_CHARS - this.escritos));
      this.truncado = true;
    }
    this.escritos += t.length;
    this.cola.push({ cls, t });
    if (this.truncado) this.cola.push({ cls: 'rdc-c-meta', t: '\n… salida cortada: el programa escribió más de 1 MB.\n' });
    if (!this.programado) {
      this.programado = true;
      requestAnimationFrame(() => this.volcar());
    }
  }

  private volcar() {
    this.programado = false;
    const frag = document.createDocumentFragment();
    for (const { cls, t } of this.cola) {
      const ultimo = frag.lastChild as HTMLElement | null;
      if (ultimo && ultimo.nodeType === 1 && ultimo.className === cls) ultimo.textContent += t;
      else { const s = document.createElement('span'); if (cls) s.className = cls; s.textContent = t; frag.appendChild(s); }
    }
    this.cola = [];
    this.texto.appendChild(frag);
    this.raiz.dataset.vacia = this.texto.textContent ? 'false' : 'true';
    this.texto.scrollTop = this.texto.scrollHeight;
  }

  pedirEntrada() {
    this.form.hidden = false;
    this.setEstado('esperando tu entrada');
    this.input.focus({ preventScroll: true });
    this.form.scrollIntoView({ block: 'nearest' });
  }
  ocultarEntrada() { this.form.hidden = true; }

  /** Ejecuta y muestra el resultado. Devuelve el resultado (para comparar en los retos). */
  async correr(op: Omit<Opciones, 'onOut' | 'onNecesitaEntrada'>, extra: { nota?: string | null } = {}): Promise<Resultado> {
    this.limpiar();
    this.setEstado('ejecutando…');
    this.raiz.dataset.estado = 'corriendo';
    this.detener.hidden = false;
    if (!op.interactivo && op.stdin) this.eco(op.stdin.endsWith('\n') ? op.stdin : op.stdin + '\n');
    this.ejecucion = ejecutar({
      ...op,
      onOut: (fd, t) => this.escribir(fd, t),
      onNecesitaEntrada: () => this.pedirEntrada(),
    });
    const r = await this.ejecucion.terminado;
    this.ejecucion = null;
    this.detener.hidden = true;
    this.ocultarEntrada();
    await new Promise((res) => requestAnimationFrame(res));
    this.mostrarFin(r, extra);
    return r;
  }

  mostrarFin(r: Resultado, extra: { nota?: string | null } = {}) {
    this.raiz.dataset.estado = 'fin';
    const ms = r.ms < 10 ? r.ms.toFixed(1) : Math.round(r.ms).toString();
    const pan = extraerPanico(r.error);
    this.raiz.dataset.conPanico = pan ? 'true' : 'false';
    if (r.motivo === 'tiempo') {
      this.setEstado('detenido');
      this.pie.className = 'rdc-consola-pie aviso';
      this.pie.textContent = `Lo hemos detenido: llevaba más de ${LIMITE_MS / 1000} segundos ejecutándose. ¿Hay un bucle que nunca termina?`;
    } else if (r.motivo === 'cancelado') {
      this.setEstado('detenido');
      this.pie.textContent = 'Has detenido el programa.';
    } else if (pan) {
      this.setEstado('pánico');
      this.panico.hidden = false;
      this.panicoMsg.textContent = pan.mensaje;
      this.panicoTexto.textContent = `${explicarPanico(pan.mensaje)} (en ${pan.ubicacion})`;
      this.pie.className = 'rdc-consola-pie error';
      this.pie.textContent = `Terminó con un pánico · código de salida 101${r.ms ? ` · ${ms} ms` : ''}`;
    } else if (r.trap) {
      this.setEstado('error en ejecución');
      this.pie.className = 'rdc-consola-pie error';
      this.pie.textContent = `${explicarTrap(r.trap)} (${r.trap})`;
    } else {
      this.setEstado('terminado');
      this.pie.className = 'rdc-consola-pie' + (r.code === 0 ? '' : ' info');
      this.pie.textContent = `Terminó con código de salida ${r.code}${r.ms ? ` · ${ms} ms` : ''}`;
    }
    if (this.texto.textContent === '') {
      const s = document.createElement('span');
      s.className = 'rdc-c-meta';
      s.textContent = '(el programa no escribió nada)';
      this.texto.appendChild(s);
    }
    if (extra.nota) {
      const p = document.createElement('p');
      p.className = 'rdc-consola-nota';
      p.textContent = extra.nota;
      this.pie.after(p);
      this.notaPrevia = p;
    }
  }

  /** Muestra una salida ya conocida (precalculada en el build, o devuelta por el Playground). */
  mostrarPrecalculado(salida: string, error: string, stdin: string, exit: number | null, trap: string | null, extra: { nota?: string | null }, aviso: string) {
    this.limpiar();
    if (stdin) this.eco(stdin.endsWith('\n') ? stdin : stdin + '\n');
    this.escribir(1, salida);
    if (error) this.escribir(2, error);
    requestAnimationFrame(() => {
      this.mostrarFin({ code: exit, ms: 0, trap: trap ?? undefined, salida, error }, extra);
      this.setEstado('salida guardada');
      if (aviso) {
        const p = document.createElement('p');
        p.className = 'rdc-consola-nota';
        p.dataset.notaExtra = 'true';
        p.textContent = aviso;
        this.raiz.appendChild(p);
      }
    });
  }
}

export { hayJSPI };

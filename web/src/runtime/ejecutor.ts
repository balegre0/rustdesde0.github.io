// Lanza un programa wasm en el worker JSPI, con tiempo límite (que se pausa mientras espera tu entrada).
export const hayJSPI = typeof (globalThis as any).WebAssembly?.Suspending === 'function';
export const LIMITE_MS = 5000;

export type Resultado = {
  code: number | null;
  ms: number;
  trap?: string;
  motivo?: 'tiempo' | 'cancelado';
  salida: string;
  error: string;
};

export type Opciones = {
  url?: string;
  bytes?: ArrayBuffer | Uint8Array;
  /** Entrada fija (se usa si no es interactivo) */
  stdin?: string;
  interactivo: boolean;
  limiteMs?: number;
  onOut?: (fd: 1 | 2, text: string) => void;
  onNecesitaEntrada?: () => void;
};

export type Ejecucion = {
  terminado: Promise<Resultado>;
  enviar: (texto: string) => void;
  finEntrada: () => void;
  cancelar: () => void;
};

export function ejecutar(op: Opciones): Ejecucion {
  const w = new Worker(new URL('./wasi-worker.ts', import.meta.url), { type: 'module' });
  const limite = op.limiteMs ?? LIMITE_MS;
  let usado = 0, desde = performance.now(), timer: ReturnType<typeof setTimeout> | null = null;
  let salida = '', error = '';
  let resolver!: (r: Resultado) => void;
  let acabado = false;
  const terminado = new Promise<Resultado>((r) => (resolver = r));

  const acabar = (r: Omit<Resultado, 'salida' | 'error'>) => {
    if (acabado) return;
    acabado = true;
    if (timer) clearTimeout(timer);
    w.terminate();
    resolver({ ...r, salida, error });
  };
  const arrancarReloj = () => {
    desde = performance.now();
    if (timer) clearTimeout(timer);
    timer = setTimeout(() => acabar({ code: null, ms: usado + (performance.now() - desde), motivo: 'tiempo' }), Math.max(0, limite - usado));
  };
  const pararReloj = () => {
    if (timer) { clearTimeout(timer); timer = null; }
    usado += performance.now() - desde;
  };

  w.onmessage = (e) => {
    const m = e.data;
    if (m.type === 'out') {
      if (m.fd === 1) salida += m.text; else error += m.text;
      op.onOut?.(m.fd, m.text);
    } else if (m.type === 'need-input') {
      pararReloj();
      op.onNecesitaEntrada?.();
    } else if (m.type === 'exit') {
      acabar({ code: m.code, ms: m.ms, trap: m.trap });
    }
  };
  w.onerror = (e) => {
    e.preventDefault();
    acabar({ code: null, ms: 0, trap: e.message || 'No se pudo arrancar el programa' });
  };

  const bytes = op.bytes ? (op.bytes instanceof Uint8Array ? op.bytes : new Uint8Array(op.bytes)) : undefined;
  w.postMessage({ type: 'run', url: op.url ? new URL(op.url, location.href).href : undefined, bytes, stdin: op.stdin ?? '', interactivo: op.interactivo && hayJSPI });
  arrancarReloj();

  return {
    terminado,
    enviar: (texto) => { if (!acabado) { w.postMessage({ type: 'stdin', text: texto }); arrancarReloj(); } },
    finEntrada: () => { if (!acabado) { w.postMessage({ type: 'eof' }); arrancarReloj(); } },
    cancelar: () => acabar({ code: null, ms: usado, motivo: 'cancelado' }),
  };
}

export type Panico = { mensaje: string; ubicacion: string };

/** Extrae el pánico de Rust de stderr: «thread 'main' (1) panicked at src/main.rs:5:20:\n<mensaje>». */
export function extraerPanico(stderr: string): Panico | null {
  const m = stderr.match(/thread '[^']*'(?: \(\d+\))? panicked at ([^\n]+?):\n([\s\S]*?)(?:\nnote: run with `RUST_BACKTRACE|$)/);
  return m ? { ubicacion: m[1], mensaje: m[2].trim() } : null;
}

/** Explicación en español de un pánico (el mensaje real de Rust se muestra siempre). */
export function explicarPanico(mensaje: string): string {
  const t = mensaje.toLowerCase();
  if (/attempt to (add|subtract|multiply|negate|shift left|shift right) with overflow/.test(t))
    return 'Un entero se salió del rango de su tipo (desbordamiento). En modo debug Rust lo comprueba y para; en release, el valor da la vuelta.';
  if (t.includes('attempt to divide by zero') || t.includes('remainder with a divisor of zero')) return 'Se dividió un entero entre cero: Rust para el programa en vez de dar un resultado sin sentido.';
  if (t.includes('index out of bounds')) return 'Se usó una posición que no existe. Rust comprueba los límites siempre (también en release) y para antes de leer memoria ajena.';
  if (t.includes('option::unwrap()') && t.includes('none')) return 'Se llamó a unwrap() sobre un None: no había valor. Usa match, if let o ? para tratar el caso vacío.';
  if (t.includes('result::unwrap()') || t.includes('result::expect()')) return 'Se llamó a unwrap() sobre un Err: la operación falló y el programa no trató el error.';
  if (t.includes('failed to spawn thread')) return 'El navegador (wasm32-wasip1) no tiene hilos. Este ejemplo hay que ejecutarlo en el Playground o en tu ordenador.';
  if (t.includes('is not a char boundary')) return 'Se cortó un texto (str) por la mitad de un carácter UTF-8: los índices de un str son bytes, no letras.';
  if (t.includes('already borrowed') || t.includes('already mutably borrowed')) return 'Un RefCell detectó dos préstamos incompatibles al ejecutar (la regla del borrow checker, comprobada en tiempo de ejecución).';
  if (t.includes('not yet implemented')) return 'Se llegó a un todo!(): esa parte del código aún no está escrita.';
  if (t.includes('not implemented')) return 'Se llegó a un unimplemented!().';
  if (t.includes('entered unreachable code')) return 'Se llegó a un unreachable!(): el programa pasó por donde se suponía imposible.';
  if (t.startsWith('assertion')) return 'Falló un assert!: la condición que el programa daba por cierta no lo era.';
  if (t.includes('capacity overflow') || t.includes('memory allocation')) return 'No se pudo reservar la memoria pedida.';
  return 'El programa se detuvo a propósito: Rust prefiere parar antes que seguir con un estado incorrecto.';
}

export function explicarTrap(trap: string): string {
  const t = trap.toLowerCase();
  if (t.includes('call stack') || t.includes('stack overflow') || t.includes('too much recursion')) return 'Se agotó la pila (stack overflow): probablemente una recursión que nunca termina.';
  if (t.includes('out of bounds memory access') || t.includes('memory access out of bounds')) return 'El programa intentó acceder fuera de su memoria (solo puede pasar con unsafe).';
  if (t.includes('unreachable')) return 'El programa se detuvo en una instrucción «inalcanzable» (unreachable) de WebAssembly.';
  return 'El programa se detuvo de forma anormal.';
}

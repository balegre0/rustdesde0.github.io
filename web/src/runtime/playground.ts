// Cliente opt-in del Rust Playground oficial (play.rust-lang.org). Solo se llama tras un clic explícito,
// con el aviso visible de que el código se envía allí. CORS abierto; /execute no admite stdin.
// Ojo: /execute y /compile devuelven `exitDetail`; /clippy, /miri y /macro-expansion, `exit_detail`.
export const PG = 'https://play.rust-lang.org';
export const AVISO_PG = 'Tu código se enviará a play.rust-lang.org (el Rust Playground oficial, un servicio externo) para compilarlo y ejecutarlo allí.';

export type ResultadoPG = {
  ok: boolean;
  /** stdout del programa (o MIR / ensamblador) */
  salida: string;
  /** stderr del programa (sin las líneas de cargo) */
  error: string;
  /** mensajes de compilación (errores/avisos de rustc, líneas de cargo) */
  compilacion: string;
  detalle: string;
};

export type Herramienta = 'mir' | 'asm' | 'miri' | 'clippy';
export const NOMBRES: Record<Herramienta, string> = { mir: 'MIR', asm: 'Ensamblador (x86-64)', miri: 'Miri (detector de UB)', clippy: 'Clippy (consejos)' };

const cache = new Map<string, Promise<ResultadoPG>>();

async function post(ruta: string, cuerpo: unknown, ms = 30000): Promise<any> {
  const ctl = new AbortController();
  const t = setTimeout(() => ctl.abort(), ms);
  try {
    const r = await fetch(PG + ruta, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(cuerpo),
      signal: ctl.signal,
    });
    const texto = await r.text();
    let j: any;
    try { j = JSON.parse(texto); } catch { j = null; }
    if (!r.ok) throw new Error(j?.error ? `el Playground respondió ${r.status}: ${j.error}` : `el Playground respondió ${r.status}`);
    if (!j) throw new Error('respuesta no válida del Playground');
    if (j.error && typeof j.error === 'string' && j.success === undefined) throw new Error(j.error);
    return j;
  } catch (e) {
    if ((e as Error).name === 'AbortError') throw new Error('el Playground tardó demasiado en responder (más de 30 s)');
    if (e instanceof TypeError) throw new Error(navigator.onLine === false ? 'no hay conexión a Internet' : 'no se pudo contactar con play.rust-lang.org (¿sin conexión o bloqueado por la red?)');
    throw e;
  } finally {
    clearTimeout(t);
  }
}

/** Separa la salida de cargo («Compiling…», «Finished…», «Running…», errores de rustc) del stderr del programa. */
function separar(stderr: string) {
  const m = stderr.match(/^\s*Running `[^`]*`\n?/m);
  if (!m) return { compilacion: stderr, error: '' };
  return { compilacion: stderr.slice(0, m.index! + m[0].length), error: stderr.slice(m.index! + m[0].length) };
}

export function ejecutarEnPlayground(codigo: string, modo: 'debug' | 'release' = 'debug'): Promise<ResultadoPG> {
  const clave = `exec\0${modo}\0${codigo}`;
  if (!cache.has(clave)) {
    const p = post('/execute', { channel: 'stable', mode: modo, edition: '2024', crateType: 'bin', tests: false, backtrace: false, code: codigo })
      .then((j) => {
        const { compilacion, error } = separar(String(j.stderr ?? ''));
        return { ok: !!j.success, salida: String(j.stdout ?? ''), error, compilacion, detalle: String(j.exitDetail ?? j.exit_detail ?? '') };
      });
    p.catch(() => cache.delete(clave));
    cache.set(clave, p);
  }
  return cache.get(clave)!;
}

export function herramienta(tipo: Herramienta, codigo: string): Promise<ResultadoPG> {
  const clave = `${tipo}\0${codigo}`;
  if (!cache.has(clave)) {
    let p: Promise<any>;
    if (tipo === 'mir' || tipo === 'asm')
      p = post('/compile', {
        target: tipo, channel: 'stable', mode: tipo === 'asm' ? 'release' : 'debug', edition: '2024', crateType: 'bin', tests: false,
        backtrace: false, code: codigo, assemblyFlavor: 'intel', demangleAssembly: 'demangle', processAssembly: 'filter',
      });
    else if (tipo === 'miri') p = post('/miri', { code: codigo, edition: '2024', tests: false });
    else p = post('/clippy', { channel: 'stable', crateType: 'bin', edition: '2024', code: codigo });
    const r = p.then((j) => ({
      ok: !!j.success,
      salida: String(j.code ?? j.stdout ?? ''),
      error: tipo === 'mir' || tipo === 'asm' ? '' : String(j.stderr ?? ''),
      compilacion: tipo === 'mir' || tipo === 'asm' ? String(j.stderr ?? '') : '',
      detalle: String(j.exitDetail ?? j.exit_detail ?? ''),
    }));
    r.catch(() => cache.delete(clave));
    cache.set(clave, r);
  }
  return cache.get(clave)!;
}

export const enlacePlayground = (codigo: string, modo: 'debug' | 'release' = 'debug') =>
  `${PG}/?version=stable&mode=${modo}&edition=2024&code=${encodeURIComponent(codigo)}`;

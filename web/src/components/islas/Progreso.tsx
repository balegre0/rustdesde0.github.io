// Página de progreso: avance por mundo, resumen y exportar/importar (JSON).
import { useEffect, useRef, useState } from 'preact/hooks';
import { EVENTO, exportar, importar, leer, type Progreso as P } from '../../runtime/progreso';
import { cargarIndice, MAX_SESION, minutosPara, pendientes, tarjetasActivas, type Indice } from '../../runtime/tarjetas';

export default function Progreso() {
  const [ind, setInd] = useState<Indice | null>(null);
  const [p, setP] = useState<P>(leer());
  const [msg, setMsg] = useState<{ ok: boolean; texto: string } | null>(null);
  const archivo = useRef<HTMLInputElement>(null);

  useEffect(() => {
    cargarIndice().then(setInd).catch(() => setInd({ mundos: [] }));
    const f = () => setP(leer());
    window.addEventListener(EVENTO, f);
    return () => window.removeEventListener(EVENTO, f);
  }, []);

  function descargar() {
    const blob = new Blob([exportar()], { type: 'application/json' });
    const a = document.createElement('a');
    a.href = URL.createObjectURL(blob);
    a.download = `c-desde-cero-progreso-${new Date().toISOString().slice(0, 10)}.json`;
    a.click();
    setTimeout(() => URL.revokeObjectURL(a.href), 1000);
    setMsg({ ok: true, texto: 'Progreso exportado. Guarda el archivo en un sitio seguro.' });
  }

  async function subir(e: Event) {
    const f = (e.target as HTMLInputElement).files?.[0];
    if (!f) return;
    try {
      const r = importar(await f.text());
      setP(leer());
      setMsg({ ok: true, texto: `Importado: ${r.lecciones} lecciones y ${r.tarjetas} tarjetas (se han fusionado con lo que ya tenías).` });
    } catch (err) {
      setMsg({ ok: false, texto: (err as Error).message || 'No se pudo leer el archivo.' });
    } finally {
      if (archivo.current) archivo.current.value = '';
    }
  }

  const mundos = ind?.mundos ?? [];
  const totalLecciones = mundos.reduce((s, m) => s + m.lecciones.length, 0);
  const hechas = mundos.reduce((s, m) => s + m.lecciones.filter((l) => p.lecciones[l.id]).length, 0);
  const activas = ind ? tarjetasActivas(ind, p).length : 0;
  const hoy = ind ? Math.min(pendientes(ind, p).length, MAX_SESION) : 0;
  const ejercicios = Object.values(p.actividades).filter((a) => a.ok).length;

  return (
    <div class="rdc-progreso not-content">
      <dl class="rdc-cifras">
        <div><dt>Lecciones completadas</dt><dd>{ind ? hechas : '…'}<span> de {totalLecciones}</span></dd></div>
        <div><dt>Ejercicios resueltos</dt><dd>{ejercicios}</dd></div>
        <div><dt>Tarjetas en tu repaso</dt><dd>{activas}</dd></div>
        <div><dt>Para hoy</dt><dd>{hoy}{hoy > 0 && <span> · ~{minutosPara(hoy)} min</span>}</dd></div>
      </dl>
      {hoy > 0 && <p><a class="rdc-boton" href="/repaso/">Repasar ahora</a></p>}

      <h2 id="por-mundo">Por mundo</h2>
      {!ind && <p>Cargando…</p>}
      {ind && mundos.length === 0 && <p>Todavía no hay lecciones publicadas.</p>}
      <ol class="rdc-progreso-mundos" aria-labelledby="por-mundo">
        {mundos.filter((m) => m.lecciones.length > 0).map((m) => {
          const n = m.lecciones.filter((l) => p.lecciones[l.id]).length;
          const pct = Math.round((100 * n) / m.lecciones.length);
          return (
            <li key={m.slug}>
              <div class="rdc-pm-cab">
                <a href={m.href ?? m.lecciones[0].href}>{m.numero !== null ? `${m.numero} · ` : ''}{m.titulo}</a>
                <span>{n} de {m.lecciones.length}{n === m.lecciones.length && ' · completo'}</span>
              </div>
              <div class="rdc-pm-barra" role="progressbar" aria-valuemin={0} aria-valuemax={m.lecciones.length} aria-valuenow={n} aria-label={`${m.titulo}: ${n} de ${m.lecciones.length} lecciones`}>
                <span style={{ width: `${pct}%` }} />
              </div>
            </li>
          );
        })}
      </ol>

      <h2 id="copia">Copia de seguridad</h2>
      <p>Tu progreso se guarda solo en este navegador (no hay cuentas). Algunos navegadores, como Safari, lo borran si pasas una semana sin entrar: exporta una copia de vez en cuando.</p>
      <div class="rdc-barra">
        <button type="button" class="rdc-boton secundario" onClick={descargar}>Exportar progreso (JSON)</button>
        <label class="rdc-boton secundario rdc-subir">
          Importar progreso
          <input ref={archivo} type="file" accept="application/json,.json" onChange={subir} class="rdc-sr" />
        </label>
      </div>
      {msg && <p class={msg.ok ? 'rdc-msg-ok' : 'rdc-msg-error'} role="status">{msg.texto}</p>}
    </div>
  );
}

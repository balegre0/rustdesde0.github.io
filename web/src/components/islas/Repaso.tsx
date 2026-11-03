// Repaso espaciado con FSRS (ts-fsrs 6, retención objetivo 0,9). Sesiones de 10 tarjetas como máximo.
import { useEffect, useRef, useState } from 'preact/hooks';
import { createEmptyCard, fsrs, generatorParameters, Rating } from 'ts-fsrs';
import { mdEnLinea } from './md';
import { leer, modificar } from '../../runtime/progreso';
import { cargarIndice, MAX_SESION, minutosPara, pendientes, tarjetasActivas, type Tarjeta } from '../../runtime/tarjetas';

const planificador = fsrs(generatorParameters({ request_retention: 0.9, enable_fuzz: true, enable_short_term: false }));

const revivir = (c: any) => ({ ...c, due: new Date(c.due), last_review: c.last_review ? new Date(c.last_review) : undefined });

function proximaFecha(d: Date) {
  const dias = Math.round((d.getTime() - Date.now()) / 86400000);
  if (dias <= 0) return 'hoy';
  if (dias === 1) return 'mañana';
  if (dias < 30) return `dentro de ${dias} días`;
  const meses = Math.round(dias / 30);
  return `dentro de ${meses} ${meses === 1 ? 'mes' : 'meses'}`;
}

export default function Repaso() {
  const [estado, setEstado] = useState<'cargando' | 'error' | 'listo'>('cargando');
  const [cola, setCola] = useState<Tarjeta[]>([]);
  const [activas, setActivas] = useState(0);
  const [i, setI] = useState(0);
  const [mostrar, setMostrar] = useState(false);
  const [aciertos, setAciertos] = useState(0);
  const [siguiente, setSiguiente] = useState<string | null>(null);
  const botonRef = useRef<HTMLButtonElement>(null);

  useEffect(() => {
    cargarIndice()
      .then((ind) => {
        const p = leer();
        setActivas(tarjetasActivas(ind, p).length);
        setCola(pendientes(ind, p).slice(0, MAX_SESION));
        const futuras = tarjetasActivas(ind, p).map((t) => p.tarjetas[t.id]).filter(Boolean).map((c) => new Date(c.due)).sort((a, b) => +a - +b);
        setSiguiente(futuras.length ? proximaFecha(futuras[0]) : null);
        setEstado('listo');
      })
      .catch(() => setEstado('error'));
  }, []);

  useEffect(() => { botonRef.current?.focus({ preventScroll: true }); }, [i, mostrar]);

  if (estado === 'cargando') return <p class="rdc-repaso-estado">Cargando tus tarjetas…</p>;
  if (estado === 'error') return <p class="rdc-repaso-estado">No se pudo cargar el índice de tarjetas. Si estás sin conexión, abre esta página una vez con conexión para guardarla.</p>;

  if (activas === 0)
    return (
      <div class="rdc-repaso-vacio not-content">
        <p class="rdc-feedback-titulo">Aún no tienes tarjetas.</p>
        <p>Las tarjetas de cada lección entran en tu repaso cuando marcas la lección como completada (el botón está al final de cada lección).</p>
        <p><a class="rdc-boton" href="/curso/">Ir al curso</a></p>
      </div>
    );

  if (i >= cola.length)
    return (
      <div class="rdc-repaso-fin not-content" aria-live="polite">
        {cola.length > 0 ? (
          <>
            <p class="rdc-feedback-titulo">Repaso de hoy terminado.</p>
            <p>Te acordabas de {aciertos} de {cola.length}. Las que se te escaparon volverán antes; las que sabías, más tarde. Así funciona: no hay que acertarlas todas.</p>
          </>
        ) : (
          <>
            <p class="rdc-feedback-titulo">Nada que repasar hoy.</p>
            <p>Tienes {activas} {activas === 1 ? 'tarjeta' : 'tarjetas'} en el repaso{siguiente ? `; la próxima toca ${siguiente}` : ''}. Vuelve entonces, o sigue con el curso.</p>
          </>
        )}
        <p><a class="rdc-boton" href="/curso/">Seguir con el curso</a></p>
      </div>
    );

  const t = cola[i];

  function responder(sabia: boolean) {
    const ahora = new Date();
    modificar((p) => {
      const previo = p.tarjetas[t.id] ? revivir(p.tarjetas[t.id]) : createEmptyCard(ahora);
      const r = planificador.next(previo, ahora, sabia ? Rating.Good : Rating.Again);
      p.tarjetas[t.id] = JSON.parse(JSON.stringify(r.card));
    });
    if (sabia) setAciertos((a) => a + 1);
    setMostrar(false);
    setI((x) => x + 1);
  }

  return (
    <div class="rdc-repaso not-content">
      <p class="rdc-repaso-cab">
        <span>Tarjeta {i + 1} de {cola.length}</span>
        <span>~{minutosPara(cola.length - i)} min</span>
      </p>
      <div class="rdc-repaso-progreso" aria-hidden="true"><span style={{ transform: `scaleX(${i / cola.length})` }} /></div>
      <article class="rdc-tarjeta" aria-live="polite">
        <p class="rdc-tarjeta-origen">De la lección <a href={t.href}>{t.titulo}</a></p>
        <p class="rdc-tarjeta-p" dangerouslySetInnerHTML={{ __html: mdEnLinea(t.p) }} />
        {mostrar && <p class="rdc-tarjeta-r" dangerouslySetInnerHTML={{ __html: mdEnLinea(t.r) }} />}
      </article>
      <div class="rdc-barra rdc-repaso-botones">
        {!mostrar ? (
          <button type="button" class="rdc-boton" ref={botonRef} onClick={() => setMostrar(true)}>Mostrar respuesta</button>
        ) : (
          <>
            <button type="button" class="rdc-boton" ref={botonRef} onClick={() => responder(true)}>Lo sabía</button>
            <button type="button" class="rdc-boton secundario" onClick={() => responder(false)}>No me acordaba</button>
          </>
        )}
      </div>
      {!mostrar && <p class="rdc-repaso-ayuda">Intenta responder de memoria (en voz alta o en un papel) antes de mirar.</p>}
    </div>
  );
}

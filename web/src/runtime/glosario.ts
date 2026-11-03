// Definiciones del glosario en las lecciones: los términos marcados al compilar (scripts/lib/rehype-glosario.mjs)
// abren un globo con la definición al pasar el ratón, al enfocarlos con el teclado o al tocarlos.
// data-href ya lleva la base de publicación: la pone scripts/aplicar-base.ts al compilar.

let globo: HTMLDivElement | null = null;
let actual: HTMLElement | null = null;
let cerrarTrasPausa = 0;

function crearGlobo() {
  const g = document.createElement('div');
  g.className = 'rdc-term-globo';
  g.id = 'rdc-term-globo';
  g.setAttribute('role', 'tooltip');
  g.hidden = true;
  g.addEventListener('pointerenter', () => clearTimeout(cerrarTrasPausa));
  g.addEventListener('pointerleave', (e) => { if (e.pointerType === 'mouse') programarCierre(); });
  document.body.append(g);
  return g;
}

function colocar(t: HTMLElement, g: HTMLElement) {
  const r = t.getBoundingClientRect();
  const margen = 12;
  const ancho = Math.min(g.offsetWidth, innerWidth - margen * 2);
  let x = r.left + r.width / 2 - ancho / 2;
  x = Math.max(margen, Math.min(x, innerWidth - margen - ancho));
  const abajo = r.bottom + 8;
  const arriba = r.top - 8 - g.offsetHeight;
  const y = abajo + g.offsetHeight > innerHeight - margen && arriba > margen ? arriba : abajo;
  g.style.left = `${x}px`;
  g.style.top = `${y}px`;
}

function abrir(t: HTMLElement) {
  clearTimeout(cerrarTrasPausa);
  globo ??= crearGlobo();
  if (actual && actual !== t) actual.setAttribute('aria-expanded', 'false');
  actual = t;
  const href = t.dataset.href ?? '';
  globo.innerHTML = `<p class="rdc-term-globo-titulo"></p><p class="rdc-term-globo-def">${t.dataset.def ?? ''}</p>`;
  globo.querySelector('.rdc-term-globo-titulo')!.textContent = t.dataset.term ?? t.textContent ?? '';
  if (href && !location.pathname.startsWith(href)) {
    const a = document.createElement('a');
    a.href = href;
    a.className = 'rdc-term-globo-enlace';
    a.textContent = 'Ver la lección →';
    globo.append(a);
  }
  globo.hidden = false;
  t.setAttribute('aria-expanded', 'true');
  t.setAttribute('aria-describedby', globo.id);
  colocar(t, globo);
}

function cerrar() {
  clearTimeout(cerrarTrasPausa);
  if (globo) globo.hidden = true;
  actual?.setAttribute('aria-expanded', 'false');
  actual?.removeAttribute('aria-describedby');
  actual = null;
}

function programarCierre() {
  clearTimeout(cerrarTrasPausa);
  cerrarTrasPausa = window.setTimeout(cerrar, 180);
}

const termino = (e: Event) => (e.target as Element | null)?.closest?.<HTMLElement>('.rdc-term') ?? null;

document.addEventListener('pointerover', (e) => { const t = termino(e); if (t && e.pointerType === 'mouse') abrir(t); });
document.addEventListener('pointerout', (e) => { const t = termino(e); if (t && e.pointerType === 'mouse') programarCierre(); });
document.addEventListener('focusin', (e) => { const t = termino(e); if (t) abrir(t); });
document.addEventListener('focusout', (e) => {
  const t = termino(e);
  const destino = (e as FocusEvent).relatedTarget as Node | null;
  if (t && !(destino && globo?.contains(destino))) programarCierre();
});
// Al pulsar, el foco (y con ratón, el paso por encima) ya abre el globo; por eso el clic decide con el estado
// que tenía ANTES de pulsar: con ratón siempre lo deja abierto; con el dedo, si ya estaba abierto, lo cierra.
let tipoPuntero = '';
let abiertoAlPulsar = false;
document.addEventListener('pointerdown', (e) => {
  tipoPuntero = e.pointerType;
  const t = termino(e);
  abiertoAlPulsar = !!t && actual === t && !!globo && !globo.hidden && e.pointerType !== 'mouse';
}, true);
document.addEventListener('click', (e) => {
  const t = termino(e);
  if (t) { if (abiertoAlPulsar && tipoPuntero !== 'mouse') cerrar(); else abrir(t); abiertoAlPulsar = false; return; }
  if (globo && !globo.contains(e.target as Node)) cerrar();
});
document.addEventListener('keydown', (e) => { if (e.key === 'Escape' && actual) { const t = actual; cerrar(); t.focus(); } });
addEventListener('scroll', () => { if (actual && globo && !globo.hidden) colocar(actual, globo); }, { passive: true });
addEventListener('resize', () => { if (actual && globo && !globo.hidden) colocar(actual, globo); });

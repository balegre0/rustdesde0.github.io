// Tablas de práctica (/preguntas/, /problemas/): filtros, búsqueda, orden por columna y estado personal.
// Los filtros se reflejan en la URL (?tema=prestamos&dificultad=facil) para poder compartirlos.
type Estado = string;
type Opciones = { estadoDe: (id: string) => Estado; textos: Record<Estado, [string, string]> };

const normalizar = (s: string) => s.toLowerCase().normalize('NFD').replace(/[̀-ͯ]/g, '').trim();

export function iniciarTabla(op: Opciones) {
  const form = document.querySelector<HTMLFormElement>('[data-filtros]')!;
  const tabla = document.querySelector<HTMLTableElement>('[data-tabla]')!;
  const cuerpo = tabla.tBodies[0];
  const filas = [...cuerpo.rows] as HTMLTableRowElement[];
  const cuenta = document.querySelector<HTMLElement>('[data-cuenta]');
  const vacia = document.querySelector<HTMLElement>('[data-vacia]');
  const limpiar = document.querySelector<HTMLButtonElement>('[data-limpiar]');
  const campos = [...form.elements].filter((x): x is HTMLInputElement | HTMLSelectElement => 'name' in x && !!(x as HTMLInputElement).name);

  // Valores iniciales desde la URL
  const url = new URL(location.href);
  for (const c of campos) { const v = url.searchParams.get(c.name); if (v !== null) c.value = v; }

  function pintarEstados() {
    for (const f of filas) {
      const e = op.estadoDe(f.dataset.id!);
      f.dataset.estado = e;
      const cel = f.querySelector<HTMLElement>('.rdc-estado');
      if (cel) {
        cel.dataset.estado = e;
        const [sim, txt] = op.textos[e] ?? ['—', ''];
        cel.innerHTML = `<span aria-hidden="true">${sim}</span><span class="rdc-sr">${txt}</span>`;
        cel.title = txt;
      }
    }
  }

  function filtrar() {
    const v = Object.fromEntries(campos.map((c) => [c.name, c.value]));
    const q = normalizar(v.q ?? '');
    let n = 0;
    for (const f of filas) {
      let ok = !q || q.split(/\s+/).every((t) => (f.dataset.texto ?? '').includes(t));
      for (const [k, val] of Object.entries(v)) {
        if (!ok || k === 'q' || !val) continue;
        ok = (f.dataset[k] ?? '') === val;
      }
      f.hidden = !ok;
      if (ok) n++;
    }
    if (cuenta) cuenta.textContent = String(n);
    if (vacia) vacia.hidden = n > 0 || filas.length === 0;
    const activos = campos.some((c) => c.value);
    if (limpiar) limpiar.hidden = !activos;
    const u = new URL(location.href);
    for (const c of campos) { if (c.value) u.searchParams.set(c.name, c.value); else u.searchParams.delete(c.name); }
    history.replaceState(null, '', u);
  }

  let tq: ReturnType<typeof setTimeout> | undefined;
  form.addEventListener('input', (e) => {
    clearTimeout(tq);
    tq = setTimeout(filtrar, (e.target as HTMLElement).tagName === 'INPUT' ? 120 : 0);
  });
  limpiar?.addEventListener('click', () => { for (const c of campos) c.value = ''; filtrar(); campos[0]?.focus(); });

  // Orden por columna (botones en las cabeceras, con aria-sort)
  const ORDEN_ESTADO: Record<string, number> = { mal: 0, nueva: 1, ok: 2, intentado: 0, resuelto: 2 };
  for (const th of tabla.querySelectorAll<HTMLTableCellElement>('th[data-orden]')) {
    th.querySelector('button')?.addEventListener('click', () => {
      const clave = th.dataset.orden!;
      const asc = th.getAttribute('aria-sort') !== 'ascending';
      for (const o of tabla.querySelectorAll('th[data-orden]')) o.setAttribute('aria-sort', 'none');
      th.setAttribute('aria-sort', asc ? 'ascending' : 'descending');
      const val = (f: HTMLTableRowElement) => (clave === 'estado' ? String(ORDEN_ESTADO[f.dataset.estado ?? ''] ?? 1) : f.getAttribute(`data-k-${clave}`) ?? '');
      const orden = [...filas].sort((a, b) => {
        const r = val(a).localeCompare(val(b), 'es', { numeric: true }) || Number(a.dataset.i) - Number(b.dataset.i);
        return asc ? r : -r;
      });
      cuerpo.append(...orden);
    });
  }

  pintarEstados();
  filtrar();
  return { refrescar: () => { pintarEstados(); filtrar(); } };
}

// Acceso en build a src/generated/ejemplos.json (lo escribe scripts/compilar-ejemplos.ts).
import datos from '../generated/ejemplos.json';
import type { Ejemplo, Diagnostico, Ejecucion } from '../../scripts/compilar-ejemplos';

export type { Ejemplo, Diagnostico, Ejecucion };

const todos = datos as unknown as Record<string, Ejemplo>;
export const todosLosEjemplos = () => todos;

export function ejemplo(id: string, componente: string): Ejemplo {
  const limpio = id.replace(/^\/+|\.rs$/g, '');
  const e = todos[limpio];
  if (!e) {
    throw new Error(
      `<${componente} ej="${id}" />: no existe el ejemplo «ejemplos/${limpio}.rs» (o no compiló). ` +
        `Crea el archivo (ver web/CONTENIDO.md §4) y vuelve a ejecutar «bun run ejemplos».`,
    );
  }
  return e;
}

const base = () => import.meta.env.BASE_URL.replace(/\/$/, '');
export const urlWasm = (rel: string | null | undefined) => (rel ? `${base()}/${rel}` : '');
export const nombreArchivo = (e: Ejemplo) => e.id.split('/').pop() + '.rs';

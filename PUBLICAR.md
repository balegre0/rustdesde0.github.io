# Publicar «Rust desde cero» en GitHub Pages

Todo está preparado: el repositorio Git ya existe en esta carpeta, con un primer commit, y
`.github/workflows/pages.yml` compila la web y la publica sola cada vez que subes cambios a `main`.
GitHub instala Rust 1.99.1 y Bun, compila los ejemplos, construye la web y la publica.

La dirección final será `https://TU-USUARIO.github.io/rust-desde-cero/`.

**Ya publicado:** el repositorio es <https://github.com/balegre0/rustdesde0.github.io> y la web queda en
<https://balegre0.github.io/rustdesde0.github.io/>. Para que estuviera en la raíz (`https://balegre0.github.io/`),
el repositorio tendría que llamarse exactamente `balegre0.github.io`.

## 1. Crea el repositorio en GitHub (una sola vez)

1. Entra en <https://github.com/new>.
2. En **Repository name** escribe `rust-desde-cero`.
3. Márcalo como **Public**. Con una cuenta gratuita, Pages solo funciona en repositorios públicos.
4. **No** marques «Add a README», «.gitignore» ni «license». El repositorio tiene que quedar vacío.
5. Pulsa **Create repository**.

> ¿Ya tenías un repositorio de antes para la web? Puedes usarlo: pon su dirección en el paso 2. Si
> tiene otro historial, `git push` lo rechazará. Para sustituirlo por completo usa
> `git push -u --force origin main`, que **borra** lo que hubiera allí.

## 2. Sube el código

En una terminal, dentro de `C:\Users\brian\Documents\Rust-desde-cero`, cambia `TU-USUARIO` por tu
usuario de GitHub y ejecuta:

```bash
git remote add origin https://github.com/TU-USUARIO/rust-desde-cero.git
```

```bash
git push -u origin main
```

La primera vez, Git abre una ventana para que inicies sesión en GitHub.

## 3. Activa Pages (una sola vez)

1. En el repositorio, ve a **Settings → Pages**.
2. En **Build and deployment → Source**, elige **GitHub Actions**.
3. Ve a la pestaña **Actions**, abre **Publicar en GitHub Pages** y pulsa **Run workflow**. Esto hace
   falta porque la primera ejecución, la del `push`, falla si Pages aún no estaba activado.

La primera compilación tarda unos minutos, porque compila todos los ejemplos de Rust. Las
siguientes van más rápido, porque reutilizan los ejemplos ya compilados. Cuando termine con ✓, la
web estará en `https://TU-USUARIO.github.io/rust-desde-cero/`.

## Para actualizar la web más adelante

```bash
git add -A
```

```bash
git commit -m "Describe el cambio"
```

```bash
git push
```

GitHub vuelve a compilar y publicar solo.

## Si algo sale mal

- **El workflow falla en «Configurar Pages»:** Pages no está activado con «GitHub Actions» (paso 3).
- **Falla en «Compilar la web»:** abre el paso para ver el error. Es el mismo que darían
  `bun run build` o `bun run ejemplos` en tu equipo, así que puedes reproducirlo en local.
- **Probar en local tal y como quedará en GitHub** (desde `web/`, en PowerShell):

  ```powershell
  $env:BASE_PATH="/rust-desde-cero/"; bun run build; bun run preview
  ```

  Después abre <http://localhost:4332/rust-desde-cero/>. Para volver a la versión normal, cierra la
  terminal o ejecuta `Remove-Item Env:BASE_PATH` y compila de nuevo.

- **Si el repositorio tiene otro nombre**, no hay que cambiar nada: el workflow detecta la
  subcarpeta solo. Si se llama `TU-USUARIO.github.io`, la web queda en la raíz.

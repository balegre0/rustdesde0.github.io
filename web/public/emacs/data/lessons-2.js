export const lessons2 = [
  {
    id: 'm04-l01', module: 'm04', n: 1, title: 'Dividir y moverse entre ventanas', min: 8, level: 'Base',
    obj: 'Crear, cerrar, redimensionar y alternar ventanas sin perder el buffer correcto.',
    frase: '`C-x 2` y `C-x 3` dividen; `C-x o` alterna; `C-x 1` deja solo una. La ventana es una vista, no un archivo.',
    idea: [
      '`C-x 2` (`split-window-below`) divide arriba/abajo; `C-x 3` (`split-window-right`) izquierda/derecha. `C-x o` (`other-window`) pasa a la siguiente; `C-x 0` cierra la actual; `C-x 1` (`delete-other-windows`) deja solo la actual. `C-x ^` agranda verticalmente; `C-x {` y `C-x }` ajustan el ancho.',
      'Cada ventana tiene su propio punto y su propio buffer. Podés mostrar el mismo archivo en dos ventanas para comparar secciones, o dos buffers distintos. `C-x 4` es un prefijo para “abrir en otra ventana”: `C-x 4 f` archivo, `C-x 4 b` buffer, `C-x 4 d` Dired.'
    ],
    code: { lang: 'text', src: 'C-x 2   dividir abajo\nC-x 3   dividir derecha\nC-x o   otra ventana\nC-x 0   cerrar esta\nC-x 1   dejar solo esta\nC-x 4 f abrir archivo en otra ventana', cap: 'Mapa mínimo de ventanas.' },
    under: [
      'El **window tree** es un árbol binario de splits. `C-x 2`/`C-x 3` agregan nodos; `C-x 0` elimina una hoja; `C-x 1` colapsa a la raíz. `balance-windows` (`C-x +`) reparte el espacio en partes iguales.',
      'Las ventanas tienen parámetros (`window-parameters`), como `dedicated` o `no-other-window`. `display-buffer-alist` decide dónde se muestra un buffer; `C-x 4` fuerza otra ventana. Si una ventana “se abre sola”, casi siempre es `display-buffer` siguiendo reglas.'
    ],
    trap: { bad: 'Cerrar ventanas hasta quedar con una y perder el layout que habías armado.', good: 'Activar `winner-mode` antes de experimentar: `C-c <left>` deshace el layout anterior.', why: 'El layout es estado; sin winner, cada cambio es manual.' },
    q: [
      { q: '¿Qué hace `C-x 1`?', o: ['Cierra Emacs', 'Deja solo la ventana actual', 'Abre un archivo', 'Divide'], a: 1, why: '`delete-other-windows` cierra las demás ventanas del frame.' },
      { q: '¿Qué prefijo abre algo en otra ventana?', o: ['`C-x 4`', '`C-x 5`', '`C-x 6`', '`C-c 4`'], a: 0, why: '`C-x 4` es el prefijo `other-window` para archivos, buffers y Dired.' }
    ],
    ex: { p: 'Abrí dos archivos en dos ventanas verticales, alterná con `C-x o` y equilibrá con `C-x +`.', start: ';; C-x 3\n;; C-x 4 f ~/a.txt RET\n;; C-x o\n;; C-x +', sol: ';; C-x 3\n;; C-x 4 f ~/a.txt RET\n;; C-x o\n;; C-x +', hint: '`C-x 4 b` abre un buffer en otra ventana sin cambiar el foco.' },
    cards: [['¿Qué hace `C-x 3`?', 'Divide la ventana en dos, izquierda/derecha.'], ['¿Qué hace `C-x +`?', 'Equilibra el tamaño de las ventanas.']],
    tags: ['ventanas', 'comandos', 'comandos']
  },
  {
    id: 'm04-l02', module: 'm04', n: 2, title: 'winner-mode y layouts', min: 7, level: 'Intermedio',
    obj: 'Deshacer y rehacer cambios de layout de ventanas, y guardar configuraciones.',
    frase: '`winner-mode` recuerda cada cambio de ventanas; `C-c <left>` vuelve al layout anterior.',
    idea: [
      'Activá `M-x winner-mode`. Cada vez que cambiás la disposición de ventanas, winner guarda el estado anterior. `C-c <left>` (`winner-undo`) vuelve atrás; `C-c <right>` (`winner-redo`) rehace. Es el “undo” de la disposición, no del texto.',
      'Para layouts con nombre, `C-x r w` guarda la configuración de ventanas en un registro y `C-x r j` la restaura. `M-x window-configuration-to-register` y `jump-to-register` son las funciones. `tab-bar` es otra forma de tener “workspaces” persistentes.'
    ],
    code: { lang: 'elisp', src: '(winner-mode 1)\n(window-configuration-to-register ?w)\n(jump-to-register ?w)\n;; C-c <left> / C-c <right>', cap: 'Winner y registers de ventanas.' },
    under: [
      'Winner escucha `window-configuration-change-hook` y guarda un anillo de configuraciones. No guarda el contenido de los buffers, solo qué buffer muestra cada ventana y cómo están divididas. Si matás un buffer, la configuración puede quedar inválida.',
      'Los registers de ventanas guardan una `window-configuration` completa. Son útiles para alternar entre “modo coding” y “modo debug” con una tecla. `tab-bar` guarda una configuración por pestaña y sobrevive mejor a cambios de buffers.'
    ],
    trap: { bad: 'Activar winner después de haber perdido el layout y esperar que recupere el pasado.', good: 'Activar `winner-mode` al inicio de la sesión y usarlo antes de cerrar ventanas.', why: 'Winner solo recuerda cambios ocurridos mientras está activo.' },
    q: [
      { q: '¿Qué hace `C-c <left>` con winner-mode?', o: ['Deshace el último cambio de layout', 'Cierra Emacs', 'Guarda', 'Compila'], a: 0, why: '`winner-undo` restaura la configuración de ventanas anterior.' },
      { q: '¿Qué guarda `C-x r w`?', o: ['Un archivo', 'La configuración de ventanas en un registro', 'El kill ring', 'Un bookmark'], a: 1, why: '`window-configuration-to-register` guarda el layout en un registro.' }
    ],
    ex: { p: 'Activá winner, dividí, abrí un buffer en otra ventana y deshacé el layout con `C-c <left>`.', start: ';; M-x winner-mode RET\n;; C-x 2\n;; C-x 4 b *scratch* RET\n;; C-c <left>', sol: ';; M-x winner-mode RET\n;; C-x 2\n;; C-x 4 b *scratch* RET\n;; C-c <left>', hint: '`C-c <right>` rehace; `C-x r w a` guarda en el registro a.' },
    cards: [['¿Qué guarda winner?', 'Configuraciones de ventanas, no texto.'], ['¿Qué comando guarda un layout en un registro?', '`C-x r w` (`window-configuration-to-register`).']],
    tags: ['ventanas', 'winner', 'comandos']
  },
  {
    id: 'm04-l03', module: 'm04', n: 3, title: 'Frames, tab-bar y tab-line', min: 8, level: 'Intermedio',
    obj: 'Distinguir frames de ventanas y usar pestañas para separar contextos.',
    frase: 'Un frame es una ventana del sistema operativo; una pestaña de `tab-bar` es un workspace dentro de un frame.',
    idea: [
      '`C-x 5 2` (`make-frame-command`) abre un frame nuevo; `C-x 5 o` pasa a otro frame; `C-x 5 0` cierra el actual. En el daemon, los frames son baratos: `emacsclient -c` abre uno nuevo sin recargar el estado.',
      '`M-x tab-bar-mode` muestra una barra de pestañas arriba. Cada pestaña tiene su propia configuración de ventanas y su propio buffer actual. `M-x tab-new`, `M-x tab-next`, `M-x tab-close`; también hay prefijo `C-x t`. `tab-line-mode` es otra cosa: pestañas por buffer, no por workspace.'
    ],
    code: { lang: 'text', src: 'C-x 5 2   nuevo frame\nC-x 5 o   otro frame\nC-x 5 0   cerrar frame\nM-x tab-bar-mode\nM-x tab-new / tab-next / tab-close', cap: 'Frames y tab-bar.' },
    under: [
      'Un frame contiene un **window tree**. `tab-bar` guarda una lista de tabs, cada una con su window configuration y su buffer. Al cambiar de tab, Emacs restaura esa configuración. Es un workspace ligero, sin procesos separados.',
      '`tab-line-mode` muestra una pestaña por buffer dentro de una ventana; es útil para editar varios archivos relacionados. No confundas `tab-bar` (workspace) con `tab-line` (buffers) ni con `display-buffer` (reglas de ventanas).'
    ],
    trap: { bad: 'Cerrar un frame del daemon con `C-x 5 0` y creer que cerraste Emacs.', good: 'Recordar que el daemon sigue vivo; `C-x C-c` cierra la conexión/frame, no necesariamente el servidor.', why: 'En el daemon, frames y proceso son cosas distintas.' },
    q: [
      { q: '¿Qué es un frame?', o: ['Una ventana del sistema operativo', 'Un buffer', 'Un archivo', 'Una tecla'], a: 0, why: 'El frame es la ventana del SO que contiene ventanas de Emacs.' },
      { q: '¿Qué diferencia hay entre tab-bar y tab-line?', o: ['tab-bar es workspace; tab-line es buffers', 'Son iguales', 'tab-line es para archivos', 'tab-bar cierra Emacs'], a: 0, why: '`tab-bar` agrupa configuraciones de ventanas; `tab-line` muestra buffers en una ventana.' }
    ],
    ex: { p: 'Abrí un frame nuevo con `C-x 5 2`, volvé con `C-x 5 o` y cerralo con `C-x 5 0`.', start: ';; C-x 5 2\n;; C-x 5 o\n;; C-x 5 0', sol: ';; C-x 5 2\n;; C-x 5 o\n;; C-x 5 0', hint: 'En el daemon, `emacsclient -c` hace lo mismo que `C-x 5 2`.' },
    cards: [['¿Qué hace `C-x 5 2`?', 'Crea un frame nuevo.'], ['¿Qué guarda una pestaña de tab-bar?', 'Una configuración de ventanas y un buffer actual.']],
    tags: ['frames', 'ventanas', 'comandos']
  },
  {
    id: 'm04-l04', module: 'm04', n: 4, title: 'Perspectivas y workspaces', min: 8, level: 'Avanzado',
    obj: 'Separar buffers y layouts por contexto usando perspective.el o tab-bar.',
    frase: 'Una perspectiva es un conjunto de buffers y un layout: “trabajo”, “proyecto A”, “notas”, cada uno con su mundo.',
    idea: [
      '`perspective.el` agrupa buffers en perspectivas. `M-x perspective-mode`, `M-x persp-switch` para cambiar, `M-x persp-add-buffer` para sumar el buffer actual, `M-x persp-remove-buffer` para sacarlo. Cada perspectiva tiene su lista de buffers y su configuración de ventanas.',
      'La alternativa nativa es `tab-bar`: cada tab tiene su layout, pero los buffers siguen siendo globales. `perspective` filtra la lista de buffers, lo que ayuda cuando tenés 80 buffers abiertos. `project.el` puede combinarse: una perspectiva por proyecto.'
    ],
    code: { lang: 'elisp', src: '(perspective-mode 1)\n(persp-switch "trabajo")\n(persp-add-buffer (current-buffer))\n(persp-switch "notas")', cap: 'Perspectivas básicas.' },
    under: [
      'Perspective mantiene una lista de buffers por nombre y hookea `switch-to-buffer` para filtrar. No mueve buffers entre procesos ni los mata: solo cambia lo que ves en la lista. Podés tener el mismo buffer en varias perspectivas.',
      'El costo es que algunos paquetes que listan buffers (ibuffer, consult-buffer) necesitan integración para respetar la perspectiva. `persp-mode` ofrece funciones para eso; verificá que tu paquete de completion lo soporte.'
    ],
    trap: { bad: 'Crear veinte perspectivas y no recordar en cuál está el buffer que buscás.', good: 'Usar pocas perspectivas (3–5) con nombres claros y apoyarte en `consult-buffer` para buscar globalmente.', why: 'Las perspectivas ocultan buffers; si abusás, buscás a ciegas.' },
    q: [
      { q: '¿Qué hace `persp-switch`?', o: ['Cambia de perspectiva', 'Mata un buffer', 'Compila', 'Guarda'], a: 0, why: 'Cambia el conjunto de buffers visible y el layout asociado.' },
      { q: '¿Qué diferencia hay con tab-bar?', o: ['Perspective filtra buffers; tab-bar guarda layouts', 'Son iguales', 'tab-bar filtra buffers', 'Perspective cierra Emacs'], a: 0, why: 'Perspective agrupa buffers; tab-bar agrupa configuraciones de ventanas.' }
    ],
    ex: { p: 'Creá dos perspectivas, agregá un buffer a cada una y alterná con `persp-switch`.', start: ';; M-x perspective-mode RET\n;; M-x persp-switch RET trabajo RET\n;; M-x persp-add-buffer\n;; M-x persp-switch RET notas RET', sol: ';; M-x perspective-mode RET\n;; M-x persp-switch RET trabajo RET\n;; M-x persp-add-buffer\n;; M-x persp-switch RET notas RET', hint: '`persp-switch` con un nombre nuevo la crea.' },
    cards: [['¿Qué agrupa una perspectiva?', 'Buffers y layout por contexto.'], ['¿Qué paquete nativo cumple un rol parecido?', '`tab-bar-mode` (layouts), aunque no filtra buffers.']],
    tags: ['workspace', 'perspectivas', 'experto']
  },
  {
    id: 'm04-l05', module: 'm04', n: 5, title: 'display-buffer-alist y ventanas dedicadas', min: 9, level: 'Avanzado',
    obj: 'Controlar dónde y cómo se muestran los buffers automáticos.',
    frase: '`display-buffer-alist` es la política de “dónde aparece cada buffer”; sin ella, Emacs decide por su cuenta.',
    idea: [
      'Cuando un comando abre un buffer auxiliar (Help, Compile, REPL, *Messages*), llama a `display-buffer`. Esa función consulta `display-buffer-alist`: una lista de reglas (regexp del nombre, acción y parámetros). Podés decir “Help siempre abajo, 20% de altura” o “REPL a la derecha”.',
      'Las acciones incluyen `display-buffer-below-selected`, `display-buffer-in-side-window`, `display-buffer-reuse-window` y `display-buffer-pop-up-window`. Los parámetros incluyen `window-height`, `side`, `slot`, `dedicated`. `C-x 4` y `C-x 5` son atajos, pero la política global es el alist.'
    ],
    code: { lang: 'elisp', src: '(add-to-list \'display-buffer-alist\n  \'("\\\\*Help\\\\*"\n    (display-buffer-below-selected)\n    (window-height . 0.25)))\n\n(add-to-list \'display-buffer-alist\n  \'("\\\\*Compilation\\\\*"\n    (display-buffer-in-side-window)\n    (side . bottom) (slot . 0)))', cap: 'Reglas para Help y Compilation.' },
    under: [
      'Una ventana **dedicada** no se reutiliza para otro buffer: `set-window-dedicated-p` la marca. Es útil para REPLs y ventanas laterales. `display-buffer-in-side-window` crea una ventana lateral que no participa del layout principal.',
      'El orden de las reglas importa: se aplica la primera que coincide. `display-buffer-alist` se combina con `display-buffer-base-action` y `display-buffer-fallback-action`. Si una regla no funciona, `M-x debug-on-entry display-buffer` te muestra qué acción se eligió.'
    ],
    trap: { bad: 'Copiar una regla de internet sin entender la regexp y terminar con todos los buffers en una ventana lateral.', good: 'Probar una regla a la vez con `M-x display-buffer` y revisar `C-h v display-buffer-alist`.', why: 'Las reglas se aplican por orden y una regexp demasiado amplia captura todo.' },
    q: [
      { q: '¿Qué controla `display-buffer-alist`?', o: ['Dónde se muestran los buffers', 'El tema', 'Las teclas', 'El kill ring'], a: 0, why: 'Es la política de visualización de buffers.' },
      { q: '¿Qué es una ventana dedicada?', o: ['Una que no se reutiliza para otro buffer', 'Una ventana cerrada', 'Un frame', 'Un buffer de solo lectura'], a: 0, why: '`set-window-dedicated-p` impide que `display-buffer` la use para otro buffer.' }
    ],
    ex: { p: 'Hacé que `*Help*` aparezca abajo al 25% y `*Compilation*` en una ventana lateral inferior.', start: '(add-to-list \'display-buffer-alist\n  \'("\\\\*Help\\\\*"\n    (display-buffer-below-selected)\n    (window-height . 0.25)))', sol: '(add-to-list \'display-buffer-alist\n  \'("\\\\*Help\\\\*"\n    (display-buffer-below-selected)\n    (window-height . 0.25)))\n(add-to-list \'display-buffer-alist\n  \'("\\\\*Compilation\\\\*"\n    (display-buffer-in-side-window)\n    (side . bottom) (slot . 0)))', hint: 'Reiniciá el buffer de ayuda o usá `M-x display-buffer` para probar.' },
    cards: [['¿Qué función decide dónde mostrar un buffer?', '`display-buffer`, guiada por `display-buffer-alist`.'], ['¿Qué hace `display-buffer-in-side-window`?', 'Abre el buffer en una ventana lateral.']],
    tags: ['ventanas', 'display-buffer', 'experto']
  },
  {
    id: 'm05-l01', module: 'm05', n: 1, title: 'Dired: el gestor de archivos', min: 8, level: 'Base',
    obj: 'Navegar directorios, abrir archivos y refrescar el listado.',
    frase: 'Dired no es un “explorador”: es un buffer editable con una línea por archivo y comandos que operan sobre ellas.',
    idea: [
      '`C-x d` (`dired`) pide un directorio; `C-x C-j` (`dired-jump`) abre el directorio del archivo actual y deja el cursor en él. `RET` abre el archivo o entra al directorio; `^` sube al padre; `g` refresca; `q` cierra. `C-x C-f` sobre un directorio también abre Dired.',
      'Cada línea muestra permisos, dueño, tamaño y fecha. Podés ocultar columnas con `dired-hide-details-mode`, ordenar con `s` o `C-u s`, y alternar archivos ocultos con `M-x dired-toggle-hide-details` o `(setq dired-listing-switches "-alh")`.'
    ],
    code: { lang: 'text', src: 'C-x d ~/proyectos RET\nRET      abrir\n^        subir\ng        refrescar\nq        cerrar\nC-x C-j  dired-jump desde un archivo', cap: 'Navegación básica de Dired.' },
    under: [
      'Dired ejecuta `ls` (o el equivalente) y formatea la salida en un buffer. `dired-listing-switches` controla las opciones; en Windows se usa `ls-lisp` o el `ls` de Git/MSYS. Los nombres son botones: `RET` y el mouse funcionan.',
      'Dired tiene un modo de edición (`wdired`) que convierte el listado en texto editable. Es la forma más rápida de renombrar en lote, pero hay que confirmar con `C-c C-c` y revisar el resultado.'
    ],
    trap: { bad: 'Editar el buffer de Dired como si fuera texto normal y esperar que los archivos cambien.', good: 'Usar `wdired-change-to-wdired-mode` (`C-x C-q`), editar los nombres y confirmar con `C-c C-c`.', why: 'Dired es una vista; los cambios de texto no se aplican hasta entrar en wdired y confirmar.' },
    q: [
      { q: '¿Qué hace `C-x C-j`?', o: ['Abre Dired en el directorio del archivo actual', 'Cierra Emacs', 'Compila', 'Guarda'], a: 0, why: '`dired-jump` abre el directorio y posiciona el cursor en el archivo.' },
      { q: '¿Qué hace `^` en Dired?', o: ['Sube al directorio padre', 'Borra', 'Renombra', 'Copia'], a: 0, why: '`dired-up-directory` sube un nivel.' }
    ],
    ex: { p: 'Abrí Dired en tu home, entrá a una carpeta, volvé con `^` y refrescá con `g`.', start: ';; C-x d ~ RET\n;; RET en una carpeta\n;; ^\n;; g', sol: ';; C-x d ~ RET\n;; RET en una carpeta\n;; ^\n;; g', hint: '`C-x C-j` desde un archivo abierto te deja en su línea.' },
    cards: [['¿Qué hace `g` en Dired?', 'Refresca el listado.'], ['¿Qué es wdired?', 'Un modo para editar nombres/permisos como texto y aplicarlos al confirmar.']],
    tags: ['dired', 'archivos', 'comandos']
  },
  {
    id: 'm05-l02', module: 'm05', n: 2, title: 'Marcar, operar y wdired', min: 9, level: 'Intermedio',
    obj: 'Marcar archivos y ejecutar operaciones en lote; renombrar con wdired.',
    frase: 'En Dired primero marcás (`m`), después operás (`D`, `R`, `C`). Las operaciones afectan solo lo marcado.',
    idea: [
      '`m` marca el archivo actual y avanza; `u` desmarca; `t` invierte las marcas; `U` desmarca todo. `D` (`dired-delete-file`) borra los marcados; `R` renombra; `C` copia; `+` crea un directorio; `S` crea un symlink. Emacs pide confirmación para operaciones destructivas.',
      '`C-x C-q` (`wdired-change-to-wdired-mode`) convierte el listado en un buffer editable: cambiás nombres, permisos o fechas y `C-c C-c` aplica. `C-c C-k` cancela. Es ideal para renombrar 30 archivos con un patrón, pero conviene hacer una copia antes.'
    ],
    code: { lang: 'text', src: 'm        marcar\nD        borrar marcados\nR        renombrar\nC        copiar\n+        crear directorio\nC-x C-q  entrar a wdired\nC-c C-c  aplicar', cap: 'Operaciones en lote.' },
    under: [
      'Dired usa un **mark ring** interno y muestra las marcas con `*`. Las operaciones se ejecutan sobre la lista de archivos marcados, no sobre la línea del cursor. Si no hay marcas, algunos comandos usan el archivo actual.',
      '`wdired` cambia el buffer a `wdired-mode`, permite editar y al confirmar compara el texto con el listado original. Los cambios se aplican secuencialmente; si un rename falla, los siguientes pueden quedar inconsistentes. Revisá el buffer de mensajes.'
    ],
    trap: { bad: 'Pulsar `D` con varios archivos marcados sin revisar y borrar algo importante.', good: 'Revisar las marcas, usar `C-x C-q` para renombrar y confirmar con `C-c C-c` después de leer el resumen.', why: 'Las operaciones en lote son rápidas y peligrosas; la confirmación existe por algo.' },
    q: [
      { q: '¿Qué hace `m` en Dired?', o: ['Marca el archivo y avanza', 'Borra', 'Renombra', 'Abre'], a: 0, why: '`dired-mark` marca el archivo actual y mueve el cursor.' },
      { q: '¿Qué hace `C-x C-q` en Dired?', o: ['Entra a wdired', 'Cierra', 'Copia', 'Compila'], a: 0, why: '`wdired-change-to-wdired-mode` permite editar el listado.' }
    ],
    ex: { p: 'Marcá tres archivos con `m`, copialos a una carpeta nueva con `C` y verificá el resultado.', start: ';; m m m\n;; C\n;; ~/copias RET', sol: ';; m m m\n;; C\n;; ~/copias RET', hint: '`t` invierte marcas; `U` desmarca todo.' },
    cards: [['¿Qué hace `t` en Dired?', 'Invierte las marcas.'], ['¿Cómo se aplican los cambios de wdired?', 'Con `C-c C-c`; `C-c C-k` cancela.']],
    tags: ['dired', 'archivos', 'comandos']
  },
  {
    id: 'm05-l03', module: 'm05', n: 3, title: 'Tramp: editar en remoto', min: 9, level: 'Avanzado',
    obj: 'Abrir archivos remotos con SSH, sudo y contenedores usando rutas Tramp.',
    frase: 'Tramp convierte una ruta remota en un buffer local: `/ssh:user@host:/ruta` y a editar.',
    idea: [
      '`C-x C-f` y escribí `/ssh:usuario@servidor:/etc/nginx/nginx.conf`. Tramp abre una conexión SSH, monta el archivo y te deja editarlo con los comandos normales. Al guardar, escribe en el remoto. `/sudo:root@localhost:/etc/hosts` usa sudo local; `/docker:container:/app` entra a un contenedor (según configuración).',
      'Tramp cachea conexiones y rutas. `M-x tramp-cleanup-connection` limpia una; `M-x tramp-cleanup-all-connections` todas. `tramp-default-method` define el método por defecto (`ssh`), y `tramp-default-user` el usuario. Para archivos grandes, la latencia se nota: usá `tramp-verbose` solo para depurar.'
    ],
    code: { lang: 'text', src: '/ssh:deploy@example.com:/var/www/app\n/sudo:root@localhost:/etc/hosts\n/docker:web:/app\nM-x tramp-cleanup-connection', cap: 'Rutas Tramp típicas.' },
    under: [
      'Tramp usa un **método** (`ssh`, `scp`, `sudo`, `docker`, `podman`) y un **host**. El formato es `/método:usuario@host:ruta`. Si el método no está instalado, falla con un mensaje en `*tramp/...*`.',
      'Para SSH con clave, Tramp usa `~/.ssh/config` y `auth-source`. Si pide contraseña, puede usar `auth-source` o el minibuffer. `ControlMaster` de OpenSSH acelera conexiones repetidas. En Windows, Tramp necesita un cliente SSH en el PATH.'
    ],
    trap: { bad: 'Editar un archivo remoto con Tramp y ejecutar comandos locales sobre la ruta remota sin darte cuenta.', good: 'Recordar que el buffer es remoto, pero `M-x compile` y `shell-command` corren localmente salvo que uses `tramp` explícitamente.', why: 'Tramp virtualiza el acceso a archivos, no todo el entorno de ejecución.' },
    q: [
      { q: '¿Qué ruta abre un archivo por SSH?', o: ['`/ssh:user@host:/ruta`', '`ssh://user@host/ruta`', '`C-x C-f ssh`', '`/remote:/ruta`'], a: 0, why: 'El formato de Tramp es `/método:usuario@host:ruta`.' },
      { q: '¿Qué hace `tramp-cleanup-connection`?', o: ['Cierra la conexión y limpia buffers remotos', 'Borra el archivo', 'Compila', 'Guarda'], a: 0, why: 'Limpia la conexión Tramp y los buffers asociados.' }
    ],
    ex: { p: 'Abrí un archivo remoto con `/ssh:tu-usuario@tu-servidor:/tmp/prueba.txt` y guardalo.', start: ';; C-x C-f /ssh:user@host:/tmp/prueba.txt RET', sol: ';; C-x C-f /ssh:user@host:/tmp/prueba.txt RET\n;; escribe\n;; C-x C-s', hint: 'Si falla, revisá `*tramp/ssh ...*` y que `ssh` esté en el PATH.' },
    cards: [['¿Qué formato tiene una ruta Tramp?', '`/método:usuario@host:ruta`.'], ['¿Qué comando limpia conexiones Tramp?', '`M-x tramp-cleanup-all-connections`.']],
    tags: ['tramp', 'remoto', 'experto']
  },
  {
    id: 'm05-l04', module: 'm05', n: 4, title: 'Shell, eshell, term y vterm', min: 9, level: 'Intermedio',
    obj: 'Elegir el entorno de shell adecuado y usar historial, interrupciones y directorios.',
    frase: '`shell` usa tu shell pero con edición limitada; `eshell` es un shell en Elisp; `term`/`vterm` son terminales reales.',
    idea: [
      '`M-x shell` abre un shell interactivo (bash/zsh) en un buffer de comint. `M-x eshell` abre Eshell, un shell escrito en Elisp que entiende expresiones Lisp y comandos externos. `M-x term` y `M-x vterm` (paquete) emulan una terminal completa, con programas de pantalla completa.',
      'En `shell`, `M-p`/`M-n` recorren el historial, `C-c C-c` interrumpe, `C-d` envía EOF y `C-c C-z` vuelve al prompt. En Eshell, podés mezclar comandos y Lisp: `ls | grep elisp`, `(+ 1 2)`, `cd /ssh:host:/tmp`. `term` tiene un modo de línea y un modo char.'
    ],
    code: { lang: 'text', src: 'M-x shell\nM-x eshell\nM-x term\nM-x vterm\n;; Eshell:\nls | grep elisp\n(+ 1 2)\ncd /ssh:host:/tmp', cap: 'Cuatro formas de shell.' },
    under: [
      '`shell` usa `comint-mode`, que intercepta la entrada y la envía al proceso. No es una terminal real: los programas que usan cursor positioning pueden romperse. `term` implementa un emulador de terminal en Elisp; `vterm` usa un módulo nativo y es más rápido y fiel.',
      'Eshell es un shell completo en Lisp: parsea, expande globs y pipelines, y puede evaluar Lisp si la expresión empieza con `(`. Es más lento para procesos intensivos, pero integra el sistema operativo con Emacs de forma única.'
    ],
    trap: { bad: 'Usar `shell` para ejecutar `htop` o `vim` y obtener una pantalla rota.', good: 'Usar `vterm`/`term` para programas de pantalla completa y `shell`/`eshell` para comandos de línea.', why: '`shell` no emula una terminal; los programas que mueven el cursor necesitan un emulador real.' },
    q: [
      { q: '¿Qué shell está escrito en Elisp?', o: ['Eshell', 'bash', 'zsh', 'vterm'], a: 0, why: 'Eshell es un shell implementado en Emacs Lisp.' },
      { q: '¿Qué usarías para `htop`?', o: ['`shell`', '`eshell`', '`vterm` o `term`', '`compile`'], a: 2, why: 'Los programas de pantalla completa necesitan un emulador de terminal.' }
    ],
    ex: { p: 'Abrí Eshell, ejecutá `ls`, `(+ 1 2)` y `cd /tmp`; después abrí una shell normal y compará.', start: 'M-x eshell\nls\n(+ 1 2)\ncd /tmp', sol: 'M-x eshell\nls\n(+ 1 2)\ncd /tmp\nM-x shell', hint: 'En Eshell, una expresión Lisp empieza con `(`.' },
    cards: [['Diferencia entre shell y term', '`shell` usa comint y no emula terminal; `term`/`vterm` sí.'], ['¿Qué permite Eshell?', 'Mezclar comandos, pipelines y expresiones Lisp.']],
    tags: ['shell', 'procesos', 'comandos']
  },
  {
    id: 'm05-l05', module: 'm05', n: 5, title: 'Proced, procesos y comandos asíncronos', min: 8, level: 'Avanzado',
    obj: 'Ver y controlar procesos, y lanzar comandos sin bloquear Emacs.',
    frase: '`M-&` ejecuta un comando en segundo plano; `proced` te da una tabla de procesos viva.',
    idea: [
      '`M-!` (`shell-command`) ejecuta y espera; `M-&` (`async-shell-command`) ejecuta en segundo plano y escribe la salida en `*Async Shell Command*`; `M-|` (`shell-command-on-region`) pasa la región por un comando. `M-x compile` usa `compilation-mode` y permite navegar errores con `M-g M-n`.',
      '`M-x proced` muestra una tabla de procesos estilo `top`, actualizada con `g`; podés marcar y matar. `M-x list-processes` lista los procesos que Emacs conoce (shells, servidores LSP, subprocess). `M-x proced` usa la lista del sistema, no solo los hijos de Emacs.'
    ],
    code: { lang: 'elisp', src: '(async-shell-command "sleep 10 && echo listo")\n(shell-command "ls -la")\n(start-process "mi-proc" "*mi-proc*" "sleep" "30")\n(list-processes)', cap: 'Comandos y procesos.' },
    under: [
      'Cada proceso tiene un buffer asociado, un filtro (`process-filter`) y un sentinel (`process-sentinel`). El filtro recibe la salida; el sentinel se llama cuando cambia el estado (terminó, se cayó). Es la base de LSP, REPLs y shells.',
      '`compile` usa un proceso y parsea la salida con `compilation-error-regexp-alist`. Por eso puede convertir `archivo:línea:columna` en enlaces. Si tu herramienta tiene otro formato, agregá una regexp a esa lista.'
    ],
    trap: { bad: 'Ejecutar un comando largo con `M-!` y bloquear Emacs hasta que termine.', good: 'Usar `M-&` o `M-x compile` para no bloquear el editor.', why: '`shell-command` espera; `async-shell-command` devuelve el control.' },
    q: [
      { q: '¿Qué hace `M-&`?', o: ['Ejecuta en segundo plano', 'Borra', 'Guarda', 'Cierra'], a: 0, why: '`async-shell-command` lanza el comando sin bloquear Emacs.' },
      { q: '¿Qué comando muestra una tabla de procesos?', o: ['`M-x proced`', '`M-x compile`', '`M-x dired`', '`M-x shell`'], a: 0, why: '`proced` muestra y permite gestionar procesos del sistema.' }
    ],
    ex: { p: 'Lanzá `sleep 5 && echo listo` con `M-&`, revisá el buffer de salida y listá procesos con `M-x list-processes`.', start: 'M-& sleep 5 && echo listo RET\nM-x list-processes', sol: 'M-& sleep 5 && echo listo RET\nM-x list-processes', hint: 'Si el comando falla, `M-x shell` te muestra el error completo.' },
    cards: [['Diferencia entre `M-!` y `M-&`', '`M-!` espera; `M-&` ejecuta en segundo plano.'], ['¿Qué es un process sentinel?', 'Una función que se llama cuando el proceso cambia de estado.']],
    tags: ['procesos', 'async', 'comandos']
  },
  {
    id: 'm06-l01', module: 'm06', n: 1, title: 'Estructura y navegación en Org', min: 8, level: 'Base',
    obj: 'Crear encabezados, plegarlos y moverte por la jerarquía.',
    frase: 'Org es texto plano con estructura: los `*` son encabezados y `TAB` pliega la jerarquía.',
    idea: [
      'Un archivo Org usa `*` para nivel 1, `**` para nivel 2, etc. `TAB` (`org-cycle`) pliega/despliega el nodo actual; `S-TAB` cicla todo el documento; `M-RET` inserta un encabezado hermano; `M-<down>`/`M-<up>` mueven el nodo. `C-c C-n`/`C-c C-p` van al siguiente/anterior encabezado.',
      '`C-c C-u` sube al padre; `C-c C-f`/`C-c C-b` avanzan/retroceden por el mismo nivel. `C-c C-*` convierte la región en un árbol de encabezados. Org no necesita una base de datos: la estructura está en el texto.'
    ],
    code: { lang: 'org', src: '* Proyecto\n** TODO Diseñar\n*** Subtarea\n** DONE Investigar\n* Notas\n- idea', cap: 'Jerarquía Org mínima.' },
    under: [
      'Org usa `org-element` para parsear el buffer en un árbol de elementos (headline, paragraph, list, table, block). Los comandos de navegación se apoyan en ese parser, no en regexps frágiles. Por eso respetan el contexto.',
      'El plegado se controla con `org-startup-folded`, `org-cycle-separator-lines` y `org-show-context`. `org-indent-mode` alinea visualmente los niveles sin cambiar el texto. El archivo sigue siendo texto plano.'
    ],
    trap: { bad: 'Usar `*` al inicio de una línea normal y crear un encabezado sin querer.', good: 'Escapar con `,*` o usar una lista `-` para viñetas; revisá `org-element-context` si dudás.', why: 'En Org, `*` al inicio de línea es sintaxis de encabezado.' },
    q: [
      { q: '¿Qué hace `TAB` en un encabezado Org?', o: ['Indenta', 'Pliega/despliega el nodo', 'Borra', 'Exporta'], a: 1, why: '`org-cycle` pliega o despliega la jerarquía bajo el encabezado.' },
      { q: '¿Qué inserta `M-RET`?', o: ['Un encabezado hermano', 'Una tabla', 'Un enlace', 'Un bloque'], a: 0, why: '`org-insert-heading` crea un encabezado al mismo nivel.' }
    ],
    ex: { p: 'Creá un archivo con tres niveles, plegá todo con `S-TAB`, desplegá un nodo con `TAB` y movete con `C-c C-n`.', start: '* A\n** B\n*** C\n* D', sol: '* A\n** B\n*** C\n* D\n;; S-TAB, TAB, C-c C-n', hint: '`M-<down>` mueve el nodo completo, no solo el cursor.' },
    cards: [['¿Qué hace `S-TAB`?', 'Cicla el plegado de todo el documento.'], ['¿Qué comando va al siguiente encabezado?', '`C-c C-n` (`org-next-visible-heading`).']],
    tags: ['org', 'estructura', 'comandos']
  },
  {
    id: 'm06-l02', module: 'm06', n: 2, title: 'TODO, tags, properties y prioridades', min: 9, level: 'Intermedio',
    obj: 'Convertir encabezados en tareas y organizarlas con metadatos.',
    frase: '`C-c C-t` rota el estado TODO; las tags y propiedades añaden contexto sin romper el texto.',
    idea: [
      '`C-c C-t` (`org-todo`) rota entre `TODO` y `DONE`; con `C-u C-c C-t` elegís un estado concreto. Podés definir más estados con `org-todo-keywords`, por ejemplo `TODO NEXT WAITING | DONE CANCELLED`. `C-c C-s` programa (`SCHEDULED`), `C-c C-d` pone deadline.',
      'Las tags se escriben al final del encabezado (`:trabajo:urgente:`) y se editan con `C-c C-q`. Las propiedades van en un bloque `:PROPERTIES:` debajo del encabezado; `C-c C-x p` agrega una. Las prioridades (`[#A]`) se cambian con `C-c ,`. Todo eso alimenta la agenda y los filtros.'
    ],
    code: { lang: 'org', src: '* TODO Escribir informe :trabajo:urgente:\n  DEADLINE: <2026-11-20 Fri>\n  :PROPERTIES:\n  :ID: informe-2026\n  :END:', cap: 'Tarea con deadline, tags y propiedad.' },
    under: [
      'Org guarda los estados en `org-todo-keywords` y los interpreta como palabras clave al inicio del encabezado. Las tags tienen herencia: un hijo hereda las tags del padre si `org-use-tag-inheritance` está activo.',
      'Las propiedades son pares clave-valor en un drawer. Se usan para IDs, effort, owner, contexto y para exportación. `org-property-actions` permite acciones específicas; `C-c C-x p` es la puerta de entrada.'
    ],
    trap: { bad: 'Escribir `TODO` en medio del texto y esperar que Org lo reconozca como tarea.', good: 'Poner el estado inmediatamente después de los `*` del encabezado: `* TODO Tarea`.', why: 'Org solo interpreta la palabra clave en la posición sintáctica del encabezado.' },
    q: [
      { q: '¿Qué hace `C-c C-t`?', o: ['Rota el estado TODO', 'Inserta una tabla', 'Exporta', 'Abre agenda'], a: 0, why: '`org-todo` cambia el estado de la tarea.' },
      { q: '¿Dónde van las propiedades?', o: ['En un bloque `:PROPERTIES:` debajo del encabezado', 'Al final del archivo', 'En un comentario', 'En la mode line'], a: 0, why: 'Las propiedades se guardan en un drawer inmediatamente debajo del encabezado.' }
    ],
    ex: { p: 'Creá una tarea con deadline, dos tags y una propiedad `:ID:`.', start: '* TODO Mi tarea\n  DEADLINE: <2026-12-01 Tue>', sol: '* TODO Mi tarea :trabajo:urgente:\n  DEADLINE: <2026-12-01 Tue>\n  :PROPERTIES:\n  :ID: mi-tarea\n  :END:', hint: '`C-c C-q` edita tags; `C-c C-x p` agrega propiedades.' },
    cards: [['¿Qué comando pone un deadline?', '`C-c C-d` (`org-deadline`).'], ['¿Qué comando edita tags?', '`C-c C-q` (`org-set-tags-command`).']],
    tags: ['org', 'todo', 'comandos']
  },
  {
    id: 'm06-l03', module: 'm06', n: 3, title: 'Agenda, capture y refile', min: 10, level: 'Intermedio',
    obj: 'Recolectar tareas, ver la agenda y moverlas a su lugar.',
    frase: '`C-c c` captura una idea sin perder el contexto; `C-c a` muestra la agenda; `C-c C-w` la mueve a su proyecto.',
    idea: [
      '`C-c c` (`org-capture`) abre una plantilla de captura. Podés definir plantillas en `org-capture-templates`: tarea, nota, cita, con destino y encabezado. La captura guarda y vuelve a lo que estabas haciendo; es la forma de no perder ideas.',
      '`C-c a` (`org-agenda`) abre el menú de agenda: `a` agenda semanal, `t` tareas, `m` búsqueda por tags, `s` búsqueda por palabra. `C-c C-w` (`org-refile`) mueve el nodo actual a otro archivo/encabezado. `org-agenda-files` define qué archivos entran.'
    ],
    code: { lang: 'elisp', src: '(setq org-agenda-files \'("~/org/inbox.org" "~/org/proyectos.org"))\n(setq org-capture-templates\n      \'(("t" "Tarea" entry (file+headline "~/org/inbox.org" "Tareas")\n         "* TODO %?\\n  %U\\n  %a")))\n;; C-c c t, C-c a a, C-c C-w', cap: 'Agenda y captura mínimas.' },
    under: [
      'La agenda no es una base de datos: recorre `org-agenda-files`, parsea encabezados y fechas, y genera una vista. `org-agenda-span` controla días/semanas; `org-agenda-start-day` el inicio. Los comandos de agenda operan sobre las líneas y pueden cambiar estados.',
      '`org-refile` usa `org-refile-targets` para decidir destinos. Con `C-u C-c C-w` podés refilear a un destino que no está en la lista. Si el destino está en otro archivo, Org lo abre y mueve el subárbol.'
    ],
    trap: { bad: 'Capturar todo en un solo archivo gigante y nunca procesarlo.', good: 'Capturar en `inbox.org` y hacer un refile periódico a proyectos; revisar la agenda una vez al día.', why: 'La captura sin procesamiento se convierte en otro cementerio de notas.' },
    q: [
      { q: '¿Qué hace `C-c a`?', o: ['Abre la agenda', 'Captura', 'Exporta', 'Guarda'], a: 0, why: '`org-agenda` abre el menú de vistas de agenda.' },
      { q: '¿Qué hace `C-c C-w`?', o: ['Refilear el nodo', 'Cerrar', 'Compilar', 'Insertar tabla'], a: 0, why: '`org-refile` mueve el nodo a otro destino.' }
    ],
    ex: { p: 'Definí una plantilla de captura para `inbox.org`, capturá una tarea y refileala a `proyectos.org`.', start: '(setq org-capture-templates\n      \'(("t" "Tarea" entry (file+headline "~/org/inbox.org" "Tareas")\n         "* TODO %?")))', sol: '(setq org-capture-templates\n      \'(("t" "Tarea" entry (file+headline "~/org/inbox.org" "Tareas")\n         "* TODO %?")))\n;; C-c c t\n;; C-c C-w', hint: '`%?` marca dónde queda el cursor; `%a` inserta un enlace al contexto.' },
    cards: [['¿Qué variable define los archivos de la agenda?', '`org-agenda-files`.'], ['¿Qué comando refilear?', '`C-c C-w` (`org-refile`).']],
    tags: ['org', 'agenda', 'comandos']
  },
  {
    id: 'm06-l04', module: 'm06', n: 4, title: 'Tablas, links e imágenes', min: 8, level: 'Intermedio',
    obj: 'Crear tablas alineadas, enlaces internos y previsualizar imágenes.',
    frase: '`C-c |` crea una tabla; `TAB` alinea; `C-c C-l` inserta un link; Org es texto, pero se ve como un documento.',
    idea: [
      '`C-c |` (`org-table-create-or-convert-from-region`) crea una tabla; `TAB`/`S-TAB` navegan celdas y recalculan; `C-c C-c` aplica una fórmula o actualiza. Las fórmulas empiezan con `=` y pueden referirse a rangos (`@2$3`, `$1..$3`). `C-c =` edita la fórmula de la celda.',
      '`C-c C-l` (`org-insert-link`) inserta un enlace; puede ser una URL, un archivo, un encabezado o un `id:`. `C-c C-o` abre el enlace. `org-toggle-inline-images` muestra imágenes locales; `org-display-inline-images` las renderiza. Los enlaces internos usan `[[#id]]` o `[[*Encabezado]]`.'
    ],
    code: { lang: 'org', src: '| Nombre | Edad |\n|--------+------|\n| Ada    |   36 |\n| Grace  |   85 |\n#+TBLFM: $2=vsum(@2$2..@-1$2)\n\n[[https://www.gnu.org/software/emacs/][Emacs]]\n[[file:~/imagen.png]]', cap: 'Tabla con fórmula y enlaces.' },
    under: [
      'Org tables son texto con `|`; el alineado es visual. `org-table-align` rellena espacios; `org-table-recalculate` evalúa fórmulas. Las referencias `@fila$columna` y los rangos permiten cálculos sin salir del archivo.',
      'Los links tienen tipo (`https`, `file`, `id`, `elisp`) y descripción. `org-store-link` (`C-c l`) guarda un enlace al contexto actual; `C-c C-l` lo inserta. Los IDs son estables si usas `org-id`; los enlaces por encabezado se rompen si renombrás.'
    ],
    trap: { bad: 'Editar una tabla a mano y romper la alineación o las fórmulas.', good: 'Usar `TAB` para moverse y `C-c C-c` para recalcular; `C-c -` agrega una línea separadora.', why: 'Org tables dependen de la estructura de `|`; mover texto a mano puede desalinear y romper referencias.' },
    q: [
      { q: '¿Qué hace `C-c |`?', o: ['Crea una tabla', 'Cierra', 'Exporta', 'Inserta imagen'], a: 0, why: '`org-table-create-or-convert-from-region` crea una tabla.' },
      { q: '¿Qué hace `C-c C-l`?', o: ['Inserta un link', 'Compila', 'Abre agenda', 'Guarda'], a: 0, why: '`org-insert-link` inserta un enlace.' }
    ],
    ex: { p: 'Creá una tabla de dos columnas, agregá una fórmula de suma y un enlace a un archivo local.', start: '| A | B |\n|---+---|\n| 1 | 2 |', sol: '| A | B |\n|---+---|\n| 1 | 2 |\n#+TBLFM: $2=vsum(@2$2..@-1$2)\n[[file:~/notas.org][Notas]]', hint: '`C-c C-c` recalcula; `C-c C-o` abre el link.' },
    cards: [['¿Qué hace `TAB` en una tabla Org?', 'Pasa a la siguiente celda y alinea.'], ['¿Qué comando inserta un link?', '`C-c C-l` (`org-insert-link`).']],
    tags: ['org', 'tablas', 'comandos']
  },
  {
    id: 'm06-l05', module: 'm06', n: 5, title: 'Babel, export y publicación', min: 10, level: 'Avanzado',
    obj: 'Ejecutar código dentro de Org, exportar a HTML/PDF y publicar un sitio.',
    frase: 'Org Babel convierte un documento en un cuaderno ejecutable; `C-c C-e` lo exporta a casi cualquier formato.',
    idea: [
      'Un bloque `#+BEGIN_SRC elisp ... #+END_SRC` se ejecuta con `C-c C-c`; `C-c \'` abre un buffer de edición con el modo correcto. Podés pasar variables, capturar resultados y combinarlos: `:results output`, `:exports both`, `:tangle archivo.el`. `org-babel-execute-src-block` es la función.',
      '`C-c C-e` (`org-export-dispatch`) abre el menú de exportación: HTML, PDF (LaTeX), Markdown, ODT, ASCII. `org-publish-project-alist` define un proyecto de publicación; `org-publish` genera el sitio. `org-roam` y `denote` añaden notas enlazadas y búsqueda.'
    ],
    code: { lang: 'org', src: '#+BEGIN_SRC elisp :results output\n(princ \"hola desde Org\")\n#+END_SRC\n\n#+BEGIN_SRC python :results output\nprint(2 + 2)\n#+END_SRC\n\n;; C-c C-c ejecuta; C-c C-e exporta', cap: 'Bloques Babel y exportación.' },
    under: [
      'Babel usa `org-babel-execute:lenguaje` para cada lenguaje. La seguridad importa: `org-confirm-babel-evaluate` pide confirmación antes de ejecutar código. No lo desactives en archivos que no controlás.',
      'La exportación usa backends (`ox-html`, `ox-latex`, `ox-md`). Las opciones `#+OPTIONS` controlan títulos, TOC y números. Para PDF necesitás LaTeX instalado; para HTML, nada extra. `org-reveal` y `org-reveal-export` generan presentaciones.'
    ],
    trap: { bad: 'Ejecutar bloques de un Org descargado sin revisar y desactivar la confirmación de Babel.', good: 'Mantener `org-confirm-babel-evaluate` en `t` y revisar el bloque antes de ejecutarlo.', why: 'Babel ejecuta código arbitrario; un archivo Org es un programa, no solo texto.' },
    q: [
      { q: '¿Qué hace `C-c C-c` en un bloque Babel?', o: ['Ejecuta el bloque', 'Exporta', 'Cierra', 'Borra'], a: 0, why: '`org-babel-execute-src-block` ejecuta el bloque en el lenguaje indicado.' },
      { q: '¿Qué comando abre el menú de exportación?', o: ['`C-c C-e`', '`C-c C-c`', '`C-c a`', '`C-c C-t`'], a: 0, why: '`org-export-dispatch` abre las opciones de exportación.' }
    ],
    ex: { p: 'Creá un bloque Elisp que imprima `hola`, ejecutalo con `C-c C-c` y exportá el archivo a HTML.', start: '#+BEGIN_SRC elisp :results output\n(princ \"hola\")\n#+END_SRC', sol: '#+BEGIN_SRC elisp :results output\n(princ \"hola\")\n#+END_SRC\n;; C-c C-c\n;; C-c C-e h h', hint: 'Si `C-c C-c` no ejecuta, el bloque no está en un modo Babel o falta el lenguaje.' },
    cards: [['¿Qué variable controla la confirmación de Babel?', '`org-confirm-babel-evaluate`.'], ['¿Qué comando exporta?', '`C-c C-e` (`org-export-dispatch`).']],
    tags: ['org', 'babel', 'export', 'experto']
  },
  {
    id: 'm07-l01', module: 'm07', n: 1, title: 'Magit status y staging', min: 8, level: 'Intermedio',
    obj: 'Abrir el estado del repo, navegar secciones y preparar cambios.',
    frase: '`C-x g` abre Magit status: el estado real del repo, con acciones sobre cada sección.',
    idea: [
      '`C-x g` (`magit-status`) abre el buffer de estado. Arriba ves la rama y el remoto; luego `Staged changes`, `Unstaged changes`, `Untracked files`, `Stashes` y `Recent commits`. `TAB` pliega/despliega una sección; `RET` visita el archivo o commit bajo el cursor.',
      '`s` (`magit-stage`) prepara el archivo o hunk bajo el cursor; `u` (`magit-unstage`) lo saca; `S` prepara todo; `U` saca todo. `x` borra algo; `k` descarta cambios. `g` refresca. Magit muestra el diff y te deja operar por hunks, no solo por archivo.'
    ],
    code: { lang: 'text', src: 'C-x g      magit-status\nTAB        plegar sección\ns / u      stage / unstage\nS / U      todo\nRET        visitar\ng          refrescar\nq          cerrar', cap: 'Navegación y staging.' },
    under: [
      'Magit no es un wrapper de texto: habla con Git y parsea la salida, pero presenta secciones con `magit-section`. Cada sección sabe qué acción aplicar. `magit-refresh` reconstruye el buffer; `magit-section-show`/`hide` controlan la visibilidad.',
      'El staging por hunks es la razón principal para usar Magit: podés preparar una parte de un archivo y dejar el resto para otro commit. `s` sobre un hunk, no sobre el archivo, hace exactamente eso.'
    ],
    trap: { bad: 'Pulsar `S` y preparar todo, incluidos archivos que no querías commitear.', good: 'Navegar por archivo/hunk y usar `s` selectivamente; revisar `Staged changes` antes de commitear.', why: '`S` stagea todo el repo; el staging selectivo es la ventaja de Magit.' },
    q: [
      { q: '¿Qué abre `C-x g`?', o: ['Magit status', 'Dired', 'Org agenda', 'Shell'], a: 0, why: '`magit-status` abre el estado del repositorio.' },
      { q: '¿Qué hace `s` en Magit?', o: ['Stagea el archivo/hunk', 'Guarda', 'Sale', 'Hace commit'], a: 0, why: '`magit-stage` prepara el cambio bajo el cursor.' }
    ],
    ex: { p: 'Abrí un repo, modificá un archivo, stagealo con `s`, revisá `Staged changes` y sacalo con `u`.', start: ';; C-x g\n;; s sobre el archivo\n;; TAB en Staged changes\n;; u', sol: ';; C-x g\n;; s\n;; TAB\n;; u', hint: '`S` stagea todo; `U` saca todo. Cuidado con `k`, que descarta.' },
    cards: [['¿Qué hace `TAB` en Magit?', 'Pliega/despliega la sección bajo el cursor.'], ['¿Qué hace `u`?', 'Unstagea el cambio bajo el cursor.']],
    tags: ['magit', 'git', 'comandos']
  },
  {
    id: 'm07-l02', module: 'm07', n: 2, title: 'Commits, amend y log', min: 9, level: 'Intermedio',
    obj: 'Crear commits, enmendar el último y navegar el historial.',
    frase: '`c c` crea un commit; `c a` enmienda el último; `l l` muestra el log con detalle.',
    idea: [
      'Con cambios staged, `c c` (`magit-commit-create`) abre un buffer de mensaje. Escribí el asunto, dejá una línea en blanco y el cuerpo; `C-c C-c` confirma, `C-c C-k` cancela. `c a` (`magit-commit-amend`) enmienda el último commit con lo staged y te deja editar el mensaje.',
      '`l l` (`magit-log-current`) muestra el log de la rama; `l o` el de otra; `l a` todos; `l L` con opciones. `RET` en un commit muestra el diff; `d` lo muestra en otro buffer. `C-c C-d` muestra el diff de un rango. `M-x magit-log` acepta argumentos de `git log`.'
    ],
    code: { lang: 'text', src: 'c c      commit\nC-c C-c  confirmar\nC-c C-k  cancelar\nc a      amend\nl l      log\nRET      ver commit', cap: 'Commits y log.' },
    under: [
      'Magit crea el commit ejecutando Git con un mensaje temporal; el buffer de commit es un `magit-commit-message` con hooks de edición. `commit-msg` hooks de Git siguen funcionando. Si el commit falla, el buffer queda abierto con el error.',
      '`amend` reescribe el último commit: cambia el hash. No lo uses en commits ya publicados sin coordinación. `c w` (`magit-commit-reword`) cambia solo el mensaje; `c e` extiende el commit actual (para `--amend` sin cambiar mensaje, según flujo).'
    ],
    trap: { bad: 'Hacer `amend` de un commit ya pusheado a una rama compartida y forzar push sin avisar.', good: 'Reservar `amend` para commits locales; para historia pública, usar `revert` o un commit nuevo.', why: 'Amend cambia el hash y reescribe historia; los demás ya tienen el commit viejo.' },
    q: [
      { q: '¿Qué hace `c c` en Magit?', o: ['Crea un commit', 'Borra', 'Hace checkout', 'Rebase'], a: 0, why: '`magit-commit-create` abre el buffer de mensaje de commit.' },
      { q: '¿Qué hace `c a`?', o: ['Enmienda el último commit', 'Crea una rama', 'Hace merge', 'Push'], a: 0, why: '`magit-commit-amend` reescribe el último commit con lo staged.' }
    ],
    ex: { p: 'Modificá un archivo, stageá, hacé un commit con `c c`, cambiá el mensaje con `c w` y mirá el log con `l l`.', start: ';; C-x g\n;; s\n;; c c\n;; mensaje\n;; C-c C-c\n;; l l', sol: ';; C-x g\n;; s\n;; c c\n;; mensaje\n;; C-c C-c\n;; c w\n;; l l', hint: '`C-c C-k` cancela el mensaje y vuelve al status.' },
    cards: [['¿Qué confirma el mensaje de commit?', '`C-c C-c`.'], ['¿Qué comando cambia solo el mensaje del último commit?', '`c w` (`magit-commit-reword`).']],
    tags: ['magit', 'git', 'comandos']
  },
  {
    id: 'm07-l03', module: 'm07', n: 3, title: 'Diff, hunks y blame', min: 9, level: 'Intermedio',
    obj: 'Leer diffs, stagear hunks y encontrar al autor de una línea.',
    frase: 'Magit muestra el diff como secciones accionables; `s` sobre un hunk stagea solo esa parte.',
    idea: [
      'En `magit-status`, `d d` muestra el diff del archivo; `d s` el diff staged; `d u` el unstaged. `TAB` pliega/despliega hunks. `s` stagea el hunk bajo el cursor; `u` lo saca. `RET` en un hunk puede abrir el archivo en esa posición. `C-c C-d` muestra el diff de un rango de commits.',
      '`M-x magit-blame` (o `b` en un archivo) muestra el blame inline: cada línea con commit, autor y fecha. `magit-blame-addition`/`removal` cambian la vista. `M-x magit-log-buffer-file` muestra el historial del archivo actual. `C-c C-s`? No: usá `magit-diff` y `magit-blame` por nombre si dudás.'
    ],
    code: { lang: 'text', src: 'd d      diff del archivo\nd s      staged\nd u      unstaged\nTAB      plegar hunk\ns        stagear hunk\nM-x magit-blame', cap: 'Diff y blame.' },
    under: [
      'Magit genera diffs con `git diff` y los presenta en secciones. Un hunk es un bloque de líneas con contexto; stagear un hunk aplica ese fragmento al índice con `git apply --cached`. Por eso podés preparar cambios parciales sin editar el archivo.',
      'Blame usa `git blame` y puede ser lento en archivos grandes. `magit-blame` tiene modos de visualización y permite navegar al commit. `magit-log-buffer-file` combina log y diff para seguir la evolución.'
    ],
    trap: { bad: 'Stagear un archivo entero cuando solo querías una corrección y mezclar cambios no relacionados.', good: 'Entrar al diff, plegar y stagear el hunk correcto con `s`; revisar `Staged changes` antes del commit.', why: 'El staging por hunks existe para commits atómicos y revisables.' },
    q: [
      { q: '¿Qué hace `s` sobre un hunk?', o: ['Stagea solo ese hunk', 'Borra el archivo', 'Hace commit', 'Push'], a: 0, why: 'Magit aplica ese fragmento al índice.' },
      { q: '¿Qué muestra `magit-blame`?', o: ['El autor y commit de cada línea', 'El log completo', 'Las ramas', 'Los remotos'], a: 0, why: 'Blame anota cada línea con su último commit.' }
    ],
    ex: { p: 'Hacé dos cambios separados en un archivo, stageá solo uno con `s` sobre el hunk y commiteá.', start: ';; C-x g\n;; d d\n;; TAB\n;; s en un hunk\n;; c c', sol: ';; C-x g\n;; d d\n;; TAB\n;; s\n;; c c\n;; mensaje\n;; C-c C-c', hint: '`u` saca el hunk del índice si te equivocaste.' },
    cards: [['¿Qué hace `d d`?', 'Muestra el diff del archivo bajo el cursor.'], ['¿Qué comando muestra el historial de un archivo?', '`M-x magit-log-buffer-file`.']],
    tags: ['magit', 'git', 'comandos']
  },
  {
    id: 'm07-l04', module: 'm07', n: 4, title: 'Ramas, merge, rebase y conflictos', min: 10, level: 'Avanzado',
    obj: 'Crear y cambiar ramas, integrar historia y resolver conflictos con smerge/ediff.',
    frase: '`b b` cambia de rama; `m` hace merge; `r r` rebase. Los conflictos se resuelven con `smerge` o `ediff`.',
    idea: [
      '`b b` (`magit-checkout`) cambia de rama; `b c` (`magit-branch-create`) crea una; `b d` borra; `b r` renombra. `m` (`magit-merge`) integra otra rama en la actual. `r r` (`magit-rebase`) reaplica commits; `r i` abre el rebase interactivo. `F` pull, `P` push, `f` fetch.',
      'Si hay conflictos, Git deja marcadores `<<<<<<<`, `=======`, `>>>>>>>`. Magit los muestra; `M-x smerge-mode` ofrece `smerge-keep-current`, `smerge-keep-other`, `smerge-keep-all`, `smerge-next`. `M-x ediff` o `M-x ediff-merge-files` da una vista de tres vías. Después `git add` y continuar.'
    ],
    code: { lang: 'text', src: 'b b      checkout\nb c      crear rama\nm        merge\nr r      rebase\nr i      rebase interactivo\nsmerge-next / smerge-keep-all\nC-c C-c  continuar rebase', cap: 'Ramas, integración y conflictos.' },
    under: [
      'Rebase reescribe commits: cambia hashes. Si la rama es compartida, preferí merge. `magit-rebase-interactive` permite `pick`, `reword`, `edit`, `squash`, `fixup`, `drop`. Los conflictos se resuelven commit a commit.',
      '`smerge-mode` detecta los marcadores y ofrece comandos de resolución. `ediff` muestra las tres versiones (base, ours, theirs) y permite combinar. Al terminar, `git add` marca el conflicto como resuelto y `git rebase --continue` sigue.'
    ],
    trap: { bad: 'Hacer rebase de una rama pública y force-push sin coordinar con el equipo.', good: 'Rebase solo en ramas locales; en ramas compartidas, merge o revert.', why: 'Rebase reescribe la historia y obliga a todos a reconciliar hashes.' },
    q: [
      { q: '¿Qué hace `m` en Magit?', o: ['Merge', 'Rebase', 'Commit', 'Push'], a: 0, why: '`magit-merge` integra otra rama en la actual.' },
      { q: '¿Qué comando resuelve un conflicto quedándose con una versión?', o: ['`smerge-keep-other`', '`magit-stage`', '`git commit`', '`C-x C-c`'], a: 0, why: '`smerge` ofrece keep-current/other/all para resolver marcadores.' }
    ],
    ex: { p: 'Creá una rama, hacé un commit, volvé a main, mergeá y resolvé un conflicto simulado con smerge.', start: ';; b c feature RET\n;; commit\n;; b b main RET\n;; m feature RET', sol: ';; b c feature RET\n;; edita, s, c c, mensaje, C-c C-c\n;; b b main RET\n;; m feature RET\n;; si hay conflicto: M-x smerge-mode, smerge-keep-all, C-c C-c', hint: '`C-c C-c` en el buffer de merge/rebase confirma la resolución.' },
    cards: [['¿Qué hace `r r`?', 'Rebase de la rama actual.'], ['¿Qué modo resuelve conflictos?', '`smerge-mode`, con `smerge-keep-*` y `smerge-next`.']],
    tags: ['magit', 'git', 'experto']
  },
  {
    id: 'm07-l05', module: 'm07', n: 5, title: 'Forge, PR y CI', min: 8, level: 'Avanzado',
    obj: 'Gestionar pull requests, issues y checks desde Emacs con Forge.',
    frase: 'Forge convierte GitHub/GitLab en secciones de Magit: PRs, issues y CI sin abrir el navegador.',
    idea: [
      '`M-x forge-dispatch` abre el menú de Forge. `forge-list-pullreqs` lista PRs; `forge-create-pullreq` crea uno; `forge-visit-pullreq` abre el detalle. `forge-list-issues` y `forge-create-issue` hacen lo mismo con issues. Forge necesita un token configurado en `~/.authinfo.gpg` o `auth-source`.',
      'En un PR, `RET` visita el diff; `c` comenta; `e` edita; `M-x forge-refresh-pullreq` actualiza. Los checks de CI aparecen como secciones; `RET` abre el log. `magit-section` permite plegar y navegar. Forge no reemplaza a Git: lo complementa.'
    ],
    code: { lang: 'text', src: 'M-x forge-dispatch\nM-x forge-list-pullreqs\nM-x forge-create-pullreq\nM-x forge-list-issues\n;; token en ~/.authinfo.gpg', cap: 'Forge: PRs e issues.' },
    under: [
      'Forge usa la API de GitHub/GitLab y guarda metadatos localmente en un repositorio de datos. `forge-database-file` define dónde. `forge-add-repository` registra un repo; `forge-pull` sincroniza. Los tokens se manejan con `auth-source`, no en texto plano.',
      'La integración con CI depende del proveedor. GitHub Actions se muestra como checks; GitLab pipelines también. Para logs completos, Forge abre una URL o descarga el log. No es un reemplazo del panel web para configuraciones avanzadas.'
    ],
    trap: { bad: 'Guardar el token de GitHub en `init.el` en texto plano y subirlo a un repo público.', good: 'Usar `~/.authinfo.gpg` o `auth-source` con `gpg`, y nunca commitear credenciales.', why: 'Un token filtrado da acceso a tu cuenta y a todos tus repos.' },
    q: [
      { q: '¿Qué hace `forge-list-pullreqs`?', o: ['Lista PRs', 'Borra el repo', 'Hace commit', 'Push'], a: 0, why: 'Forge lista los pull requests del repositorio.' },
      { q: '¿Dónde debería vivir el token?', o: ['En `init.el`', 'En `~/.authinfo.gpg` o auth-source', 'En el README', 'En un commit'], a: 1, why: 'Las credenciales van cifradas y fuera del control de versiones.' }
    ],
    ex: { p: 'Configurá Forge con un token, listá PRs y abrí uno para ver el diff y los checks.', start: 'M-x forge-add-repository\nM-x forge-list-pullreqs\nRET', sol: 'M-x forge-add-repository\nM-x forge-list-pullreqs\nRET\n;; c para comentar, q para cerrar', hint: 'Si no hay token, Forge pide configurarlo; usá `auth-source`. ' },
    cards: [['¿Qué comando crea un PR?', '`M-x forge-create-pullreq`.'], ['¿Qué archivo se recomienda para tokens?', '`~/.authinfo.gpg` (cifrado con GPG).']],
    tags: ['magit', 'forge', 'git', 'experto']
  }
];

export const lessons1 = [
  {
    id: 'm00-l01', module: 'm00', n: 1, title: 'Qué es Emacs (y qué no)', min: 8, level: 'Base',
    obj: 'Entender por qué Emacs no es “un editor con plugins” y qué implica que sea un intérprete de Lisp.',
    frase: 'Emacs es un intérprete de Emacs Lisp con un editor encima. Cada tecla ejecuta una función, y puedes cambiar esa función.',
    idea: [
      'Emacs arranca un intérprete de **Emacs Lisp** y entra en un bucle: lee una tecla, busca qué comando corresponde y lo ejecuta. El editor es la interfaz, pero el motor es Lisp. Por eso se puede extender sin recompilar: defines una función, la enlazas a una tecla y ya forma parte del editor.',
      'Eso no significa que sea fácil de fábrica. Emacs no es un IDE con todo integrado: es una plataforma que trae lo básico y una comunidad enorme de paquetes. La ventaja es que puedes inspeccionar y cambiar casi cualquier cosa; la desventaja es que al principio hay que aprender el modelo, no solo los atajos.'
    ],
    code: { lang: 'elisp', src: '(emacs-version)\n;; => "30.2" (o la versión que tengas)\n\n(current-buffer)\n;; => #<buffer *scratch*>', cap: 'Evalúa con C-x C-e al final de cada línea en *scratch*.' },
    under: [
      'El **command loop** lee una secuencia de teclas, la resuelve contra los **keymaps** activos y llama a un comando interactivo. Un comando no es más que una función con `(interactive)`. Si una tecla no hace lo que esperas, puedes preguntar qué comando ejecuta con `C-h k`.',
      'Los buffers son objetos Lisp que contienen texto; las ventanas muestran buffers; los frames son ventanas del sistema operativo. Casi todo lo que ves es un objeto que puedes inspeccionar con `C-h v`, `C-h f` o `M-:`.'
    ],
    trap: { bad: 'Instalar una distribución enorme y copiar 800 líneas de configuración antes de entender qué hace cada cosa.', good: 'Empezar con `emacs -Q`, aprender el modelo y añadir configuración cuando sepas qué problema resuelve.', why: 'Una configuración que no entendés es deuda técnica: cuando algo falla, no sabés dónde mirar.' },
    q: [
      { q: '¿Qué es Emacs en su núcleo?', o: ['Un editor con plugins', 'Un intérprete de Lisp con un editor encima', 'Un IDE cerrado', 'Un lenguaje compilado'], a: 1, why: 'Emacs es un intérprete de Emacs Lisp; el editor es la interfaz que usa ese intérprete.' },
      { q: '¿Qué hace una tecla en Emacs?', o: ['Escribe un carácter fijo', 'Ejecuta un comando interactivo', 'Abre un menú', 'Compila el buffer'], a: 1, why: 'Cada secuencia se resuelve contra keymaps y termina llamando a un comando interactivo.' }
    ],
    ex: { p: 'Evalúa `(emacs-version)` y `(buffer-name)` en `*scratch*`. Después pregunta con `C-h k` qué comando ejecuta `C-x C-f`.', start: '(emacs-version)\n(buffer-name)', sol: '(emacs-version)\n(buffer-name)\n;; C-h k C-x C-f muestra: C-x C-f runs the command find-file', hint: 'Usa `C-x C-e` al final de cada forma o `M-:` para evaluar una expresión.' },
    cards: [['¿Qué motor hay debajo de Emacs?', 'Un intérprete de Emacs Lisp y un command loop.'], ['¿Qué es un comando?', 'Una función Lisp marcada con `(interactive)` que puede invocarse desde una tecla o `M-x`.']],
    tags: ['modelo', 'comandos', 'comandos']
  },
  {
    id: 'm00-l02', module: 'm00', n: 2, title: 'Instalar, arrancar y salir', min: 9, level: 'Base',
    obj: 'Instalar Emacs, arrancarlo sin configuración, usar el daemon y salir sin perder trabajo.',
    frase: '`emacs -Q` arranca sin tu configuración. Es el mejor diagnóstico cuando algo se rompe.',
    idea: [
      'En Windows podés instalar Emacs desde el instalador oficial o con `winget install GNU.Emacs`. En macOS, `brew install --cask emacs`. En Linux, el paquete de tu distribución o una compilación con native-comp. La versión importa: 29.1 o superior trae mejor soporte de treesit y `use-package` integrado.',
      'Hay dos formas de arrancar: una instancia normal (`emacs`) o un **daemon** en segundo plano (`emacs --daemon`) al que te conectás con `emacsclient -c`. El daemon arranca una vez y abre frames casi instantáneos; es la forma recomendada si usás Emacs todo el día.'
    ],
    code: { lang: 'bash', src: 'emacs -Q                 # sin init: para diagnosticar\nemacs --daemon           # servidor en segundo plano\nemacsclient -c           # abre un frame nuevo\nemacsclient -e "(+ 1 2)" # evalúa una expresión en el daemon', cap: 'Arranque normal, daemon y cliente.' },
    under: [
      'La configuración vive en `~/.emacs.d/init.el` o `~/.config/emacs/init.el`. `-Q` ignora todo eso y también los paquetes del sitio. Si Emacs arranca con `-Q` pero falla con tu configuración, el problema está en tu init o en un paquete.',
      '`C-x C-c` (`save-buffers-kill-terminal`) pregunta por los buffers modificados y cierra el frame. Si estás en una terminal, `C-z` suspende Emacs; para volver, `fg`. En el daemon, cerrar el frame no mata el servidor: los buffers siguen vivos.'
    ],
    trap: { bad: 'Cerrar la ventana del sistema operativo y asumir que Emacs guardó todo.', good: 'Salir con `C-x C-c` o revisar `C-x s` (`save-some-buffers`) antes de cerrar.', why: 'Cerrar un frame del daemon no cierra la sesión; matar el proceso sin guardar puede perder cambios.' },
    q: [
      { q: '¿Qué hace `emacs -Q`?', o: ['Abre un archivo llamado Q', 'Arranca sin configuración de usuario', 'Cierra Emacs', 'Instala paquetes'], a: 1, why: '`-Q` ignora init files y paquetes del sitio; sirve para aislar problemas.' },
      { q: '¿Qué ventaja tiene `emacs --daemon` + `emacsclient -c`?', o: ['Menos memoria y arranque instantáneo de frames', 'Compila más rápido', 'Cierra los buffers al cerrar el frame', 'Desactiva Lisp'], a: 0, why: 'El daemon mantiene el estado y los frames nuevos se abren sin recargar todo.' }
    ],
    ex: { p: 'Arranca `emacs -Q`, abrí `*scratch*` y evaluá `(emacs-version)`. Después salí con `C-x C-c`.', start: ';; En *scratch*\n(emacs-version)', sol: '(emacs-version)\n;; C-x C-e\n;; C-x C-c para salir', hint: 'Si estás en terminal y `C-x C-c` no responde, `C-g` para cancelar y reintentar.' },
    cards: [['¿Qué comando guarda todos los buffers modificados?', '`C-x s` (`save-some-buffers`).'], ['¿Qué hace `C-x C-c`?', 'Sale de Emacs; pregunta por buffers sin guardar.']],
    tags: ['instalacion', 'daemon', 'comandos']
  },
  {
    id: 'm00-l03', module: 'm00', n: 3, title: 'Frame, ventana, buffer y mode line', min: 9, level: 'Base',
    obj: 'Distinguir los cuatro objetos que confunden a todo el mundo y leer la mode line.',
    frase: 'Un buffer es texto en memoria; una ventana es una vista; un frame es la ventana del sistema operativo; la mode line es el estado.',
    idea: [
      'El **buffer** es el texto. Puede estar asociado a un archivo o no (`*scratch*`, `*Messages*`). La **ventana** muestra un buffer; podés tener varias ventanas con el mismo buffer o buffers distintos. El **frame** es la ventana del sistema operativo que contiene ventanas de Emacs. La **mode line** es la línea de estado debajo de cada ventana.',
      'La mode line te dice mucho: nombre del buffer, modo mayor, modos menores, si está modificado (`**`), la posición y el porcentaje. La **echo area**, abajo, muestra mensajes y el **minibuffer** pide entrada. Si no sabés dónde estás, mirá la mode line.'
    ],
    code: { lang: 'elisp', src: '(current-buffer)   ; => #<buffer *scratch*>\n(buffer-name)      ; => "*scratch*"\n(selected-window)  ; => #<window 3 on *scratch*>\n(frame-list)       ; => lista de frames', cap: 'Los objetos que puedes inspeccionar desde Lisp.' },
    under: [
      'Un buffer puede estar visible en cero, una o varias ventanas. Matar el buffer (`C-x k`) no cierra la ventana: la ventana pasa a mostrar otro buffer. Matar la ventana (`C-x 0`) no mata el buffer: el texto sigue vivo.',
      'La mode line se construye con `mode-line-format`; podés personalizarla, pero primero aprendé a leerla. `**` significa modificado; `%%` es el porcentaje; `(Fundamental)` o `(Org)` es el modo mayor; los modos menores aparecen a la derecha.'
    ],
    trap: { bad: 'Confundir “cerrar la ventana” con “cerrar el archivo” y perder de vista un buffer modificado.', good: 'Usar `C-x C-b` o `M-x ibuffer` para ver buffers y su estado; `C-x 0` solo cierra la vista.', why: 'El buffer es el dato; la ventana es solo una vista. Cerrar la vista no guarda ni borra el dato.' },
    q: [
      { q: '¿Qué es un buffer?', o: ['Una ventana del sistema operativo', 'Texto en memoria, con o sin archivo', 'Una tecla', 'Un paquete'], a: 1, why: 'El buffer contiene texto; puede estar asociado a un archivo o no.' },
      { q: '¿Qué indica `**` en la mode line?', o: ['Buffer de solo lectura', 'Buffer modificado sin guardar', 'Modo mayor activo', 'Error de compilación'], a: 1, why: '`**` marca cambios sin guardar; `%%` es la posición y `%*` el estado.' }
    ],
    ex: { p: 'Abrí `*scratch*`, creá un buffer nuevo con `(get-buffer-create "*prueba*")`, cambiate con `C-x b` y mirá la mode line.', start: '(get-buffer-create "*prueba*")\n;; C-x b *prueba* RET', sol: '(get-buffer-create "*prueba*")\n;; C-x b *prueba* RET\n;; C-x k para matarlo', hint: '`C-x b` acepta el nombre con autocompletado; `C-x C-b` lista todos.' },
    cards: [['Diferencia entre buffer y ventana', 'El buffer es el texto; la ventana es una vista que lo muestra.'], ['Qué hace `C-x 0`', 'Cierra la ventana actual, no el buffer.']],
    tags: ['buffers', 'ventanas', 'modelo']
  },
  {
    id: 'm00-l04', module: 'm00', n: 4, title: 'Comandos, teclas y M-x', min: 10, level: 'Base',
    obj: 'Leer la notación de teclas, invocar comandos por nombre y crear tu primer atajo.',
    frase: '`M-x` es la puerta de entrada a todo: si sabés el nombre del comando, no necesitás recordar la tecla.',
    idea: [
      'La notación de Emacs: `C-` es Control, `M-` es Meta (Alt o Escape), `S-` es Shift, `C-M-` combina ambos. `C-x` es un **prefijo**: no hace nada por sí solo, espera la siguiente tecla. `C-c` está reservado para el usuario; por convención, tus atajos empiezan con `C-c` y una letra.',
      'Un comando tiene nombre y se invoca con `M-x nombre`. Los atajos son solo una forma cómoda de llamarlo. Si no recordás la tecla, `M-x` con autocompletado te salva; si querés saber qué comando hay detrás de una tecla, `C-h k` y pulsá la tecla.'
    ],
    code: { lang: 'elisp', src: '(defun mi-saludo ()\n  "Muestra un saludo."\n  (interactive)\n  (message "Hola desde Emacs Lisp"))\n\n(global-set-key (kbd "C-c h") #\'mi-saludo)', cap: 'Definir un comando y enlazarlo a C-c h.' },
    under: [
      'Los **keymaps** son tablas que asocian secuencias de teclas con comandos. Hay keymaps globales y locales por modo. `global-set-key` modifica el global; `local-set-key` el del modo actual; `define-key` permite elegir el keymap exacto.',
      '`C-h w` (`where-is`) te dice en qué teclas está un comando. `C-h k` (`describe-key`) te dice qué comando ejecuta una tecla. Esas dos preguntas resuelven casi todo el descubrimiento inicial.'
    ],
    trap: { bad: 'Enlazar comandos a `C-c` solo o a teclas que ya usa el modo mayor.', good: 'Usar `C-c` + letra para tus comandos y `C-h w` para comprobar que la tecla está libre.', why: '`C-c` es un prefijo reservado al usuario; pisar teclas de un modo rompe paquetes y flujos existentes.' },
    q: [
      { q: '¿Qué significa `C-c h`?', o: ['Control + c, luego h', 'Control + h', 'Meta + c + h', 'Shift + c + h'], a: 0, why: '`C-c` es un prefijo; después se pulsa `h`.' },
      { q: '¿Qué comando te dice qué función ejecuta una tecla?', o: ['`C-h f`', '`C-h k`', '`C-h v`', '`C-h m`'], a: 1, why: '`C-h k` describe la tecla y muestra el comando enlazado.' }
    ],
    ex: { p: 'Definí un comando `mi-nombre` que muestre tu nombre con `message` y enlazalo a `C-c n`.', start: '(defun mi-nombre ()\n  (interactive)\n  (message "Tu nombre"))', sol: '(defun mi-nombre ()\n  "Muestra mi nombre."\n  (interactive)\n  (message "Ada Lovelace"))\n(global-set-key (kbd "C-c n") #\'mi-nombre)', hint: '`(interactive)` es lo que convierte la función en comando; sin eso, `M-x` no la lista.' },
    cards: [['¿Qué hace `M-x`?', 'Pide el nombre de un comando y lo ejecuta.'], ['¿Para qué está reservado `C-c` + letra?', 'Para atajos del usuario.']],
    tags: ['comandos', 'keymaps', 'comandos']
  },
  {
    id: 'm00-l05', module: 'm00', n: 5, title: 'La ayuda como superpoder', min: 8, level: 'Base',
    obj: 'Usar el sistema de ayuda para responder cualquier duda sin salir de Emacs.',
    frase: 'Emacs es autodocumentado: cada función, variable y tecla lleva su manual incorporado.',
    idea: [
      '`C-h k` describe una tecla, `C-h f` una función, `C-h v` una variable, `C-h m` el modo actual, `C-h b` los atajos del buffer, `C-h o` el símbolo bajo el cursor y `C-h e` el buffer de mensajes. `C-h i` abre el manual Info; `M-x apropos` busca por patrón en todos los símbolos.',
      'La ayuda no es un extra: es la forma normal de trabajar. Si no entendés un mensaje de error, `C-h e` te muestra el mensaje completo; si una función no hace lo que esperás, `C-h f` te muestra su docstring y su firma.'
    ],
    code: { lang: 'elisp', src: '(describe-function #\'save-buffer)\n(describe-variable \'kill-ring-max)\n(apropos "buffer")', cap: 'También puedes invocarlas con C-h f, C-h v y M-x apropos.' },
    under: [
      'Cada función y variable tiene un **docstring** y un **symbol property**. `C-h f` lee la definición y muestra la documentación; `C-h v` muestra el valor actual y su tipo. `C-h o` (`describe-symbol`) encuentra todo lo que se llama igual: función, variable, cara o tecla.',
      '`M-x apropos` busca en nombres y docstrings. `M-x apropos-documentation` busca solo en documentación. Son la forma de descubrir funciones que no sabías que existían.'
    ],
    trap: { bad: 'Buscar en Google un error de Emacs antes de leer `C-h e` y el docstring.', good: 'Leer el mensaje completo en `*Messages*`, después `C-h f`/`C-h v` del símbolo implicado.', why: 'La respuesta suele estar en el propio Emacs y es específica de tu versión y configuración.' },
    q: [
      { q: '¿Qué muestra `C-h f`?', o: ['Las teclas libres', 'La documentación de una función', 'El manual completo', 'Los buffers abiertos'], a: 1, why: '`describe-function` muestra firma, docstring y archivo de definición.' },
      { q: '¿Qué comando busca por patrón en nombres y documentación?', o: ['`M-x apropos`', '`C-h k`', '`C-x b`', '`M-x compile`'], a: 0, why: '`apropos` busca símbolos cuyo nombre o documentación coincide con el patrón.' }
    ],
    ex: { p: 'Usá `C-h f` para leer la documentación de `kill-ring-save` y `C-h v` para `kill-ring-max`.', start: ';; C-h f kill-ring-save RET\n;; C-h v kill-ring-max RET', sol: ';; C-h f kill-ring-save RET\n;; C-h v kill-ring-max RET\n;; C-h o kill-ring RET', hint: '`C-h o` es útil cuando no sabés si algo es función, variable o cara.' },
    cards: [['¿Qué muestra `C-h k`?', 'El comando que ejecuta una tecla y su documentación.'], ['¿Qué abre `C-h i`?', 'El manual Info de Emacs.']],
    tags: ['ayuda', 'comandos', 'comandos']
  },
  {
    id: 'm01-l01', module: 'm01', n: 1, title: 'Abrir, guardar y cerrar', min: 7, level: 'Base',
    obj: 'Visitar archivos, guardarlos, guardarlos como y cerrar sin perder cambios.',
    frase: '`C-x C-f` visita un archivo; si no existe, lo crea. `C-x C-s` guarda; `C-x C-w` guarda como.',
    idea: [
      '`C-x C-f` (`find-file`) pide una ruta y abre un buffer asociado a ese archivo. Si el archivo no existe, el buffer queda vacío y se crea al guardar. `C-x C-s` (`save-buffer`) escribe el buffer en su archivo; `C-x C-w` (`write-file`) pide otro nombre y cambia la asociación.',
      '`C-x s` (`save-some-buffers`) recorre todos los buffers modificados y pregunta qué hacer. `C-x C-c` sale y pregunta por cada uno. Si el buffer no está asociado a un archivo, `C-x C-s` te pide una ruta.'
    ],
    code: { lang: 'elisp', src: '(find-file "~/notas.org")\n(save-buffer)\n(write-file "~/notas-copia.org")', cap: 'Las mismas operaciones desde Lisp.' },
    under: [
      'Visitar un archivo crea un buffer con `buffer-file-name` apuntando a la ruta. Guardar escribe el contenido y limpia el flag de modificación. Emacs guarda una copia de seguridad (`archivo~`) y, si `auto-save-mode` está activo, un auto-save (`#archivo#`) cada cierto tiempo.',
      'La ruta se interpreta con `~` como home y variables de entorno. En Windows, las barras invertidas se pueden escribir con `/`; Emacs las normaliza. `C-x C-f` tiene autocompletado y permite navegar directorios con `RET`.'
    ],
    trap: { bad: 'Guardar con `C-x C-s` un buffer que no está asociado a ningún archivo y sorprenderte de que pida una ruta.', good: 'Usar `C-x C-w` para “guardar como” y verificar la ruta en la mode line.', why: '`C-x C-s` guarda en `buffer-file-name`; si es `nil`, no hay archivo destino.' },
    q: [
      { q: '¿Qué hace `C-x C-f` si el archivo no existe?', o: ['Error', 'Abre un buffer vacío que se crea al guardar', 'Lo crea vacío en disco', 'Pide permisos'], a: 1, why: '`find-file` visita el archivo; el buffer existe aunque el archivo todavía no.' },
      { q: '¿Qué diferencia hay entre `C-x C-s` y `C-x C-w`?', o: ['Ninguna', '`C-x C-s` guarda en el archivo actual; `C-x C-w` pide otro nombre', '`C-x C-w` cierra', '`C-x C-s` compila'], a: 1, why: '`save-buffer` escribe en el archivo visitado; `write-file` cambia el archivo asociado.' }
    ],
    ex: { p: 'Creá `~/emacs-prueba.txt`, escribí una línea, guardá con `C-x C-s` y comprobá el archivo en el sistema.', start: ';; C-x C-f ~/emacs-prueba.txt RET\n;; escribe una línea\n;; C-x C-s', sol: ';; C-x C-f ~/emacs-prueba.txt RET\n;; Hola Emacs\n;; C-x C-s', hint: 'Si el buffer no tiene archivo, `C-x C-w` te pide la ruta.' },
    cards: [['¿Qué comando guarda todos los buffers modificados?', '`C-x s` (`save-some-buffers`).'], ['¿Qué es `buffer-file-name`?', 'La ruta del archivo asociado al buffer, o `nil` si no tiene.']],
    tags: ['archivos', 'comandos', 'comandos']
  },
  {
    id: 'm01-l02', module: 'm01', n: 2, title: 'Buffers: el espacio de trabajo real', min: 8, level: 'Base',
    obj: 'Cambiar, listar, enterrar y matar buffers sin perder el contexto.',
    frase: 'En Emacs no cambiás de archivo: cambiás de buffer. Un buffer puede no tener archivo.',
    idea: [
      '`C-x b` (`switch-to-buffer`) pide un nombre con autocompletado. `C-x C-b` (`list-buffers`) abre una lista; `M-x ibuffer` es la versión potente, con filtros, marcas y operaciones en lote. `C-x k` (`kill-buffer`) mata el buffer actual; si está modificado, pregunta.',
      'Los buffers especiales (`*Messages*`, `*Help*`, `*Compile-Log*`, `*scratch*`) no tienen archivo y son parte del flujo de trabajo. `M-x bury-buffer` los manda al final de la lista sin matarlos; `M-x unbury-buffer` vuelve.'
    ],
    code: { lang: 'elisp', src: '(get-buffer-create "*notas*")\n(with-current-buffer "*notas*"\n  (insert "hola"))\n(buffer-list)', cap: 'Crear un buffer, escribir dentro y listar buffers.' },
    under: [
      'Cada buffer tiene variables locales (`buffer-local`) que no afectan a los demás: modo mayor, `tab-width`, `buffer-file-name`, etc. `setq` en un buffer puede crear una variable local si la variable está marcada como automática (`make-variable-buffer-local`).',
      'Matar un buffer libera su texto, pero no borra el archivo. Si el buffer tenía un proceso asociado, el proceso puede seguir vivo; `M-x list-processes` lo muestra. `ibuffer` permite marcar varios buffers y matarlos, guardarlos o filtrarlos por modo.'
    ],
    trap: { bad: 'Cerrar la ventana con `C-x 0` y creer que cerraste el archivo.', good: 'Usar `C-x k` para matar el buffer y `C-x 0` solo para cerrar la vista.', why: 'La ventana es una vista; el buffer sigue vivo y puede seguir modificado.' },
    q: [
      { q: '¿Qué hace `C-x k`?', o: ['Cierra la ventana', 'Mata el buffer actual', 'Cierra Emacs', 'Guarda el buffer'], a: 1, why: '`kill-buffer` elimina el buffer; si hay cambios, pide confirmación.' },
      { q: '¿Qué es un buffer especial?', o: ['Un buffer sin archivo, como `*Messages*`', 'Un buffer de solo lectura', 'Un archivo binario', 'Un modo mayor'], a: 0, why: 'Los buffers especiales suelen no tener archivo y los crea Emacs para su propia salida.' }
    ],
    ex: { p: 'Creá un buffer `*borrador*`, escribí texto, cambiate a otro buffer y volvé con `C-x b`. Después matalo con `C-x k`.', start: ';; C-x b *borrador* RET\n;; escribe\n;; C-x b *scratch* RET\n;; C-x b *borrador* RET\n;; C-x k', sol: ';; C-x b *borrador* RET\n;; texto\n;; C-x b *scratch* RET\n;; C-x b *borrador* RET\n;; C-x k RET', hint: '`M-x ibuffer` te da una vista completa con marcas y filtros.' },
    cards: [['Diferencia entre matar buffer y cerrar ventana', 'Matar buffer elimina el texto; cerrar ventana solo la vista.'], ['¿Qué comando lista buffers con filtros?', '`M-x ibuffer`.']],
    tags: ['buffers', 'comandos', 'comandos']
  },
  {
    id: 'm01-l03', module: 'm01', n: 3, title: 'Undo, redo y auto-save', min: 8, level: 'Base',
    obj: 'Deshacer y rehacer en el árbol de undo, y recuperar trabajo desde auto-save o backup.',
    frase: 'El undo de Emacs es un árbol, no una pila: podés deshacer un cambio antiguo y crear una rama nueva.',
    idea: [
      '`C-/` o `C-_` (`undo`) deshace; `C-x u` es un alias. Para rehacer, `M-x undo-redo` (o `C-?` en versiones recientes). A diferencia de otros editores, el historial no es lineal: cada grupo de cambios es un nodo y podés moverte por ramas.',
      '`auto-save-mode` guarda periódicamente en `#archivo#` cuando hay cambios; `M-x recover-file` recupera. Los backups (`archivo~`) se crean al guardar, según `backup-by-copying` y `kept-new-versions`. Si Emacs se cierra, `M-x recover-session` puede recuperar buffers.'
    ],
    code: { lang: 'elisp', src: '(undo)\n(undo-redo)\n(recover-file "~/notas.org")\n;; auto-save-file-name-p', cap: 'Comandos de undo y recuperación.' },
    under: [
      'El **undo ring** guarda cambios con posiciones y texto. `undo-boundary` separa grupos; por eso un `C-/` deshace una palabra o una inserción completa, no un carácter. `undo-tree` es un paquete que visualiza el árbol, pero el mecanismo base ya es un árbol.',
      'El auto-save no reemplaza al guardado: es una red de seguridad. Si el archivo original está bien, `recover-file` te muestra el auto-save y te deja comparar. Los backups son copias de la versión anterior, no del estado actual.'
    ],
    trap: { bad: 'Deshacer muchas veces y luego escribir: el nuevo cambio crea una rama y el “redo” anterior ya no está en la línea principal.', good: 'Usar `undo-redo` para moverte por el historial y guardar puntos de control con commits o copias.', why: 'El undo tree conserva ramas; rehacer no siempre sigue la misma línea que imaginás.' },
    q: [
      { q: '¿Qué estructura tiene el undo de Emacs?', o: ['Una pila lineal', 'Un árbol con ramas', 'Una cola', 'Un archivo'], a: 1, why: 'El undo ring registra cambios y permite ramas; no es una pila simple.' },
      { q: '¿Qué archivo crea el auto-save?', o: ['`archivo~`', '`#archivo#`', '`archivo.bak`', '`.archivo`'], a: 1, why: 'El auto-save se escribe en `#archivo#`; el backup es `archivo~`.' }
    ],
    ex: { p: 'Escribí tres líneas, deshacé dos veces, rehacé una y comprobá el estado. Después buscá el auto-save de un archivo modificado.', start: ';; escribe tres líneas\n;; C-/ C-/\n;; M-x undo-redo', sol: ';; línea 1\n;; línea 2\n;; línea 3\n;; C-/ C-/\n;; M-x undo-redo', hint: '`C-g` cancela un comando a medias; no borra el historial.' },
    cards: [['¿Qué comando rehace en Emacs moderno?', '`M-x undo-redo` (o `C-?` según la versión).'], ['Diferencia entre auto-save y backup', 'El auto-save guarda el estado actual en `#archivo#`; el backup guarda la versión anterior en `archivo~`.']],
    tags: ['undo', 'archivos', 'comandos']
  },
  {
    id: 'm01-l04', module: 'm01', n: 4, title: 'Cancelar, abortar y salir de estados', min: 6, level: 'Base',
    obj: 'Salir de minibuffers, comandos a medias, búsquedas y modos especiales sin cerrar Emacs.',
    frase: '`C-g` es el botón de pánico: cancela el comando actual y vuelve al bucle principal.',
    idea: [
      '`C-g` (`keyboard-quit`) cancela el comando que está esperando entrada, cierra el minibuffer y detiene una búsqueda. Si estás en un estado más profundo, `ESC ESC ESC` (`keyboard-escape-quit`) sale de más cosas: cierra ventanas temporales, abandona el minibuffer y cancela. `C-]` (`abort-recursive-edit`) aborta un `recursive-edit`.',
      'En buffers especiales (Dired, Magit, Help, Occur, grep) la tecla `q` suele cerrar la vista y volver. `C-x C-c` sale de Emacs, pero no es un “cancelar”: pregunta por los buffers. Si algo se quedó colgado, `C-g` repetido suele liberar.'
    ],
    code: { lang: 'text', src: 'C-g          ; cancela el comando actual\nESC ESC ESC  ; sale de más contextos\nC-]          ; aborta un recursive-edit\nq            ; cierra buffers especiales (Dired, Magit, Help)', cap: 'Jerarquía de escape.' },
    under: [
      'Emacs usa **recursive edit** para pedir entrada sin salir del comando original: el minibuffer es un recursive edit. `C-g` lo aborta y vuelve al nivel anterior. Por eso a veces hace falta pulsarlo dos veces: una para el minibuffer y otra para el comando que lo abrió.',
      '`keyboard-quit` también desactiva la región y cancela `isearch`. Si estás en un proceso que no responde, `C-g` puede no bastar; `M-x list-processes` y matar el proceso es la siguiente parada.'
    ],
    trap: { bad: 'Cerrar Emacs con `C-x C-c` porque un comando quedó esperando entrada.', good: 'Pulsar `C-g` (o `ESC ESC ESC`) y volver al estado normal.', why: 'Cerrar Emacs puede perder trabajo; `C-g` cancela sin tocar los buffers.' },
    q: [
      { q: '¿Qué hace `C-g`?', o: ['Guarda el buffer', 'Cancela el comando actual', 'Cierra Emacs', 'Borra la línea'], a: 1, why: '`keyboard-quit` aborta el comando en curso y cierra el minibuffer.' },
      { q: '¿Qué tecla cierra un buffer especial como Dired o Help?', o: ['`q`', '`C-x C-c`', '`C-k`', '`M-x`'], a: 0, why: 'Los modos especiales suelen enlazar `q` a `quit-window`.' }
    ],
    ex: { p: 'Abrí `M-x occur` con un patrón, cancelá con `C-g`, y después abrí Dired y cerralo con `q`.', start: ';; M-x occur RET buffer RET\n;; C-g\n;; C-x d RET\n;; q', sol: ';; M-x occur RET buffer RET\n;; C-g\n;; C-x d RET\n;; q', hint: 'Si `C-g` no cierra Dired, `q` es la tecla específica del modo.' },
    cards: [['¿Qué hace `ESC ESC ESC`?', '`keyboard-escape-quit`: sale de más contextos que `C-g`.'], ['¿Qué hace `q` en buffers especiales?', 'Cierra la vista y vuelve al buffer anterior.']],
    tags: ['comandos', 'cancelar', 'comandos']
  },
  {
    id: 'm01-l05', module: 'm01', n: 5, title: 'Teclas esenciales y ergonomía', min: 8, level: 'Base',
    obj: 'Leer cualquier combinación de teclas y elegir una ergonomía sostenible.',
    frase: 'No memorices 500 atajos: entendé la notación y usá `M-x` para lo raro.',
    idea: [
      'La notación: `C-` Control, `M-` Meta (Alt o Escape), `S-` Shift, `C-M-` ambos. `C-x` y `C-c` son prefijos: se pulsan y luego la tecla final. `M-x` no es un prefijo: abre el minibuffer para escribir el nombre del comando. `RET` es Enter, `TAB` es tabulación, `SPC` es espacio.',
      'La ergonomía importa: Emacs usa mucho Control y Meta. Mucha gente remapea Caps Lock a Control a nivel de sistema; otra usa `evil-mode` para modal editing; otra usa `god-mode` para reducir combinaciones. No hay una sola forma correcta: elegí la que no te lesione las manos.'
    ],
    code: { lang: 'text', src: 'C-x C-f   abrir archivo\nC-x C-s   guardar\nC-x C-c   salir\nC-x b     cambiar buffer\nC-x 2     dividir ventana\nM-x       ejecutar comando por nombre\nC-h k     describir tecla', cap: 'El kit mínimo.' },
    under: [
      'Las teclas se traducen con `key-translation-map` y `function-key-map`; por eso `C-h` puede ser help o backspace según la terminal. `C-h k` te muestra la traducción efectiva. En terminales, algunas combinaciones no llegan a Emacs porque las intercepta la terminal o el multiplexor.',
      '`repeat-mode` permite repetir comandos como `C-x o` con `o`; `which-key` muestra las continuaciones posibles después de un prefijo. Son dos mejoras de descubrimiento que no cambian el modelo.'
    ],
    trap: { bad: 'Forzar el meñique en Control todo el día y terminar con dolor.', good: 'Remapear Caps Lock a Control, usar `M-x` para comandos raros y considerar un modo modal si te resulta cómodo.', why: 'La ergonomía no es estética: una lesión por esfuerzo repetitivo te saca del teclado.' },
    q: [
      { q: '¿Qué significa `C-M-f`?', o: ['Control + Meta + f', 'Control + f', 'Meta + f', 'Shift + f'], a: 0, why: 'Se pulsan Control y Meta junto con f.' },
      { q: '¿Qué diferencia hay entre `C-c` y `M-x`?', o: ['Ninguna', '`C-c` es un prefijo de teclas; `M-x` pide un comando por nombre', '`M-x` guarda', '`C-c` cierra'], a: 1, why: '`C-c` espera otra tecla; `M-x` abre el minibuffer para escribir el nombre del comando.' }
    ],
    ex: { p: 'Practicá cinco combinaciones: `C-x C-f`, `C-x C-s`, `C-x b`, `C-x 2`, `C-x o`. Después usá `C-h k` con cada una.', start: ';; repite cada combinación y observa la mode line', sol: ';; C-x C-f ~/prueba.txt RET\n;; C-x C-s\n;; C-x b *scratch* RET\n;; C-x 2\n;; C-x o\n;; C-h k C-x 2', hint: 'Si una tecla no llega, probá `C-h k` y mirá qué comando aparece.' },
    cards: [['¿Qué hace `C-x 2`?', 'Divide la ventana actual en dos, arriba y abajo.'], ['¿Qué hace `C-x o`?', 'Pasa el foco a la otra ventana.']],
    tags: ['comandos', 'ergonomia', 'comandos']
  },
  {
    id: 'm02-l01', module: 'm02', n: 1, title: 'Punto, mark y región', min: 7, level: 'Base',
    obj: 'Entender el punto, el mark y la región, y operar sobre texto seleccionado.',
    frase: 'El punto es el cursor; el mark es una posición guardada; la región es el texto entre ambos.',
    idea: [
      'El **punto** es la posición del cursor en el buffer. El **mark** es otra posición que se guarda con `C-SPC` (`set-mark-command`). La **región** es el texto entre el mark y el punto. Muchos comandos actúan “sobre la región” si está activa.',
      '`C-x C-x` (`exchange-point-and-mark`) intercambia punto y mark; es útil para volver a donde estabas. `C-x h` (`mark-whole-buffer`) selecciona todo. `C-g` desactiva la región. `transient-mark-mode` resalta la región cuando está activa.'
    ],
    code: { lang: 'elisp', src: '(point)              ; posición actual\n(mark)               ; posición del mark o nil\n(region-beginning)   ; inicio de la región\n(use-region-p)       ; ¿hay región activa?', cap: 'Inspeccionar punto, mark y región.' },
    under: [
      'El punto es un entero que cuenta caracteres desde el inicio del buffer, no una línea y columna. Los **markers** son posiciones que se ajustan cuando se inserta o borra texto; el mark es un marker. `save-excursion` guarda punto y mark y los restaura al salir.',
      'La región puede estar activa o no. `use-region-p` comprueba ambas cosas. Si un comando dice “región”, casi siempre acepta el texto entre punto y mark; si no hay mark, puede usar la línea actual o pedir dos posiciones.'
    ],
    trap: { bad: 'Mover el cursor después de `C-SPC` y creer que la región sigue donde estaba.', good: 'Recordar que la región es dinámica: va del mark al punto actual; `C-x C-x` vuelve al mark.', why: 'La región no es una selección fija: se recalcula con cada movimiento del punto.' },
    q: [
      { q: '¿Qué hace `C-SPC`?', o: ['Guarda el buffer', 'Activa el mark', 'Copia', 'Cierra la ventana'], a: 1, why: '`set-mark-command` guarda la posición actual como mark y activa la región.' },
      { q: '¿Qué es la región?', o: ['El texto entre mark y punto', 'Todo el buffer', 'La ventana actual', 'El kill ring'], a: 0, why: 'La región es el rango entre mark y punto; puede estar activa o no.' }
    ],
    ex: { p: 'Poné el mark al inicio de una línea, movete al final y copiá la línea con `M-w`.', start: ';; C-SPC\n;; C-e\n;; M-w', sol: ';; C-SPC\n;; C-e\n;; M-w\n;; C-y en otro buffer', hint: '`C-x h` selecciona todo el buffer; `C-x C-x` intercambia punto y mark.' },
    cards: [['¿Qué hace `C-x C-x`?', 'Intercambia punto y mark.'], ['¿Qué hace `C-x h`?', 'Marca todo el buffer.']],
    tags: ['edicion', 'region', 'comandos']
  },
  {
    id: 'm02-l02', module: 'm02', n: 2, title: 'Movimiento por unidades', min: 9, level: 'Base',
    obj: 'Moverte por caracteres, palabras, líneas, frases, párrafos, sexps y funciones.',
    frase: 'Emacs no solo mueve el cursor: entiende unidades sintácticas. Aprender esas unidades es ganar velocidad.',
    idea: [
      'Movimiento básico: `C-f`/`C-b` carácter, `C-n`/`C-p` línea, `C-a`/`C-e` inicio/fin de línea. Por palabra: `M-f`/`M-b`. Por frase: `M-a`/`M-e`. Por párrafo: `M-{`/`M-}`. Por **sexp** (expresión balanceada): `C-M-f`/`C-M-b`. Por función: `C-M-a`/`C-M-e`.',
      'En código, `C-M-f` y `C-M-b` respetan paréntesis, corchetes y llaves; son mucho más útiles que `M-f` para moverte por una llamada. `M-g g` (`goto-line`) va a una línea; `M-g M-g` usa el número bajo el cursor. `C-M-n`/`C-M-p` mueven entre listas.'
    ],
    code: { lang: 'elisp', src: '(forward-word 1)\n(forward-sexp 1)\n(beginning-of-defun)\n(goto-line 42)', cap: 'Las funciones detrás de las teclas.' },
    under: [
      'Las unidades dependen de la **syntax table** del modo. En `prog-mode`, los paréntesis son sexps; en `org-mode`, los encabezados son unidades; en texto, las frases se detectan por puntuación. Por eso las mismas teclas se sienten distintas en cada modo.',
      '`C-M-f` usa `forward-sexp`, que entiende la sintaxis del modo. `M-f` usa `forward-word`, que solo entiende palabras. Si estás dentro de una expresión y querés salir de ella, `C-M-f` suele ser lo correcto.'
    ],
    trap: { bad: 'Usar `M-f` para moverte por código con paréntesis anidados y terminar dentro de una expresión.', good: 'Usar `C-M-f`/`C-M-b` para sexps y `C-M-a`/`C-M-e` para funciones.', why: '`M-f` no conoce paréntesis; `C-M-f` sí, porque usa la syntax table del modo.' },
    q: [
      { q: '¿Qué mueve `C-M-f`?', o: ['Una palabra', 'Una sexp (expresión balanceada)', 'Una línea', 'Un párrafo'], a: 1, why: '`forward-sexp` avanza sobre una expresión balanceada.' },
      { q: '¿Qué tecla va a una línea concreta?', o: ['`M-g g`', '`C-g`', '`C-x l`', '`M-x line`'], a: 0, why: '`goto-line` está en `M-g g`.' }
    ],
    ex: { p: 'En un buffer con `(foo (bar baz))`, movete con `C-M-f` hasta después de `baz` y volvé con `C-M-b`.', start: ';; (foo (bar baz))', sol: ';; coloca el cursor al inicio\n;; C-M-f dos veces\n;; C-M-b una vez', hint: '`C-M-n`/`C-M-p` mueven entre listas hermanas.' },
    cards: [['Diferencia entre `M-f` y `C-M-f`', '`M-f` mueve por palabras; `C-M-f` por expresiones balanceadas.'], ['¿Qué hace `M-g M-g`?', 'Va a la línea del número bajo el cursor.']],
    tags: ['movimiento', 'edicion', 'comandos']
  },
  {
    id: 'm02-l03', module: 'm02', n: 3, title: 'Kill ring, yank y portapapeles', min: 8, level: 'Base',
    obj: 'Cortar, copiar, pegar y navegar el historial de textos matados.',
    frase: 'El kill ring es una pila de textos cortados; `C-y` pega el último y `M-y` rota al anterior.',
    idea: [
      '`C-k` (`kill-line`) mata desde el punto hasta el final de la línea; si ya estás al final, mata el salto de línea. `C-w` (`kill-region`) mata la región. `M-w` (`kill-ring-save`) la copia sin matarla. `C-y` (`yank`) pega el último texto; `M-y` (`yank-pop`) rota al siguiente del kill ring.',
      '`C-M-w` (`append-next-kill`) hace que el siguiente kill se agregue al anterior en vez de reemplazarlo. Es útil para juntar varias líneas. El kill ring no es el portapapeles del sistema por defecto: `select-enable-clipboard` conecta ambos en muchas plataformas.'
    ],
    code: { lang: 'elisp', src: '(current-kill 0)     ; último texto del kill ring\n(kill-new "hola")    ; agrega al kill ring\n(kill-ring-save (point-min) (point-max))', cap: 'Inspeccionar y manipular el kill ring.' },
    under: [
      'El kill ring es una lista de strings con un índice. `kill-ring-max` controla cuántos se guardan (por defecto 60). `yank-pop` solo funciona inmediatamente después de un `yank`; si hacés otra cosa, el ciclo se corta.',
      'En Windows/macOS/Linux, el portapapeles del sistema se sincroniza según `select-enable-clipboard` y `select-enable-primary`. Si copiás en Emacs y no aparece fuera, revisá esas variables y el soporte de la terminal.'
    ],
    trap: { bad: 'Usar `C-k` esperando borrar solo el texto seleccionado; `C-k` mata hasta el final de la línea.', good: 'Para la región, `C-w`; para copiar sin borrar, `M-w`; para una línea completa, `C-a C-k`.', why: '`C-k` opera desde el punto, no sobre la región, salvo que esté activa en algunos contextos.' },
    q: [
      { q: '¿Qué diferencia hay entre `C-w` y `M-w`?', o: ['Ninguna', '`C-w` mata la región; `M-w` la copia', '`M-w` cierra', '`C-w` pega'], a: 1, why: '`kill-region` borra y agrega al kill ring; `kill-ring-save` solo copia.' },
      { q: '¿Qué hace `M-y` después de `C-y`?', o: ['Pega el anterior del kill ring', 'Borra', 'Guarda', 'Deshace'], a: 0, why: '`yank-pop` rota al elemento anterior del kill ring.' }
    ],
    ex: { p: 'Copiá dos líneas con `C-k`, pegá con `C-y`, rotá con `M-y` y comprobá el resultado.', start: ';; línea 1\n;; línea 2\n;; C-a C-k C-k\n;; C-y M-y', sol: ';; línea 1\n;; línea 2\n;; C-a C-k C-k\n;; C-y M-y', hint: '`C-M-w` antes de un kill hace que se agregue al anterior.' },
    cards: [['¿Qué comando pega?', '`C-y` (`yank`).'], ['¿Qué comando rota el kill ring?', '`M-y` (`yank-pop`).']],
    tags: ['kill-ring', 'edicion', 'comandos']
  },
  {
    id: 'm02-l04', module: 'm02', n: 4, title: 'Rectángulos y edición en columnas', min: 8, level: 'Intermedio',
    obj: 'Cortar, pegar, insertar y limpiar rectángulos de texto alineado.',
    frase: 'Un rectángulo es la intersección entre un rango de líneas y un rango de columnas. Emacs lo trata como una unidad.',
    idea: [
      'Marcá una región que abarque varias líneas y columnas. `C-x r k` (`kill-rectangle`) corta el rectángulo; `C-x r y` (`yank-rectangle`) lo pega; `C-x r t` (`string-rectangle`) reemplaza cada línea del rectángulo por un texto; `C-x r o` (`open-rectangle`) inserta espacios; `C-x r c` (`clear-rectangle`) borra el contenido dejando espacios.',
      'Los rectángulos son ideales para alinear tablas, comentarios, imports o columnas de datos. `C-x r N` numera líneas, `C-x r d` borra el rectángulo y `C-x r i` inserta un string en cada línea. No necesitás un modo especial: es edición de texto normal.'
    ],
    code: { lang: 'text', src: 'nombre    edad\nAda       36\nGrace     85\n\n;; marca la columna de edad y C-x r k', cap: 'Un rectángulo típico: una columna de una tabla.' },
    under: [
      'Un rectángulo no es una región rectangular en pantalla: es un par de columnas aplicado a cada línea del rango. Si una línea es más corta, Emacs rellena con espacios al pegar. Por eso sirve para alinear, pero puede introducir espacios donde no querías.',
      '`string-rectangle` es la operación más potente: reemplaza el rectángulo por un texto en todas las líneas. `C-x r t` pide el string y lo inserta en cada línea; con `C-u` puedes reemplazar en vez de insertar. Es la base de muchas alineaciones manuales.'
    ],
    trap: { bad: 'Usar `C-x r t` con una región que no abarca todas las líneas que querías y dejar la tabla a medias.', good: 'Verificar el inicio y el fin de la región con `C-x C-x` y usar `C-x r t` solo después de seleccionar la columna completa.', why: 'El rectángulo se aplica línea por línea; si la región no cubre una línea, esa línea no se toca.' },
    q: [
      { q: '¿Qué hace `C-x r k`?', o: ['Copia un rectángulo', 'Mata un rectángulo', 'Abre una ventana', 'Cierra Emacs'], a: 1, why: '`kill-rectangle` corta el rectángulo seleccionado.' },
      { q: '¿Qué hace `C-x r t`?', o: ['Inserta un string en cada línea del rectángulo', 'Borra la región', 'Guarda', 'Cambia de buffer'], a: 0, why: '`string-rectangle` reemplaza/inserta texto en cada línea del rango.' }
    ],
    ex: { p: 'Creá una tabla de tres filas, marcá la segunda columna y reemplazala por “OK” con `C-x r t`.', start: 'a  1\nb  2\nc  3', sol: ';; marca la columna de números\n;; C-x r t OK RET', hint: '`C-x r o` abre espacio; `C-x r c` limpia sin borrar la estructura.' },
    cards: [['¿Qué hace `C-x r y`?', 'Pega el último rectángulo matado.'], ['¿Qué hace `C-x r o`?', 'Abre un rectángulo insertando espacios.']],
    tags: ['rectangulos', 'edicion', 'comandos']
  },
  {
    id: 'm02-l05', module: 'm02', n: 5, title: 'Macros de teclado', min: 9, level: 'Intermedio',
    obj: 'Grabar, repetir, nombrar y guardar macros de teclado como Elisp.',
    frase: 'Una macro de teclado graba teclas, no intenciones: graba lo que hacés, incluidos los errores.',
    idea: [
      '`C-x (` (`start-kbd-macro`) empieza a grabar; `C-x )` (`end-kbd-macro`) termina; `C-x e` (`call-last-kbd-macro`) ejecuta. `C-u 10 C-x e` la repite 10 veces. `C-x q` inserta una pausa para pedir confirmación. `M-x name-last-kbd-macro` le pone nombre y `M-x insert-kbd-macro` la escribe como Elisp en el buffer.',
      'Las macros son la forma más rápida de automatizar una edición repetitiva sin escribir Lisp. Pero son frágiles: si la estructura del texto cambia, la secuencia de teclas puede hacer otra cosa. Para algo estable, convertí la macro en una función.'
    ],
    code: { lang: 'elisp', src: '(name-last-kbd-macro \'mi-macro)\n(insert-kbd-macro \'mi-macro)\n;; => (fset \'mi-macro (lambda () (interactive) ...))', cap: 'Convertir la última macro en una función con nombre.' },
    under: [
      'Una macro de teclado se guarda como un vector de eventos. `insert-kbd-macro` genera una forma `lambda` con `(interactive)` y la secuencia. Podés editarla, añadir condiciones y convertirla en un comando real.',
      '`C-x q` pausa la macro y pide confirmación; `C-u C-x q` entra en un recursive edit para corregir algo. `M-x edit-kbd-macro` abre el editor de macros, donde podés borrar o reordenar eventos.'
    ],
    trap: { bad: 'Grabar una macro con movimientos absolutos y ejecutarla en otro buffer con otra estructura.', good: 'Usar movimientos relativos (`C-n`, `M-f`) y probar la macro en una copia antes de repetirla 100 veces.', why: 'La macro repite teclas tal cual; si el contexto cambia, el resultado cambia.' },
    q: [
      { q: '¿Qué hace `C-x (`?', o: ['Cierra Emacs', 'Empieza a grabar una macro', 'Abre un archivo', 'Guarda'], a: 1, why: '`start-kbd-macro` inicia la grabación.' },
      { q: '¿Qué comando convierte la última macro en Elisp?', o: ['`insert-kbd-macro`', '`save-buffer`', '`eval-buffer`', '`compile`'], a: 0, why: '`insert-kbd-macro` inserta la forma Lisp que reproduce la macro.' }
    ],
    ex: { p: 'Grabá una macro que agregue `- ` al inicio de tres líneas y ejecutala con `C-u 3 C-x e`.', start: ';; línea 1\n;; línea 2\n;; línea 3', sol: ';; C-x (\n;; C-a\n;; - \n;; C-n\n;; C-x )\n;; C-u 3 C-x e', hint: 'Empezá la macro con el cursor al inicio de la primera línea.' },
    cards: [['¿Qué hace `C-x e`?', 'Ejecuta la última macro grabada.'], ['¿Cómo ves la macro como Lisp?', 'Con `M-x insert-kbd-macro` después de nombrarla.']],
    tags: ['macros', 'edicion', 'comandos']
  },
  {
    id: 'm03-l01', module: 'm03', n: 1, title: 'isearch incremental', min: 7, level: 'Base',
    obj: 'Buscar mientras escribís, navegar coincidencias y usar el historial.',
    frase: '`C-s` no abre un cuadro de búsqueda: busca mientras escribís y te muestra la primera coincidencia al instante.',
    idea: [
      '`C-s` (`isearch-forward`) inicia búsqueda hacia adelante; `C-r` hacia atrás. Escribir extiende el patrón; `C-s` de nuevo salta a la siguiente; `C-r` a la anterior; `RET` termina en la coincidencia; `C-g` cancela y vuelve al punto original. `M-n`/`M-p` recorren búsquedas anteriores.',
      'Dentro de isearch, `C-w` agrega la palabra bajo el cursor al patrón; `M-s` (o `C-M-s`) pasa a búsqueda regexp; `M-%` durante isearch ofrece reemplazo. `isearch-yank-line` (`C-y`) agrega la línea actual. Es una mini-interfaz con su propio keymap.'
    ],
    code: { lang: 'text', src: 'C-s palabra     buscar hacia adelante\nC-r palabra     buscar hacia atrás\nC-s C-s         siguiente\nRET             terminar\nC-g             cancelar', cap: 'Flujo básico de isearch.' },
    under: [
      'Isearch es un **minor mode** con un keymap propio. Mientras está activo, las teclas normales se interpretan como parte del patrón, no como comandos. `isearch-mode-map` define qué teclas siguen siendo comandos (`C-s`, `C-r`, `RET`, `C-g`, etc.).',
      'La búsqueda es incremental y perezosa: no recorre todo el buffer de golpe; avanza a medida que escribís. `isearch-lazy-highlight` resalta todas las coincidencias visibles, no solo la actual.'
    ],
    trap: { bad: 'Pulsar `C-s` y escribir el patrón, y luego `C-g` pensando que vuelve a la coincidencia; `C-g` vuelve al punto original.', good: 'Usar `RET` para quedarte en la coincidencia y `C-g` solo para abortar.', why: '`C-g` cancela la búsqueda y restaura la posición inicial; `RET` acepta el resultado.' },
    q: [
      { q: '¿Qué hace `C-s` durante isearch?', o: ['Cancela', 'Salta a la siguiente coincidencia', 'Reemplaza', 'Guarda'], a: 1, why: 'Repetir `C-s` avanza a la siguiente coincidencia.' },
      { q: '¿Qué hace `C-g` en isearch?', o: ['Acepta', 'Cancela y vuelve al punto original', 'Reemplaza todo', 'Cierra el buffer'], a: 1, why: '`keyboard-quit` aborta la búsqueda y restaura la posición.' }
    ],
    ex: { p: 'Buscá una palabra en este archivo, saltá tres coincidencias con `C-s`, volvé con `C-r` y terminá con `RET`.', start: ';; C-s Emacs\n;; C-s C-s C-s\n;; C-r\n;; RET', sol: ';; C-s Emacs\n;; C-s C-s C-s\n;; C-r\n;; RET', hint: '`C-h k C-s` muestra el comando exacto y su keymap.' },
    cards: [['¿Qué hace `C-r`?', 'Inicia isearch hacia atrás.'], ['¿Qué hace `C-w` dentro de isearch?', 'Agrega la palabra bajo el cursor al patrón.']],
    tags: ['busqueda', 'isearch', 'comandos']
  },
  {
    id: 'm03-l02', module: 'm03', n: 2, title: 'Reemplazo literal y regexp', min: 10, level: 'Intermedio',
    obj: 'Reemplazar texto con confirmación y escribir expresiones regulares al estilo Emacs.',
    frase: '`M-%` reemplaza con confirmación; `C-M-%` usa regexp. La sintaxis de Emacs no es PCRE.',
    idea: [
      '`M-%` (`query-replace`) pide el texto viejo y el nuevo, y va preguntando: `y` reemplaza y avanza, `n` salta, `!` reemplaza todo, `q` termina, `C-r` entra en recursive edit. `C-M-%` (`query-replace-regexp`) hace lo mismo con regexp. `M-x replace-string` reemplaza sin preguntar.',
      'La regexp de Emacs usa `\\(...\\)` para grupos, `\\|` para alternancia, `\\{n,m\\}` para repetición y `\\w` para palabra. En el reemplazo, `\\1` inserta el grupo 1. Es diferente de PCRE: no uses `(...)` ni `|` sin escapar. `M-x rx` construye regexps legibles.'
    ],
    code: { lang: 'elisp', src: '(query-replace "foo" "bar")\n(query-replace-regexp "\\\\([a-z]+\\\\)_\\\\([0-9]+\\\\)" "\\\\2-\\\\1")\n(rx bol (one-or-more (any "a-z")) eol)', cap: 'Reemplazo literal, regexp con grupos y rx.' },
    under: [
      '`query-replace` mantiene un historial y permite reemplazos condicionales. `perform-replace` es la función interna. Las regexps se compilan a un autómata; `rx` genera una forma más legible y se expande a la sintaxis clásica.',
      'Cuidado con `\\` en Lisp: para escribir una barra en la regexp necesitás `\\\\` en el string. En el minibuffer escribís una sola barra. Esa doble capa es la fuente de la mayoría de los errores.'
    ],
    trap: { bad: 'Escribir `(foo|bar)` en `C-M-%` esperando alternancia; en Emacs es `\\(foo\\|bar\\)`.', good: 'Usar `M-x rx` para construir la regexp o probar con `M-x re-builder`.', why: 'La sintaxis de regexp de Emacs es distinta de PCRE y de la de otros editores.' },
    q: [
      { q: '¿Qué hace `!` en query-replace?', o: ['Cancela', 'Reemplaza todas las restantes', 'Reemplaza una', 'Guarda'], a: 1, why: '`!` aplica el reemplazo a todas las coincidencias restantes sin preguntar.' },
      { q: '¿Cómo se agrupa en una regexp de Emacs?', o: ['`(...)`', '`\\(...\\)`', '`[...]`', '`{...}`'], a: 1, why: 'Emacs usa `\\(...\\)` para grupos; los paréntesis sin escapar son literales.' }
    ],
    ex: { p: 'Reemplazá todas las apariciones de `foo` por `bar` con `M-%`, y después convertí `nombre_42` en `42-nombre` con regexp.', start: 'foo foo\nnombre_42', sol: ';; M-% foo RET bar RET !\n;; C-M-% \\\\([a-z]+\\\\)_\\\\([0-9]+\\\\) RET \\\\2-\\\\1 RET', hint: 'En el minibuffer escribís `\\(...\\)`; en un string Lisp necesitás `\\\\(...\\\\)`.' },
    cards: [['¿Qué comando reemplaza sin preguntar?', '`M-x replace-string`.'], ['¿Qué hace `C-M-%`?', '`query-replace-regexp`: reemplazo con regexp y confirmación.']],
    tags: ['busqueda', 'regexp', 'comandos']
  },
  {
    id: 'm03-l03', module: 'm03', n: 3, title: 'occur, grep y wgrep', min: 9, level: 'Intermedio',
    obj: 'Listar coincidencias de un buffer, buscar en un proyecto y editar los resultados.',
    frase: '`occur` te da una vista filtrada del buffer; `grep` busca en archivos; `wgrep` hace editable el resultado.',
    idea: [
      '`M-x occur` pide una regexp y abre un buffer con todas las líneas que coinciden, con un enlace a su posición. `M-x multi-occur` hace lo mismo en varios buffers. `M-x grep` ejecuta `grep` externo y muestra los resultados en un buffer navegable; `M-x lgrep`/`rgrep` construyen el comando por vos. `M-g M-n` (`next-error`) salta al siguiente resultado.',
      'Si tenés `wgrep`, en un buffer de grep podés pulsar `C-c C-p` o `M-x wgrep-change-to-wgrep-mode` y editar las líneas; al guardar, wgrep aplica los cambios a los archivos. Es una forma muy eficiente de hacer un refactor masivo con revisión.'
    ],
    code: { lang: 'text', src: 'M-x occur RET TODO RET\nM-x rgrep RET TODO RET *.rs RET\nM-g M-n   ; siguiente resultado\nM-x wgrep-change-to-wgrep-mode', cap: 'Buscar, navegar y editar resultados.' },
    under: [
      '`occur` usa `occur-mode`, un modo derivado de `special-mode`; `RET` visita la línea, `n`/`p` navegan, `q` cierra. `grep` usa `compilation-mode`, que entiende el formato `archivo:línea:columna: texto` y por eso puede saltar al lugar exacto.',
      '`wgrep` modifica el buffer de resultados y luego escribe en los archivos originales. Es potente y peligroso: revisá el diff antes de confirmar. `M-x wgrep-save-all-buffers` guarda todos los cambios.'
    ],
    trap: { bad: 'Editar el buffer de `occur` esperando que cambie el archivo original.', good: 'Usar `wgrep` sobre un buffer de `grep` para editar los archivos, o ir a cada ocurrencia con `RET`.', why: '`occur` es una vista de solo lectura; `grep` + `wgrep` es el flujo editable.' },
    q: [
      { q: '¿Qué hace `M-x occur`?', o: ['Busca en archivos', 'Lista las líneas de un buffer que coinciden', 'Reemplaza', 'Compila'], a: 1, why: '`occur` crea un buffer con las coincidencias del buffer actual.' },
      { q: '¿Qué permite `wgrep`?', o: ['Editar los resultados de grep y aplicarlos a los archivos', 'Compilar', 'Cerrar Emacs', 'Cambiar el tema'], a: 0, why: '`wgrep` convierte el buffer de resultados en editable y escribe los cambios.' }
    ],
    ex: { p: 'Buscá `TODO` en un proyecto con `rgrep`, navegá con `M-g M-n` y abrí una ocurrencia.', start: 'M-x rgrep RET TODO RET *.el RET', sol: 'M-x rgrep RET TODO RET *.el RET\nM-g M-n\nRET', hint: '`grep-find` y `rgrep` buscan recursivamente; `lgrep` solo en archivos abiertos.' },
    cards: [['¿Qué comando salta al siguiente error/resultado?', '`M-g M-n` (`next-error`).'], ['¿Qué modo usa el buffer de grep?', '`compilation-mode`.']],
    tags: ['busqueda', 'grep', 'comandos']
  },
  {
    id: 'm03-l04', module: 'm03', n: 4, title: 'imenu, xref y project.el', min: 9, level: 'Intermedio',
    obj: 'Navegar definiciones, referencias y proyectos con los mecanismos nativos.',
    frase: '`imenu` navega dentro de un archivo; `xref` salta a definiciones y referencias; `project.el` define qué es un proyecto.',
    idea: [
      '`M-x imenu` lista las definiciones del buffer actual (funciones, clases, secciones) y te deja saltar. `M-.` (`xref-find-definitions`) va a la definición del símbolo bajo el cursor; `M-,` (`xref-go-back`) vuelve; `M-x xref-find-references` lista usos. El backend puede ser Eglot/LSP, etags o ripgrep.',
      '`project.el` detecta la raíz de un proyecto (`.git`, `package.json`, `Cargo.toml`, etc.). `C-x p p` (`project-switch-project`) cambia de proyecto; `C-x p f` busca archivos; `C-x p c` compila; `C-x p g` busca con grep. `projectile` es la alternativa histórica, pero `project.el` ya viene incluido.'
    ],
    code: { lang: 'text', src: 'M-x imenu RET\nM-.          ; definición\nM-,          ; volver\nM-x xref-find-references\nC-x p p      ; cambiar proyecto\nC-x p f      ; archivo del proyecto', cap: 'Navegación nativa.' },
    under: [
      '`xref` es una abstracción: los backends implementan `xref-backend-definitions` y `xref-backend-references`. Eglot registra un backend cuando conecta un LSP; `etags` usa un TAGS; `ripgrep` puede usarse para referencias. Por eso `M-.` funciona distinto según el modo y los paquetes.',
      '`project.el` no necesita configuración para la mayoría de proyectos: busca hacia arriba un marcador de raíz. Podés definir proyectos con `project-remember-projects-under` o `project-vc-extra-root-markers`.'
    ],
    trap: { bad: 'Esperar que `M-.` funcione en cualquier archivo sin backend y concluir que Emacs no navega.', good: 'Iniciar Eglot o generar un TAGS; verificar el backend con `M-x xref-find-definitions` y `C-h v xref-backend-functions`.', why: '`xref` delega en un backend; sin backend, no hay definiciones que encontrar.' },
    q: [
      { q: '¿Qué hace `M-.`?', o: ['Va a la definición del símbolo', 'Borra', 'Guarda', 'Cierra'], a: 0, why: '`xref-find-definitions` salta a la definición usando el backend activo.' },
      { q: '¿Qué comando cambia de proyecto?', o: ['`C-x p p`', '`C-x C-f`', '`M-x compile`', '`C-x b`'], a: 0, why: '`project-switch-project` está en `C-x p p`.' }
    ],
    ex: { p: 'Abrí un archivo de código, usá `M-x imenu` para saltar a una función y `M-.` sobre una llamada.', start: ';; M-x imenu RET\n;; M-. sobre una función', sol: ';; M-x imenu RET\n;; elige una definición\n;; M-. sobre una llamada\n;; M-, para volver', hint: 'Si `M-.` no encuentra nada, `M-x eglot` o generá un TAGS.' },
    cards: [['Diferencia entre imenu y xref', 'imenu lista definiciones del buffer; xref salta a definiciones/referencias entre archivos.'], ['¿Qué comando vuelve después de xref?', '`M-,` (`xref-go-back`).']],
    tags: ['navegacion', 'xref', 'comandos']
  },
  {
    id: 'm03-l05', module: 'm03', n: 5, title: 'Bookmarks, registers y recentf', min: 8, level: 'Intermedio',
    obj: 'Guardar posiciones, textos, ventanas y archivos recientes para volver rápido.',
    frase: 'Un bookmark sobrevive reinicios; un register vive en la sesión; recentf recuerda lo que abriste.',
    idea: [
      '`C-x r m` (`bookmark-set`) guarda la posición actual con un nombre; `C-x r b` (`bookmark-jump`) salta; `C-x r l` (`list-bookmarks`) los administra. Los bookmarks se guardan en un archivo y sobreviven a reinicios. `C-x r SPC` (`point-to-register`) guarda una posición en un registro; `C-x r j` (`jump-to-register`) salta.',
      'Los registers también guardan texto (`C-x r s`), números (`C-x r n`), ventanas (`C-x r w`) y rectángulos. `M-x recentf-open-files` muestra archivos recientes; `savehist` guarda historiales y `saveplace` recuerda la posición en cada archivo.'
    ],
    code: { lang: 'elisp', src: '(bookmark-set "proyecto")\n(bookmark-jump "proyecto")\n(point-to-register ?a)\n(jump-to-register ?a)\n(recentf-open-files)', cap: 'Bookmarks, registers y recentf.' },
    under: [
      'Los bookmarks se guardan en `bookmark-default-file` (normalmente `~/.emacs.d/bookmarks`). Cada bookmark tiene archivo, posición y posiblemente texto de anotación. Los registers son un vector indexado por carácter; `?a` es el registro `a`.',
      '`recentf` mantiene una lista de archivos y la guarda si `recentf-save-file` está configurado. `saveplace` guarda la posición por archivo; `savehist` guarda minibuffer history, search history y más. Juntos hacen que Emacs “recuerde” tu contexto.'
    ],
    trap: { bad: 'Confundir un bookmark con un register: el bookmark persiste y se administra; el register es efímero y por carácter.', good: 'Usar bookmarks para proyectos/notas y registers para saltos temporales dentro de la sesión.', why: 'Tienen modelos y persistencia distintos; elegir mal hace que pierdas posiciones al reiniciar.' },
    q: [
      { q: '¿Qué comando guarda un bookmark?', o: ['`C-x r m`', '`C-x r j`', '`C-x r s`', '`C-x r k`'], a: 0, why: '`bookmark-set` guarda la posición actual con un nombre.' },
      { q: '¿Qué hace `C-x r j`?', o: ['Guarda un registro', 'Salta a un registro', 'Borra un bookmark', 'Cierra'], a: 1, why: '`jump-to-register` restaura el contenido del registro indicado.' }
    ],
    ex: { p: 'Guardá un bookmark llamado `notas`, cerrá el buffer, volvé a abrirlo y saltá con `C-x r b`.', start: ';; abre un archivo\n;; C-x r m notas RET\n;; C-x k RET\n;; C-x r b notas RET', sol: ';; C-x r m notas RET\n;; C-x k RET\n;; C-x r b notas RET', hint: '`C-x r l` lista y edita bookmarks.' },
    cards: [['¿Dónde se guardan los bookmarks?', 'En `bookmark-default-file`, normalmente `~/.emacs.d/bookmarks`.'], ['¿Qué hace `saveplace`?', 'Recuerda la posición del punto en cada archivo visitado.']],
    tags: ['bookmarks', 'registers', 'comandos']
  }
];

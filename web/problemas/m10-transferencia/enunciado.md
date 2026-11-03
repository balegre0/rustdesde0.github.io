---
titulo: "Transferencia bancaria"
mundo: 10-errores
tema: errores
dificultad: media
---

Escribe `transferir(origen: &mut Cuenta, destino: &mut Cuenta, cantidad: u64) -> Result<(), ErrorBanco>`:

1. Si `cantidad` es 0 → `Err(ErrorBanco::CantidadCero)`.
2. Si la cuenta de origen está bloqueada → `Err(ErrorBanco::Bloqueada)` (aunque tenga saldo).
3. Si el saldo de origen no llega → `Err(ErrorBanco::SaldoInsuficiente { falta })`, donde `falta` es lo que le falta.
4. Si sumar la cantidad al destino desbordaría el `u64` → `Err(ErrorBanco::Desbordamiento)`.
5. Si no, mueve el dinero y devuelve `Ok(())`.

**Importante:** si devuelves un error, **ninguna** de las dos cuentas debe haber cambiado. Las comprobaciones, en el orden de la lista.

<details>
<summary>Pista 1</summary>

Haz todas las comprobaciones antes de tocar ningún saldo. Para el desbordamiento: `destino.saldo.checked_add(cantidad)` devuelve `Option`; `.ok_or(ErrorBanco::Desbordamiento)?` lo convierte.
</details>

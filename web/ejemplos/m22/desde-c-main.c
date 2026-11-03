#include <stdio.h>
#include <stdint.h>
uint32_t rs_contar_vocales(const char *s);
int32_t rs_puede_entrar_en_panico(int32_t x);
int main(void) {
    printf("vocales: %u\n", rs_contar_vocales("murcielago"));
    printf("nulo: %u\n", rs_contar_vocales(0));
    printf("doble: %d\n", rs_puede_entrar_en_panico(21));
    return 0;
}

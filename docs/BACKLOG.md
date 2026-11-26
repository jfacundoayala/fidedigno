# Backlog inicial (pasar a Issues)

## G0: decisiones (26–27/11)
- [ ] Cerrar el alcance del P0 y las decisiones pendientes de ARQUITECTURA.md
- [ ] Definir la licencia
- [ ] Confirmar el registro en el Hub con la organización
- [ ] Probar una atestación en AttestProtocol Testnet con su SDK

## SDK (27/11–2/12)
- [ ] Huella SHA-256 y huella perceptual de imágenes
- [ ] Bloqueantes `AT-01`, `LE-01`, `OR-01`, `CP-01` y `PE-01`
- [ ] Señales `S-CO-01` (fecha EXIF), `S-EX-01` (GPS), `S-OR-02` (repetición), `S-AT-02` (C2PA) y `S-CS-01` (consistencia)
- [ ] Fórmula de puntaje con pesos versionados (`reglas v0.1`) y pruebas con fotos sintéticas buenas y malas

## Contrato (27/11–1/12)
- [ ] Contrato `fidedigno`: `submit`, `dispute`, `resolve`, `finalize` y `require_verified`, con pruebas unitarias
- [ ] Tabla GAMP 5 (`set_policy`) y fianzas con token de prueba
- [ ] Regla: el árbitro tiene que ser distinto del emisor
- [ ] Registrar el resultado en AttestProtocol
- [ ] Desplegar en Testnet y anotar la dirección y los hashes en `docs/evidencia/`
- [ ] Crate `fidedigno` para builders

## Demo (2–4/12)
- [ ] Escrow de ejemplo, con dos casos:
  - foto buena: pasa y paga;
  - foto reciclada: se rechaza.
- [ ] Caso de disputa: puntaje medio, la contraparte disputa y el árbitro resuelve
- [ ] Verificador web público
- [ ] Video de 3 minutos como máximo
- [ ] Pitch de 3 minutos para el Demo Day
- [ ] Entrega final: 4/12, 23:59, hora de Lima

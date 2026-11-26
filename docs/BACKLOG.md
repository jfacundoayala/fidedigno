# Plan de trabajo

## 1. Definiciones (26–27/11)
- [ ] Cerrar las decisiones pendientes de [ARQUITECTURA.md](ARQUITECTURA.md)
- [ ] Definir la licencia
- [ ] Confirmar el registro en el Hub

## 2. Módulo de verificación (27/11–2/12)
- [ ] Huella SHA-256 y huella perceptual
- [ ] Reglas bloqueantes `AT-01`, `LE-01`, `OR-01`, `CP-01` y `PE-01`
- [ ] Señales `S-CO-01`, `S-EX-01`, `S-OR-02`, `S-AT-02` y `S-CS-01`
- [ ] Puntaje con pesos versionados (`reglas v0.1`)
- [ ] Pruebas con imágenes sintéticas válidas e inválidas

## 3. Contratos (27/11–1/12)
- [ ] Contrato de verificación: `submit`, `dispute`, `resolve`, `finalize` y `require_verified`
- [ ] Política por nivel de riesgo y fianzas con token de prueba
- [ ] Pruebas unitarias, incluida la regla "árbitro distinto del emisor"
- [ ] Escrow de ejemplo
- [ ] Despliegue en Testnet y registro de direcciones y hashes en `docs/evidencia/`

## 4. Entrega (2–4/12)
- [ ] Casos de demostración:
  - imagen válida: se paga;
  - imagen reutilizada: se rechaza;
  - imagen disputada: resuelve el árbitro.
- [ ] Video de la demo (máximo 3 minutos)
- [ ] Presentación del Demo Day
- [ ] Entrega final: 4/12, 23:59, hora de Lima

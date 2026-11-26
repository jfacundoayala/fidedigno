# Plan de trabajo

## 1. Definiciones (26–27/11)
- [ ] Cerrar las decisiones pendientes de [ARQUITECTURA.md](ARQUITECTURA.md)
- [ ] Definir la licencia
- [ ] Confirmar el registro en el Hub

## 2. Captura y módulo de verificación (27/11–2/12)
- [ ] Página de captura: imagen, huellas SHA-256 y perceptual, ancla de red y firma del emisor
- [ ] Reglas bloqueantes `AT-01`, `LE-01`, `OR-01`, `CO-00`, `CP-01` y `PE-01`
- [ ] Señales `S-CA-01`, `S-CO-01`, `S-EX-01`, `S-AT-02`, `S-OR-02` y `S-CS-01`
- [ ] Cálculo del ICE con la tabla `reglas v0.1` y el límite por familia
- [ ] Pruebas: los tres ejemplos de [MARCO-DE-VERIFICACION.md](MARCO-DE-VERIFICACION.md) y la reproducibilidad del cálculo

## 3. Contratos (27/11–1/12)
- [x] Contrato de verificación: `submit`, `dispute`, `resolve`, `finalize` y `require_verified`
- [x] Roles y política por nivel de riesgo (sin depósitos; ver [ARQUITECTURA.md](ARQUITECTURA.md))
- [x] Fianza proporcional al ICE y al monto, con token de prueba
- [x] Pruebas unitarias:
  - el verificador tiene que ser distinto del emisor;
  - el ancla fuera de margen se rechaza;
  - el árbitro no puede ser parte.
- [x] Escrow de ejemplo
- [x] Despliegue en Testnet y registro de direcciones y hashes en `docs/evidencia/`

## 4. Entrega (2–4/12)
- [ ] Casos de demostración:
  - captura firmada válida: se paga;
  - imagen reutilizada: se rechaza;
  - disputa de cálculo: se recalcula el ICE;
  - disputa de hecho: resuelve el árbitro.
- [ ] Video de la demo (máximo 3 minutos)
- [ ] Presentación del Demo Day
- [ ] Entrega final: 4/12, 23:59, hora de Lima

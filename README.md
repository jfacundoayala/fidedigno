# Fidedigno

Prototipo para verificar el contenido de la evidencia (por ejemplo, la foto de una entrega) antes de que un contrato inteligente en Stellar la use para liberar un pago.

Proyecto para **Stellar Codember Perú 2026**.

> **Estado: diseño inicial.** Este repositorio contiene la especificación, el marco de verificación y el plan de trabajo. Todavía no hay código, contratos desplegados ni pruebas en Testnet.

## Problema
Un contrato puede guardar la huella de un archivo y probar que no cambió. No puede saber si ese archivo es una prueba válida: si la foto es de ese día, si es original o si ya se usó antes.

## Enfoque
La verificación se divide en tres pasos:

1. **Reglas bloqueantes**, basadas en los principios de integridad de datos ALCOA+. Si una falla, la evidencia se rechaza.
2. **Puntaje de confianza:** una suma ponderada de señales calculadas fuera de la red, como los metadatos de la foto, la repetición de imágenes o el manifiesto C2PA.
3. **Plazo de disputa con fianza**, siguiendo el modelo de oráculo optimista (UMA). La evidencia queda aceptada si nadie la disputa dentro del plazo.

Cuánto puntaje se exige, cuánto dura el plazo y qué fianza se pide dependen del nivel de riesgo que declara cada contrato. Ese criterio está inspirado en el enfoque basado en riesgo de GAMP 5.

Detalle: [docs/MARCO-ALCOA-GAMP.md](docs/MARCO-ALCOA-GAMP.md).

## Alcance del prototipo
Objetivo para la entrega del 4/12/2026:

- [ ] Módulo TypeScript con las reglas bloqueantes y el puntaje para **imágenes**
- [ ] Contrato Soroban con el plazo de disputa, las fianzas (token de prueba) y la consulta `require_verified`
- [ ] Contrato de escrow de ejemplo que consulta el resultado antes de pagar
- [ ] Despliegue en Stellar Testnet, con direcciones y hashes en `docs/evidencia/`
- [ ] Pruebas automáticas y video de la demo

Queda fuera del prototipo: audio y video, ajuste automático de pesos, árbitros descentralizados, red principal y auditoría.

## Limitaciones
- El puntaje aumenta la confianza en la evidencia; no prueba que un hecho haya ocurrido.
- Los metadatos (EXIF) se pueden falsificar. Por eso nunca se usan como única señal.
- Este proyecto toma ideas de ALCOA+ y GAMP 5. No es una certificación ni una validación regulatoria.
- Solo Stellar Testnet y tokens de prueba.

## Documentación
| Documento | Contenido |
|---|---|
| [MARCO-ALCOA-GAMP.md](docs/MARCO-ALCOA-GAMP.md) | Reglas, señales, niveles de riesgo y disputa |
| [ARQUITECTURA.md](docs/ARQUITECTURA.md) | Componentes, interfaz del contrato y decisiones pendientes |
| [INVESTIGACION-SIMILARES.md](docs/INVESTIGACION-SIMILARES.md) | Proyectos relacionados en Stellar y otras redes |
| [BACKLOG.md](docs/BACKLOG.md) | Plan de trabajo hasta la entrega |

## Hitos del evento
| Hito | Fecha (hora de Lima) |
|---|---|
| Registro del repositorio en el Hub | 25/11/2026 23:59 |
| Entrega final | 4/12/2026 23:59 |
| Demo Day (Lima) | 5/12/2026 |

## Autoría
- El código se escribe durante el período del evento (del 14/11 al 4/12/2026).
- No incorpora código previo ni de terceros. Cualquier reutilización futura se declarará con su origen y licencia.
- La documentación se preparó con asistencia de IA.

## Licencia
Pendiente de definir.

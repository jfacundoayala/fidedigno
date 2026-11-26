# Fidedigno

Prototipo para verificar el contenido de la evidencia (por ejemplo, la foto de una entrega) antes de que un contrato inteligente en Stellar la use para liberar un pago.

Proyecto para **Stellar Codember Perú 2026**.

> **Estado: diseño inicial.** Este repositorio contiene la especificación, el marco de verificación y el plan de trabajo. Todavía no hay código, contratos desplegados ni pruebas en Testnet.

## Problema
Un contrato puede guardar la huella de un archivo y probar que no cambió. No puede saber si ese archivo es una prueba válida: si la foto es de ese momento, si es original o si ya se usó antes.

## Enfoque
1. **Captura firmada con ancla de red.** La imagen se captura y se firma en el momento, junto con el hash de un ledger reciente de Stellar. Eso acota cuándo se tomó, sin depender de los metadatos del archivo.
2. **Índice de confianza de la evidencia (ICE).** Combina las señales de la imagen como razones de verosimilitud, distingue una señal ausente de una señal en contra y limita el peso de las señales que dependen entre sí. Lo calcula y lo firma un verificador distinto de quien presenta la evidencia.
3. **Fianza proporcional y disputa.** Quien presenta la evidencia deposita una fianza que depende del ICE y del monto en juego. Durante un plazo se puede disputar; quien pierde la disputa pierde su fianza.

Como base, el marco usa reglas bloqueantes tomadas de los principios de integridad de datos ALCOA+ y niveles de riesgo con un criterio inspirado en GAMP 5.

```
odds   = Π_familias limitar( Π_señales LR , 1/20 , 20 )
ICE    = odds / (1 + odds)
fianza = máx( fianza_mínima , k × (1 − ICE) × monto )
```

Detalle, tabla de señales y ejemplos: [docs/MARCO-DE-VERIFICACION.md](docs/MARCO-DE-VERIFICACION.md).

## Alcance del prototipo
Objetivo para la entrega del 4/12/2026:

- [ ] Página de captura firmada con ancla de red
- [ ] Módulo TypeScript con las reglas bloqueantes y el cálculo del ICE para **imágenes**
- [ ] Contrato Soroban con roles, fianza proporcional (token de prueba), plazo de disputa y la consulta `require_verified`
- [ ] Contrato de escrow de ejemplo que consulta el resultado antes de pagar
- [ ] Despliegue en Stellar Testnet, con direcciones y hashes en `docs/evidencia/`
- [ ] Pruebas automáticas y video de la demo

Queda fuera del prototipo: audio y video, varios verificadores con quórum, panel de árbitros, calibración de la tabla de señales, red principal y auditoría.

## Limitaciones
- El ICE estima confianza; no prueba que un hecho haya ocurrido.
- La captura firmada prueba cuándo se firmó la imagen, no qué había delante de la cámara.
- Los valores de la tabla de señales son iniciales y no están calibrados con datos reales.
- El verificador y el árbitro son roles de confianza en el prototipo.
- Este proyecto toma ideas de ALCOA+ y GAMP 5. No es una certificación ni una validación regulatoria.
- Solo Stellar Testnet y tokens de prueba.

## Documentación
| Documento | Contenido |
|---|---|
| [MARCO-DE-VERIFICACION.md](docs/MARCO-DE-VERIFICACION.md) | Captura firmada, ICE, fianza, disputa, reglas y niveles de riesgo |
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

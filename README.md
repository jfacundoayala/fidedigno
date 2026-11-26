# Fidedigno

Prototipo para verificar el contenido de la evidencia (por ejemplo, la foto de una entrega) antes de que un contrato inteligente en Stellar la use para liberar un pago.

Proyecto para **Stellar Codember Perú 2026**.

> **Estado: en construcción.** Los dos contratos están implementados, probados y desplegados en Stellar Testnet. Faltan el módulo que calcula el índice de confianza y la página de captura.

| Contrato en Testnet | ID |
|---|---|
| Verificación | [`CAWLRO5ND3ZUSVISVWN5OZB4PTVGZ2AUOSGOGZVTYE7N6W7IYWFYXSAV`](https://stellar.expert/explorer/testnet/contract/CAWLRO5ND3ZUSVISVWN5OZB4PTVGZ2AUOSGOGZVTYE7N6W7IYWFYXSAV) |
| Escrow de ejemplo | [`CA34LHQIDUJYYSMEQIRJZ6U6P662IWFPPJCFBSXOIXGYBQ2AKD6SITJX`](https://stellar.expert/explorer/testnet/contract/CA34LHQIDUJYYSMEQIRJZ6U6P662IWFPPJCFBSXOIXGYBQ2AKD6SITJX) |

Recorrido completo con sus transacciones: [docs/evidencia/TESTNET.md](docs/evidencia/TESTNET.md).

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

- [x] Contrato de verificación: roles, fianza proporcional, plazo de disputa y consulta
- [x] Contrato de escrow de ejemplo que consulta el resultado antes de pagar
- [x] Despliegue en Stellar Testnet, con IDs y transacciones en `docs/evidencia/`
- [x] Pruebas automáticas de los contratos (15)
- [ ] Módulo TypeScript con las reglas bloqueantes y el cálculo del ICE para **imágenes**
- [ ] Página de captura firmada con ancla de red
- [ ] Video de la demo

Queda fuera del prototipo: audio y video, varios verificadores con quórum, depósitos de verificador y árbitro, panel de árbitros, calibración de la tabla de señales, red principal y auditoría.

## Cómo ejecutarlo
Requiere Docker y Bash. No hace falta instalar Rust ni el CLI de Stellar.

```bash
bash scripts/contratos.sh test      # corre las 15 pruebas
bash scripts/contratos.sh build     # compila los .wasm en wasm/
bash scripts/contratos.sh script scripts/demo-testnet.sh   # despliega y recorre el flujo en Testnet
```

El último comando crea cuentas de prueba en Testnet. Sus claves quedan en un volumen de Docker, fuera del repositorio.

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
| [ARQUITECTURA.md](docs/ARQUITECTURA.md) | Componentes, interfaz del contrato y diferencias con el marco |
| [evidencia/TESTNET.md](docs/evidencia/TESTNET.md) | Contratos desplegados y transacciones del recorrido |
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

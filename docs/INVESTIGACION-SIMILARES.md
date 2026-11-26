# Proyectos relacionados

Revisión del 26/11/2026. Lo marcado como **verificado** se comprobó en la documentación oficial. El resto proviene de búsquedas preliminares y queda pendiente de confirmar.

## En Stellar
| Proyecto | Qué hace | Estado | Relación con este proyecto |
|---|---|---|---|
| [AttestProtocol](https://docs.attest.so/) ([repo](https://github.com/daccred/attest.so)) | Registro de atestaciones en Soroban: autoridades, *schemas*, *resolvers*, revocación y SDK | **Verificado:** Testnet y Mainnet (v2) | Registra quién afirma qué. No analiza el contenido de la evidencia. Es una opción para publicar resultados |
| [Trustless Work](https://www.trustlesswork.com/) | Escrow con hitos, aprobaciones y disputas | Según su sitio: V1 en Mainnet, V2 beta en Testnet | Las aprobaciones son manuales. Es un posible caso de integración |
| Reflector, Pyth, RedStone | Oráculos de precios | En producción | No cubren evidencia ni hechos |

## En otras redes
| Proyecto | Red | Qué hace | Idea aplicable |
|---|---|---|---|
| [EAS](https://docs.attest.org/docs/core--concepts/schemas) | EVM | Atestaciones con *schemas* y *resolvers* | Modelo de datos de la atestación |
| [Verax](https://ver.ax/) | Linea | Registro compartido de atestaciones | Validar antes de registrar |
| [Sign Protocol](https://docs.sign.global/developers/quickstart) | Varias | Afirmaciones dentro y fuera de la red | Resultado fuera de la red, anclado por hash |
| UMA Optimistic Oracle | EVM | Afirmación con fianza y plazo de disputa | Base del paso de disputa |
| Kleros | EVM | Arbitraje con jurados | Posible árbitro para el nivel alto, fuera del alcance actual |
| C2PA / Content Credentials | Estándar abierto | Manifiesto firmado de origen y ediciones de imágenes | Señal de origen |
| ProofMode, Numbers Protocol | Apps y varias redes | Firma de la imagen al capturarla | Señal de captura |
| MediLedger | Red privada (pharma) | Integridad de datos en la cadena de suministro | Antecedente de integridad de datos regulada |

## Observaciones
- No se encontró en Stellar un componente que analice el contenido de la evidencia ni un oráculo optimista para hechos. La búsqueda fue preliminar.
- No se encontraron proyectos que apliquen ALCOA+ y GAMP 5 a contratos inteligentes en redes públicas.

## Pendiente de verificar
- Formato con el que Trustless Work guarda la evidencia de un hito.
- Uso de *resolvers* propios en AttestProtocol y su costo.
- Existencia de un oráculo optimista en Soroban.

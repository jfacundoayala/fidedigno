# Proyectos similares y qué tomar (26/11/2026)

Lo marcado como **verificado** lo revisé directamente en la fuente. El resto sale de agentes de búsqueda y hay que confirmarlo antes de citarlo en el pitch.

## 1. Lo que ya existe en Stellar
| Proyecto | Qué hace | Estado | Relación con Fidedigno |
|---|---|---|---|
| **AttestProtocol** ([docs](https://docs.attest.so/), [repo](https://github.com/daccred/attest.so)) | Servicio de atestaciones en Soroban: autoridades, *schemas*, *resolvers*, revocación, firma fuera de la red con BLS y SDK `@attestprotocol/stellar-sdk` | **Verificado:** en Testnet y Mainnet (v2) | **Es el "registro" que pensábamos construir.** Registra atestaciones pero **no revisa el contenido** de la evidencia. Sus casos de uso son KYC, asistencia, membresías y credenciales |
| **Trustless Work** ([web](https://www.trustlesswork.com/), [docs](https://docs.trustlesswork.com/)) | Escrow como servicio con hitos, aprobaciones, liberaciones parciales y disputas | V1 en Mainnet, V2 beta en Testnet (según su web) | Las aprobaciones y las disputas las hacen personas. **No verifica la evidencia del hito.** Es el cliente natural del kit |
| Reflector, Pyth, RedStone | Oráculos de **precios** | En producción | No cubren hechos ni evidencia |

**Hueco en Stellar:** hay dónde **registrar** atestaciones y dónde **pagar por hitos**, pero no encontré nada que **revise la evidencia** (fotos, audios, documentos) con reglas y que **resuelva disputas** sobre hechos.

## 2. Lo que existe en otras redes
| Proyecto | Red | Qué hace | Qué sirve |
|---|---|---|---|
| **EAS** ([docs](https://docs.attest.org/docs/core--concepts/schemas)) | EVM | Atestaciones con *schemas* y *resolvers* | Es el patrón que ya replicó AttestProtocol |
| **Verax** ([web](https://ver.ax/)) | Linea | Registro compartido de atestaciones | Los módulos que validan antes de guardar |
| **Sign Protocol** ([docs](https://docs.sign.global/developers/quickstart)) | Varias | Afirmaciones dentro y fuera de la red | Atestaciones fuera de la red ancladas por hash |
| **UMA Optimistic Oracle** | EVM | Alguien afirma un hecho y deja una fianza; si nadie lo disputa en el plazo, queda como verdad | **No encontré un equivalente en Stellar.** Es el candidato más fuerte para traer |
| **Kleros** | EVM | Jurados que arbitran disputas con evidencia | Para disputas de nivel alto, más adelante |
| **C2PA / Content Credentials** | Estándar abierto | Manifiesto firmado de origen y ediciones de fotos y videos | Leer el manifiesto C2PA si existe = chequeo de origen (`OR`) y atribución (`AT`) |
| **ProofMode, Numbers Protocol** | Apps y varias redes | Firma de la foto al capturarla, con metadatos | La firma en el momento de captura sirve para el chequeo de contemporaneidad (`CO`) |
| **MediLedger** | Pharma (red privada) | Integridad de datos en la cadena de suministro farmacéutica | Precedente de integridad regulada: validar sin exponer datos |

No encontré otro proyecto blockchain que aplique ALCOA+ y GAMP 5 como kit para devs. Esa es nuestra diferencia, pero hay que confirmarlo mejor.

## 3. Qué cambia en Fidedigno
**No construir otro registro.** Fidedigno pasa a ser la **capa de verificación** que se apoya en lo que ya existe:

1. **Motor de chequeos ALCOA+ y GAMP 5** (lo propio): revisa la evidencia fuera de la red y decide el nivel.
2. **Emite el resultado como atestación de AttestProtocol** con un *schema* `EvidenceVerification`: huella, nivel, reglas y versión.
3. **Resolver de AttestProtocol:** solo pueden atestar verificadores registrados y distintos del emisor.
4. **Período de disputa estilo UMA:** la atestación queda "pendiente" durante X horas con fianza. Si nadie la disputa, pasa a "verificada". Esta es la pieza que traemos de otra red.
5. **Adaptador para Trustless Work:** el hito se aprueba solo si la evidencia tiene una atestación verificada.
6. **Lectura de C2PA** cuando la foto trae Content Credentials.

## 4. Pitch en una línea
> AttestProtocol registra, Trustless Work paga y **Fidedigno revisa la evidencia antes**, con reglas de integridad de industrias reguladas (ALCOA+ y GAMP 5) y disputa optimista como UMA, algo que hoy no existe en Stellar.

## Pendiente de verificar
- Cómo guarda Trustless Work la evidencia del hito: el campo exacto en su API.
- Si AttestProtocol permite resolvers propios en Mainnet y cuánto cuesta atestar.
- Que no exista ya un oráculo optimista en Soroban.

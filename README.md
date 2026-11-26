# Fidedigno (nombre de trabajo)

Kit para builders de Stellar que **revisa el contenido de la evidencia** (fotos, audios, documentos y datos) antes de que un escrow libere un pago o un contrato la dé por buena.

Preparado para **Stellar Codember Perú 2026**.

> **Estado: punto de partida (checkpoint).** El repo tiene el diseño, el alcance y el backlog. Todavía **no** hay código implementado, contratos desplegados, hashes de Testnet ni video demo. Esas cosas se agregan con evidencia real antes del 4/12/2026.

## El problema
En Stellar ya se puede **registrar** quién afirma algo (AttestProtocol) y **pagar por hitos** (Trustless Work). Pero cuando alguien sube "la prueba" de un hito, sea una foto de la entrega, un audio de conformidad o un PDF, **nadie revisa si esa evidencia es real**:
- si la foto es de ese día y de ese lugar;
- si es original o está copiada de internet;
- si fue editada o generada con IA;
- si contradice hitos anteriores.

Hoy lo revisa una persona a ojo o no lo revisa nadie. Anclar un dato en la red prueba que **no cambió**, no que **sea verdad**.

## La propuesta: verificación mixta en tres capas
| Capa | Qué hace | De dónde viene |
|---|---|---|
| **1. Reglas de integridad** | Bloqueantes: sin firma del emisor o con huella repetida, se rechaza directo | **ALCOA+**, usado en industrias reguladas |
| **2. Puntaje de confianza** | Algoritmos propios suman señales: fecha y GPS de la foto contra lo declarado, foto firmada desde la cámara (C2PA), imagen repetida, edición o IA, coherencia con hitos anteriores | **Capa propia** |
| **3. Disputa con fianza** | Según el puntaje, la evidencia queda en un plazo donde cualquiera puede disputarla poniendo una fianza; el que mintió pierde la suya | Modelo del **oráculo optimista de UMA** (Ethereum), que no encontramos en Stellar |

El **nivel de riesgo GAMP 5** de cada contrato (bajo, medio o alto) define cuánto puntaje se exige, cuánto dura el plazo y qué fianza se pide.

| Puntaje | Resultado |
|---|---|
| Igual o mayor al corte alto | Pasa con un plazo de disputa corto y fianza baja |
| Entre el corte bajo y el alto | Plazo largo y fianza alta, o un verificador humano |
| Menor al corte bajo, o falla un bloqueante | Se rechaza y no hay pago |

Con cada disputa resuelta, el sistema **ajusta el peso de las señales**: la verificación mejora con el uso.

Detalle completo: [docs/MARCO-ALCOA-GAMP.md](docs/MARCO-ALCOA-GAMP.md).

## Cómo lo usaría un builder (objetivo del P0)
```rust
// Dentro del escrow del builder, antes de liberar un hito:
let ok = fidedigno::require_verified(&env, &fidedigno_id, &evidence_hash, RiskLevel::Medium);
if !ok { panic_with_error!(&env, Error::EvidenceNotVerified); }
```
Fuera de la red, el SDK en TypeScript revisa la evidencia y calcula el puntaje. El resultado se registra como atestación en **AttestProtocol** y el contrato de Fidedigno maneja el plazo de disputa y las fianzas. Arquitectura: [docs/ARQUITECTURA.md](docs/ARQUITECTURA.md).

## En qué se apoya (no lo reinventamos)
- **AttestProtocol:** registro de atestaciones en Soroban, en Testnet y Mainnet ([docs](https://docs.attest.so/)).
- **Trustless Work:** escrow por hitos en Stellar ([web](https://www.trustlesswork.com/)). Es el primer caso de integración.
- Comparación completa: [docs/INVESTIGACION-SIMILARES.md](docs/INVESTIGACION-SIMILARES.md).

## Público: devs de Stellar
Fidedigno es una **pieza del kit del builder**, como una librería o un oráculo. Un dev la agrega a su proyecto para que **los datos que entran a su contrato sean verdad** y su contrato no se apoye en pruebas falsas.

El usuario final no la ve: la usa el dev. Casos donde encaja:
- escrows y pagos por hitos;
- trazabilidad y activos del mundo real (RWA);
- seguros paramétricos;
- marketplaces y subsidios por resultados.

Lo que gana el dev: no tiene que inventar su propia verificación, su contrato es más confiable para sus usuarios e inversores, y cada verificación queda auditable en la red.

## Alcance del P0 (hasta el 4/12)
1. **SDK TypeScript** con bloqueantes ALCOA+ y 4 señales de puntaje para **fotos**: fecha y GPS de la foto (EXIF) contra lo declarado, foto repetida (huella perceptual), manifiesto C2PA si viene, y coherencia con el hito anterior.
2. **Tabla GAMP 5** de cortes, plazos y fianzas, con versión.
3. **Contrato Soroban `fidedigno`** en Testnet: recibe el resultado, abre el plazo de disputa, maneja las fianzas con un token de prueba y expone `require_verified`.
4. **Atestación del resultado en AttestProtocol** (Testnet).
5. **Escrow de ejemplo** que libera el hito solo con evidencia verificada.
6. **Evidencia:** direcciones, hashes de Testnet, pruebas automáticas y video de 3 minutos como máximo.

**Fuera del P0:** audio y video (solo huella y metadatos), detección de IA con modelos propios, jurado descentralizado, ajuste automático de pesos (en el P0 los pesos son fijos y declarados), red principal y auditoría externa.

## Qué NO promete
- No prueba al 100% que algo pasó en el mundo físico. **Sube la confianza** juntando señales independientes y **hace caro mentir** con la fianza.
- No es un oráculo de precios.
- No usa dinero real: solo Stellar Testnet y tokens de prueba.

## Hitos del evento
- Registro del repositorio en el Hub: 25/11/2026 23:59, hora de Lima. **Pendiente: confirmar con la organización si se acepta el registro.** Crear este repo no equivale a registrarlo.
- Entrega final: 4/12/2026 23:59, hora de Lima.
- Demo Day: 5/12/2026, Lima, presencial.

## Backlog
Se lleva en los Issues del repo. Lista inicial en [docs/BACKLOG.md](docs/BACKLOG.md).

## Autoría, trabajo previo e IA
- El código de este repo se escribe durante el período del evento, del 14/11 al 4/12/2026.
- **Trabajo previo declarado:** el autor desarrolló antes GALCOA+ (prototipo de remitos verificados con ALCOA+ en Stellar Testnet, para distribuidoras). Fidedigno reutiliza **ideas** de ese proyecto: reglas con identificador, huella de 32 bytes, falla cerrada y que la IA nunca decide el veredicto. Cambia el público (devs de Stellar) y agrega el puntaje y la disputa con fianza. **No se copia código.** Si más adelante se reutiliza algo, se declara con su origen y licencia.
- La documentación inicial se preparó con asistencia de IA.
- Licencia: pendiente de definir.

## Seguridad
Nunca se suben al repo claves ni semillas (tampoco de Testnet), archivos `.env`, datos personales ni evidencia real de terceros. Para las demos se usan datos sintéticos.

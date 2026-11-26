# Evidencia en Stellar Testnet

Corrida del 26/11/2026 con [`scripts/demo-testnet.sh`](../../scripts/demo-testnet.sh). Usa cuentas de prueba, XLM de Testnet y datos sintéticos.

## Contratos
| Contrato | ID | Hash del wasm |
|---|---|---|
| Verificación | [`CAWLRO5ND3ZUSVISVWN5OZB4PTVGZ2AUOSGOGZVTYE7N6W7IYWFYXSAV`](https://stellar.expert/explorer/testnet/contract/CAWLRO5ND3ZUSVISVWN5OZB4PTVGZ2AUOSGOGZVTYE7N6W7IYWFYXSAV) | `db488f30418ce1979f52d8a91a6a5e9037f72be10c01346d92412fbf02d34079` |
| Escrow de ejemplo | [`CA34LHQIDUJYYSMEQIRJZ6U6P662IWFPPJCFBSXOIXGYBQ2AKD6SITJX`](https://stellar.expert/explorer/testnet/contract/CA34LHQIDUJYYSMEQIRJZ6U6P662IWFPPJCFBSXOIXGYBQ2AKD6SITJX) | `9902995c0849f280005503eddcf07e05a38af0ccb01b2b4c7d78dd0bfed943b6` |

Token de las fianzas y del pago: XLM nativo (`CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC`).

## Cuentas de prueba
| Rol | Cuenta |
|---|---|
| Admin | `GCJ24M745K64LRZOQYHKEFDZPTCVKPKO4FBW2GJYNBO5VMN342NBL5MZ` |
| Emisor y beneficiario del escrow | `GBOGRXOG7WAUPHOGFD5Z2UDDXHBQOLY3E2UHTHYFJZMUV6SIQKDEBUOF` |
| Verificador | `GAYHTB2CQATH7LSA676CTXMAFJQPCIFK3N4LF2UJTQ5I4DBOLPRKVKIQ` |
| Pagador del escrow | `GCSHL3XQP3VYXOEUODCGC52SV6VASCSZTPXPWLQGF3GA63XIB74WRPGU` |

## Parámetros de la corrida
| Parámetro | Valor |
|---|---|
| Nivel de riesgo | Medio (ICE mínimo 0,80; k = 2) |
| Plazo de disputa | 60 segundos (valor de demostración) |
| Monto del hito | 10 XLM |
| ICE enviado | 0,9677 |
| Huella de la evidencia | `32d5659b78b396a87e89ecb24b2ca23a24085ff39cc8aada4648930d61eb8505` |
| Ancla | Ledger 5929442 |

## Recorrido
| Paso | Resultado | Transacción |
|---|---|---|
| Despliegue del contrato de verificación | Desplegado | [`c7fcfeaf…`](https://stellar.expert/explorer/testnet/tx/c7fcfeaf27f4738e20f6e3d5a063caf0ded270d842e3f649eb543953d20ff116) |
| Alta del emisor | Hecha | [`0f76e7b0…`](https://stellar.expert/explorer/testnet/tx/0f76e7b011deaa2d3c08d8092817a8872c0515206332b5e87ef9fc363d1cb974) |
| Alta del verificador | Hecha | [`8bd8acca…`](https://stellar.expert/explorer/testnet/tx/8bd8accae937b04abb3fadf0f33c3ba8364deb5dc82ac4c1dfd7613c9cfb3890) |
| Política del nivel medio | Guardada | [`a49b6bfd…`](https://stellar.expert/explorer/testnet/tx/a49b6bfd3d8ea3ba0310dfad7269c5eb5072b9074fcd376df92ff962d6b3e3b8) |
| Despliegue del escrow | Desplegado | [`98e73e1d…`](https://stellar.expert/explorer/testnet/tx/98e73e1d26bf2e8d3e77a7b0c82a9a4f5c634313ab2582323277494ac74e84ee) |
| Fondeo del escrow | 10 XLM depositados | [`e3ed6bb9…`](https://stellar.expert/explorer/testnet/tx/e3ed6bb94ad19b7cf572b5174c5b2d043bfc10937dcbdbd47d8a1660e4e4760f) |
| Envío de la evidencia | Fianza retenida: 0,646 XLM | [`8ebf1052…`](https://stellar.expert/explorer/testnet/tx/8ebf1052c2670127d28e77a40721c66955b6be566b68694fb2039e3da36b87e4) |
| Consulta con el plazo abierto | `false` | Solo lectura |
| Pago del escrow con el plazo abierto | Rechazado (error 4, evidencia no verificada) | No se envía |
| Cierre después del plazo | Evidencia verificada, fianza devuelta | [`bafe26ca…`](https://stellar.expert/explorer/testnet/tx/bafe26ca87ee177c7ee61be6eff48f446facc74e0bdcac25bf695f0a00b2954d) |
| Consulta después del cierre | `true` | Solo lectura |
| Pago del escrow | 10 XLM al beneficiario | [`841f5163…`](https://stellar.expert/explorer/testnet/tx/841f51632ebc8bcced95ab019db6246ce3dd9379877377a74b52c2c76dd2ec63) |

La fianza coincide con la fórmula: `2 × (1 − 0,9677) × 10 XLM = 0,646 XLM`.

## Qué no cubre esta corrida
- La disputa y su resolución. Están cubiertas por las pruebas automáticas, no por una corrida en Testnet.
- El cálculo del ICE. El valor se envió como dato; el módulo que lo calcula todavía no está implementado.
- La captura firmada. La huella es de un texto sintético, no de una imagen.

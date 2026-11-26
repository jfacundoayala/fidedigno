# Marco de verificación

> Diseño para el prototipo. Los valores numéricos son iniciales y declarados; no están calibrados con datos reales.

El marco tiene cuatro partes. Las tres primeras son propias de este proyecto; la cuarta toma principios conocidos como base.

1. [Captura firmada con ancla de red](#1-captura-firmada-con-ancla-de-red)
2. [Índice de confianza de la evidencia](#2-índice-de-confianza-de-la-evidencia-ice)
3. [Fianza proporcional y disputa](#3-fianza-proporcional-y-disputa)
4. [Base: reglas bloqueantes y niveles de riesgo](#4-base-reglas-bloqueantes-y-niveles-de-riesgo)

## 1. Captura firmada con ancla de red
Los metadatos de un archivo (EXIF) se pierden al reenviarlo y se pueden editar. Por eso la evidencia no depende de ellos: se captura desde una página del proyecto, que arma y firma este registro en el momento.

| Campo | Origen |
|---|---|
| Huella SHA-256 de la imagen | Calculada en el dispositivo |
| Huella perceptual | Calculada en el dispositivo |
| Ancla de red | Hash de un ledger reciente de Stellar |
| Ubicación | Geolocalización del dispositivo, si el emisor la autoriza |
| Firma | Clave del emisor |

El ancla acota el momento de la captura por los dos lados:
- **No antes:** el registro incluye el hash de un ledger, que no se podía conocer antes de que ese ledger cerrara.
- **No después:** el registro queda incluido en una transacción con fecha de red.

El contrato rechaza el envío si pasaron más ledgers que los permitidos entre el ancla y la transacción.

**Límite:** esto prueba cuándo se firmó la captura, no qué había delante de la cámara. Fotografiar una pantalla sigue siendo posible.

## 2. Índice de confianza de la evidencia (ICE)
Una suma de puntos trata igual una señal ausente y una señal en contra, y deja sumar señales que dependen unas de otras. El ICE combina las señales como **razones de verosimilitud**: cada resultado indica cuánto más probable es en evidencia auténtica que en evidencia falsa.

```
odds = Π_familias  limitar( Π_señales LR , 1/20 , 20 )
ICE  = odds / (1 + odds)
```

- `LR > 1` aporta a favor, `LR < 1` en contra y `LR = 1` es neutro.
- Una señal **ausente** tiene su propio LR, distinto del de una señal en contra.
- Las señales se agrupan en **familias**. El producto de cada familia se limita entre 1/20 y 20, para que señales que dependen entre sí no acumulen confianza de más.
- Se parte de odds 1 (ICE = 0,50): sin señales no hay confianza a favor ni en contra.

### Tabla de señales (`reglas v0.1`)
| Familia | Señal | ID | A favor | Ausente | En contra |
|---|---|---|---|---|---|
| Captura | Captura firmada con ancla válida | `S-CA-01` | 8 | 0,7 | rechazo |
| Captura | Fecha del ancla dentro de la ventana del hito | `S-CO-01` | 3 | 0,8 | 0,1 |
| Captura | Ubicación cerca del lugar declarado | `S-EX-01` | 3 | 0,9 | 0,15 |
| Captura | Manifiesto C2PA válido | `S-AT-02` | 5 | 1 | 0,1 |
| Contenido | Sin imágenes similares anteriores (huella perceptual) | `S-OR-02` | 1,5 | n/a | rechazo |
| Contexto | Coherencia con hitos anteriores del contrato | `S-CS-01` | 1,5 | 1 | 0,2 |

"Rechazo" significa que la evidencia no continúa: una firma de captura inválida o una imagen similar ya usada son bloqueantes.

### Ejemplos
| Caso (primer hito del contrato) | Odds | ICE | Resultado |
|---|---|---|---|
| Imagen reenviada, sin captura firmada | 0,504 × 1,5 = 0,756 | 0,43 | No alcanza ningún nivel |
| Captura firmada, sin ubicación | 8 × 3 × 0,9 × 1,5 = 32,4 → 20 × 1,5 = 30 | 0,97 | Alcanza el nivel alto |
| Captura firmada, fecha fuera de la ventana | 8 × 0,1 × 3 × 1,5 = 3,6 | 0,78 | Alcanza solo el nivel bajo |

### Cálculo reproducible
El módulo de verificación es determinista: con la misma imagen, el mismo registro de captura y la misma versión de reglas, da el mismo ICE. Cualquiera con acceso al archivo puede recalcularlo.

En el contrato, el ICE se guarda en puntos básicos (0 a 10.000).

## 3. Fianza proporcional y disputa
### Quién firma el resultado
El ICE **no lo declara el emisor**. Lo calcula y lo firma un **verificador** registrado, distinto del emisor, que tiene un depósito en el contrato. El envío requiere las dos firmas: la del emisor sobre la captura y la del verificador sobre el resultado.

### Fianza del emisor
La fianza cubre la pérdida esperada de la contraparte si la evidencia fuera falsa:

```
fianza = máx( fianza_mínima , k × (1 − ICE) × monto_del_hito )
```

Con mayor confianza, la fianza es menor. `k` depende del nivel de riesgo.

| Ejemplo (hito de 100) | ICE | k | Fianza |
|---|---|---|---|
| Nivel alto, captura firmada | 0,97 | 3 | 9,7 |
| Nivel medio, captura firmada | 0,97 | 2 | 6,5 |
| Nivel bajo, fecha fuera de la ventana | 0,78 | 1 | 21,7 |

### Disputa
Durante el plazo, quien tenga acceso al archivo puede disputar depositando una fianza igual. Hay dos tipos:

| Tipo | Qué se discute | Cómo se resuelve |
|---|---|---|
| De cálculo | El ICE firmado no corresponde al archivo y a las reglas | Se recalcula. Si el verificador firmó un valor incorrecto, pierde su depósito |
| De hecho | La evidencia es auténtica pero no prueba lo declarado | Decide un árbitro registrado |

- Quien pierde la disputa entrega su fianza a la otra parte, menos la comisión del árbitro.
- El árbitro deposita una garantía para registrarse, no puede ser el emisor, el verificador ni quien disputa, y cobra una comisión de la fianza perdida.
- Si vence el plazo sin disputa, la evidencia queda verificada y la fianza se devuelve.

**Límites del prototipo:**
- En la práctica, quien disputa es la contraparte del contrato, porque es quien conoce el hecho y accede al archivo.
- El árbitro de las disputas de hecho es un rol de confianza. El prototipo usa un único árbitro registrado.
- El verificador también es un rol de confianza; la disputa de cálculo permite controlarlo.

## 4. Base: reglas bloqueantes y niveles de riesgo
### Reglas bloqueantes
Toman los principios de integridad de datos ALCOA+. Si una falla, la evidencia se rechaza sin calcular el ICE.

| Principio | Regla | ID |
|---|---|---|
| Atribuible | La captura está firmada por un emisor registrado en el contrato | `AT-01` |
| Legible | El archivo es una imagen válida y decodificable | `LE-01` |
| Original | No existe una imagen similar ya usada (huella perceptual dentro del umbral) | `OR-01` |
| Contemporáneo | El ancla de red está dentro del margen de ledgers permitido | `CO-00` |
| Completo | Están todos los campos obligatorios del hito | `CP-01` |
| Perdurable y disponible | La huella queda en la red y el archivo es accesible para las partes durante el plazo | `PE-01` |

La regla `OR-01` usa la huella perceptual y no la SHA-256, para que recomprimir o recortar la imagen no la evite.

### Niveles de riesgo
Cada contrato declara su nivel, con un criterio de esfuerzo proporcional al riesgo inspirado en GAMP 5.

| Nivel | ICE mínimo | Captura firmada | Plazo de disputa | k |
|---|---|---|---|---|
| Bajo | 0,50 | Opcional | 2 h | 1 |
| Medio | 0,80 | Obligatoria | 24 h | 2 |
| Alto | 0,95 | Obligatoria | 72 h | 3 |

## Datos en la red
En la red se guarda solo la huella de la imagen (32 bytes), el ICE, la huella de las señales, la versión de reglas, el nivel y las cuentas que firmaron. Nunca imágenes ni datos personales.

Si no hay un resultado verificado, la consulta del contrato devuelve falso.

## Limitaciones
- El ICE estima confianza; no prueba que un hecho haya ocurrido.
- Los LR de la tabla son valores iniciales elegidos por criterio. Calibrarlos requiere datos de evidencia auténtica y falsa, y queda fuera del prototipo.
- El límite por familia reduce el efecto de señales dependientes, pero no lo elimina.
- Este proyecto toma ideas de ALCOA+ y GAMP 5. No es una certificación ni una validación regulatoria.

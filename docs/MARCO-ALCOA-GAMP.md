# Marco de verificación mixta: ALCOA+ + puntaje propio + disputa con fianza + GAMP 5

Flujo: **bloqueantes (ALCOA+) → puntaje (señales propias) → decisión según el nivel GAMP 5 → plazo de disputa con fianza → verificada o rechazada.**

## 1. Bloqueantes (ALCOA+)
Si alguno falla, la evidencia se rechaza sin calcular el puntaje.

| Principio | Chequeo | ID |
|---|---|---|
| Atribuible | La evidencia viene firmada por una cuenta Stellar registrada como emisora del contrato | `AT-01` |
| Legible | El archivo es válido y decodificable (imagen, PDF, JSON con esquema) | `LE-01` |
| Original | La huella SHA-256 no se usó antes en otro hito | `OR-01` |
| Completo | Están todos los archivos y campos obligatorios del hito | `CP-01` |
| Perdurable / Disponible | La huella queda en la red y el archivo en un almacenamiento declarado y consultable | `PE-01` |

## 2. Puntaje de confianza (capa propia)
`puntaje = Σ (peso_i × señal_i)`, con cada señal entre 0 y 1 y la suma de los pesos igual a 100.

| Señal | Qué mide | Principio ALCOA+ | Peso inicial P0 | ID |
|---|---|---|---|---|
| Fecha coherente | Fecha EXIF dentro de la ventana del hito y cerca del momento del envío | Contemporáneo | 25 | `S-CO-01` |
| Lugar coherente | GPS EXIF a menos de X km del lugar declarado | Exacto | 20 | `S-EX-01` |
| Sin repetición | La huella perceptual no se parece a evidencia anterior (detecta fotos recicladas o recortadas) | Original | 25 | `S-OR-02` |
| Origen firmado | Trae un manifiesto C2PA válido y sin ediciones declaradas | Atribuible / Original | 15 | `S-AT-02` |
| Consistencia | Coherente con los hitos anteriores del mismo contrato (orden, cantidades, emisor) | Consistente | 15 | `S-CS-01` |

- Si una señal **no se puede calcular**, por ejemplo una foto sin EXIF, vale 0. Nunca se asume a favor (falla cerrada).
- En el P0 los pesos son **fijos y declarados**, con versión (`reglas v0.1`).
- Más adelante, los pesos se ajustan con los resultados de las disputas: si una señal dio "alta" en evidencias que después se probaron falsas, baja su peso. Cada ajuste genera una versión nueva de las reglas.
- La IA puede **aportar una señal**, pero **nunca decide el veredicto**.

## 3. Nivel de riesgo (GAMP 5)
Cada contrato declara su nivel al integrarse. El nivel fija los cortes, el plazo y la fianza.

| Nivel | Ejemplo | Corte alto | Corte bajo | Plazo de disputa | Fianza del emisor |
|---|---|---|---|---|---|
| Bajo | Constancia de asistencia | 70 | 40 | 2 h | Mínima |
| Medio | Hito de entrega con pago | 85 | 60 | 24 h | % del hito |
| Alto | Liberación grande, seguro | 95 | 75 | 72 h + revisión humana | % mayor del hito |

Los valores son **iniciales, para la demo**. Se calibran con datos.

## 4. Disputa con fianza (modelo optimista de UMA)
1. Si la evidencia pasa los bloqueantes y el corte bajo, el emisor deja la **fianza** y se abre el **plazo**.
2. Durante el plazo, cualquiera (por ejemplo, la contraparte del escrow) puede **disputar** poniendo una fianza igual.
3. Si nadie la disputa, queda **verificada**, el emisor recupera su fianza y `require_verified` devuelve verdadero.
4. Si alguien la disputa, decide un **árbitro** (en el P0, un verificador registrado distinto del emisor). El que pierde entrega su fianza al otro.
5. El resultado de cada disputa alimenta el ajuste de los pesos del punto 2.

## 5. Reglas de la capa propia
- On-chain va solo la **huella** (32 bytes), nunca fotos, audios ni datos personales.
- **Falla cerrada:** sin resultado válido, `require_verified` devuelve falso.
- **Separación de roles:** el emisor no puede ser árbitro de su propia evidencia.
- Cada resultado registra: huella, puntaje, señales, versión de reglas, nivel, verificadores y fecha.

## Límites
- Este marco toma ideas de ALCOA+ y GAMP 5. **No es una certificación ni una validación regulatoria.**
- El puntaje **sube la confianza**, no prueba la verdad. La fianza hace caro mentir, pero no lo impide.
- El EXIF se puede falsificar. Por eso pesa junto a otras señales y nunca solo.

#!/usr/bin/env bash
# Despliega los dos contratos en Stellar Testnet y recorre el flujo completo:
# envío de una evidencia, plazo de disputa, cierre y pago del escrow.
#
# Uso: bash scripts/contratos.sh script scripts/demo-testnet.sh
# Requiere haber corrido antes: bash scripts/contratos.sh build
#
# Usa valores de demostración: plazo de 60 segundos y datos sintéticos.
set -euo pipefail

RED=testnet
RPC=https://soroban-testnet.stellar.org
MONTO=100000000        # 10 XLM, en stroops
NIVEL=2                # medio
ICE=9677               # 0,9677
PLAZO=60               # segundos

paso() { echo; echo "== $*"; }
invocar() { # invocar <id> <cuenta> <función y argumentos...>
  local id="$1" cuenta="$2"; shift 2
  stellar contract invoke --id "$id" --source-account "$cuenta" --network "$RED" -- "$@"
}

paso "Cuentas de prueba"
for cuenta in fd-admin fd-emisor fd-verificador fd-pagador; do
  stellar keys address "$cuenta" >/dev/null 2>&1 || stellar keys generate "$cuenta" --network "$RED" --fund
  echo "$cuenta: $(stellar keys address "$cuenta")"
done
ADMIN=$(stellar keys address fd-admin)
EMISOR=$(stellar keys address fd-emisor)
VERIFICADOR=$(stellar keys address fd-verificador)
PAGADOR=$(stellar keys address fd-pagador)
TOKEN=$(stellar contract id asset --asset native --network "$RED")
echo "token (XLM nativo): $TOKEN"

paso "Despliegue del contrato de verificación"
VERIFICACION=$(stellar contract deploy --wasm wasm/fidedigno_verificacion.wasm \
  --source-account fd-admin --network "$RED" \
  -- --admin "$ADMIN" --token "$TOKEN" --max_anchor_age 120)
echo "verificacion: $VERIFICACION"

paso "Roles y política del nivel medio"
invocar "$VERIFICACION" fd-admin register_issuer --issuer "$EMISOR"
invocar "$VERIFICACION" fd-admin register_verifier --verifier "$VERIFICADOR"
invocar "$VERIFICACION" fd-admin set_policy --level "$NIVEL" \
  --policy "{\"min_ice_bps\":8000,\"window_secs\":$PLAZO,\"k\":2,\"min_bond\":\"1000000\"}"

paso "Despliegue y fondeo del escrow de ejemplo"
ESCROW=$(stellar contract deploy --wasm wasm/fidedigno_escrow_ejemplo.wasm \
  --source-account fd-admin --network "$RED" \
  -- --config "{\"verification\":\"$VERIFICACION\",\"token\":\"$TOKEN\",\"payer\":\"$PAGADOR\",\"payee\":\"$EMISOR\",\"amount\":\"$MONTO\",\"min_level\":$NIVEL}")
echo "escrow: $ESCROW"
invocar "$ESCROW" fd-pagador fund

paso "Envío de la evidencia"
HUELLA=$(printf 'fidedigno-demo-%s' "$(date +%s)" | sha256sum | cut -d' ' -f1)
SENALES=$(printf 'reglas-v0.1' | sha256sum | cut -d' ' -f1)
ANCLA=$(curl -s -X POST "$RPC" -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","id":1,"method":"getLatestLedger"}' | grep -o '"sequence":[0-9]*' | cut -d: -f2)
echo "huella: $HUELLA"
echo "ancla (ledger): $ANCLA"
invocar "$VERIFICACION" fd-emisor submit --hash "$HUELLA" --issuer "$EMISOR" --verifier "$VERIFICADOR" \
  --amount "$MONTO" --level "$NIVEL" \
  --result "{\"ice_bps\":$ICE,\"signals_hash\":\"$SENALES\",\"rules_version\":1}" \
  --anchor_ledger "$ANCLA"

paso "Con el plazo abierto, la consulta da falso y el escrow no paga"
invocar "$VERIFICACION" fd-admin require_verified --hash "$HUELLA" --min_level "$NIVEL"
invocar "$ESCROW" fd-admin release --hash "$HUELLA" || echo "(rechazado, como se esperaba)"

paso "Espera del plazo de disputa ($PLAZO s)"
sleep $((PLAZO + 10))

paso "Cierre: la evidencia queda verificada y se devuelve la fianza"
invocar "$VERIFICACION" fd-admin finalize --hash "$HUELLA"
invocar "$VERIFICACION" fd-admin require_verified --hash "$HUELLA" --min_level "$NIVEL"

paso "El escrow paga"
invocar "$ESCROW" fd-admin release --hash "$HUELLA"
invocar "$ESCROW" fd-admin state

paso "Resumen"
echo "VERIFICACION=$VERIFICACION"
echo "ESCROW=$ESCROW"
echo "HUELLA=$HUELLA"

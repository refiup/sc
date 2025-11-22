# 📖 Guía de Deployment - ReFi Universe Contracts

## 📋 Tabla de Contenidos

- [Contratos Desplegados](#-contratos-desplegados)
- [Vault Distributor Deployment](#-vault-distributor-deployment)
- [Event Distributor Deployment](#-event-distributor-deployment)
- [Verificación](#-verificación)
- [Uso Básico](#-uso-básico)

---

## 🎯 Contratos Desplegados

### Vault Distributor (Production)
**Contract ID:** `CC4XEZG3JIVNTWGNPL4YKIWYECSOTS66SFLIDI3WU6RIJFNDNWPIMVHM`  
**Network:** Stellar Testnet  
**Status:** ✅ Active  
**Explorer:** [Ver en Stellar Expert](https://stellar.expert/explorer/testnet/contract/CC4XEZG3JIVNTWGNPL4YKIWYECSOTS66SFLIDI3WU6RIJFNDNWPIMVHM)

### Event Distributor (Ready)
**Status:** ✅ Ready for deployment  
**Script:** `./deploy_event_distributor.sh`

### Vault RefiUp (Reference)
**Vault Contract:** `CA3N53CPBLSVM5342DZ25LK47WFQDS6R3BT62327SGZCNJ54CDDXO7KZ`  
**Manager/Admin:** `GAUSHAKPQJEHLKT4FBUVMWXOX3HUVV5OXNFUY54OBWS2R7ZUQY6QUBR6`  
**Token Pair:** XLM/USDC  
**Network:** Stellar Testnet

---

## 🏦 Vault Distributor Deployment

---

## 🚀 Deployment Rápido

### Opción 1: Script Automatizado (Recomendado)

```bash
# Ejecutar script de deployment
./deploy.sh
```

El script automáticamente:
- ✅ Compila el contrato si es necesario
- ✅ Crea/verifica la wallet admin
- ✅ Despliega el contrato a testnet
- ✅ Inicializa el contrato con el admin
- ✅ Verifica el deployment
- ✅ Guarda la configuración en `.env`

### Opción 2: Deployment Manual

```bash
# 1. Compilar contrato
cd contracts/vault-distributor
cargo build --target wasm32v1-none --release
cd ../..

# 2. Crear wallet admin (si no existe)
stellar keys generate admin --network testnet --fund

# 3. Desplegar contrato
stellar contract deploy \
  --wasm target/wasm32v1-none/release/vault_distributor.wasm \
  --source-account admin \
  --network testnet \
  --alias vault_distributor

# 4. Obtener Contract ID
CONTRACT_ID=$(stellar contract id vault_distributor --network testnet)
ADMIN_ADDRESS=$(stellar keys address admin)

# 5. Inicializar contrato
stellar contract invoke \
  --id $CONTRACT_ID \
  --source-account admin \
  --network testnet \
  -- init \
  --admin $ADMIN_ADDRESS

# 6. Verificar
stellar contract invoke \
  --id $CONTRACT_ID \
  --network testnet \
  -- get_admin
```

---

## 💰 Fondear el Contrato

Para poder distribuir tokens, primero debes transferir tokens al contrato:

```bash
# Ejemplo: Transferir USDC al contrato distribuidor
stellar contract invoke \
  --id <USDC_TOKEN_CONTRACT> \
  --source-account admin \
  --network testnet \
  -- transfer \
  --from $ADMIN_ADDRESS \
  --to $CONTRACT_ID \
  --amount 10000
```

---

## 📤 Distribuir Fondos

### Ejemplo 1: Distribución Simple (3 destinatarios)

```bash
# Crear archivo con destinatarios
cat > recipients.json << 'EOF'
[
  "GAUSHAKPQJEHLKT4FBUVMWXOX3HUVV5OXNFUY54OBWS2R7ZUQY6QUBR6",
  "GBXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
  "GCXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
]
EOF

# Distribuir 9000 tokens (3000 cada uno)
stellar contract invoke \
  --id $CONTRACT_ID \
  --source-account admin \
  --network testnet \
  -- distribute \
  --token <TOKEN_CONTRACT_ID> \
  --recipients "$(cat recipients.json)" \
  --total_amount 9000
```

### Ejemplo 2: Distribución con Vector Inline

```bash
stellar contract invoke \
  --id $CONTRACT_ID \
  --source-account admin \
  --network testnet \
  -- distribute \
  --token CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC \
  --recipients '["GABC123...", "GDEF456...", "GHIJ789..."]' \
  --total_amount 15000
```

### Ejemplo 3: Distribución desde Vault RefiUp

```bash
# Token USDC en testnet
USDC_TOKEN="CBIELTK6YBZJU5UP2WWQEUCYKLPU6AUNZ2BQ4WWFEIE3USCIHMXQDAMA"

# Distribuir ganancias del vault
stellar contract invoke \
  --id $CONTRACT_ID \
  --source-account admin \
  --network testnet \
  -- distribute \
  --token $USDC_TOKEN \
  --recipients '["GAUSHAKPQJEHLKT4FBUVMWXOX3HUVV5OXNFUY54OBWS2R7ZUQY6QUBR6", "GBXXX..."]' \
  --total_amount 50000
```

---

## 🔍 Consultas

### Ver Admin del Contrato

```bash
stellar contract invoke \
  --id $CONTRACT_ID \
  --network testnet \
  -- get_admin
```

### Ver Balance del Contrato

```bash
# Ver balance de un token específico
stellar contract invoke \
  --id <TOKEN_CONTRACT> \
  --network testnet \
  -- balance \
  --id $CONTRACT_ID
```

### Ver Eventos del Contrato

```bash
# Ver últimos eventos emitidos
stellar events \
  --start-ledger <LEDGER_NUMBER> \
  --network testnet \
  --id $CONTRACT_ID
```

---

## 🧪 Testing en Testnet

### Crear Wallets de Prueba

```bash
# Crear 3 wallets de prueba
stellar keys generate recipient1 --network testnet --fund
stellar keys generate recipient2 --network testnet --fund
stellar keys generate recipient3 --network testnet --fund

# Ver direcciones
stellar keys address recipient1
stellar keys address recipient2
stellar keys address recipient3
```

### Distribución de Prueba

```bash
RECIPIENT1=$(stellar keys address recipient1)
RECIPIENT2=$(stellar keys address recipient2)
RECIPIENT3=$(stellar keys address recipient3)

stellar contract invoke \
  --id $CONTRACT_ID \
  --source-account admin \
  --network testnet \
  -- distribute \
  --token <TEST_TOKEN> \
  --recipients "[\"$RECIPIENT1\", \"$RECIPIENT2\", \"$RECIPIENT3\"]" \
  --total_amount 3000
```

---

## ⚠️ Validaciones y Errores

El contrato valida automáticamente:

- ✅ **Solo el admin puede distribuir**: Otros wallets recibirán error de autenticación
- ✅ **Lista de destinatarios no vacía**: Mínimo 1 destinatario requerido
- ✅ **Monto positivo**: `total_amount` debe ser > 0
- ✅ **División válida**: `amount_each` debe ser > 0 después de dividir

### Ejemplos de Errores

```bash
# ❌ Lista vacía
-- distribute --recipients "[]" --total_amount 1000
# Error: "Recipients list cannot be empty"

# ❌ Monto cero
-- distribute --recipients "[\"GABC...\"]" --total_amount 0
# Error: "Total amount must be positive"

# ❌ Monto muy pequeño
-- distribute --recipients "[\"GA1\", \"GA2\", \"GA3\"]" --total_amount 2
# Error: "Amount per recipient must be greater than zero"

# ❌ Wallet no autorizada
stellar contract invoke --source-account unauthorized ...
# Error: require_auth failed
```

---

## 📊 Integración con Vault RefiUp

### Flujo Recomendado

1. **Generar rendimientos en el vault**
   - Los usuarios depositan en: `CA3N53CPBLSVM5342DZ25LK47WFQDS6R3BT62327SGZCNJ54CDDXO7KZ`
   - Las estrategias (XlmBlendAutocompound) generan intereses

2. **Retirar ganancias del vault**
   ```bash
   stellar contract invoke \
     --id CA3N53CPBLSVM5342DZ25LK47WFQDS6R3BT62327SGZCNJ54CDDXO7KZ \
     --source-account admin \
     --network testnet \
     -- withdraw \
     --amount <PROFITS>
   ```

3. **Transferir al distribuidor**
   ```bash
   stellar contract invoke \
     --id <TOKEN_CONTRACT> \
     --source-account admin \
     --network testnet \
     -- transfer \
     --from GAUSHAKPQJEHLKT4FBUVMWXOX3HUVV5OXNFUY54OBWS2R7ZUQY6QUBR6 \
     --to $CONTRACT_ID \
     --amount <PROFITS>
   ```

4. **Distribuir a beneficiarios**
   ```bash
   stellar contract invoke \
     --id $CONTRACT_ID \
     --source-account admin \
     --network testnet \
     -- distribute \
     --token <TOKEN_CONTRACT> \
     --recipients '["BENEFICIARY1", "BENEFICIARY2", ...]' \
     --total_amount <PROFITS>
   ```

---

## 🎪 Event Distributor Deployment

### Opción 1: Script Automatizado (Recomendado)

```bash
# Ejecutar script de deployment
./deploy_event_distributor.sh
```

El script automáticamente:
- ✅ Compila el contrato a WASM
- ✅ Verifica/crea la wallet admin
- ✅ Despliega el contrato a testnet
- ✅ Inicializa con el admin
- ✅ Verifica el deployment
- ✅ Guarda configuración en `.env.event-distributor`

### Opción 2: Deployment Manual

```bash
# 1. Compilar contrato
cd contracts/event-distributor
cargo build --target wasm32-unknown-unknown --release
cd ../..

# 2. Desplegar
stellar contract deploy \
  --wasm contracts/event-distributor/target/wasm32-unknown-unknown/release/event_distributor.wasm \
  --source-account admin \
  --network testnet \
  --alias event_distributor

# 3. Obtener Contract ID
CONTRACT_ID=$(stellar contract id event_distributor --network testnet)
ADMIN_ADDRESS=$(stellar keys address admin)

# 4. Inicializar
stellar contract invoke \
  --id $CONTRACT_ID \
  --source-account admin \
  --network testnet \
  -- init \
  --admin $ADMIN_ADDRESS

# 5. Verificar
stellar contract invoke \
  --id $CONTRACT_ID \
  --network testnet \
  -- get_admin
```

---

## 🎯 Uso del Event Distributor

### 1. Agregar Participantes

```bash
# Agregar participante con IPFS metadata
stellar contract invoke \
  --id $CONTRACT_ID \
  --source-account admin \
  --network testnet \
  -- add_human \
  --human_address GABC123... \
  --ipfs_hash "QmX7fK8R9dH3zLq4wN8pY2tM5cB6vJ1eA9oF4xK3nH2gP8"
```

### 2. Validar Participantes

```bash
# Validar participante (solo admin)
stellar contract invoke \
  --id $CONTRACT_ID \
  --source-account admin \
  --network testnet \
  -- update_human_validation \
  --human_address GABC123... \
  --validated true

# Invalidar participante
stellar contract invoke \
  --id $CONTRACT_ID \
  --source-account admin \
  --network testnet \
  -- update_human_validation \
  --human_address GABC123... \
  --validated false
```

### 3. Crear Evento

```bash
# Crear evento con pool de fondos
stellar contract invoke \
  --id $CONTRACT_ID \
  --source-account admin \
  --network testnet \
  -- create_event \
  --event_id "refi_meetup_001" \
  --location "Buenos Aires, Argentina" \
  --pool 1000000000  # 100 XLM
```

### 4. Agregar Participantes al Evento

```bash
# Agregar participante al evento
stellar contract invoke \
  --id $CONTRACT_ID \
  --source-account admin \
  --network testnet \
  -- add_human_to_event \
  --event_id "refi_meetup_001" \
  --human_address GABC123...
```

### 5. Ver Participantes Validados

```bash
# Listar solo participantes validados del evento
stellar contract invoke \
  --id $CONTRACT_ID \
  --network testnet \
  -- get_event_validated_humans \
  --event_id "refi_meetup_001"
```

### 6. Distribuir Pool del Evento

```bash
# Distribuir equitativamente solo a participantes validados
stellar contract invoke \
  --id $CONTRACT_ID \
  --source-account admin \
  --network testnet \
  -- distribute_event_pool \
  --event_id "refi_meetup_001" \
  --token CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC
```

### 7. Consultar Participantes (con Paginación)

```bash
# Obtener primeros 10 participantes
stellar contract invoke \
  --id $CONTRACT_ID \
  --network testnet \
  -- get_all_humans \
  --start_index 0 \
  --limit 10

# Obtener siguientes 10
stellar contract invoke \
  --id $CONTRACT_ID \
  --network testnet \
  -- get_all_humans \
  --start_index 10 \
  --limit 10
```

---

## 📊 Ejemplo de Flujo Completo

### Escenario: ReFi Meetup con 5 participantes, 3 validados

```bash
# Variables
CONTRACT_ID="<YOUR_CONTRACT_ID>"
EVENT_ID="refi_ba_2025"
LOCATION="Buenos Aires"
POOL=5000000000  # 500 XLM
TOKEN="CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC"

# 1. Agregar 5 participantes
for i in {1..5}; do
  stellar contract invoke --id $CONTRACT_ID --source-account admin --network testnet \
    -- add_human --human_address "$(stellar keys address participant$i)" \
    --ipfs_hash "QmHash$i"
done

# 2. Validar solo 3 participantes
for i in {1..3}; do
  stellar contract invoke --id $CONTRACT_ID --source-account admin --network testnet \
    -- update_human_validation --human_address "$(stellar keys address participant$i)" \
    --validated true
done

# 3. Crear evento
stellar contract invoke --id $CONTRACT_ID --source-account admin --network testnet \
  -- create_event --event_id "$EVENT_ID" --location "$LOCATION" --pool $POOL

# 4. Agregar los 5 participantes al evento
for i in {1..5}; do
  stellar contract invoke --id $CONTRACT_ID --source-account admin --network testnet \
    -- add_human_to_event --event_id "$EVENT_ID" \
    --human_address "$(stellar keys address participant$i)"
done

# 5. Distribuir pool (solo a los 3 validados)
# Resultado: 166.67 XLM cada uno (500 / 3)
stellar contract invoke --id $CONTRACT_ID --source-account admin --network testnet \
  -- distribute_event_pool --event_id "$EVENT_ID" --token $TOKEN
```

---

## 🔗 Enlaces Útiles

- **Vault Distributor Explorer**: https://stellar.expert/explorer/testnet/contract/CC4XEZG3JIVNTWGNPL4YKIWYECSOTS66SFLIDI3WU6RIJFNDNWPIMVHM
- **Vault RefiUp Explorer**: https://stellar.expert/explorer/testnet/contract/CA3N53CPBLSVM5342DZ25LK47WFQDS6R3BT62327SGZCNJ54CDDXO7KZ
- **Stellar Expert Testnet**: https://stellar.expert/explorer/testnet
- **Stellar Lab**: https://lab.stellar.org/
- **Stellar RPC**: https://soroban-testnet.stellar.org
- **Friendbot (Fondeo)**: https://friendbot.stellar.org

---

## 📝 Notas Importantes

### General
1. **Gas Fees**: Todas las transacciones requieren XLM para fees
2. **TTL (Time To Live)**: Los contratos en testnet expiran después de cierto tiempo
3. **Testnet Reset**: Testnet se resetea ocasionalmente, guarda tu configuración
4. **Eventos**: Todas las operaciones emiten eventos para auditoría

### Vault Distributor
- **Balance del Contrato**: Verifica que el contrato tenga fondos antes de distribuir
- **Parámetros en tiempo real**: Recipients se proveen en cada distribución

### Event Distributor
- **Almacenamiento On-chain**: Participants y events se guardan permanentemente
- **Validación Requerida**: Solo participantes con `validated: true` reciben fondos
- **IPFS Metadata**: Contract solo guarda hash, no valida contenido
- **Pagination**: Usa `get_all_humans` con índices para listas grandes

---

## 🆘 Troubleshooting

### Vault Distributor

#### El contrato no tiene fondos suficientes
```bash
# Verificar balance
stellar contract invoke --id <TOKEN> --network testnet -- balance --id $CONTRACT_ID

# Transferir más fondos
stellar contract invoke --id <TOKEN> --source-account admin --network testnet \
  -- transfer --from <YOUR_WALLET> --to $CONTRACT_ID --amount <AMOUNT>
```

#### Error de autenticación
```bash
# Verificar que eres el admin
stellar contract invoke --id $CONTRACT_ID --network testnet -- get_admin

# Comparar con tu address
stellar keys address admin
```

### Event Distributor

#### Error: HumanNotFound
```bash
# Verificar que el participante existe
stellar contract invoke --id $CONTRACT_ID --network testnet \
  -- get_human --human_address <ADDRESS>

# Si no existe, agregarlo primero
stellar contract invoke --id $CONTRACT_ID --source-account admin --network testnet \
  -- add_human --human_address <ADDRESS> --ipfs_hash "<HASH>"
```

#### Error: NoValidatedHumans
```bash
# Ver participantes validados del evento
stellar contract invoke --id $CONTRACT_ID --network testnet \
  -- get_event_validated_humans --event_id "<EVENT_ID>"

# Validar al menos un participante
stellar contract invoke --id $CONTRACT_ID --source-account admin --network testnet \
  -- update_human_validation --human_address <ADDRESS> --validated true
```

#### Error: ZeroAmountPerRecipient
```bash
# El pool es muy pequeño para la cantidad de validados
# Solución: Aumentar el pool o reducir participantes validados

# Ver detalles del evento
stellar contract invoke --id $CONTRACT_ID --network testnet \
  -- get_event --event_id "<EVENT_ID>"
```

### Ambos Contratos

#### Contrato expiró en testnet
```bash
# Re-desplegar Vault Distributor
./deploy.sh

# Re-desplegar Event Distributor
./deploy_event_distributor.sh
```

---

**Última actualización**: 22 de noviembre de 2025

# 📖 Guía de Deployment y Uso - Vault Distributor

## 🎯 Información del Vault RefiUp

**Vault Contract:** `CA3N53CPBLSVM5342DZ25LK47WFQDS6R3BT62327SGZCNJ54CDDXO7KZ`  
**Manager/Admin:** `GAUSHAKPQJEHLKT4FBUVMWXOX3HUVV5OXNFUY54OBWS2R7ZUQY6QUBR6`  
**Token Pair:** XLM/USDC  
**Network:** Stellar Testnet  
**Explorer:** [Ver en Stellar Expert](https://stellar.expert/explorer/testnet/contract/CA3N53CPBLSVM5342DZ25LK47WFQDS6R3BT62327SGZCNJ54CDDXO7KZ)

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

## 🔗 Enlaces Útiles

- **Vault RefiUp Explorer**: https://stellar.expert/explorer/testnet/contract/CA3N53CPBLSVM5342DZ25LK47WFQDS6R3BT62327SGZCNJ54CDDXO7KZ
- **Stellar Expert Testnet**: https://stellar.expert/explorer/testnet
- **Stellar Lab**: https://lab.stellar.org/
- **Stellar RPC**: https://soroban-testnet.stellar.org
- **Friendbot (Fondeo)**: https://friendbot.stellar.org

---

## 📝 Notas Importantes

1. **Gas Fees**: Todas las transacciones requieren XLM para fees
2. **TTL (Time To Live)**: Los contratos en testnet expiran después de cierto tiempo
3. **Testnet Reset**: Testnet se resetea ocasionalmente, guarda tu configuración
4. **Balance del Contrato**: Verifica que el contrato tenga suficientes fondos antes de distribuir
5. **Eventos**: Todos los distribuciones emiten eventos para auditoría

---

## 🆘 Troubleshooting

### El contrato no tiene fondos suficientes
```bash
# Verificar balance
stellar contract invoke --id <TOKEN> --network testnet -- balance --id $CONTRACT_ID

# Transferir más fondos
stellar contract invoke --id <TOKEN> --source-account admin --network testnet \
  -- transfer --from <YOUR_WALLET> --to $CONTRACT_ID --amount <AMOUNT>
```

### Error de autenticación
```bash
# Verificar que eres el admin
stellar contract invoke --id $CONTRACT_ID --network testnet -- get_admin

# Comparar con tu address
stellar keys address admin
```

### Contrato expiró en testnet
```bash
# Re-desplegar
./deploy.sh
```

---

**Última actualización**: 21 de noviembre de 2025

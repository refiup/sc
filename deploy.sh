#!/bin/bash

# Script de deployment para Vault Distributor
# Network: Stellar Testnet

set -e

echo "🚀 Iniciando deployment de Vault Distributor..."
echo ""

# Colores para output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuración
NETWORK="testnet"
ADMIN_KEY_NAME="admin"
WASM_PATH="target/wasm32v1-none/release/vault_distributor.wasm"

# Paso 1: Verificar que el WASM existe
echo -e "${BLUE}📦 Paso 1: Verificando archivo WASM...${NC}"
if [ ! -f "$WASM_PATH" ]; then
    echo -e "${YELLOW}⚠️  WASM no encontrado. Compilando...${NC}"
    cd contracts/vault-distributor
    cargo build --target wasm32v1-none --release
    cd ../..
fi
echo -e "${GREEN}✓ WASM listo${NC}"
echo ""

# Paso 2: Verificar/Crear wallet admin
echo -e "${BLUE}🔑 Paso 2: Configurando wallet admin...${NC}"
if stellar keys show $ADMIN_KEY_NAME 2>/dev/null; then
    echo -e "${GREEN}✓ Wallet admin ya existe${NC}"
else
    echo -e "${YELLOW}⚠️  Creando nueva wallet admin...${NC}"
    stellar keys generate $ADMIN_KEY_NAME --network $NETWORK --fund
    echo -e "${GREEN}✓ Wallet admin creada y fondeada${NC}"
fi

ADMIN_ADDRESS=$(stellar keys address $ADMIN_KEY_NAME)
echo -e "${GREEN}Admin Address: $ADMIN_ADDRESS${NC}"
echo ""

# Paso 3: Desplegar contrato
echo -e "${BLUE}🚀 Paso 3: Desplegando contrato a testnet...${NC}"
CONTRACT_ID=$(stellar contract deploy \
  --wasm $WASM_PATH \
  --source-account $ADMIN_KEY_NAME \
  --network $NETWORK \
  2>&1 | grep -o 'C[A-Z0-9]\{55\}' | head -1)

if [ -z "$CONTRACT_ID" ]; then
    echo -e "${YELLOW}⚠️  No se pudo extraer CONTRACT_ID del output. Intentando con alias...${NC}"
    stellar contract deploy \
      --wasm $WASM_PATH \
      --source-account $ADMIN_KEY_NAME \
      --network $NETWORK \
      --alias vault_distributor
    
    CONTRACT_ID=$(stellar contract id vault_distributor --network $NETWORK)
fi

echo -e "${GREEN}✓ Contrato desplegado${NC}"
echo -e "${GREEN}Contract ID: $CONTRACT_ID${NC}"
echo ""

# Paso 4: Inicializar contrato
echo -e "${BLUE}⚙️  Paso 4: Inicializando contrato con admin...${NC}"
stellar contract invoke \
  --id $CONTRACT_ID \
  --source-account $ADMIN_KEY_NAME \
  --network $NETWORK \
  -- init \
  --admin $ADMIN_ADDRESS

echo -e "${GREEN}✓ Contrato inicializado${NC}"
echo ""

# Paso 5: Verificar deployment
echo -e "${BLUE}✅ Paso 5: Verificando deployment...${NC}"
STORED_ADMIN=$(stellar contract invoke \
  --id $CONTRACT_ID \
  --source-account $ADMIN_KEY_NAME \
  --network $NETWORK \
  -- get_admin)

echo -e "${GREEN}Admin almacenado: $STORED_ADMIN${NC}"
echo ""

# Guardar información en archivo .env
echo -e "${BLUE}💾 Guardando configuración en .env...${NC}"
cat > .env << EOF
# Vault Distributor Configuration
# Generated: $(date)

# Network Configuration
STELLAR_NETWORK=$NETWORK
STELLAR_RPC_URL=https://soroban-testnet.stellar.org

# Contract Configuration
CONTRACT_ID=$CONTRACT_ID
CONTRACT_ALIAS=vault_distributor

# Admin Wallet Configuration
ADMIN_KEY_NAME=$ADMIN_KEY_NAME
ADMIN_ADDRESS=$ADMIN_ADDRESS

# Vault Information (RefiUp)
VAULT_CONTRACT_ID=CA3N53CPBLSVM5342DZ25LK47WFQDS6R3BT62327SGZCNJ54CDDXO7KZ
VAULT_MANAGER=GAUSHAKPQJEHLKT4FBUVMWXOX3HUVV5OXNFUY54OBWS2R7ZUQY6QUBR6

# Explorer URLs
EXPLORER_CONTRACT=https://stellar.expert/explorer/testnet/contract/$CONTRACT_ID
EXPLORER_ADMIN=https://stellar.expert/explorer/testnet/account/$ADMIN_ADDRESS
EOF

echo -e "${GREEN}✓ Configuración guardada en .env${NC}"
echo ""

# Resumen
echo -e "${GREEN}═══════════════════════════════════════════════════${NC}"
echo -e "${GREEN}🎉 DEPLOYMENT COMPLETADO EXITOSAMENTE${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════${NC}"
echo ""
echo -e "${BLUE}📋 Información del Contrato:${NC}"
echo -e "  Contract ID:     ${YELLOW}$CONTRACT_ID${NC}"
echo -e "  Admin Address:   ${YELLOW}$ADMIN_ADDRESS${NC}"
echo -e "  Network:         ${YELLOW}$NETWORK${NC}"
echo -e "  Explorer:        ${YELLOW}https://stellar.expert/explorer/testnet/contract/$CONTRACT_ID${NC}"
echo ""
echo -e "${BLUE}🔗 Vault RefiUp:${NC}"
echo -e "  Vault ID:        ${YELLOW}CA3N53CPBLSVM5342DZ25LK47WFQDS6R3BT62327SGZCNJ54CDDXO7KZ${NC}"
echo -e "  Manager:         ${YELLOW}GAUSHAKPQJEHLKT4FBUVMWXOX3HUVV5OXNFUY54OBWS2R7ZUQY6QUBR6${NC}"
echo ""
echo -e "${BLUE}📝 Próximos pasos:${NC}"
echo "  1. Verificar el contrato en Stellar Explorer"
echo "  2. Fondear el contrato con tokens para distribución"
echo "  3. Usar la función 'distribute' para enviar fondos"
echo ""
echo -e "${YELLOW}Ejemplo de distribución:${NC}"
echo "  stellar contract invoke \\"
echo "    --id $CONTRACT_ID \\"
echo "    --source-account $ADMIN_KEY_NAME \\"
echo "    --network $NETWORK \\"
echo "    -- distribute \\"
echo "    --token <TOKEN_ADDRESS> \\"
echo "    --recipients '[\"GABC...\", \"GDEF...\"]' \\"
echo "    --total_amount 1000"
echo ""

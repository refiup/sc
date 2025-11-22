#!/bin/bash

# Script de deployment para Event Distributor
# Network: Stellar Testnet

set -e

echo "🎪 Iniciando deployment de Event Distributor..."
echo ""

# Colores para output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Configuración
NETWORK="testnet"
ADMIN_KEY_NAME="admin"
CONTRACT_DIR="contracts/event-distributor"
WASM_PATH="$CONTRACT_DIR/target/wasm32-unknown-unknown/release/event_distributor.wasm"

# Paso 1: Verificar directorio del contrato
echo -e "${BLUE}📂 Paso 1: Verificando directorio del contrato...${NC}"
if [ ! -d "$CONTRACT_DIR" ]; then
    echo -e "${RED}❌ Error: Directorio $CONTRACT_DIR no encontrado${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Directorio verificado${NC}"
echo ""

# Paso 2: Compilar WASM
echo -e "${BLUE}🔨 Paso 2: Compilando contrato a WASM...${NC}"
cd $CONTRACT_DIR
cargo build --target wasm32-unknown-unknown --release
cd ../..

if [ ! -f "$WASM_PATH" ]; then
    echo -e "${RED}❌ Error: WASM no generado en $WASM_PATH${NC}"
    exit 1
fi

WASM_SIZE=$(du -h "$WASM_PATH" | cut -f1)
echo -e "${GREEN}✓ WASM compilado exitosamente (Tamaño: $WASM_SIZE)${NC}"
echo ""

# Paso 3: Verificar/Crear wallet admin
echo -e "${BLUE}🔑 Paso 3: Configurando wallet admin...${NC}"
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

# Paso 4: Verificar balance
echo -e "${BLUE}💰 Paso 4: Verificando balance...${NC}"
BALANCE=$(stellar keys show $ADMIN_KEY_NAME --network $NETWORK 2>/dev/null | grep -o '[0-9]*\.[0-9]* XLM' || echo "Balance no disponible")
echo -e "${GREEN}Balance: $BALANCE${NC}"
echo ""

# Paso 5: Desplegar contrato
echo -e "${BLUE}🚀 Paso 5: Desplegando contrato a testnet...${NC}"
echo "Esto puede tomar unos segundos..."

DEPLOY_OUTPUT=$(stellar contract deploy \
  --wasm $WASM_PATH \
  --source-account $ADMIN_KEY_NAME \
  --network $NETWORK \
  2>&1)

CONTRACT_ID=$(echo "$DEPLOY_OUTPUT" | grep -o 'C[A-Z0-9]\{55\}' | head -1)

if [ -z "$CONTRACT_ID" ]; then
    echo -e "${YELLOW}⚠️  No se pudo extraer CONTRACT_ID del output. Intentando con alias...${NC}"
    stellar contract deploy \
      --wasm $WASM_PATH \
      --source-account $ADMIN_KEY_NAME \
      --network $NETWORK \
      --alias event_distributor
    
    CONTRACT_ID=$(stellar contract id event_distributor --network $NETWORK)
fi

if [ -z "$CONTRACT_ID" ]; then
    echo -e "${RED}❌ Error: No se pudo obtener CONTRACT_ID${NC}"
    echo "Output del deployment:"
    echo "$DEPLOY_OUTPUT"
    exit 1
fi

echo -e "${GREEN}✓ Contrato desplegado exitosamente${NC}"
echo -e "${GREEN}Contract ID: $CONTRACT_ID${NC}"
echo ""

# Paso 6: Inicializar contrato
echo -e "${BLUE}⚙️  Paso 6: Inicializando contrato con admin...${NC}"
INIT_OUTPUT=$(stellar contract invoke \
  --id $CONTRACT_ID \
  --source-account $ADMIN_KEY_NAME \
  --network $NETWORK \
  -- init \
  --admin $ADMIN_ADDRESS \
  2>&1)

if echo "$INIT_OUTPUT" | grep -q "error"; then
    echo -e "${RED}❌ Error al inicializar contrato${NC}"
    echo "$INIT_OUTPUT"
    exit 1
fi

echo -e "${GREEN}✓ Contrato inicializado${NC}"
echo ""

# Paso 7: Verificar admin
echo -e "${BLUE}🔍 Paso 7: Verificando inicialización...${NC}"
STORED_ADMIN=$(stellar contract invoke \
  --id $CONTRACT_ID \
  --source-account $ADMIN_KEY_NAME \
  --network $NETWORK \
  -- get_admin \
  2>&1)

if echo "$STORED_ADMIN" | grep -q "$ADMIN_ADDRESS"; then
    echo -e "${GREEN}✓ Admin verificado correctamente${NC}"
else
    echo -e "${RED}⚠️  Advertencia: No se pudo verificar el admin${NC}"
fi
echo ""

# Paso 8: Generar archivo de configuración
echo -e "${BLUE}📝 Paso 8: Generando archivo de configuración...${NC}"
CONFIG_FILE=".env.event-distributor"
cat > $CONFIG_FILE <<EOF
# Event Distributor Configuration
# Generated: $(date)

NETWORK=$NETWORK
CONTRACT_ID=$CONTRACT_ID
ADMIN_ADDRESS=$ADMIN_ADDRESS
ADMIN_KEY_NAME=$ADMIN_KEY_NAME
WASM_PATH=$WASM_PATH

# Stellar Explorer
EXPLORER_URL=https://stellar.expert/explorer/$NETWORK/contract/$CONTRACT_ID

# For frontend integration
STELLAR_NETWORK=$NETWORK
EVENT_DISTRIBUTOR_CONTRACT_ID=$CONTRACT_ID
EOF

echo -e "${GREEN}✓ Configuración guardada en $CONFIG_FILE${NC}"
echo ""

# Resumen final
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${GREEN}✅ DEPLOYMENT COMPLETADO EXITOSAMENTE${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo -e "${BLUE}📋 Información del Contrato:${NC}"
echo ""
echo "  Network:     $NETWORK"
echo "  Contract ID: $CONTRACT_ID"
echo "  Admin:       $ADMIN_ADDRESS"
echo "  WASM Size:   $WASM_SIZE"
echo ""
echo -e "${BLUE}🔗 Enlaces Útiles:${NC}"
echo ""
echo "  Explorer: https://stellar.expert/explorer/$NETWORK/contract/$CONTRACT_ID"
echo "  Config:   ./$CONFIG_FILE"
echo ""
echo -e "${BLUE}📚 Próximos Pasos:${NC}"
echo ""
echo "  1. Agregar participantes:"
echo "     stellar contract invoke --id $CONTRACT_ID --source-account $ADMIN_KEY_NAME --network $NETWORK -- add_human --human_address <ADDRESS> --ipfs_hash <HASH>"
echo ""
echo "  2. Validar participantes:"
echo "     stellar contract invoke --id $CONTRACT_ID --source-account $ADMIN_KEY_NAME --network $NETWORK -- update_human_validation --human_address <ADDRESS> --validated true"
echo ""
echo "  3. Crear evento:"
echo "     stellar contract invoke --id $CONTRACT_ID --source-account $ADMIN_KEY_NAME --network $NETWORK -- create_event --event_id <ID> --location <LOCATION> --pool <AMOUNT>"
echo ""
echo "  4. Ver documentación completa:"
echo "     cat contracts/event-distributor/README.md"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# 🎨 Guía de Implementación - Vista del Juez (Judge View)

## 📋 Índice
1. [Resumen Ejecutivo](#resumen-ejecutivo)
2. [Arquitectura del Componente](#arquitectura-del-componente)
3. [Servicios Backend](#servicios-backend)
4. [Componentes React](#componentes-react)
5. [Gestión de Imágenes (Base64 → IPFS)](#gestión-de-imágenes)
6. [Integración con Event Distributor](#integración-con-event-distributor)
7. [Estilos y UX](#estilos-y-ux)
8. [Testing y Debugging](#testing-y-debugging)

---

## 🎯 Resumen Ejecutivo

### ¿Qué es la Vista del Juez?
Panel de administración para validar participantes de eventos ReFi mediante:
- **Galería de imágenes** - Visualiza fotos de participantes (Base64 convertidas desde IPFS)
- **Validación on-chain** - Toggle de validación que actualiza el contrato
- **Gestión de eventos** - Crear eventos y asociar participantes
- **Copiar direcciones** - Botón para copiar Public Address de cada participante

### Flujo de Trabajo
```
1. Usuario sube foto → 2. Foto → Base64 → 3. Hash IPFS on-chain →
4. Juez ve galería → 5. Juez valida participante → 6. Distribución automática
```

### Contratos Involucrados
- **Event Distributor**: `CCHFGFX3S52UX46HZEBEGW5N2LDDYMSJDLZF4CQOZ6TSWWKFEG4TFPLS`
- **Testnet Explorer**: [Ver contrato](https://stellar.expert/explorer/testnet/contract/CCHFGFX3S52UX46HZEBEGW5N2LDDYMSJDLZF4CQOZ6TSWWKFEG4TFPLS)

---

## 🏗️ Arquitectura del Componente

### Estructura de Carpetas
```
src/
├── components/
│   ├── JudgeView/
│   │   ├── JudgeView.jsx              // Componente principal
│   │   ├── ImageGallery.jsx           // Galería de imágenes
│   │   ├── ParticipantCard.jsx        // Card individual
│   │   ├── EventCreator.jsx           // Modal crear evento
│   │   └── JudgeView.module.css       // Estilos
├── services/
│   ├── ContractService.js             // Llamadas al contrato
│   ├── IPFSService.js                 // Conversión IPFS ↔ Base64
│   └── WalletService.js               // Autenticación Stellar
├── hooks/
│   ├── useContract.js                 // Hook custom para contratos
│   └── useIPFS.js                     // Hook para imágenes
└── config/
    └── contracts.js                    // Configuración de contratos
```

---

## 🔧 Servicios Backend

### 1. ContractService.js
```javascript
// src/services/ContractService.js
import { Contract, SorobanRpc, TransactionBuilder, Networks, BASE_FEE } from '@stellar/stellar-sdk';

const RPC_URL = 'https://soroban-testnet.stellar.org';
const CONTRACT_ID = 'CCHFGFX3S52UX46HZEBEGW5N2LDDYMSJDLZF4CQOZ6TSWWKFEG4TFPLS';
const NETWORK_PASSPHRASE = Networks.TESTNET;

export class ContractService {
  constructor() {
    this.server = new SorobanRpc.Server(RPC_URL);
    this.contract = new Contract(CONTRACT_ID);
  }

  /**
   * Obtener todos los participantes con paginación
   * @param {number} start - Índice inicial
   * @param {number} limit - Cantidad de resultados
   * @returns {Promise<Array>} Lista de participantes
   */
  async getAllHumans(start = 0, limit = 50) {
    try {
      const result = await this.contract.call('get_all_humans', start, limit);
      
      return result.map(human => ({
        address: human.address,
        ipfsHash: human.ipfs_hash,
        validated: human.validated
      }));
    } catch (error) {
      console.error('Error fetching humans:', error);
      throw error;
    }
  }

  /**
   * Obtener un participante específico
   * @param {string} address - Dirección pública del participante
   * @returns {Promise<Object>} Datos del participante
   */
  async getHuman(address) {
    try {
      const result = await this.contract.call('get_human', address);
      
      return {
        address: result.address,
        ipfsHash: result.ipfs_hash,
        validated: result.validated
      };
    } catch (error) {
      console.error('Error fetching human:', error);
      return null;
    }
  }

  /**
   * Validar o invalidar un participante
   * @param {string} address - Dirección del participante
   * @param {boolean} validated - Estado de validación
   * @param {Keypair} adminKeypair - Keypair del admin
   * @returns {Promise<string>} Hash de la transacción
   */
  async updateHumanValidation(address, validated, adminKeypair) {
    try {
      const account = await this.server.getAccount(adminKeypair.publicKey());
      
      const transaction = new TransactionBuilder(account, {
        fee: BASE_FEE,
        networkPassphrase: NETWORK_PASSPHRASE,
      })
        .addOperation(
          this.contract.call('update_human_validation', address, validated)
        )
        .setTimeout(30)
        .build();
      
      transaction.sign(adminKeypair);
      
      const response = await this.server.sendTransaction(transaction);
      
      // Esperar confirmación
      let status = await this.server.getTransaction(response.hash);
      while (status.status === 'NOT_FOUND') {
        await new Promise(resolve => setTimeout(resolve, 1000));
        status = await this.server.getTransaction(response.hash);
      }
      
      if (status.status === 'SUCCESS') {
        return response.hash;
      } else {
        throw new Error('Transaction failed');
      }
    } catch (error) {
      console.error('Error updating validation:', error);
      throw error;
    }
  }

  /**
   * Crear un nuevo evento
   * @param {number} eventId - ID único del evento
   * @param {string} location - Ubicación del evento
   * @param {number} pool - Cantidad de tokens para distribuir
   * @param {Keypair} adminKeypair - Keypair del admin
   * @returns {Promise<string>} Hash de la transacción
   */
  async createEvent(eventId, location, pool, adminKeypair) {
    try {
      const account = await this.server.getAccount(adminKeypair.publicKey());
      
      const transaction = new TransactionBuilder(account, {
        fee: BASE_FEE,
        networkPassphrase: NETWORK_PASSPHRASE,
      })
        .addOperation(
          this.contract.call('create_event', eventId, location, pool)
        )
        .setTimeout(30)
        .build();
      
      transaction.sign(adminKeypair);
      
      const response = await this.server.sendTransaction(transaction);
      
      let status = await this.server.getTransaction(response.hash);
      while (status.status === 'NOT_FOUND') {
        await new Promise(resolve => setTimeout(resolve, 1000));
        status = await this.server.getTransaction(response.hash);
      }
      
      if (status.status === 'SUCCESS') {
        return response.hash;
      } else {
        throw new Error('Transaction failed');
      }
    } catch (error) {
      console.error('Error creating event:', error);
      throw error;
    }
  }

  /**
   * Agregar participante a un evento
   * @param {number} eventId - ID del evento
   * @param {string} address - Dirección del participante
   * @param {Keypair} adminKeypair - Keypair del admin
   * @returns {Promise<string>} Hash de la transacción
   */
  async addHumanToEvent(eventId, address, adminKeypair) {
    try {
      const account = await this.server.getAccount(adminKeypair.publicKey());
      
      const transaction = new TransactionBuilder(account, {
        fee: BASE_FEE,
        networkPassphrase: NETWORK_PASSPHRASE,
      })
        .addOperation(
          this.contract.call('add_human_to_event', eventId, address)
        )
        .setTimeout(30)
        .build();
      
      transaction.sign(adminKeypair);
      
      const response = await this.server.sendTransaction(transaction);
      
      let status = await this.server.getTransaction(response.hash);
      while (status.status === 'NOT_FOUND') {
        await new Promise(resolve => setTimeout(resolve, 1000));
        status = await this.server.getTransaction(response.hash);
      }
      
      return response.hash;
    } catch (error) {
      console.error('Error adding human to event:', error);
      throw error;
    }
  }

  /**
   * Obtener todos los eventos
   * @returns {Promise<Array>} Lista de eventos
   */
  async getAllEvents() {
    try {
      // Nota: Esta función debería implementarse en el contrato
      // Por ahora, retorna un array vacío
      const result = await this.contract.call('get_all_events');
      return result;
    } catch (error) {
      console.error('Error fetching events:', error);
      return [];
    }
  }
}

export default new ContractService();
```

### 2. IPFSService.js
```javascript
// src/services/IPFSService.js

/**
 * Servicio para manejar conversiones entre IPFS y Base64
 */
export class IPFSService {
  constructor() {
    // Puedes usar gateways públicos o tu propio nodo IPFS
    this.gateways = [
      'https://ipfs.io/ipfs/',
      'https://cloudflare-ipfs.com/ipfs/',
      'https://gateway.pinata.cloud/ipfs/',
      'https://dweb.link/ipfs/'
    ];
    this.currentGatewayIndex = 0;
  }

  /**
   * Obtener imagen de IPFS y convertir a Base64
   * @param {string} ipfsHash - Hash IPFS (ejemplo: "QmXxx...")
   * @returns {Promise<string>} Imagen en formato Base64 (data:image/jpeg;base64,...)
   */
  async fetchImageAsBase64(ipfsHash) {
    // Intentar con diferentes gateways en caso de falla
    for (let i = 0; i < this.gateways.length; i++) {
      try {
        const gatewayUrl = this.gateways[this.currentGatewayIndex] + ipfsHash;
        
        const response = await fetch(gatewayUrl, {
          method: 'GET',
          headers: {
            'Accept': 'image/*'
          }
        });

        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }

        const blob = await response.blob();
        const base64 = await this.blobToBase64(blob);
        
        return base64;
      } catch (error) {
        console.error(`Error with gateway ${this.gateways[this.currentGatewayIndex]}:`, error);
        
        // Rotar al siguiente gateway
        this.currentGatewayIndex = (this.currentGatewayIndex + 1) % this.gateways.length;
        
        // Si fue el último gateway, lanzar error
        if (i === this.gateways.length - 1) {
          throw new Error(`Failed to fetch image from IPFS: ${ipfsHash}`);
        }
      }
    }
  }

  /**
   * Convertir Blob a Base64
   * @param {Blob} blob - Blob de la imagen
   * @returns {Promise<string>} String Base64
   */
  blobToBase64(blob) {
    return new Promise((resolve, reject) => {
      const reader = new FileReader();
      reader.onloadend = () => resolve(reader.result);
      reader.onerror = reject;
      reader.readAsDataURL(blob);
    });
  }

  /**
   * Subir imagen a IPFS y obtener hash
   * @param {File} file - Archivo de imagen
   * @returns {Promise<string>} Hash IPFS
   */
  async uploadImage(file) {
    // Opción 1: Usar Pinata (requiere API key)
    // return await this.uploadToPinata(file);
    
    // Opción 2: Usar tu propio nodo IPFS
    return await this.uploadToLocalIPFS(file);
  }

  /**
   * Subir a Pinata (servicio IPFS gestionado)
   */
  async uploadToPinata(file) {
    const PINATA_API_KEY = process.env.REACT_APP_PINATA_API_KEY;
    const PINATA_SECRET_KEY = process.env.REACT_APP_PINATA_SECRET_KEY;

    const formData = new FormData();
    formData.append('file', file);

    const response = await fetch('https://api.pinata.cloud/pinning/pinFileToIPFS', {
      method: 'POST',
      headers: {
        'pinata_api_key': PINATA_API_KEY,
        'pinata_secret_api_key': PINATA_SECRET_KEY
      },
      body: formData
    });

    const data = await response.json();
    return data.IpfsHash;
  }

  /**
   * Subir a nodo IPFS local (ipfs daemon)
   */
  async uploadToLocalIPFS(file) {
    const formData = new FormData();
    formData.append('file', file);

    const response = await fetch('http://localhost:5001/api/v0/add', {
      method: 'POST',
      body: formData
    });

    const data = await response.json();
    return data.Hash;
  }

  /**
   * Validar si un hash IPFS es válido
   * @param {string} hash - Hash a validar
   * @returns {boolean}
   */
  isValidIPFSHash(hash) {
    // IPFS hashes comienzan con "Qm" (v0) o "bafy" (v1)
    return /^(Qm[1-9A-HJ-NP-Za-km-z]{44}|bafy[0-9a-z]{50,})$/.test(hash);
  }
}

export default new IPFSService();
```

### 3. WalletService.js
```javascript
// src/services/WalletService.js
import { Keypair } from '@stellar/stellar-sdk';

export class WalletService {
  constructor() {
    this.adminKeypair = null;
  }

  /**
   * Inicializar con secret key del admin
   * @param {string} secretKey - Secret key de Stellar
   */
  setAdminKey(secretKey) {
    try {
      this.adminKeypair = Keypair.fromSecret(secretKey);
      localStorage.setItem('admin_public', this.adminKeypair.publicKey());
      return true;
    } catch (error) {
      console.error('Invalid secret key:', error);
      return false;
    }
  }

  /**
   * Obtener Keypair del admin
   * @returns {Keypair|null}
   */
  getAdminKeypair() {
    return this.adminKeypair;
  }

  /**
   * Obtener public key del admin
   * @returns {string|null}
   */
  getAdminPublicKey() {
    return this.adminKeypair ? this.adminKeypair.publicKey() : null;
  }

  /**
   * Verificar si hay admin autenticado
   * @returns {boolean}
   */
  isAuthenticated() {
    return this.adminKeypair !== null;
  }

  /**
   * Cerrar sesión
   */
  logout() {
    this.adminKeypair = null;
    localStorage.removeItem('admin_public');
  }
}

export default new WalletService();
```

---

## ⚛️ Componentes React

### 1. JudgeView.jsx (Componente Principal)
```jsx
// src/components/JudgeView/JudgeView.jsx
import React, { useState, useEffect } from 'react';
import ContractService from '../../services/ContractService';
import WalletService from '../../services/WalletService';
import ImageGallery from './ImageGallery';
import EventCreator from './EventCreator';
import styles from './JudgeView.module.css';

export default function JudgeView() {
  const [participants, setParticipants] = useState([]);
  const [loading, setLoading] = useState(false);
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [secretKey, setSecretKey] = useState('');
  const [showEventModal, setShowEventModal] = useState(false);
  const [filter, setFilter] = useState('all'); // all, validated, unvalidated

  useEffect(() => {
    if (WalletService.isAuthenticated()) {
      setIsAuthenticated(true);
      loadParticipants();
    }
  }, []);

  /**
   * Autenticarse como admin
   */
  const handleLogin = () => {
    const success = WalletService.setAdminKey(secretKey);
    if (success) {
      setIsAuthenticated(true);
      loadParticipants();
      setSecretKey(''); // Limpiar campo
    } else {
      alert('Secret key inválido');
    }
  };

  /**
   * Cargar participantes del contrato
   */
  const loadParticipants = async () => {
    setLoading(true);
    try {
      const humans = await ContractService.getAllHumans(0, 100);
      setParticipants(humans);
    } catch (error) {
      console.error('Error loading participants:', error);
      alert('Error al cargar participantes');
    } finally {
      setLoading(false);
    }
  };

  /**
   * Toggle validación de un participante
   */
  const handleToggleValidation = async (address, currentStatus) => {
    try {
      const adminKeypair = WalletService.getAdminKeypair();
      
      await ContractService.updateHumanValidation(
        address,
        !currentStatus,
        adminKeypair
      );

      // Actualizar estado local
      setParticipants(prev =>
        prev.map(p =>
          p.address === address
            ? { ...p, validated: !currentStatus }
            : p
        )
      );

      alert('✅ Validación actualizada on-chain');
    } catch (error) {
      console.error('Error toggling validation:', error);
      alert('❌ Error al actualizar validación');
    }
  };

  /**
   * Filtrar participantes
   */
  const filteredParticipants = participants.filter(p => {
    if (filter === 'validated') return p.validated;
    if (filter === 'unvalidated') return !p.validated;
    return true; // 'all'
  });

  // Login Screen
  if (!isAuthenticated) {
    return (
      <div className={styles.loginContainer}>
        <div className={styles.loginCard}>
          <h1>🔐 Judge Panel - Login</h1>
          <p>Ingresa tu Secret Key de admin para continuar</p>
          <input
            type="password"
            placeholder="SXXX..."
            value={secretKey}
            onChange={(e) => setSecretKey(e.target.value)}
            className={styles.secretInput}
          />
          <button onClick={handleLogin} className={styles.loginButton}>
            Autenticar
          </button>
        </div>
      </div>
    );
  }

  // Main Judge View
  return (
    <div className={styles.judgeContainer}>
      {/* Header */}
      <header className={styles.header}>
        <div>
          <h1>👨‍⚖️ Judge Panel</h1>
          <p>Admin: {WalletService.getAdminPublicKey()?.substring(0, 10)}...</p>
        </div>
        <div className={styles.headerActions}>
          <button
            onClick={() => setShowEventModal(true)}
            className={styles.createEventButton}
          >
            ➕ Crear Evento
          </button>
          <button
            onClick={loadParticipants}
            className={styles.refreshButton}
          >
            🔄 Actualizar
          </button>
          <button
            onClick={() => {
              WalletService.logout();
              setIsAuthenticated(false);
            }}
            className={styles.logoutButton}
          >
            Cerrar Sesión
          </button>
        </div>
      </header>

      {/* Filters */}
      <div className={styles.filters}>
        <button
          onClick={() => setFilter('all')}
          className={filter === 'all' ? styles.activeFilter : ''}
        >
          Todos ({participants.length})
        </button>
        <button
          onClick={() => setFilter('validated')}
          className={filter === 'validated' ? styles.activeFilter : ''}
        >
          ✅ Validados ({participants.filter(p => p.validated).length})
        </button>
        <button
          onClick={() => setFilter('unvalidated')}
          className={filter === 'unvalidated' ? styles.activeFilter : ''}
        >
          ❌ Sin Validar ({participants.filter(p => !p.validated).length})
        </button>
      </div>

      {/* Gallery */}
      {loading ? (
        <div className={styles.loading}>
          <div className={styles.spinner}></div>
          <p>Cargando participantes...</p>
        </div>
      ) : (
        <ImageGallery
          participants={filteredParticipants}
          onToggleValidation={handleToggleValidation}
        />
      )}

      {/* Event Creator Modal */}
      {showEventModal && (
        <EventCreator
          onClose={() => setShowEventModal(false)}
          onEventCreated={loadParticipants}
        />
      )}
    </div>
  );
}
```

### 2. ImageGallery.jsx
```jsx
// src/components/JudgeView/ImageGallery.jsx
import React from 'react';
import ParticipantCard from './ParticipantCard';
import styles from './ImageGallery.module.css';

export default function ImageGallery({ participants, onToggleValidation }) {
  if (participants.length === 0) {
    return (
      <div className={styles.emptyState}>
        <p>📭 No hay participantes para mostrar</p>
      </div>
    );
  }

  return (
    <div className={styles.gallery}>
      {participants.map((participant) => (
        <ParticipantCard
          key={participant.address}
          participant={participant}
          onToggleValidation={onToggleValidation}
        />
      ))}
    </div>
  );
}
```

### 3. ParticipantCard.jsx
```jsx
// src/components/JudgeView/ParticipantCard.jsx
import React, { useState, useEffect } from 'react';
import IPFSService from '../../services/IPFSService';
import styles from './ParticipantCard.module.css';

export default function ParticipantCard({ participant, onToggleValidation }) {
  const [imageBase64, setImageBase64] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(false);
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    loadImage();
  }, [participant.ipfsHash]);

  /**
   * Cargar imagen desde IPFS
   */
  const loadImage = async () => {
    setLoading(true);
    setError(false);

    try {
      const base64 = await IPFSService.fetchImageAsBase64(participant.ipfsHash);
      setImageBase64(base64);
    } catch (err) {
      console.error('Error loading image:', err);
      setError(true);
    } finally {
      setLoading(false);
    }
  };

  /**
   * Copiar dirección al portapapeles
   */
  const handleCopyAddress = () => {
    navigator.clipboard.writeText(participant.address);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className={`${styles.card} ${participant.validated ? styles.validated : styles.unvalidated}`}>
      {/* Imagen */}
      <div className={styles.imageContainer}>
        {loading && (
          <div className={styles.imagePlaceholder}>
            <div className={styles.spinner}></div>
          </div>
        )}
        
        {error && (
          <div className={styles.imageError}>
            <p>❌</p>
            <p>Error al cargar</p>
          </div>
        )}
        
        {imageBase64 && !loading && !error && (
          <img
            src={imageBase64}
            alt={`Participant ${participant.address}`}
            className={styles.image}
          />
        )}

        {/* Badge de validación */}
        <div className={styles.validationBadge}>
          {participant.validated ? '✅ Validado' : '⏳ Sin validar'}
        </div>
      </div>

      {/* Información */}
      <div className={styles.info}>
        <p className={styles.address}>
          {participant.address.substring(0, 6)}...{participant.address.substring(participant.address.length - 4)}
        </p>
        <p className={styles.ipfsHash}>
          IPFS: {participant.ipfsHash.substring(0, 10)}...
        </p>
      </div>

      {/* Acciones */}
      <div className={styles.actions}>
        {/* Botón copiar dirección */}
        <button
          onClick={handleCopyAddress}
          className={styles.copyButton}
          title="Copiar dirección completa"
        >
          {copied ? '✓ Copiado' : '📋 Copiar Address'}
        </button>

        {/* Toggle validación */}
        <button
          onClick={() => onToggleValidation(participant.address, participant.validated)}
          className={participant.validated ? styles.invalidateButton : styles.validateButton}
        >
          {participant.validated ? '❌ Invalidar' : '✅ Validar'}
        </button>
      </div>
    </div>
  );
}
```

### 4. EventCreator.jsx
```jsx
// src/components/JudgeView/EventCreator.jsx
import React, { useState } from 'react';
import ContractService from '../../services/ContractService';
import WalletService from '../../services/WalletService';
import styles from './EventCreator.module.css';

export default function EventCreator({ onClose, onEventCreated }) {
  const [eventData, setEventData] = useState({
    eventId: '',
    location: '',
    pool: ''
  });
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (e) => {
    e.preventDefault();
    setLoading(true);

    try {
      const adminKeypair = WalletService.getAdminKeypair();
      
      const txHash = await ContractService.createEvent(
        parseInt(eventData.eventId),
        eventData.location,
        parseInt(eventData.pool),
        adminKeypair
      );

      alert(`✅ Evento creado!\nTX: ${txHash}`);
      onEventCreated();
      onClose();
    } catch (error) {
      console.error('Error creating event:', error);
      alert('❌ Error al crear evento');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className={styles.modalOverlay} onClick={onClose}>
      <div className={styles.modal} onClick={(e) => e.stopPropagation()}>
        <h2>➕ Crear Nuevo Evento</h2>
        
        <form onSubmit={handleSubmit}>
          <div className={styles.formGroup}>
            <label>Event ID (número único)</label>
            <input
              type="number"
              value={eventData.eventId}
              onChange={(e) => setEventData({...eventData, eventId: e.target.value})}
              required
              placeholder="1"
            />
          </div>

          <div className={styles.formGroup}>
            <label>Ubicación</label>
            <input
              type="text"
              value={eventData.location}
              onChange={(e) => setEventData({...eventData, location: e.target.value})}
              required
              placeholder="Buenos Aires, Argentina"
            />
          </div>

          <div className={styles.formGroup}>
            <label>Pool (stroops)</label>
            <input
              type="number"
              value={eventData.pool}
              onChange={(e) => setEventData({...eventData, pool: e.target.value})}
              required
              placeholder="10000000"
            />
            <small>1 XLM = 10,000,000 stroops</small>
          </div>

          <div className={styles.buttons}>
            <button type="button" onClick={onClose} disabled={loading}>
              Cancelar
            </button>
            <button type="submit" disabled={loading}>
              {loading ? 'Creando...' : 'Crear Evento'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
```

---

## 🎨 Estilos y UX

### JudgeView.module.css
```css
/* src/components/JudgeView/JudgeView.module.css */

.loginContainer {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.loginCard {
  background: white;
  padding: 3rem;
  border-radius: 20px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  max-width: 400px;
  width: 100%;
}

.loginCard h1 {
  margin-bottom: 0.5rem;
  color: #333;
}

.loginCard p {
  margin-bottom: 1.5rem;
  color: #666;
}

.secretInput {
  width: 100%;
  padding: 12px;
  border: 2px solid #e0e0e0;
  border-radius: 8px;
  font-size: 14px;
  font-family: monospace;
  margin-bottom: 1rem;
}

.secretInput:focus {
  outline: none;
  border-color: #667eea;
}

.loginButton {
  width: 100%;
  padding: 12px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 16px;
  font-weight: 600;
  cursor: pointer;
  transition: transform 0.2s;
}

.loginButton:hover {
  transform: translateY(-2px);
}

.judgeContainer {
  max-width: 1400px;
  margin: 0 auto;
  padding: 2rem;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
  padding: 1.5rem;
  background: white;
  border-radius: 12px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
}

.header h1 {
  margin: 0;
  color: #333;
}

.header p {
  margin: 0.5rem 0 0 0;
  color: #666;
  font-family: monospace;
}

.headerActions {
  display: flex;
  gap: 1rem;
}

.headerActions button {
  padding: 10px 20px;
  border: none;
  border-radius: 8px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.createEventButton {
  background: #10b981;
  color: white;
}

.refreshButton {
  background: #3b82f6;
  color: white;
}

.logoutButton {
  background: #ef4444;
  color: white;
}

.headerActions button:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.filters {
  display: flex;
  gap: 1rem;
  margin-bottom: 2rem;
  padding: 1rem;
  background: white;
  border-radius: 12px;
}

.filters button {
  padding: 10px 20px;
  border: 2px solid #e0e0e0;
  background: white;
  border-radius: 8px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.filters button:hover {
  border-color: #667eea;
  color: #667eea;
}

.activeFilter {
  background: #667eea !important;
  color: white !important;
  border-color: #667eea !important;
}

.loading {
  text-align: center;
  padding: 4rem;
}

.spinner {
  width: 50px;
  height: 50px;
  border: 4px solid #f3f3f3;
  border-top: 4px solid #667eea;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto 1rem auto;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}
```

### ImageGallery.module.css
```css
/* src/components/JudgeView/ImageGallery.module.css */

.gallery {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 2rem;
  padding: 1rem 0;
}

.emptyState {
  text-align: center;
  padding: 4rem;
  background: white;
  border-radius: 12px;
  font-size: 1.2rem;
  color: #999;
}

@media (max-width: 768px) {
  .gallery {
    grid-template-columns: 1fr;
  }
}
```

### ParticipantCard.module.css
```css
/* src/components/JudgeView/ParticipantCard.module.css */

.card {
  background: white;
  border-radius: 16px;
  overflow: hidden;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  transition: all 0.3s;
}

.card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);
}

.validated {
  border: 3px solid #10b981;
}

.unvalidated {
  border: 3px solid #f59e0b;
}

.imageContainer {
  position: relative;
  width: 100%;
  height: 280px;
  background: #f3f4f6;
}

.image {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.imagePlaceholder,
.imageError {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  height: 100%;
  color: #9ca3af;
}

.imageError p {
  margin: 0.5rem 0;
}

.validationBadge {
  position: absolute;
  top: 12px;
  right: 12px;
  padding: 6px 12px;
  background: rgba(0, 0, 0, 0.7);
  color: white;
  border-radius: 20px;
  font-size: 12px;
  font-weight: 600;
}

.info {
  padding: 1rem;
  border-bottom: 1px solid #e5e7eb;
}

.address {
  font-family: monospace;
  font-size: 14px;
  font-weight: 600;
  color: #333;
  margin: 0 0 0.5rem 0;
}

.ipfsHash {
  font-family: monospace;
  font-size: 12px;
  color: #6b7280;
  margin: 0;
}

.actions {
  padding: 1rem;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.actions button {
  padding: 10px;
  border: none;
  border-radius: 8px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.copyButton {
  background: #e5e7eb;
  color: #374151;
}

.copyButton:hover {
  background: #d1d5db;
}

.validateButton {
  background: #10b981;
  color: white;
}

.validateButton:hover {
  background: #059669;
}

.invalidateButton {
  background: #ef4444;
  color: white;
}

.invalidateButton:hover {
  background: #dc2626;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 3px solid #f3f3f3;
  border-top: 3px solid #667eea;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}
```

### EventCreator.module.css
```css
/* src/components/JudgeView/EventCreator.module.css */

.modalOverlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.modal {
  background: white;
  padding: 2rem;
  border-radius: 16px;
  max-width: 500px;
  width: 90%;
  max-height: 90vh;
  overflow-y: auto;
}

.modal h2 {
  margin: 0 0 1.5rem 0;
  color: #333;
}

.formGroup {
  margin-bottom: 1.5rem;
}

.formGroup label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 600;
  color: #374151;
}

.formGroup input {
  width: 100%;
  padding: 10px;
  border: 2px solid #e5e7eb;
  border-radius: 8px;
  font-size: 14px;
}

.formGroup input:focus {
  outline: none;
  border-color: #667eea;
}

.formGroup small {
  display: block;
  margin-top: 0.5rem;
  color: #6b7280;
  font-size: 12px;
}

.buttons {
  display: flex;
  gap: 1rem;
  margin-top: 2rem;
}

.buttons button {
  flex: 1;
  padding: 12px;
  border: none;
  border-radius: 8px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.buttons button[type="button"] {
  background: #e5e7eb;
  color: #374151;
}

.buttons button[type="submit"] {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
}

.buttons button:hover {
  transform: translateY(-2px);
}

.buttons button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  transform: none;
}
```

---

## 🧪 Testing y Debugging

### Comandos CLI para Testing
```bash
# 1. Verificar participantes en el contrato
stellar contract invoke \
  --id CCHFGFX3S52UX46HZEBEGW5N2LDDYMSJDLZF4CQOZ6TSWWKFEG4TFPLS \
  --source admin-keypair \
  --network testnet \
  --network-passphrase "Test SDF Network ; September 2015" \
  -- get_all_humans \
  --start 0 \
  --limit 10

# 2. Agregar participante de prueba
stellar contract invoke \
  --id CCHFGFX3S52UX46HZEBEGW5N2LDDYMSJDLZF4CQOZ6TSWWKFEG4TFPLS \
  --source admin-keypair \
  --network testnet \
  --network-passphrase "Test SDF Network ; September 2015" \
  -- add_human \
  --address GDL5432N2JCCAZBHG7EKHHVBRG2XQUI2WJGSRBK4R5OF3QNCOMDKZBEW \
  --ipfs_hash '"QmYwAPJzv5CZsnA625s3Xf2nemtYgPpHdWEz79ojWnPbdG"'

# 3. Validar participante
stellar contract invoke \
  --id CCHFGFX3S52UX46HZEBEGW5N2LDDYMSJDLZF4CQOZ6TSWWKFEG4TFPLS \
  --source admin-keypair \
  --network testnet \
  --network-passphrase "Test SDF Network ; September 2015" \
  -- update_human_validation \
  --address GDL5432N2JCCAZBHG7EKHHVBRG2XQUI2WJGSRBK4R5OF3QNCOMDKZBEW \
  --validated true

# 4. Crear evento de prueba
stellar contract invoke \
  --id CCHFGFX3S52UX46HZEBEGW5N2LDDYMSJDLZF4CQOZ6TSWWKFEG4TFPLS \
  --source admin-keypair \
  --network testnet \
  --network-passphrase "Test SDF Network ; September 2015" \
  -- create_event \
  --event_id 1 \
  --location '"Buenos Aires, Argentina"' \
  --pool 10000000

# 5. Obtener evento
stellar contract invoke \
  --id CCHFGFX3S52UX46HZEBEGW5N2LDDYMSJDLZF4CQOZ6TSWWKFEG4TFPLS \
  --source admin-keypair \
  --network testnet \
  --network-passphrase "Test SDF Network ; September 2015" \
  -- get_event \
  --event_id 1
```

### Checklist de Testing

```markdown
## ✅ Checklist Frontend

### Login
- [ ] Login con secret key válido funciona
- [ ] Login con secret key inválido muestra error
- [ ] Public key se muestra correctamente en header
- [ ] Logout limpia la sesión

### Carga de Participantes
- [ ] Lista de participantes carga correctamente
- [ ] Imágenes se convierten de IPFS a Base64
- [ ] Fallback cuando imagen no carga (placeholder)
- [ ] Spinner muestra durante carga

### Filtros
- [ ] Filtro "Todos" muestra todos los participantes
- [ ] Filtro "Validados" muestra solo validados
- [ ] Filtro "Sin Validar" muestra solo no validados
- [ ] Contadores en botones son correctos

### Validación
- [ ] Toggle validación actualiza on-chain
- [ ] Estado local se actualiza después de transacción
- [ ] Badge de validación cambia correctamente
- [ ] Borde de card refleja estado (verde/amarillo)

### Copiar Dirección
- [ ] Botón copia dirección completa al portapapeles
- [ ] Feedback visual "✓ Copiado" aparece
- [ ] Feedback desaparece después de 2 segundos

### Crear Evento
- [ ] Modal se abre/cierra correctamente
- [ ] Validación de campos funciona
- [ ] Evento se crea on-chain
- [ ] TX hash se muestra en alerta
- [ ] Lista se actualiza después de crear evento

### Responsividad
- [ ] Vista funciona en desktop (1920px)
- [ ] Vista funciona en tablet (768px)
- [ ] Vista funciona en mobile (375px)
- [ ] Gallery se adapta a diferentes tamaños
```

---

## 📚 Recursos Adicionales

### Links Importantes
- **Contrato en Testnet**: https://stellar.expert/explorer/testnet/contract/CCHFGFX3S52UX46HZEBEGW5N2LDDYMSJDLZF4CQOZ6TSWWKFEG4TFPLS
- **Stellar SDK**: https://github.com/stellar/js-stellar-sdk
- **Soroban Docs**: https://soroban.stellar.org/docs
- **IPFS Docs**: https://docs.ipfs.tech/

### Próximos Pasos
1. **Implementar backend propio para IPFS** (actualmente usa gateways públicos)
2. **Agregar búsqueda/filtrado** por dirección o IPFS hash
3. **Implementar paginación** para listas grandes (>100 participantes)
4. **Agregar estadísticas** (total validados, eventos activos, etc.)
5. **Integrar con sistema de notificaciones** (WebSocket para actualizaciones en tiempo real)

---

## 🚀 Deployment

### Variables de Entorno
```bash
# .env
REACT_APP_CONTRACT_ID=CCHFGFX3S52UX46HZEBEGW5N2LDDYMSJDLZF4CQOZ6TSWWKFEG4TFPLS
REACT_APP_NETWORK=testnet
REACT_APP_RPC_URL=https://soroban-testnet.stellar.org
REACT_APP_PINATA_API_KEY=tu_api_key
REACT_APP_PINATA_SECRET_KEY=tu_secret_key
```

### Build para Producción
```bash
npm run build
# o
yarn build

# Deploy a Vercel/Netlify
vercel --prod
# o
netlify deploy --prod
```

---

**Versión**: 1.0.0  
**Última actualización**: 22 de noviembre de 2025  
**Autor**: ReFi Universe Team
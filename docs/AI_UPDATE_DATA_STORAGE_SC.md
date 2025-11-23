PROBLEMA

En este momento se pasa una lista de addresses directamente como parámetro a una función del smart contract. Esto es incorrecto.
La lista debe obtenerse desde el almacenamiento interno del contrato, donde los addresses están guardados junto con una propiedad validated (booleano). Solo las direcciones validadas deberían ser consideradas por la función distribute.

OBJETIVO

Actualizar el contrato para que use su propio storage como “fuente de la verdad”, de forma que los datos se procesen on-chain.

REQUERIMIENTOS

1. Modelo de datos
Human
Human {
    address: Address,
    validated: Bool,
    img: String    // Imagen en base64 almacenada en el contrato
}

Events
Events {
    location: String,
    event_name: String,
    humans: Human[],
    pool: i128
}

2. Funcionalidades nuevas necesarias

A. Gestión de Humans

Agregar un método para crear/agregar nuevas addresses al storage como Human.

Agregar método para actualizar el estado validated (true/false).

Guardar y actualizar la imagen en Base64 dentro del contrato.


B. Gestión de Events

Crear métodos para:

Crear un nuevo evento.

Asociar uno o varios humans a un evento.


C. Distribución

Actualizar la función distribute para que:

Obtenga la lista de humans desde el storage.

Filtre únicamente los que tengan validated == true.

Reparta los fondos del pool de manera equitativa entre todas esas addresses.


3. Requisitos adicionales

Generar eventos (emit logs) cada vez que se agregue/edite un human o un event.

Garantizar que todos los cambios sean procesados completamente on-chain.
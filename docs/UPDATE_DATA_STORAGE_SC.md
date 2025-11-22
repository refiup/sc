Necesito que actulicemos el actual contrato (deplegar de nuevo)

PROBLEMA: el prblema actual. es que se pasa un lista de addresses directo al SC, esto. no es correcto porque deberíamos obtener desde el almacenaminto del contrato una lista de addresses que posean una propiedad "validated" con un booleano que indique su estado, permitiendo unicamente las addresses que hayan pasado la prueba. para ello tambien necesitamos agregar metodos para agregar nuevas direcciones a la lista y para poder modificar el estado de estas. finalmente actualizaremos la funcion distribute para obtener la lista desde el storage y repartir el dinero de manera equitativa





Necesitamos actualizar el contrato para utilizar el storage del contrato como "fuente de la verdad" para que los datos sean procesados on chain, para ello necesitamos una serie de cambios que nos permitan crear eventos, agregar personas, actualizar el estado de validated. luego poder asociar estos humanos a un evento que hayamos creado, y cuando la función distribute se ejecute se realice una transferencia de fondos a todas las addresses que tengan el "validated true"
Human {
    address: Address,
    validated: Bool,
    img: String
}
Events {
    location: String,
    event_name: String,
    humans: Human[],
    pool: i128
}

agregar metodos para agregar nuevas addresses a la lista
guardar las imagenes en base64 en el contrato
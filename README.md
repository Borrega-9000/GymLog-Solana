# GymLog Solana - Bitácora de Entrenamiento en Solana

**GymLog Solana** es un programa inteligente desarrollado sobre la blockchain de **Solana** usando el framework **Anchor**, cuyo objetivo es permitir que cada usuario registre, administre y consulte su rutina de entrenamiento físico de forma descentralizada, persistente e inmutable.

La propuesta del sistema consiste en reemplazar una bitácora tradicional de gimnasio por una estructura de datos alojada en blockchain, donde cada atleta o usuario puede almacenar información relevante de sus ejercicios realizados, como el nombre del ejercicio, el número de series, las repeticiones y el peso empleado. De esta manera, el progreso físico deja de depender de registros locales o aplicaciones centralizadas, y pasa a conservarse en una cuenta propia dentro de Solana, protegida criptográficamente y asociada directamente a la clave pública del propietario.

A diferencia de otros sistemas que suelen inicializar información con valores predeterminados, **GymLog Solana exige que el usuario proporcione todos los datos necesarios de manera explícita**, lo que garantiza mayor integridad en el registro y evita entradas ambiguas o incompletas. Cada operación del programa está pensada bajo un enfoque tipo **CRUD**: crear, registrar, editar, eliminar y consultar ejercicios dentro de una rutina personal.

---

## Objetivo del Programa

El propósito principal de **GymLog Solana** es ofrecer una bitácora de entrenamiento segura, transparente y controlada por el propio usuario, aprovechando las capacidades de Solana para el almacenamiento descentralizado y la ejecución de lógica programable mediante contratos inteligentes.

Este programa permite:

- Crear un registro personal de entrenamiento asociado a una cuenta única.
- Guardar ejercicios con sus métricas obligatorias.
- Actualizar el progreso de un ejercicio conforme cambian las cargas o repeticiones.
- Eliminar ejercicios que ya no formen parte de la rutina.
- Consultar el contenido actual de la rutina registrada.

---

## Tecnologías Utilizadas

El desarrollo del programa se basa en las siguientes tecnologías:

- **Solana Blockchain**: red sobre la que se ejecuta el contrato inteligente.
- **Rust**: lenguaje de programación usado para desarrollar la lógica del programa.
- **Anchor Framework**: framework que simplifica la construcción de programas en Solana, incluyendo validación de cuentas, serialización y administración de espacio.
- **PDAs (Program Derived Addresses)**: mecanismo utilizado para generar cuentas determinísticas controladas por el programa.

---

## Arquitectura General del Sistema

El programa se apoya en una cuenta principal llamada `GymLog`, la cual representa la bitácora personal de un usuario. Esta cuenta almacena:

- La clave pública del propietario.
- El nombre del usuario o alias.
- Un vector que contiene la rutina completa, formada por varios ejercicios.

Cada ejercicio se modela como una estructura independiente llamada `Ejercicio`, que guarda las métricas esenciales del entrenamiento.

La cuenta principal es creada mediante una **PDA**, derivada a partir de:

- La semilla fija: `"gymlog"`
- La clave pública del usuario propietario

Esto permite que cada usuario pueda tener una cuenta de entrenamiento derivada de forma determinística, evitando colisiones y facilitando la localización de su información dentro del programa.

---

## Especificaciones Técnicas

### Uso de PDAs

El programa utiliza una **Program Derived Address (PDA)** para crear la cuenta `GymLog`. Esto significa que la dirección de la cuenta no es aleatoria, sino que se genera a partir de una combinación predecible de semillas y del `program_id`.

En este caso, la PDA se genera con:

- `b"gymlog"`
- `owner.key().as_ref()`

Esta estrategia asegura que cada usuario tenga una cuenta única relacionada con su propia llave pública, y que dicha cuenta sea administrada por el programa.

### Persistencia de Datos

La información del usuario queda almacenada directamente en una cuenta on-chain, lo que implica que:

- Los datos sobreviven entre transacciones.
- El contenido puede ser consultado posteriormente.
- El estado depende de la lógica del contrato y no de una base de datos externa.
- El usuario conserva una relación directa con su registro por medio de su firma.

### Uso de `InitSpace`

El programa aprovecha `InitSpace`, una característica de Anchor que ayuda a calcular el espacio necesario para las estructuras almacenadas en cuenta, especialmente cuando existen campos dinámicos como `String` y `Vec`.

Esto mejora la organización del espacio reservado y permite definir límites de almacenamiento desde el diseño del programa.

---

## Modelo de Datos

### Estructura `Ejercicio`

Cada elemento de la rutina está representado por la estructura `Ejercicio`, que contiene los siguientes atributos:

- **nombre**: nombre del ejercicio, con un máximo de 30 caracteres.
- **series**: número de series realizadas (`u8`).
- **repeticiones**: número de repeticiones por serie (`u8`).
- **peso_kg**: peso utilizado en kilogramos (`u16`).

Esta estructura define el bloque mínimo de información que representa una actividad de entrenamiento dentro del sistema.

### Estructura `GymLog`

La cuenta principal del usuario está modelada por la estructura `GymLog`, compuesta por:

- **owner**: clave pública del propietario (`Pubkey`).
- **nombre_usuario**: alias o nombre del atleta, con un máximo de 40 caracteres.
- **rutina**: vector de ejercicios con capacidad máxima definida para 12 elementos.

En términos funcionales, `GymLog` es la bitácora completa del usuario dentro de la blockchain.

---

## Requisitos de Datos

El programa fue diseñado **sin valores por defecto**, por lo que cada registro debe incluir toda la información necesaria al momento de ser creado o actualizado.

### Datos obligatorios para registrar un ejercicio

Para registrar correctamente un ejercicio, el usuario debe proporcionar:

- **Nombre del ejercicio**
- **Series**
- **Repeticiones**
- **Peso en kilogramos**

Esto significa que el sistema no rellena automáticamente campos vacíos ni asume valores implícitos. La integridad del registro depende de la entrada completa del usuario.

---

## Funcionamiento CRUD del Programa

## 1. Inicializar log

La instrucción `inicializar_log` crea la cuenta `GymLog` del usuario.

### Qué hace
- Crea una nueva cuenta en cadena usando una PDA.
- Guarda la clave pública del usuario como propietario.
- Almacena el nombre del usuario.
- Inicializa la rutina como un vector vacío.

### Parámetros requeridos
- `nombre_usuario: String`

### Resultado
Se crea una bitácora vacía lista para comenzar a registrar ejercicios.

### Mensaje emitido
- `"Log de entrenamiento creado para: {}"`

---

## 2. Registrar ejercicio

La instrucción `registrar_ejercicio` agrega un nuevo ejercicio al vector `rutina`.

### Qué hace
- Verifica que quien firma la transacción sea el propietario del log.
- Crea un nuevo objeto `Ejercicio`.
- Inserta ese objeto dentro del vector de rutina.

### Parámetros requeridos
- `nombre: String`
- `series: u8`
- `repeticiones: u8`
- `peso: u16`

### Resultado
El ejercicio queda almacenado dentro de la rutina del usuario.

### Validación importante
Antes de registrar el ejercicio, el contrato comprueba que el `owner` almacenado en la cuenta coincida con la clave pública del firmante actual.

### Mensaje emitido
- `"Ejercicio registrado exitosamente."`

---

## 3. Editar ejercicio

La instrucción `editar_ejercicio` permite actualizar los valores de un ejercicio ya existente dentro de la rutina.

### Qué hace
- Verifica la propiedad del log.
- Recorre la lista de ejercicios.
- Busca un ejercicio por coincidencia exacta de nombre.
- Sustituye sus métricas por los nuevos valores enviados.

### Parámetros requeridos
- `nombre: String`
- `nuevas_series: u8`
- `nuevas_reps: u8`
- `nuevo_peso: u16`

### Resultado
Si el ejercicio existe, sus datos quedan actualizados.

### Comportamiento interno
La búsqueda se realiza mediante un recorrido secuencial del vector. Cuando encuentra el ejercicio, modifica sus campos y termina la ejecución.

### Mensaje emitido
- `"Ejercicio '{}' actualizado."`

### Error posible
Si el nombre del ejercicio no existe dentro de la rutina, se devuelve el error:
- `EjercicioNoEncontrado`

---

## 4. Eliminar ejercicio

La instrucción `eliminar_ejercicio` remueve un ejercicio de la rutina.

### Qué hace
- Verifica que el firmante sea el propietario del log.
- Busca la posición del ejercicio dentro del vector.
- Si lo encuentra, lo elimina usando `remove`.

### Parámetros requeridos
- `nombre: String`

### Resultado
El ejercicio desaparece de la rutina almacenada.

### Mensaje emitido
- `"Ejercicio '{}' eliminado de la rutina."`

### Error posible
Si el ejercicio no existe, el programa devuelve:
- `EjercicioNoEncontrado`

---

## 5. Ver rutina

La instrucción `ver_rutina` permite visualizar el contenido del log del usuario.

### Qué hace
- Muestra el nombre del usuario almacenado.
- Imprime la rutina actual en los logs del programa.

### Resultado
La información se expone a través de mensajes de depuración (`msg!`), útiles para pruebas, desarrollo y seguimiento del estado.

### Mensajes emitidos
- `"Usuario: {}"`
- `"Rutina Actual: {:#?}"`

### Observación importante
Esta instrucción no retorna datos como una respuesta estructurada para interfaz gráfica; en su estado actual, simplemente escribe la información en los logs de ejecución del programa.

---

## Control de Acceso y Seguridad

Uno de los aspectos fundamentales del programa es el control de propiedad sobre la cuenta `GymLog`.

En las instrucciones `registrar_ejercicio`, `editar_ejercicio` y `eliminar_ejercicio`, el sistema ejecuta la validación:

```rust
require!(log.owner == ctx.accounts.owner.key(), Errores::NoEresElOwner);

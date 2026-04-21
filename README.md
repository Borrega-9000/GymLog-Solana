# GymLog Solana - Bitácora de Entrenamiento 

**GymLog Solana** es un contrato inteligente diseñado para atletas y entusiastas del fitness que desean registrar su progreso físico de manera inmutable en la blockchain de Solana. Este programa elimina el uso de estados predeterminados, obligando a una entrada de datos completa por parte del usuario.

##  Especificaciones Técnicas

El programa utiliza **PDAs** para el almacenamiento de datos, permitiendo que cada usuario sea dueño de su información mediante una cuenta derivada de su propia llave pública.

### Requisitos de Datos (Sin Valores por Defecto)
Para cada registro, el usuario debe proveer obligatoriamente:
- **Nombre del Ejercicio**: Identificador único en la rutina.
- **Series**: Cantidad de sets realizados (u8).
- **Repeticiones**: Cantidad de ejecuciones por set (u8).
- **Peso (kg)**: Carga utilizada en el ejercicio (u16).

### Estructura de la Cuenta
- **Owner**: Llave pública del usuario (32 bytes).
- **Nombre Usuario**: Alias del atleta (Max. 40 caracteres).
- **Rutina**: Vector con capacidad para 12 ejercicios, optimizado mediante `InitSpace`.

## 🛠️ Instrucciones CRUD

1. **Inicializar**: `inicializar_log` crea la PDA necesaria para empezar a guardar datos.
2. **Registrar**: `registrar_ejercicio` añade un nuevo objeto al vector. Requiere que todos los parámetros numéricos y de texto sean provistos.
3. **Editar**: `editar_ejercicio` permite actualizar las métricas de un ejercicio específico cuando el usuario progresa en sus cargas.
4. **Eliminar**: `eliminar_ejercicio` remueve un registro del vector basándose en el nombre del ejercicio.
5. **Leer**: `ver_rutina` muestra el estado actual de la cuenta en los logs.

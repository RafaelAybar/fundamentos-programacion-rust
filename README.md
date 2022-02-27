# Fundamentos-programacion-rust
Fundamentos de programación preparada por libre

## Base teórica

1. Definciones.
- **Hardware**: Parte tangible de un sistema informático, es decir, componentes eléctricos, electróncicos y electromecánicos.


- **Software**: Conjunto de instrucciones que comunican a una máquina cómo trabajar.
    - Tipos según el campo de uso:
      - Sistemas operativos.
      - Aplicaciones.
      - Aplicaciones de sistema.
      - Controladores de dispositivos.


- **Biblioteca / librería**: Conjunto de implementaciones funcionales codificadas en un lenguaje de programación que ofrecen una interfaz bien definida para la funcionalidad que se invoca.
    - Tipos de liberías:
        - Estáticas: Las bibliotecas estáticas son ficheros que en el proceso de enlazado, en la compilación, serán copiados
        y relocalizados (si es necesario) en el fichero ejecutable final.
        - Dinámicas: Las bibliotecas dinámicas son ficheros que contienen código independiente de su ubicación,
        es decir, están preparadas para ser requeridas y cargadas en tiempo y ejecución por cualquier programa.
        - Remotas: Son aquellas que permiten ser llamadas desde otros
        - Relocalización: La localización real de los datos de la biblioteca no puede conocerse hasta que el ejecutable y todas las bibliotecas dinámicas que se han enlazado han sido cargadas en memoria. Por ello, se suele usar un directorio "estándar", donde se suelen instalar las bibliotecas.


- **Sistema operativo**: Es el software que permite gestionar el hardware del sistema y ejecutar otras aplicaciones.
  - Tipos de sistema operativo:
    - Monotarea.
    - Multitarea.
    - Monousuario.
    - Multiusuario.
    - Distribuidos.
    - Embebidos.


  - **Kernel**: Es el software esencial del sistema operativo que permite la comunicación entre aplicaciones y gestionar el
    uso del hardware que requiere cada aplicación.


  - **Sistema de ficheros**: Es el método que permite estrucutrar los datos que usa el sistema operativo para controlar cómo se gestionan los datos.


- **Memoria**:
  - RAM: La memoria de acceso aleatorio es aquella que puede ser leída y cambiada en cualquier orden. En la RAM se cargan todas las instrucciones que ejecuta la CPU.
    - Tipos de RAM:
      -   SRAM (estática).
      -   DRAM (dinámica)

  - ROM: La memoria de sólo lectura, no se puede modificar y se usa para escribir el firmware que controla el hardware.


- **Lenguaje de programación**:
    - Categorizados en:
      - Interpretado.
      - Compilado.
      - Alto nivel.
      - Bajo nivel.


- **Algoritmo**: Es una secuencia de instrucciones bien definida para resolver un problema o procesar algo.


- **Compilador**: Es el software que traduce código escrito en un lenguaje de programación a código máquina, código objeto o ensamblador para crear un fichero ejecutable.
  - Tipos de compiladores:
    - Compiladores cruzados.
    - Un paso / múltiples pasos.
    - JIT (Just in time): Este tipo de compiladores formamn parte de un intérprete y compila "partes del código" según se necesitan.

- **Bytecode**: Es conjunto de instrucciones diseñado para ser ejecutado de forma eficiente por un intérprete de software, dado que no es legible por humanos.


- **Variable**: Es un "almacenamiento abstracto" asociado a un nombre simbólco asociado a dicho espacio, el cual contiene un valor (conocido, o desconocido).
  - Tipos según la longitud:
    - Fija.
    - Dinámica.
  - Tipos según el dato que almacenan:
    - Entero.
    - Lógico / Booleano.
    - Carácter.
    - String. (Suelen ser cadenas de carácteres)

- **Puntero**: Es un objeto del lenguaje de programación cuyo valor se refiere a otro valor almacenado en otra parte de la memoria usando su dirección. Es decir, un puntero referencia a una ubicación en memoria.

- **Estructuras de control**: Permiten modificar el flujo de ejecución de las instrucciones de un programa.
  - Tipos:
    - Elección:
      - If-Then-Else.
      - Switch-Case.
    - Bucles:
      - For.
      - While / Do while.
      - Foreach.


- **Función**: Parte de un subalgoritmo que forma parte del algoritmo principal.


- **Paso por referencia**: La información de la variable que se recibe por parámetro apunta a la misma dirección de memoria que la variable original, por lo que si se modifica en la función, se modifica la variable original.


- **Paso por valor**: La información de la variable que se recibe por parámetro se almacena en una dirección de memoria diferente a la original, por lo que la original no se ve alterada.

- **Recursividad**: La recursividad es una técnica que consiste en expresar la solución de un problema mediante la aplicación de un algoritmo que se llama a sí mismo.


- **Refactorización**: Es el proceso mediante el cual se modifica el código fuente sin cambiar su comportamiento, para aplicar nuevas funcuionalidades o mejorar la consistencia / claridad del código.


- **API**: Application programming interface,  es un tipo de interfaz de Software ofreciendo una capa de abstracción entre servicios. Uno de los principales propósitos es
permitir y facilitar la documentación entre software. No están diseñadas para ser usadas por el usuario final.
  - Según arquitectura:
    - REST:
    - SOAP:
    - RPC:


- **ABI**: Application Binary Interface, es una interfaz dentre dos módulos binarios de un programa. Una ABI define cómo se accede en código máquina las estructuras de datos  o "rutinas computacionales".


- **Test**:
  - En base a funcionalidad:
      - Unitarios:
      - Integración:
      - Mutación:

## Base práctica

1. Ejemplos de código.

    - Elementos básicos:
        - **Gestión de ficheros**:
        - **Gestión de fechas**:
        - **Operaciones matemáticas básicas**:
        - **Acceso DB**:
            - Postgres:
            - MySQL:
    - Testing:
        - **Unitarios**:
    - **Gestión de trazas y errores**:
    - **Debugger**:

## Bibliografía.
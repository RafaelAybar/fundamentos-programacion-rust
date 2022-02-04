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


  - Kernel: Es el software esencial del sistema operativo que permite la comunicación entre aplicaciones y gestionar el
    uso del hardware que requiere cada aplicación.


  - Sistema de ficheros: Es el método que permite estrucutrar los datos que usa el sistema operativo para controlar cómo se gestionan los datos.


  - Memoria:


- **Lenguaje de programación**:
    - Categorizados en:
      - Interpretado.
      - Compilado.
      - Alto nivel.
      - Bajo nivel.


- **Compilador**: Es el software que traduce código escrito en un lenguaje de programación a código máquina, código objeto o ensamblador para crear un fichero ejecutable.
  - Tipos de compiladores:
    - Compiladores cruzados.
    - Un paso / múltiples pasos.
    - JIT (Just in time): Este tipo de compiladores formamn parte de un intérprete y compila "partes del código" según se necesitan.


- **Funciones**:


- **Paso por referencia**:


- **Paso por parámetro**:

- **Recursividad**:

- **Variable**:
  - Tipos:


- **Bytecode**:

- **Refactorización**:


- **API**:
  - REST:


- **ABI**:


- **Dependencias**:


- **Punteros**:


- **Estructuras de control**:


- **Excepciones**:


- **Test**:
  - En base a funcionalidad:
      - Unitarios:
      - Integración:
      - Mutación:

## Base práctica

2. Ejemplos de código.
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

3. Bibliografía.
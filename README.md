# Amaia Virtual Machine

Amaia virtual machine (AVM), es un interprete para el lenguaje de programación
`Amaia`, el cual no es mas que una simplificacion del lenguaje programación Basic.

`AVM` lee ficheros con extensión __.avm__, el contenido del fichero esta definido por una series de números hexadecimales, los cuales poseen las instrucciones a ser ejecutadas por la `AVM`.

## Instruciones

La `AVM` maneja un total de 12 instrucciones, las cuales se dividen en diferentes operaciones según su tipo:

1. Operaciones de entrada y salida.
    * READ
    > Lee una palabra desde la terminal y la almacena en la ubicacion de memoria.

    * WRITE
    > Escribe una palabra desde una ubicación especifica de memoria hacia la terminal.

2. Operaciones de carga y almacenamiento.
    * LOAD
    > Cargar una palabra desde una ubicación especifica de memoria, y la almacena en el acumulador. 

    * STORE
    > Almacena una palabra desde el acumulador hacia una ubicacion especifica de memoria.

3. Operaciones aritméticas.
    * ADD
    > Suma una palabra desde una ubicación especifica de memoria con la palabra almacenada en el acumulador. Deja el resultado de la operacion en el acumulador.

    * SUB
    > Resta una palabra desde una ubicación especifica de memoria con la palabra almacenada en el acumulador. Deja el resultado de la operacion en el acumulador.

    * DIV
    > Divide una palabra desde una ubicación especifica de memoria con la palabra almacenada en el acumulador. Deja el resultado de la operacion en el acumulador.

    * MUL
    > Multiplica una palabra desde una ubicación especifica de memoria con la palabra almacenada en el acumulador. Deja el resultado de la operacion en el acumulador.

4. Operaciones de transferncia de control.
    * JUMP
    > Salta a una ubicación especifica de memoria.

    * JUMP_NEG
    > Salta hacia una ubicación especifica de memoria si el acumulador es negativo.

    * JUMP_ZERO
    > Salta hacia una ubicación especifica de memoria si el acumulador es igual a cero.

    * STOP
    > Indica la finalización del programa.

### Resumen de instrucciones:

|  N  | Instrucion | Hexadecimal | Decimal |
|:---:|:----------:|:-----------:|:-------:|
|  0  |    READ    |     0x0A    |    10   |
|  1  |    WRITE   |     0x0B    |    11   |
|  2  |    LOAD    |     0x14    |    20   |
|  3  |    STORE   |     0x15    |    21   |
|  4  |     ADD    |     0x1E    |    30   |
|  5  |     SUB    |     0x1F    |    31   |
|  6  |     DIV    |     0x20    |    32   |
|  7  |     MUL    |     0x21    |    33   |
|  8  |    JUMP    |     0x28    |    40   |
|  9  |  JUMP_NEG  |     0x29    |    41   |
|  10 |  JUMP_ZERO |     0x2A    |    42   |
|  11 |     STOP   |     0x2B    |    43   |

## Formato de una instruccion:

Las instruciones en `AVM` constan de una secuencia de números hexadecimales, los cuales contienen toda la información necesaria para ejecución de las diferentes operaciones.

### Ejemplo de instrucción:

Ejemplo de una instruccion de lectura (READ).

> `00A00000007`

La instruccion la podriamos dividir en tres números hexadecimales:

> `0 0A 00000007`

El primer dígito es un número hexadecimal, con una capacidad de almacenamiento igual a **2^4**, es decir, `1 nibble` o `4 bit`, ya que puede tomar valores del `0 al F` o del `0 al 15` en decimal. Define el signo de la instrucción, donde __0__ representa un número positivo y 
__1__ un número negativo, no se contemplan valores superiores a __1__.

El segundo dígito es un número hexadecimal con una capacidad de almacenamiento igual a **2^8**, es decir, `1 bytes` o lo que es lo mismo `8 bit`, puede tomar un rango de valores igual  `00 - F`, o lo que seria su equivalente en decimal el rango `0 - 255`. Es en este dígito donde se definen las operaciones a realizar, **operaciones de I/O**, **operaciones de transferencia de control**, etc.

El tercer dígito es un número hexadecimal con una capacidad de almacenamiento igual a **2^32**, es decir, `4.294.967.296 bit` que igual a `536.870.912 bytes`, esto define la capacidad de memoria manejada por la `AVM`.

## Ejemplos 

### Suma:

Lee dos números, los alamacena y luego los opera, el resultado lo almacena en un espacio de memoria y lo imprime en la terminal.

```
00A00000007
00A00000008
01400000007
01E00000008
01500000009
00B00000009
02B00000000
00000000000
00000000000
00000000000
00000000000
```

### Menor que:

Lee dos números, los almacena, y imprime el menor en la terminal.

```
00A0000000D
00A0000000E
0140000000D
01F0000000E
02900000009
0140000000E
01F0000000D
0290000000B
00B0000000D
02B00000000
00B0000000E
02B00000000
00000000000
00000000000
```

### Contador:

Lee dos números, los almacena y resta el primero con respecto al segundo hasta que el primero sea igual a cero, el rsultado de la operacion lo va imprimiendo en la terminal.

```
00A0000000B
00A0000000C
00B0000000B
0140000000B
02A0000000A
01F0000000C
0150000000B
0140000000B
02800000003
02B00000000
00000000000
00000000000
```
# Data File Format
Floats are little-endian.

| Byte Index | Possible Value | Meaning                                                  |
| ---------: | -------------: | :------------------------------------------------------- |
|          0 |       32 or 64 | Number of bits per float                                 |
|          1 |    any type ID | Type of data in file (see table)                         |
|         2+ |         varies | Contents of file (see format of each specific data type) |

## Type IDs
| Value | Data Type |
| ----: | :-------- |
|     1 | Float     |
|     2 | Vector    |

## Float Format
| Byte Index | Possible Value | Meaning        |
| ---------: | -------------: | :------------- |
|          0 |      arbitrary | Value of float |

## Vector Format
| Byte Index | Possible Value | Meaning                |
| ---------: | -------------: | :--------------------- |
|          0 |    any type ID | Type of data in vector |
|          1 |      arbitrary | Length of vector       |
|         2+ |      arbitrary | Data in vector         |

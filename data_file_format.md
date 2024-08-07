# Data File Format
Floats are little-endian.

| Byte Index | Possible Value | Meaning                                                  |
| ---------: | -------------: | :------------------------------------------------------- |
|          0 |       32 or 64 | Number of bits per float                                 |
|          1 |         TypeID | Type of data in file (see table)                         |
|        2.. |         varies | Contents of file (see format of each specific data type) |

## TypeIDs
| Value | Data Type |
| ----: | :-------- |
|     1 | Float     |
|     2 | Vector    |

## Float Format
|   Byte Index | Possible Value | Meaning        |
| -----------: | -------------: | :------------- |
| 0..4 or 0..8 |          Float | Value of float |

## Vector Format
| Byte Index | Possible Value | Meaning                |
| ---------: | -------------: | :--------------------- |
|          0 |         TypeID | Type of data in vector |
|          1 |             u8 | Length of vector       |
|         2+ |      arbitrary | Data in vector         |

# Data File Format
Floats are little-endian.

| Byte Index | Possible Value | Meaning                                                  |
| ---------: | -------------: | :------------------------------------------------------- |
|          0 |       32 or 64 | Number of bits per float                                 |
|          1 |         TypeID | Type of data in file (see table)                         |
|        2.. |         varies | Contents of file (see format of each specific data type) |

## TypeIDs
| Value | Data Type       |
| ----: | :-------------- |
|     0 | Int             |
|     1 | Float           |
|     2 | Vector          |
|     3 | ParticleSpecies |

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

## ParticleSpecies Format
|      Byte Index | Possible Value | Meaning |
| --------------: | -------------: | :------ |
|    0..4 or 0..8 |          Float | Mass    |
|   4..8 or 8..16 |          Float | Charge  |
| 8..12 or 16..32 |          Float | Weight  |

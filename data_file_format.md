# Data File Format
Floats are little-endian.

| Byte Index | Possible Value | Meaning                                                  |
| ---------: | -------------: | :------------------------------------------------------- |
|          0 |       32 or 64 | Number of bits per float                                 |
|          1 |              1 | File contains a float                                    |
|         2+ |         varies | Contents of file (see format of each specific data type) |

## Float Format
| Byte Index | Possible Value | Meaning        |
| ---------: | -------------: | :------------- |
|          0 |      arbitrary | Value of float |
# Data File Format
Floats are little-endian.

| Byte Index | Possible Value | Meaning                                                  |
| ---------: | -------------: | :------------------------------------------------------- |
|          0 |              0 | Floats are 32-bit                                        |
|            |              1 | Floats are 64-bit                                        |
|          1 |              0 | File contains a float                                    |
|         2+ |         varies | Contents of file (see format of each specific data type) |

## Float Format
| Byte Index | Possible Value | Meaning        |
| ---------: | -------------: | :------------- |
|          0 |      arbitrary | Value of float |
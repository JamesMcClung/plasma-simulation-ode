# Data File Format 0.1.0
The first 4 bytes specify how the file is formatted, and data starts at byte index 4. Data is stored in little-endian words.

| Byte Index | Byte Type | Meaning                  |
| ---------: | :-------- | :----------------------- |
|          0 | u8        | Major version of format  |
|          1 | u8        | Minor version of format  |
|          2 | u8        | Patch version of format  |
|          3 | 4 or 8    | Number of bytes per word |
|          4 | see below | Word index = 0           |

|       Word Index | Word Type | Meaning        |
| ---------------: | :-------- | :------------- |
|                0 | TypeID    | Type of data 0 |
|          1..N(0) | varies    | Data 0         |
|             N(i) | TypeID    | Type of data i |
| (N(i)+1)..N(i+1) | varies    | Data i         |

## TypeIDs
| Value | Data Type       |
| ----: | :-------------- |
|     0 | UInt            |
|     1 | Int             |
|     2 | Float           |
|    10 | Vector          |
|    20 | ParticleSpecies |
|    30 | ParticleList    |

## Data Formats

### Vector
| Word Index | Type   | Meaning                |
| ---------: | :----- | :--------------------- |
|          0 | TypeID | Type of data in vector |
|          1 | Int    | Length of vector       |
|        2.. | varies | Data in vector         |

### ParticleSpecies
| Word Index | Type  | Meaning |
| ---------: | :---- | :------ |
|          0 | Float | Mass    |
|          1 | Float | Charge  |
|          2 | Float | Weight  |

### ParticleList
| Word Index | Type            | Meaning                               |
| ---------: | :-------------- | :------------------------------------ |
|          0 | UInt            | Number of dimensions                  |
|          1 | UInt            | Number of particles                   |
|       2..5 | ParticleSpecies | Species of particles in list          |
|        5.. | Floats          | All positions and then all velocities |

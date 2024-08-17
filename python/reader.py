from __future__ import annotations

from abc import ABC, abstractmethod
from dataclasses import dataclass
from io import BufferedReader
import struct
from typing import Self, Type, override


class FromBytes(ABC):
    @classmethod
    @abstractmethod
    def from_bytes(cls, bytes: BytesReader) -> Self:
        pass


@dataclass
class Prelude(FromBytes):
    format_version_major: int
    format_version_minor: int
    format_version_patch: int
    bytes_per_word: int

    @override
    @classmethod
    def from_bytes(cls, bytes: BytesReader) -> Self:
        return cls(*struct.unpack("<BBBB", bytes.handle.read(4)))


class BytesReader:
    def __init__(self, path: str) -> None:
        self.path = path
        self.handle: BufferedReader | None = None
        self.prelude: Prelude | None = None

    def __enter__(self) -> Self:
        self.handle = open(self.path, "rb")
        self.prelude = Prelude.from_bytes(self)
        return self

    def __exit__(self, type, value, traceback):
        self.handle.__exit__(type, value, traceback)
        self.handle = None

    def read_uints(self, n_uints: int) -> tuple[int, ...]:
        assert self.handle
        int_format_str = "I" if self.prelude.bytes_per_word == 4 else "Q"
        return struct.unpack("<" + int_format_str * n_uints, self.handle.read(n_uints * self.prelude.bytes_per_word))

    def read_ints(self, n_ints: int) -> tuple[int, ...]:
        assert self.handle
        int_format_str = "i" if self.prelude.bytes_per_word == 4 else "q"
        return struct.unpack("<" + int_format_str * n_ints, self.handle.read(n_ints * self.prelude.bytes_per_word))

    def read_floats(self, n_floats: int) -> tuple[float, ...]:
        assert self.handle
        float_format_str = "f" if self.prelude.bytes_per_word == 4 else "d"
        return struct.unpack("<" + float_format_str * n_floats, self.handle.read(n_floats * self.prelude.bytes_per_word))

    def read[T: FromBytes](self, type: Type[T]) -> T:
        return type.from_bytes(self)


@dataclass
class ParticleSpecies(FromBytes):
    mass: float
    charge: float
    weight: float

    @override
    @classmethod
    def from_bytes(cls, bytes: BytesReader) -> Self:
        return cls(*bytes.read_floats(3))


class Vector[T]:
    def __init__(self, vals: tuple[T, ...]) -> None:
        self.vals = vals

    def len(self) -> int:
        return len(self.vals)

    def __repr__(self) -> str:
        return f"<{', '.join(str(val) for val in self.vals)}>"

    def __getitem__(self, idx: int) -> T:
        return self.vals[idx]


@dataclass
class ParticleList(FromBytes):
    species: ParticleSpecies
    positions: list[Vector[float]]
    velocities: list[Vector[float]]
    n_dims: int
    n_particles: int

    @override
    @classmethod
    def from_bytes(cls, bytes: BytesReader) -> Self:
        (n_dims, n_particles) = bytes.read_uints(2)
        species = bytes.read(ParticleSpecies)
        positions = [Vector(bytes.read_floats(n_dims)) for _ in range(n_particles)]
        velocities = [Vector(bytes.read_floats(n_dims)) for _ in range(n_particles)]
        return cls(species, positions, velocities, n_dims, n_particles)


import matplotlib.pyplot as plt

positions: list[Vector[float]] = []
for step in range(0, 1000, 10):
    with BytesReader(f"runs/run1/{step:03}.particles.dat") as reader:
        particles = reader.read(ParticleList)
        positions.append(particles.positions[0])

xs = [pos[0] for pos in positions]
ys = [pos[1] for pos in positions]

fig, ax = plt.subplots()
plt.plot(xs, ys)
ax.set_xlim(0, 3)
ax.set_ylim(0, 3)
plt.gca().set_aspect("equal")
plt.show()

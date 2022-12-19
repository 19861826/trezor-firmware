# This file is part of the Trezor project.
#
# Copyright (C) 2012-2022 SatoshiLabs and contributors
#
# This library is free software: you can redistribute it and/or modify
# it under the terms of the GNU Lesser General Public License version 3
# as published by the Free Software Foundation.
#
# This library is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Lesser General Public License for more details.
#
# You should have received a copy of the License along with this library.
# If not, see <https://www.gnu.org/licenses/lgpl-3.0.html>.

from dataclasses import dataclass, field
from hashlib import sha256
import typing as t


@dataclass
class CachedHash:
    _hash: t.Optional[bytes] = field(default=None, init=False)

    @property
    def hash(self) -> bytes:
        if self._hash is None:
            self._hash = self.compute_hash()
        return self._hash

    def compute_hash(self) -> bytes:
        raise NotImplementedError


@dataclass
class Leaf(CachedHash):
    value: bytes
    proof: t.List[bytes] = field(default_factory=list)

    def compute_hash(self) -> bytes:
        return sha256(b"\x00" + self.value).digest()

    def add_to_proof_list(self, proof_entry: bytes) -> None:
        self.proof.append(proof_entry)


@dataclass
class Node(CachedHash):
    left: "Node | Leaf"
    right: "Node | Leaf"

    def __post_init__(self) -> None:
        self.left.add_to_proof_list(self.right.hash)
        self.right.add_to_proof_list(self.left.hash)

    def compute_hash(self) -> bytes:
        hash_a = min(self.left.hash, self.right.hash)
        hash_b = max(self.left.hash, self.right.hash)
        return sha256(b"\x01" + hash_a + hash_b).digest()

    def add_to_proof_list(self, proof: bytes) -> None:
        self.left.add_to_proof_list(proof)
        self.right.add_to_proof_list(proof)


class MerkleTree:
    """
    Simple Merkle tree that implements the building of Merkle tree itself and generate proofs
    for leaf nodes.
    """

    def __init__(self, values: t.Iterable[bytes]) -> None:
        self.leaves = [Leaf(v) for v in values]
        self.height = 0

        # build the tree
        current_level = self.leaves[:]
        while len(current_level) > 1:
            # build one level of the tree
            next_level = []
            while len(current_level) >= 2:
                left = current_level.pop()
                right = current_level.pop()
                next_level.append(Node(left, right))

            # add the remaining one or zero nodes to the next level
            next_level.extend(current_level)

            # switch levels and continue
            self.height += 1
            current_level = next_level

        # set root and compute hash
        self.root_node = current_level[0]

    def get_proofs(self) -> t.Dict[bytes, t.List[bytes]]:
        return {n.value: n.proof for n in self.leaves}

    def get_tree_height(self) -> int:
        return self.height

    def get_root_hash(self) -> bytes:
        return self.root_node.hash

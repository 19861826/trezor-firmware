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

import pytest

import typing as t
from hashlib import sha256

from trezorlib.merkle_tree import MerkleTree, Leaf, Node


NODE_VECTORS = (  # node, expected_hash
    (  # leaf node
        Leaf(b"hello"),
        "8a2a5c9b768827de5a9552c38a044c66959c68f6d2f21b5260af54d2f87db827",
    ),
    (  # node with leaf nodes
        Node(left=Leaf(b"hello"), right=Leaf(b"world")),
        "24233339aadcedf287d262413f03c028eb8db397edd32a2878091151b99bf20f",
    ),
    (  # asymmetric node with leaf hanging on second level
        Node(left=Node(left=Leaf(b"hello"), right=Leaf(b"world")), right=Leaf(b"!")),
        "c3727420dc97c0dbd89678ee195957e44cfa69f5759b395a07bc171b21468633",
    ),
)

MERKLE_TREE_VECTORS = (
    (  # one value
        # values
        [bytes.fromhex("dead")],
        # expected root hash
        sha256(b"\x00" + bytes.fromhex("dead")).digest(),
        # expected tree height
        0,
        # expected dict of proof lists
        {
            bytes.fromhex("dead"): [],
        },
    ),
    (  # two values
        # values
        [bytes.fromhex("dead"), bytes.fromhex("beef")],
        # expected root hash
        sha256(
            b"\x01"
            + sha256(b"\x00" + bytes.fromhex("beef")).digest()
            + sha256(b"\x00" + bytes.fromhex("dead")).digest()
        ).digest(),
        # expected tree height
        1,
        # expected dict of proof lists
        {
            bytes.fromhex("dead"): [sha256(b"\x00" + bytes.fromhex("beef")).digest()],
            bytes.fromhex("beef"): [sha256(b"\x00" + bytes.fromhex("dead")).digest()],
        },
    ),
    (  # three values
        # values
        [bytes.fromhex("dead"), bytes.fromhex("beef"), bytes.fromhex("cafe")],
        # expected root hash
        sha256(
            b"\x01"
            + sha256(
                b"\x01"
                + sha256(b"\x00" + bytes.fromhex("cafe")).digest()
                + sha256(b"\x00" + bytes.fromhex("beef")).digest()
            ).digest()
            + sha256(b"\x00" + bytes.fromhex("dead")).digest()
        ).digest(),
        # expected tree height
        2,
        # expected dict of proof lists
        {
            bytes.fromhex("dead"): [
                sha256(
                    b"\x01"
                    + sha256(b"\x00" + bytes.fromhex("cafe")).digest()
                    + sha256(b"\x00" + bytes.fromhex("beef")).digest()
                ).digest()
            ],
            bytes.fromhex("beef"): [
                sha256(b"\x00" + bytes.fromhex("cafe")).digest(),
                sha256(b"\x00" + bytes.fromhex("dead")).digest(),
            ],
            bytes.fromhex("cafe"): [
                sha256(b"\x00" + bytes.fromhex("beef")).digest(),
                sha256(b"\x00" + bytes.fromhex("dead")).digest(),
            ],
        },
    ),
)


@pytest.mark.parametrize("node, expected_hash", NODE_VECTORS)
def test_node(node: t.Union[Node, Leaf], expected_hash: str) -> None:
    assert node.hash.hex() == expected_hash


@pytest.mark.parametrize(
    "values, expected_root_hash, expected_height, expected_proofs", MERKLE_TREE_VECTORS
)
def test_tree(
    values: t.List[bytes],
    expected_root_hash: bytes,
    expected_height: int,
    expected_proofs: t.Dict[bytes, t.List[bytes]],
) -> None:
    mt = MerkleTree(values)
    assert mt.get_root_hash() == expected_root_hash
    assert mt.get_tree_height() == expected_height
    assert mt.get_proofs() == expected_proofs

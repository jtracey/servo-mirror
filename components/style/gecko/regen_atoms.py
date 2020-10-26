#!/usr/bin/env python

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import re
import os
import sys

from io import BytesIO

GECKO_DIR = os.path.dirname(__file__.replace("\\", "/"))
sys.path.insert(0, os.path.join(os.path.dirname(GECKO_DIR), "properties"))

import build


# Matches lines like `GK_ATOM(foo, "foo", 0x12345678, true, nsStaticAtom, PseudoElementAtom)`.
PATTERN = re.compile(
    '^GK_ATOM\(([^,]*),[^"]*"([^"]*)",\s*(0x[0-9a-f]+),\s*[^,]*,\s*([^,]*),\s*([^)]*)\)',
    re.MULTILINE,
)
FILE = "include/nsGkAtomList.h"


def map_atom(ident):
    if ident in {
        "box",
        "loop",
        "match",
        "mod",
        "ref",
        "self",
        "type",
        "use",
        "where",
        "in",
    }:
        return ident + "_"
    return ident


class Atom:
    def __init__(self, ident, value, hash, ty, atom_type):
        self.ident = "nsGkAtoms_{}".format(ident)
        self.original_ident = ident
        self.value = value
        self.hash = hash
        # The Gecko type: "nsStaticAtom", "nsCSSPseudoElementStaticAtom", or
        # "nsAnonBoxPseudoStaticAtom".
        self.ty = ty
        # The type of atom: "Atom", "PseudoElement", "NonInheritingAnonBox",
        # or "InheritingAnonBox".
        self.atom_type = atom_type

        if (
            self.is_pseudo_element()
            or self.is_anon_box()
            or self.is_tree_pseudo_element()
        ):
            self.pseudo_ident = (ident.split("_", 1))[1]

        if self.is_anon_box():
            assert self.is_inheriting_anon_box() or self.is_non_inheriting_anon_box()

    def type(self):
        return self.ty

    def capitalized_pseudo(self):
        return self.pseudo_ident[0].upper() + self.pseudo_ident[1:]

    def is_pseudo_element(self):
        return self.atom_type == "PseudoElementAtom"

    def is_anon_box(self):
        if self.is_tree_pseudo_element():
            return False
        return self.is_non_inheriting_anon_box() or self.is_inheriting_anon_box()

    def is_non_inheriting_anon_box(self):
        assert not self.is_tree_pseudo_element()
        return self.atom_type == "NonInheritingAnonBoxAtom"

    def is_inheriting_anon_box(self):
        if self.is_tree_pseudo_element():
            return False
        return self.atom_type == "InheritingAnonBoxAtom"

    def is_tree_pseudo_element(self):
        return self.value.startswith(":-moz-tree-")


def collect_atoms(objdir):
    atoms = []
    path = os.path.abspath(os.path.join(objdir, FILE))
    print("cargo:rerun-if-changed={}".format(path))
    with open(path) as f:
        content = f.read()
        for result in PATTERN.finditer(content):
            atoms.append(
                Atom(
                    result.group(1),
                    result.group(2),
                    result.group(3),
                    result.group(4),
                    result.group(5),
                )
            )
    return atoms


class FileAvoidWrite(BytesIO):
    """File-like object that buffers output and only writes if content changed."""

    def __init__(self, filename):
        BytesIO.__init__(self)
        self.name = filename

    def write(self, buf):
        if isinstance(buf, str):
            buf = buf.encode("utf-8")
        BytesIO.write(self, buf)

    def close(self):
        buf = self.getvalue()
        BytesIO.close(self)
        try:
            with open(self.name, "rb") as f:
                old_content = f.read()
                if old_content == buf:
                    print("{} is not changed, skip".format(self.name))
                    return
        except IOError:
            pass
        with open(self.name, "wb") as f:
            f.write(buf)

    def __enter__(self):
        return self

    def __exit__(self, type, value, traceback):
        if not self.closed:
            self.close()


PRELUDE = """
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

// Autogenerated file created by components/style/gecko/regen_atoms.py.
// DO NOT EDIT DIRECTLY
"""[
    1:
]

RULE_TEMPLATE = """
    ("{atom}") => {{{{
        #[allow(unsafe_code)] #[allow(unused_unsafe)]
        unsafe {{ $crate::string_cache::Atom::from_index_unchecked({index}) }}
    }}}};
"""[
    1:
]

MACRO_TEMPLATE = """
/// Returns a static atom by passing the literal string it represents.
#[macro_export]
macro_rules! atom {{
{body}\
}}
"""


def write_atom_macro(atoms, file_name):
    with FileAvoidWrite(file_name) as f:
        f.write(PRELUDE)
        macro_rules = [
            RULE_TEMPLATE.format(atom=atom.value, name=atom.ident, index=i)
            for (i, atom) in enumerate(atoms)
        ]
        f.write(MACRO_TEMPLATE.format(body="".join(macro_rules)))


def write_pseudo_elements(atoms, target_filename):
    pseudos = []
    for atom in atoms:
        if (
            atom.type() == "nsCSSPseudoElementStaticAtom"
            or atom.type() == "nsCSSAnonBoxPseudoStaticAtom"
        ):
            pseudos.append(atom)

    pseudo_definition_template = os.path.join(
        GECKO_DIR, "pseudo_element_definition.mako.rs"
    )
    print("cargo:rerun-if-changed={}".format(pseudo_definition_template))
    contents = build.render(pseudo_definition_template, PSEUDOS=pseudos)

    with FileAvoidWrite(target_filename) as f:
        f.write(contents)


def generate_atoms(dist, out):
    atoms = collect_atoms(dist)
    write_atom_macro(atoms, os.path.join(out, "atom_macro.rs"))
    write_pseudo_elements(atoms, os.path.join(out, "pseudo_element_definition.rs"))


if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: {} dist out".format(sys.argv[0]))
        exit(2)
    generate_atoms(sys.argv[1], sys.argv[2])

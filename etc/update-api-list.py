# #!/usr/bin/env python3
# """Create a text file listing all public API. This can be used to ensure that all
# functions are covered by our macros.

# This file additionally does tidy-esque checks that all functions are listed where
# needed, or that lists are sorted.
# """

# # FIXME: this needs to be updated to work with compiler-builtins sources

# import difflib
# from enum import StrEnum
# import json
# import pprint
# import re
# import subprocess as sp
# import sys
# from dataclasses import dataclass, field
# from glob import glob
# from pathlib import Path
# from typing import Any, TypeAlias

# SELF_PATH = Path(__file__)
# ETC_DIR = SELF_PATH.parent
# ROOT_DIR = ETC_DIR.parent

# # These files do not trigger a retest.
# IGNORED_SOURCES = ["libm/src/libm_helper.rs", "libm/src/math/support/float_traits.rs"]

# IndexTy: TypeAlias = dict[str, dict[str, Any]]
# """Type of the `index` item in rustdoc's JSON output"""


# def eprint(*args, **kwargs):
#     """Print to stderr."""
#     print(*args, file=sys.stderr, **kwargs)


# class Crate(StrEnum):
#     LIBM = "libm"
#     COMPILER_BUILTINS = "compiler_builtins"


# class RootType(StrEnum):
#     F16 = "f16"
#     F32 = "f32"
#     F64 = "f64"
#     F128 = "f128"


# @dataclass
# class Function:
#     """One entry in our list of functions"""

#     crate: Crate
#     path: str
#     name: str
#     root_type: RootType
#     sources: list[str] = field(default_factory=list)
#     """List to source files to find all places that define a public function.
#     We track this to know which tests need to be rerun when specific files get
#     updated. """

#     def locate_sources(self):
#         pass


# class Runner:
#     """Representation of public interfaces and function definition locations in
#     `libm` or `compiler-builtins`.
#     """

#     functions: list[Function]

#     def __init__(self) -> None:
#         self.functions = []

#         j = self._get_rustdoc_json("libm/src/lib.rs", "2021")
#         index: IndexTy = j["index"]
#         self._add_crate(index, Crate.LIBM)
#         j = self._get_rustdoc_json("compiler-builtins/src/lib.rs")
#         index: IndexTy = j["index"]
#         self._add_crate(index, Crate.COMPILER_BUILTINS)

#         for func in self.functions:
#             func.locate_sources()

#         pprint.pp(self.functions)

#         exit(0)
#         # self._init_libm_function_list(libm_index)
#         # self._init_defs(libm_index)
#         # self._init_types()

#     @staticmethod
#     def _get_rustdoc_json(path: str, edition: str | None = "2024") -> dict[Any, Any]:
#         """Get rustdoc's JSON output for the `libm` crate."""

#         j = sp.check_output(
#             [
#                 "rustdoc",
#                 path,
#                 f"--edition={edition}",
#                 "--document-private-items",
#                 "--output-format=json",
#                 "--cfg=f16_enabled",
#                 "--cfg=f128_enabled",
#                 "-Zunstable-options",
#                 "-o-",
#             ],
#             cwd=ROOT_DIR,
#             text=True,
#         )
#         j = json.loads(j)
#         return j

# <<<<<<< qlmvxowu fb48f815 "chore(deps): lock file maintenance" (rebase destination)
#     def _init_function_list(self, index: IndexTy) -> None:
#         """Get a list of public functions from rustdoc JSON output.

#         Note that this only finds functions that are reexported in `lib.rs`, this will
#         need to be adjusted if we need to account for functions that are defined there, or
#         glob reexports in other locations.
#         """
#         # Filter out items that are not public
#         public = [i for i in index.values() if i["visibility"] == "public"]

#         # Collect a list of source IDs for reexported items in `lib.rs` or `mod math`.
#         use = (i for i in public if "use" in i["inner"])
#         use = (
#             i
#             for i in use
#             if i["span"]["filename"] in ["libm/src/math/mod.rs", "libm/src/lib.rs"]
#         )
#         reexported_ids = [item["inner"]["use"]["id"] for item in use]

#         # Collect a list of reexported items that are functions
#         for id in reexported_ids:
#             srcitem = index.get(str(id))
#             # External crate
#             if srcitem is None:
#                 continue

#             # Skip if not a function
#             if "function" not in srcitem["inner"]:
#                 continue

#             self.public_functions.append(srcitem["name"])
#         self.public_functions.sort()

#     def _init_defs(self, index: IndexTy) -> None:
#         defs = {name: set() for name in self.public_functions}
#         all_funcs = (i for i in index.values() if "function" in i["inner"])

#         for func_def in all_funcs:
#             func_def_name = func_def["name"]
#             for pub_func_name in self.public_functions:
#                 needles = [
#                     pub_func_name,
#                     f"{pub_func_name}_round",
#                     f"{pub_func_name}_status",
#                 ]
#                 if not any(needle == func_def_name for needle in needles):
#                     continue

#                 defs[pub_func_name].add(func_def["span"]["filename"])

#         # A lot of the `arch` module is often configured out so doesn't show up in docs. Use
#         # string matching as a fallback.
#         for fname in glob(
#             "libm/src/math/arch/**/*.rs", root_dir=ROOT_DIR, recursive=True
#         ):
#             contents = (ROOT_DIR.joinpath(fname)).read_text()

#             for name in self.public_functions:
#                 if f"fn {name}" in contents:
#                     defs[name].add(fname)

#         for name, sources in defs.items():
#             base_sources = defs[base_name(name)[0]]

#             # Also add any functions in `generic` that use this function's base name
#             for src in (s for s in base_sources if "generic" in s):
#                 sources.add(src)

#             for src in IGNORED_SOURCES:
#                 sources.discard(src)

#         # Sort the set
#         self.defs = {k: sorted(v) for (k, v) in defs.items()}

#     def _init_types(self) -> None:
#         self.types = {name: base_name(name)[1] for name in self.public_functions}

#     def write_function_list(self, check: bool) -> None:
#         """Collect the list of public functions to a simple text file."""
#         output = "# autogenerated by update-api-list.py\n"
#         for name in self.public_functions:
#             output += f"{name}\n"

#         out_file = ETC_DIR.joinpath("function-list.txt")

#         if check:
#             with open(out_file, "r") as f:
#                 current = f.read()
#             diff_and_exit(current, output, "function list")
#         else:
#             with open(out_file, "w") as f:
#                 f.write(output)

#     def write_function_defs(self, check: bool) -> None:
#         """Collect the list of information about public functions to a JSON file ."""
#         comment = (
#             "Autogenerated by update-api-list.py. "
#             "List of files that define a function with a given name. "
#             "This file is checked in to make it obvious if refactoring breaks things"
#         )

#         d = {"__comment": comment}
#         d |= {
#             name: {"sources": self.defs[name], "type": self.types[name]}
#             for name in self.public_functions
#         }

#         out_file = ETC_DIR.joinpath("function-definitions.json")
#         output = json.dumps(d, indent=4) + "\n"

#         if check:
#             with open(out_file, "r") as f:
#                 current = f.read()
#             diff_and_exit(current, output, "source list")
#         else:
#             with open(out_file, "w") as f:
#                 f.write(output)

#     def ensure_contains_api(self, fpath: Path, line_num: int, lines: list[str]):
#         """Given a list of strings, ensure that each public function we have is named
#         somewhere.
#         """
#         not_found = []
#         for func in self.public_functions:
#             # The function name may be on its own or somewhere in a snake case string.
#             pat = re.compile(rf"(\b|_){func}(\b|_)")
#             found = next((line for line in lines if pat.search(line)), None)

#             if found is None:
#                 not_found.append(func)

#         if len(not_found) == 0:
# ||||||| pntumrxv e6a33957 "api-list: Move sort and unique checks to tests" (parents of squashed revision)
#     def _init_function_list(self, index: IndexTy) -> None:
#         """Get a list of public functions from rustdoc JSON output.

#         Note that this only finds functions that are reexported in `lib.rs`, this will
#         need to be adjusted if we need to account for functions that are defined there, or
#         glob reexports in other locations.
#         """
#         # Filter out items that are not public
#         public = [i for i in index.values() if i["visibility"] == "public"]

#         # Collect a list of source IDs for reexported items in `lib.rs` or `mod math`.
#         use = (i for i in public if "use" in i["inner"])
#         use = (
#             i
#             for i in use
#             if i["span"]["filename"] in ["libm/src/math/mod.rs", "libm/src/lib.rs"]
#         )
#         reexported_ids = [item["inner"]["use"]["id"] for item in use]

#         # Collect a list of reexported items that are functions
#         for id in reexported_ids:
#             srcitem = index.get(str(id))
#             # External crate
#             if srcitem is None:
#                 continue

#             # Skip if not a function
#             if "function" not in srcitem["inner"]:
#                 continue

#             self.public_functions.append(srcitem["name"])
#         self.public_functions.sort()

#     def _init_defs(self, index: IndexTy) -> None:
#         defs = {name: set() for name in self.public_functions}
#         funcs = (i for i in index.values() if "function" in i["inner"])
#         funcs = (f for f in funcs if f["name"] in self.public_functions)
#         for func in funcs:
#             defs[func["name"]].add(func["span"]["filename"])

#         # A lot of the `arch` module is often configured out so doesn't show up in docs. Use
#         # string matching as a fallback.
#         for fname in glob(
#             "libm/src/math/arch/**/*.rs", root_dir=ROOT_DIR, recursive=True
#         ):
#             contents = (ROOT_DIR.joinpath(fname)).read_text()

#             for name in self.public_functions:
#                 if f"fn {name}" in contents:
#                     defs[name].add(fname)

#         for name, sources in defs.items():
#             base_sources = defs[base_name(name)[0]]
#             for src in (s for s in base_sources if "generic" in s):
#                 sources.add(src)

#             for src in IGNORED_SOURCES:
#                 sources.discard(src)

#         # Sort the set
#         self.defs = {k: sorted(v) for (k, v) in defs.items()}

#     def _init_types(self) -> None:
#         self.types = {name: base_name(name)[1] for name in self.public_functions}

#     def write_function_list(self, check: bool) -> None:
#         """Collect the list of public functions to a simple text file."""
#         output = "# autogenerated by update-api-list.py\n"
#         for name in self.public_functions:
#             output += f"{name}\n"

#         out_file = ETC_DIR.joinpath("function-list.txt")

#         if check:
#             with open(out_file, "r") as f:
#                 current = f.read()
#             diff_and_exit(current, output, "function list")
#         else:
#             with open(out_file, "w") as f:
#                 f.write(output)

#     def write_function_defs(self, check: bool) -> None:
#         """Collect the list of information about public functions to a JSON file ."""
#         comment = (
#             "Autogenerated by update-api-list.py. "
#             "List of files that define a function with a given name. "
#             "This file is checked in to make it obvious if refactoring breaks things"
#         )

#         d = {"__comment": comment}
#         d |= {
#             name: {"sources": self.defs[name], "type": self.types[name]}
#             for name in self.public_functions
#         }

#         out_file = ETC_DIR.joinpath("function-definitions.json")
#         output = json.dumps(d, indent=4) + "\n"

#         if check:
#             with open(out_file, "r") as f:
#                 current = f.read()
#             diff_and_exit(current, output, "source list")
#         else:
#             with open(out_file, "w") as f:
#                 f.write(output)

#     def tidy_lists(self) -> None:
#         """In each file, check annotations indicating blocks of code should be sorted or should
#         include all public API.
#         """

#         flist = sp.check_output(["git", "ls-files"], cwd=ROOT_DIR, text=True)

#         for path in flist.splitlines():
#             fpath = ROOT_DIR.joinpath(path)
#             if fpath.is_dir() or fpath == SELF_PATH:
#                 continue

#             lines = fpath.read_text().splitlines()

#             validate_delimited_block(
#                 fpath,
#                 lines,
#                 "verify-sorted-start",
#                 "verify-sorted-end",
#                 ensure_sorted,
#             )

#             validate_delimited_block(
#                 fpath,
#                 lines,
#                 "verify-apilist-start",
#                 "verify-apilist-end",
#                 lambda p, n, lines: self.ensure_contains_api(p, n, lines),
#             )

#     def ensure_contains_api(self, fpath: Path, line_num: int, lines: list[str]):
#         """Given a list of strings, ensure that each public function we have is named
#         somewhere.
#         """
#         not_found = []
#         for func in self.public_functions:
#             # The function name may be on its own or somewhere in a snake case string.
#             pat = re.compile(rf"(\b|_){func}(\b|_)")
#             found = next((line for line in lines if pat.search(line)), None)

#             if found is None:
#                 not_found.append(func)

#         if len(not_found) == 0:
# =======
#     # def _init_libm_function_list(self, index: IndexTy) -> None:
#     #     """Get a list of public functions from rustdoc JSON output.

#     #     Note that this only finds functions that are reexported in `lib.rs`, this will
#     #     need to be adjusted if we need to account for functions that are defined there, or
#     #     glob reexports in other locations.
#     #     """
#     #     # Filter out items that are not public
#     #     public = [i for i in index.values() if i["visibility"] == "public"]

#     #     # Collect a list of source IDs for reexported items in `lib.rs` or `mod math`.
#     #     use = (i for i in public if "use" in i["inner"])
#     #     use = (
#     #         i
#     #         for i in use
#     #         if i["span"]["filename"] in ["libm/src/math/mod.rs", "libm/src/lib.rs"]
#     #     )
#     #     reexported_ids = [item["inner"]["use"]["id"] for item in use]

#     #     # Collect a list of reexported items that are functions
#     #     for id in reexported_ids:
#     #         srcitem = index.get(str(id))
#     #         # External crate
#     #         if srcitem is None:
#     #             continue

#     #         # Skip if not a function
#     #         if "function" not in srcitem["inner"]:
#     #             continue

#     #         self.public_functions.append(srcitem["name"])
#     #     self.public_functions.sort()

#     def _add_crate(self, index: IndexTy, crate: Crate) -> None:
#         # Get a list of all crates
#         crate_ids = [
#             i["id"]
#             for i in index.values()
#             if i["inner"].get("module", {}).get("is_crate")
#         ]

#         print(list(crate_ids))

#         for id in crate_ids:
#             self._find_module_functions(id, index, crate, "", path_override=crate)

#     def _find_module_functions(
#         self,
#         id: int,
#         index: IndexTy,
#         crate: Crate,
#         path: str,
#         *,
#         path_override: str | None = None,
#         ignore_vis: bool = False,
#     ) -> None:
#         """Find all functions in a oule with the given ID, recursing into child
#         modules.
#         """
#         print(f"id {id}")
#         item = index[str(id)]
#         name = item["name"]

#         if (not ignore_vis) and item["visibility"] != "public":
# >>>>>>> yvzykylo e86bb2cd "wip for update-api-list" (rebased revision)
#             return

#         if path_override is not None:
#             path = path_override
#         elif name is not None:
#             path = f"{path}::{name}"

#         # Item describes a module, recurse into it
#         module = item["inner"].get("module")
#         if module is not None:
#             eprint(f"checking module `{path}`")
#             for item_id in module["items"]:
#                 self._find_module_functions(item_id, index, crate, path)

#         # Item describes a use item
#         use = item["inner"].get("use")
#         if use is not None:
#             eprint(f"checking use `{path}`")

#             # for item_id in use["items"]:
#             self._find_module_functions(use["id"], index, crate, path, ignore_vis=True)

#         if "function" in item["inner"]:
#             func = Function(crate, path, name, RootType.F16)
#             self.functions.append(func)

#     # def _init_defs(self, index: IndexTy) -> None:
#     #     defs = {name: set() for name in self.public_functions}
#     #     funcs = (i for i in index.values() if "function" in i["inner"])
#     #     funcs = (f for f in funcs if f["name"] in self.public_functions)
#     #     for func in funcs:
#     #         defs[func["name"]].add(func["span"]["filename"])

#     #     # A lot of the `arch` module is often  /configured out so doesn't show up in docs. Use
#     #     # string matching as a fallback.
#     #     for fname in glob(
#     #         "libm/src/math/arch/**/*.rs", root_dir=ROOT_DIR, recursive=True
#     #     ):
#     #         contents = (ROOT_DIR.joinpath(fname)).read_text()

#     #         for name in self.public_functions:
#     #             if f"fn {name}" in contents:
#     #                 defs[name].add(fname)

#     #     for name, sources in defs.items():
#     #         base_sources = defs[base_name(name)[0]]
#     #         for src in (s for s in base_sources if "generic" in s):
#     #             sources.add(src)

#     #         for src in IGNORED_SOURCES:
#     #             sources.discard(src)

#     #     # Sort the set
#     #     self.defs |= {k: sorted(v) for (k, v) in defs.items()}

#     # def _init_types(self) -> None:
#     #     self.types = {name: base_name(name)[1] for name in self.public_functions}

#     # def write_function_list(self, check: bool) -> None:
#     #     """Collect the list of public functions to a simple text file."""
#     #     output = "# autogenerated by update-api-list.py\n"
#     #     for name in self.functions:
#     #         output += f"{name}\n"

#     #     out_file = ETC_DIR.joinpath("function-list.txt")

#     #     if check:
#     #         with open(out_file, "r") as f:
#     #             current = f.read()
#     #         diff_and_exit(current, output, "function list")
#     #     else:
#     #         with open(out_file, "w") as f:
#     #             f.write(output)

#     # def write_function_defs(self, check: bool) -> None:
#     #     """Collect the list of information about public functions to a JSON file ."""
#     #     comment = (
#     #         "Autogenerated by update-api-list.py. "
#     #         "List of files that define a function with a given name. "
#     #         "This file is checked in to make it obvious if refactoring breaks things"
#     #     )

#     #     d = {"__comment": comment}
#     #     d |= {
#     #         name: {"sources": self.defs[name], "root_type": self.types[name]}
#     #         for name in self.public_functions
#     #     }

#     #     out_file = ETC_DIR.joinpath("function-definitions.json")
#     #     output = json.dumps(d, indent=4) + "\n"

#     #     if check:
#     #         with open(out_file, "r") as f:
#     #             current = f.read()
#     #         diff_and_exit(current, output, "source list")
#     #     else:
#     #         with open(out_file, "w") as f:
#     #             f.write(output)

#     # def tidy_lists(self) -> None:
#     #     """In each file, check annotations indicating blocks of code should be sorted or should
#     #     include all public API.
#     #     """

#     #     flist = sp.check_output(["git", "ls-files"], cwd=ROOT_DIR, text=True)

#     #     for path in flist.splitlines():
#     #         fpath = ROOT_DIR.joinpath(path)
#     #         if fpath.is_dir() or fpath == SELF_PATH:
#     #             continue

#     #         lines = fpath.read_text().splitlines()

#     #         validate_delimited_block(
#     #             fpath,
#     #             lines,
#     #             "v erify-sorted-start",
#     #             "v erify-sorted-end",
#     #             ensure_sorted,
#     #         )

#     #         validate_delimited_block(
#     #             fpath,
#     #             lines,
#     #             "v erify-apilist-start",
#     #             "v erify-apilist-end",
#     #             lambda p, n, lines: self.ensure_contains_api(p, n, lines),
#     #         )

#     # def ensure_contains_api(self, fpath: Path, line_num: int, lines: list[str]):
#     #     """Given a list of strings, ensure that each public function we have is named
#     #     somewhere.
#     #     """
#     #     not_found = []
#     #     for func in self.public_functions:
#     #         # The function name may be on its own or somewhere in a snake case string.
#     #         pat = re.compile(rf"(\b|_){func}(\b|_)")
#     #         found = next((line for line in lines if pat.search(line)), None)

#     #         if found is None:
#     #             not_found.append(func)

#     #     if len(not_found) == 0:
#     #         return

#     #     relpath = fpath.relative_to(ROOT_DIR)
#     #     eprint(f"functions not found at {relpath}:{line_num}: {not_found}")
#     #     exit(1)


# def diff_and_exit(actual: str, expected: str, name: str):
#     """If the two strings are different, print a diff between them and then exit
#     with an error.
#     """
#     if actual == expected:
#         print(f"{name} output matches expected; success")
#         return

#     a = [f"{line}\n" for line in actual.splitlines()]
#     b = [f"{line}\n" for line in expected.splitlines()]

#     diff = difflib.unified_diff(a, b, "actual", "expected")
#     sys.stdout.writelines(diff)
#     print(f"mismatched {name}")
#     exit(1)


# def base_name(name: str) -> tuple[str, str]:
#     """Return the basename and type from a full function name. Keep in sync with Rust's
#     `fn base_name`.
#     """
#     known_mappings = [
#         ("erff", ("erf", "f32")),
#         ("erf", ("erf", "f64")),
#         ("modff", ("modf", "f32")),
#         ("modf", ("modf", "f64")),
#         ("lgammaf_r", ("lgamma_r", "f32")),
#         ("lgamma_r", ("lgamma_r", "f64")),
#     ]

#     found = next((base for (full, base) in known_mappings if full == name), None)
#     if found is not None:
#         return found

#     if name.endswith("f"):
#         return (name.rstrip("f"), "f32")
#     elif name.endswith("f16"):
#         return (name.rstrip("f16"), "f16")
#     elif name.endswith("f32"):
#         return (name.rstrip("f32"), "f32")
#     elif name.endswith("f64"):
#         return (name.rstrip("f64"), "f64")
#     elif name.endswith("f128"):
#         return (name.rstrip("f128"), "f128")

#     return (name, "f64")


# def ensure_updated_list(check: bool) -> None:
#     """Runner to update the function list and JSON, or check that it is already up
#     to date.
#     """
#     crate = Runner()
#     # crate.write_function_list(check)
#     # crate.write_function_defs(check)

# <<<<<<< qlmvxowu fb48f815 "chore(deps): lock file maintenance" (rebase destination)
# ||||||| pntumrxv e6a33957 "api-list: Move sort and unique checks to tests" (parents of squashed revision)
#     crate.tidy_lists()

# =======
#     # crate.tidy_lists()

# >>>>>>> yvzykylo e86bb2cd "wip for update-api-list" (rebased revision)

# def main():
#     """By default overwrite the file. If `--check` is passed, print a diff instead and
#     error if the files are different.
#     """
#     match sys.argv:
#         case [_]:
#             ensure_updated_list(False)
#         case [_, "--check"]:
#             ensure_updated_list(True)
#         case _:
#             print("unrecognized arguments")
#             exit(1)


# if __name__ == "__main__":
#     main()

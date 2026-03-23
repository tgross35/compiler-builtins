//! Create a text file listing all public API. This can be used to ensure that all
//! functions are covered by our macros.

pub mod new;

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::LazyLock;

use api_list_common::{ALL_OPERATIONS, Group, base_name};
use regex::RegexSet;
use serde_json::Value;

/// Otherwise public items that we don't test.
static EXCLUDED_PUBLIC: LazyLock<RegexSet> = LazyLock::new(|| {
    let s = [
        // Covered by the libm crate
        "^compiler_builtins::math",
    ];
    RegexSet::new(s).unwrap()
});

/// Otherwise private items that we do want to test.
static INCLUDED_PRIVATE: LazyLock<RegexSet> = LazyLock::new(|| {
    let s: [&str; 0] = [];
    RegexSet::new(s).unwrap()
});

pub static WORKSPACE_ROOT: LazyLock<PathBuf> = LazyLock::new(|| {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_owned()
});

/// For indentation.
const SPACES: &str = "                                        ";

/// Structure of the definitions in a crate that we are interested in.
#[derive(Debug)]
pub struct CrateDefinitions {
    /// Map from Rustdoc ID to the function.
    pub functions: HashMap<String, Function>,
}

impl CrateDefinitions {
    pub fn new() -> Self {
        let mut this = Self {
            functions: HashMap::new(),
        };

        this.add_crate(Crate::Libm);

        for func in this.functions.values() {
            this.update_sources(func);
        }

        this
    }

    fn add_crate(&mut self, krate: Crate) {
        let mut cmd = Command::new("rustdoc");
        cmd.args([
            krate.entrypoint_path(),
            "--edition",
            krate.edition(),
            "--document-private-items",
            "--output-format=json",
            "--cfg=f16_enabled",
            "--cfg=f128_enabled",
            "-Zunstable-options",
            "-o-",
        ]);
        cmd.current_dir(&*WORKSPACE_ROOT);
        cmd.stderr(Stdio::inherit());

        eprintln!("+{cmd:?}");
        let out = match cmd.output() {
            Ok(v) if v.status.success() => v.stdout,
            Ok(v) => panic!("error: status {}", v.status),
            Err(e) => panic!("error: {e}"),
        };

        let j: Value = serde_json::from_slice(&out).unwrap();
        let index = &j["index"]; // We only care about the index entry

        fs::write("etc/deleteme.json", format!("{j:#}")).unwrap(); // TODO

        // Find the root crate ID, which will be the item with `is_crate: true`.
        let crate_ids = index
            .as_object()
            .unwrap()
            .values()
            .filter(|val| {
                val["inner"]
                    .get("module")
                    .and_then(|m| m.get("is_crate"))
                    .is_some_and(|v| v == true)
            })
            .map(|val| val["id"].clone())
            .collect::<Vec<_>>();
        let [crate_id] = crate_ids.as_slice() else {
            panic!("0 or >1 crate id, {crate_ids:?}");
        };

        self.add_module_functions(
            &crate_id.to_string(),
            index,
            &format!("{}", krate.crate_name()),
            krate,
            Visibility::Public,
            false,
            PathKind::Def,
            0,
            false,
        );
    }

    /// Add all functions that are present in a module.
    fn add_module_functions(
        &mut self,
        id: &str,
        index: &Value,
        parent_path: &str,
        krate: Crate,
        parent_vis: Visibility,
        append_name: bool,
        path_kind: PathKind,
        indent: usize,
        // `use` statements can ignore module visibility
        ignore_mod_pub: bool,
    ) {
        let spaces = &SPACES[..(indent * 2)];
        let item = &index[id];
        let name = &item["name"].as_str();
        let path = if append_name {
            // By default, add the name key
            name.map(|n| format!("{parent_path}::{n}"))
        } else {
            // But sometimes we don't want that, like for the root module where the name
            // is "lib".
            Some(parent_path.to_owned())
        };

        let item_vis = Visibility::from_val(&item["visibility"]);
        let mut computed_vis = item_vis.min(parent_vis);
        let vis_qual = item_vis.as_qual();
        let inner = &item["inner"];

        if let Some(mod_) = inner.get("module") {
            let mut mod_path = path.clone().expect("modules should have a name");
            let name = name.unwrap();
            if ignore_mod_pub {
                mod_path = parent_path.to_owned();
                computed_vis = parent_vis;
            }
            eprintln!(
                "{spaces}checking `{vis_qual}mod {name}` in {parent_path} ({path_kind:?}, \
                {item_vis:?}, {computed_vis:?})"
            );
            // `items` is a list of IDs for everything contained within the module.
            for item_id in mod_["items"].as_array().unwrap() {
                self.add_module_functions(
                    &item_id.to_string(),
                    index,
                    &mod_path,
                    krate,
                    computed_vis,
                    true,
                    path_kind,
                    indent + 1,
                    false,
                );
            }
        } else if let Some(use_) = inner.get("use") {
            // Reexports have an ID of what was exported, and `is_glob`.
            let source = &use_["source"].as_str().unwrap();
            eprintln!(
                "{spaces}checking `{vis_qual}use {source}` in {parent_path} ({path_kind:?}, \
                {item_vis:?}, {computed_vis:?})"
            );
            self.add_module_functions(
                use_["id"].as_str().unwrap(),
                index,
                parent_path,
                krate,
                computed_vis,
                true,
                PathKind::Use,
                indent + 1,
                true,
            );
        } else if let Some(_func) = inner.get("function") {
            // Root function object.
            let path = path.expect("functions should have a name");
            // True if the function is `pub`, meaning it can be re-exported if
            // not already public.
            let item_path = ItemPath {
                kind: path_kind,
                path,
                min_vis: parent_vis,
            };
            eprintln!(
                "{spaces}ADDING FUNCTION {item_path:?} ({path_kind:?}, {item_vis:?}, {computed_vis:?})"
            );

            match self.functions.get_mut(id) {
                Some(f) => {
                    f.paths.push(item_path);
                }
                None => {
                    let id = id.to_owned();
                    let f = Function {
                        id: id.clone(),
                        krate,
                        paths: vec![item_path],
                        name: name.unwrap().to_string(),
                        group: None,
                        def_file: item["span"]["filename"].as_str().unwrap().to_owned(),
                        should_test: Cell::new(true),
                        sources: RefCell::new(Vec::new()),
                    };
                    self.functions.insert(id, f);
                }
            };
        };
    }

    fn update_sources(&self, func: &Function) {
        func.update_should_test();

        if !func.should_test.get() {
            return;
        }

        let mut sources = func.sources.borrow_mut();
        sources.push(func.def_file.clone());

        let bn = base_name(&func.name);

        for other in self.functions.values() {
            if other.id == func.id {
                continue;
            }

            if other.name == func.name || other.name == bn {
                sources.push(other.def_file.clone());
            }
        }

        sources.sort_unstable();
        sources.dedup();
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Visibility {
    Public = 2,
    Crate = 1,
    Restricted = 0,
}

impl Visibility {
    fn from_val(vis: &Value) -> Self {
        match vis.as_str() {
            Some("public") => Self::Public,
            Some("crate") => Self::Crate,
            _ => Self::Restricted,
        }
    }

    /// Turn into a visibility qualifier like `pub(crate)`.
    fn as_qual(self) -> &'static str {
        match self {
            Visibility::Public => "pub ",
            Visibility::Crate => "pub(crate) ",
            Visibility::Restricted => "",
        }
    }
}

/// Source crate.
#[derive(Clone, Copy, Debug)]
pub enum Crate {
    Libm,
    CompilerBuiltins,
}

impl Crate {
    fn crate_name(self) -> &'static str {
        match self {
            Crate::Libm => "libm",
            Crate::CompilerBuiltins => "compiler_builtins",
        }
    }

    fn edition(self) -> &'static str {
        match self {
            Crate::Libm => "2021",
            Crate::CompilerBuiltins => "2024",
        }
    }

    fn entrypoint_path(self) -> &'static str {
        match self {
            Crate::Libm => "libm/src/lib.rs",
            Crate::CompilerBuiltins => "compiler-builtins/src/lib.rs",
        }
    }
}

#[derive(Debug)]
pub struct Function {
    pub id: String,
    pub name: String,
    pub krate: Crate,
    pub paths: Vec<ItemPath>,
    pub group: Option<Group>,
    pub def_file: String,
    pub should_test: Cell<bool>,
    pub sources: RefCell<Vec<String>>,
}

impl Function {
    /// Where this function is defined.
    fn def_path(&self) -> &ItemPath {
        let mut defs = self.paths.iter().filter(|p| p.kind == PathKind::Def);
        let def = defs.next().unwrap();
        if defs.next().is_some() {
            panic!(">1 def found in {:#?}", self.paths);
        }
        def
    }

    fn publicly_reachable(&self) -> bool {
        self.paths.iter().any(|p| p.min_vis == Visibility::Public)
    }

    fn update_should_test(&self) {
        let x = if self.publicly_reachable() {
            !EXCLUDED_PUBLIC.is_match(&self.def_path().path)
        } else {
            INCLUDED_PRIVATE.is_match(&self.def_path().path)
        };
        self.should_test.set(x);
    }
}

#[derive(Clone, Debug)]
pub struct ItemPath {
    pub kind: PathKind,
    pub path: String,
    pub min_vis: Visibility,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PathKind {
    /// This
    Def,
    ///
    Use,
}

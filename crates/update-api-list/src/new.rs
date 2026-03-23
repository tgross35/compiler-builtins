use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use serde_json::Value;

use crate::{Crate, PathKind, SPACES, Visibility, WORKSPACE_ROOT};

pub fn crate_function(krate: Crate) -> Vec<Function> {
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

    let mut mx = ModCtx {
        functions: HashMap::new(),
        index,
        krate,
    };

    add_module_functions(
        &mut mx,
        crate_id.as_u64().unwrap(),
        &format!("{}", krate.crate_name()),
        Visibility::Public,
        false,
        PathKind::Def,
        0,
        false,
    );

    let mut ret: Vec<_> = mx.functions.into_values().collect();
    ret.sort_unstable_by(|f1, f2| f1.name.cmp(&f2.name));

    ret
}

fn add_module_functions(
    mx: &mut ModCtx<'_>,
    id: u64,
    parent_path: &str,
    parent_vis: Visibility,
    append_name: bool,
    path_kind: PathKind,
    indent: usize,
    // `use` statements can ignore module visibility
    ignore_mod_pub: bool,
) {
    let spaces = &SPACES[..(indent * 2)];
    let item = &mx.index[id.to_string()];
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
            add_module_functions(
                mx,
                item_id.as_u64().unwrap(),
                &mod_path,
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
        dbg!(&use_);
        add_module_functions(
            mx,
            use_["id"].as_u64().unwrap(),
            parent_path,
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
            access_path: path,
            min_vis: parent_vis,
            file_path: item["span"]["filename"].as_str().unwrap().to_owned(),
        };
        eprintln!(
            "{spaces}ADDING FUNCTION {item_path:?} ({path_kind:?}, {item_vis:?}, {computed_vis:?})"
        );

        match mx.functions.get_mut(&id) {
            Some(f) => {
                f.paths.push(item_path);
            }
            None => {
                let f = Function {
                    id,
                    krate: mx.krate,
                    paths: vec![item_path],
                    name: name.unwrap().to_string(),
                    // def_file: item["span"]["filename"].as_str().unwrap().to_owned(),
                    // should_test: Cell::new(true),
                    // sources: RefCell::new(Vec::new()),
                };
                mx.functions.insert(id, f);
            }
        };
    };
}

struct ModCtx<'a> {
    /// List that we are appending to
    functions: HashMap<u64, Function>,
    index: &'a Value,
    krate: Crate,
}

#[derive(Debug)]
pub struct Function {
    pub krate: Crate,
    pub id: u64,
    pub name: String,
    // pub definition_file: String,
    /// A single definition and any number of `use` paths.
    pub paths: Vec<ItemPath>,
    // pub group: Option<Group>,
    // pub should_test: Cell<bool>,
    // pub sources: RefCell<Vec<String>>,
}

#[derive(Clone, Debug)]
pub struct ItemPath {
    pub kind: PathKind,
    /// Path within the crate.
    pub access_path: String,
    pub min_vis: Visibility,
    pub file_path: String,
}

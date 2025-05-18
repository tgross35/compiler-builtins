// Configuration that is shared between `compiler_builtins` and `builtins_test`.

use std::env;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Target {
    pub triple: String,
    pub triple_split: Vec<String>,
    pub opt_level: String,
    pub cargo_features: Vec<String>,
    pub os: String,
    pub arch: String,
    pub vendor: String,
    pub env: String,
    pub pointer_width: u8,
    pub little_endian: bool,
    pub features: Vec<String>,
}

impl Target {
    pub fn from_env() -> Self {
        let triple = env::var("TARGET").unwrap();
        let triple_split = triple.split('-').map(ToOwned::to_owned).collect();
        let little_endian = match env::var("CARGO_CFG_TARGET_ENDIAN").unwrap().as_str() {
            "little" => true,
            "big" => false,
            x => panic!("unknown endian {x}"),
        };
        let cargo_features = env::vars()
            .filter_map(|(name, _value)| name.strip_prefix("CARGO_FEATURE_").map(ToOwned::to_owned))
            .map(|s| s.to_lowercase().replace("_", "-"))
            .collect();

        Self {
            triple,
            triple_split,
            os: env::var("CARGO_CFG_TARGET_OS").unwrap(),
            opt_level: env::var("OPT_LEVEL").unwrap(),
            cargo_features,
            arch: env::var("CARGO_CFG_TARGET_ARCH").unwrap(),
            vendor: env::var("CARGO_CFG_TARGET_VENDOR").unwrap(),
            env: env::var("CARGO_CFG_TARGET_ENV").unwrap(),
            pointer_width: env::var("CARGO_CFG_TARGET_POINTER_WIDTH")
                .unwrap()
                .parse()
                .unwrap(),
            little_endian,
            features: env::var("CARGO_CFG_TARGET_FEATURE")
                .unwrap_or_default()
                .split(",")
                .map(ToOwned::to_owned)
                .collect(),
        }
    }

    #[allow(dead_code)]
    pub fn has_feature(&self, feature: &str) -> bool {
        self.features.iter().any(|f| f == feature)
    }
}

pub fn configure_aliases(target: &Target) {
    // To compile builtins-test-intrinsics for thumb targets, where there is no libc
    println!("cargo::rustc-check-cfg=cfg(thumb)");
    if target.triple_split[0].starts_with("thumb") {
        println!("cargo:rustc-cfg=thumb")
    }

    // compiler-rt `cfg`s away some intrinsics for thumbv6m and thumbv8m.base because
    // these targets do not have full Thumb-2 support but only original Thumb-1.
    // We have to cfg our code accordingly.
    println!("cargo::rustc-check-cfg=cfg(thumb_1)");
    if target.triple_split[0] == "thumbv6m" || target.triple_split[0] == "thumbv8m.base" {
        println!("cargo:rustc-cfg=thumb_1")
    }

    // If the feature is set, disable these types.
    println!("cargo:rustc-check-cfg=cfg(unstable_float)");
    if !env::var_os("CARGO_FEATURE_NO_F16_F128").is_some() {
        println!("cargo:rustc-cfg=unstable_float");
    }
}

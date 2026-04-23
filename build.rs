use std::io::Write as _;
use std::path::PathBuf;

// ── Macro descriptor table ────────────────────────────────────────────────────

struct MacroWrapper {
    name: &'static str,
    ret: &'static str,
    // (c_type, arg_name)
    args: &'static [(&'static str, &'static str)],
}

const ETHERFABRIC_HEADERS: &[&str] = &[
    "etherfabric/ef_vi.h",
    "etherfabric/pd.h",
    "etherfabric/memreg.h",
    "etherfabric/pio.h",
    "etherfabric/checksum.h",
    "etherfabric/capabilities.h",
];

const WRAPPERS: &[MacroWrapper] = &[
    // ── Receive ───────────────────────────────────────────────────────────────
    MacroWrapper {
        name: "ef_vi_receive_init",
        ret: "int",
        args: &[("ef_vi*", "vi"), ("ef_addr", "addr"), ("ef_request_id", "dma_id")],
    },
    MacroWrapper {
        name: "ef_vi_receive_push",
        ret: "void",
        args: &[("ef_vi*", "vi")],
    },
    MacroWrapper {
        name: "ef_vi_receive_set_discards",
        ret: "int",
        args: &[("ef_vi*", "vi"), ("unsigned", "flags")],
    },
    MacroWrapper {
        name: "ef_vi_receive_get_discards",
        ret: "unsigned",
        args: &[("ef_vi*", "vi")],
    },
    // ── Transmit ──────────────────────────────────────────────────────────────
    MacroWrapper {
        name: "ef_vi_transmit",
        ret: "int",
        args: &[
            ("ef_vi*", "vi"),
            ("ef_addr", "base"),
            ("int", "len"),
            ("ef_request_id", "dma_id"),
        ],
    },
    MacroWrapper {
        name: "ef_vi_transmitv",
        ret: "int",
        args: &[
            ("ef_vi*", "vi"),
            ("const ef_iovec*", "iov"),
            ("int", "iov_len"),
            ("ef_request_id", "dma_id"),
        ],
    },
    MacroWrapper {
        name: "ef_vi_transmitv_init",
        ret: "int",
        args: &[
            ("ef_vi*", "vi"),
            ("const ef_iovec*", "iov"),
            ("int", "iov_len"),
            ("ef_request_id", "dma_id"),
        ],
    },
    MacroWrapper {
        name: "ef_vi_transmit_push",
        ret: "void",
        args: &[("ef_vi*", "vi")],
    },
    MacroWrapper {
        name: "ef_vi_transmit_pio",
        ret: "int",
        args: &[
            ("ef_vi*", "vi"),
            ("int", "offset"),
            ("int", "len"),
            ("ef_request_id", "dma_id"),
        ],
    },
    MacroWrapper {
        name: "ef_vi_transmit_copy_pio",
        ret: "int",
        args: &[
            ("ef_vi*", "vi"),
            ("int", "pio_offset"),
            ("const void*", "src"),
            ("int", "len"),
            ("ef_request_id", "dma_id"),
        ],
    },
    MacroWrapper {
        name: "ef_vi_transmit_pio_warm",
        ret: "void",
        args: &[("ef_vi*", "vi")],
    },
    MacroWrapper {
        name: "ef_vi_transmit_copy_pio_warm",
        ret: "void",
        args: &[
            ("ef_vi*", "vi"),
            ("int", "pio_offset"),
            ("const void*", "src"),
            ("int", "len"),
        ],
    },
    MacroWrapper {
        name: "ef_vi_transmit_alt_select",
        ret: "int",
        args: &[("ef_vi*", "vi"), ("unsigned", "alt_id")],
    },
    MacroWrapper {
        name: "ef_vi_transmit_alt_select_normal",
        ret: "int",
        args: &[("ef_vi*", "vi")],
    },
    MacroWrapper {
        name: "ef_vi_transmit_alt_stop",
        ret: "int",
        args: &[("ef_vi*", "vi"), ("unsigned", "alt_id")],
    },
    // ── EF_EVENT_* accessors (ef_event passed by value, ~16 bytes) ────────────
    MacroWrapper { name: "EF_EVENT_TYPE",                        ret: "unsigned", args: &[("ef_event", "e")] },
    MacroWrapper { name: "EF_EVENT_RX_BYTES",                    ret: "unsigned", args: &[("ef_event", "e")] },
    MacroWrapper { name: "EF_EVENT_RX_Q_ID",                     ret: "unsigned", args: &[("ef_event", "e")] },
    MacroWrapper { name: "EF_EVENT_RX_RQ_ID",                    ret: "unsigned", args: &[("ef_event", "e")] },
    MacroWrapper { name: "EF_EVENT_RX_CONT",                     ret: "unsigned", args: &[("ef_event", "e")] },
    MacroWrapper { name: "EF_EVENT_RX_SOP",                      ret: "unsigned", args: &[("ef_event", "e")] },
    MacroWrapper { name: "EF_EVENT_RX_ISCSI_OKAY",               ret: "unsigned", args: &[("ef_event", "e")] },
    MacroWrapper { name: "EF_EVENT_RX_PS_NEXT_BUFFER",           ret: "unsigned", args: &[("ef_event", "e")] },
    MacroWrapper { name: "EF_EVENT_TX_Q_ID",                     ret: "unsigned", args: &[("ef_event", "e")] },
    MacroWrapper { name: "EF_EVENT_TX_CTPIO",                    ret: "unsigned", args: &[("ef_event", "e")] },
    MacroWrapper { name: "EF_EVENT_RX_DISCARD_Q_ID",             ret: "unsigned", args: &[("ef_event", "e")] },
    MacroWrapper { name: "EF_EVENT_RX_DISCARD_RQ_ID",            ret: "unsigned", args: &[("ef_event", "e")] },
    MacroWrapper { name: "EF_EVENT_RX_DISCARD_CONT",             ret: "unsigned", args: &[("ef_event", "e")] },
    MacroWrapper { name: "EF_EVENT_RX_DISCARD_SOP",              ret: "unsigned", args: &[("ef_event", "e")] },
    MacroWrapper { name: "EF_EVENT_RX_DISCARD_TYPE",             ret: "unsigned", args: &[("ef_event", "e")] },
    MacroWrapper { name: "EF_EVENT_RX_DISCARD_BYTES",            ret: "unsigned", args: &[("ef_event", "e")] },
    MacroWrapper { name: "EF_EVENT_RX_MULTI_Q_ID",               ret: "unsigned", args: &[("ef_event", "e")] },
    MacroWrapper { name: "EF_EVENT_RX_MULTI_CONT",               ret: "unsigned", args: &[("ef_event", "e")] },
    MacroWrapper { name: "EF_EVENT_RX_MULTI_SOP",                ret: "unsigned", args: &[("ef_event", "e")] },
    MacroWrapper { name: "EF_EVENT_RX_MULTI_DISCARD_TYPE",       ret: "unsigned", args: &[("ef_event", "e")] },
    MacroWrapper { name: "EF_EVENT_TX_ERROR_Q_ID",               ret: "unsigned", args: &[("ef_event", "e")] },
    MacroWrapper { name: "EF_EVENT_TX_ERROR_TYPE",               ret: "unsigned", args: &[("ef_event", "e")] },
    MacroWrapper { name: "EF_EVENT_TX_WITH_TIMESTAMP_Q_ID",      ret: "unsigned", args: &[("ef_event", "e")] },
    MacroWrapper { name: "EF_EVENT_TX_WITH_TIMESTAMP_RQ_ID",     ret: "unsigned", args: &[("ef_event", "e")] },
    MacroWrapper { name: "EF_EVENT_TX_WITH_TIMESTAMP_SEC",       ret: "unsigned", args: &[("ef_event", "e")] },
    MacroWrapper { name: "EF_EVENT_TX_WITH_TIMESTAMP_NSEC",      ret: "unsigned", args: &[("ef_event", "e")] },
    MacroWrapper { name: "EF_EVENT_TX_WITH_TIMESTAMP_NSEC_FRAC16", ret: "unsigned", args: &[("ef_event", "e")] },
    MacroWrapper { name: "EF_EVENT_TX_WITH_TIMESTAMP_SYNC_FLAGS", ret: "unsigned", args: &[("ef_event", "e")] },
    MacroWrapper { name: "EF_EVENT_TX_ALT_Q_ID",                 ret: "unsigned", args: &[("ef_event", "e")] },
    MacroWrapper { name: "EF_EVENT_TX_ALT_ALT_ID",               ret: "unsigned", args: &[("ef_event", "e")] },
    MacroWrapper { name: "EF_EVENT_RX_NO_DESC_TRUNC_Q_ID",       ret: "unsigned", args: &[("ef_event", "e")] },
    MacroWrapper { name: "EF_EVENT_SW_DATA",                     ret: "unsigned", args: &[("ef_event", "e")] },
];

// ── Code-generation helpers ───────────────────────────────────────────────────

fn arg_list(args: &[(&str, &str)]) -> String {
    if args.is_empty() {
        return "void".to_owned();
    }
    args.iter()
        .map(|(ty, name)| format!("{ty} {name}"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn call_args(args: &[(&str, &str)]) -> String {
    args.iter().map(|(_, name)| *name).collect::<Vec<_>>().join(", ")
}

fn impl_name(name: &str) -> String {
    format!("_impl_{name}")
}

fn generate_header(wrappers: &[MacroWrapper]) -> String {
    let mut out = String::new();
    for h in ETHERFABRIC_HEADERS {
        out.push_str(&format!("#include <{h}>\n"));
    }
    out.push('\n');
    for w in wrappers {
        out.push_str(&format!("#undef {}\n", w.name));
        out.push_str(&format!(
            "extern {ret} {name}({args});\n",
            ret = w.ret,
            name = w.name,
            args = arg_list(w.args),
        ));
    }
    out
}

fn generate_source(wrappers: &[MacroWrapper]) -> String {
    let mut out = String::new();

    for h in ETHERFABRIC_HEADERS {
        out.push_str(&format!("#include <{h}>\n"));
    }

    // Step 1 – static inline helpers (macros still in scope)
    out.push_str("\n/* Step 1: helpers that call macros while they are defined */\n");
    for w in wrappers {
        let body = if w.ret == "void" {
            format!("{{ {}({}); }}", w.name, call_args(w.args))
        } else {
            format!("{{ return {}({}); }}", w.name, call_args(w.args))
        };
        out.push_str(&format!(
            "static inline {ret} {impl_name}({args}) {body}\n",
            ret = w.ret,
            impl_name = impl_name(w.name),
            args = arg_list(w.args),
        ));
    }

    // Step 2 – undef all macros
    out.push_str("\n/* Step 2: undefine all macros */\n");
    for w in wrappers {
        out.push_str(&format!("#undef {}\n", w.name));
    }

    // Step 3 – real linkable functions
    out.push_str("\n/* Step 3: real linkable symbols with original names */\n");
    for w in wrappers {
        let body = if w.ret == "void" {
            format!("{{ {}({}); }}", impl_name(w.name), call_args(w.args))
        } else {
            format!("{{ return {}({}); }}", impl_name(w.name), call_args(w.args))
        };
        out.push_str(&format!(
            "{ret} {name}({args}) {body}\n",
            ret = w.ret,
            name = w.name,
            args = arg_list(w.args),
        ));
    }

    out
}

// ── main ──────────────────────────────────────────────────────────────────────

fn main() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let mut include_paths: Vec<String> = Vec::new();

    // Phase 1 – locate libciul
    match pkg_config::Config::new()
        .probe("onload")
        .or_else(|_| pkg_config::Config::new().probe("ciul1"))
    {
        Ok(lib) => {
            for path in &lib.include_paths {
                include_paths.push(path.display().to_string());
            }
        }
        Err(e) => {
            let onload_root = std::env::var("ONLOAD").ok();
            let lib_dir = std::env::var("ONLOAD_LIB_DIR")
                .ok()
                .or_else(|| onload_root.as_ref().map(|r| format!("{r}/lib")));
            let inc_dir = std::env::var("ONLOAD_INCLUDE_DIR")
                .ok()
                .or_else(|| onload_root.as_ref().map(|r| format!("{r}/include")));

            match lib_dir {
                Some(dir) => {
                    println!("cargo:rustc-link-search=native={dir}");
                    println!("cargo:rustc-link-lib=ciul1");
                    if let Some(inc) = inc_dir {
                        include_paths.push(inc);
                    }
                }
                None => panic!(
                    "Failed to find Onload/libciul: {e}\n\
                     Options:\n\
                     1. Install the system package (apt install libonload-dev / libciul1-dev)\n\
                     2. PKG_CONFIG_PATH=/path/to/pkgconfig\n\
                     3. ONLOAD=/path/to/onload-root\n\
                     4. ONLOAD_LIB_DIR=... ONLOAD_INCLUDE_DIR=..."
                ),
            }
        }
    }

    // Phase 2 – generate C source and header
    let wrapper_h = out_dir.join("wrapper.h");
    let wrapper_c = out_dir.join("wrapper.c");

    std::fs::File::create(&wrapper_h)
        .unwrap()
        .write_all(generate_header(WRAPPERS).as_bytes())
        .unwrap();

    std::fs::File::create(&wrapper_c)
        .unwrap()
        .write_all(generate_source(WRAPPERS).as_bytes())
        .unwrap();

    // Phase 3 – compile wrapper.c
    let mut cc_build = cc::Build::new();
    cc_build.file(&wrapper_c);
    for p in &include_paths {
        cc_build.include(p);
    }
    cc_build.compile("efvi_wrapper");

    // Phase 4 – run bindgen on wrapper.h
    let mut builder = bindgen::Builder::default()
        .header(wrapper_h.to_str().unwrap())
        .derive_default(true)
        .derive_debug(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));
    for p in &include_paths {
        builder = builder.clang_arg(format!("-I{p}"));
    }
    builder
        .generate()
        .expect("bindgen failed to generate bindings")
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("failed to write bindings.rs");
}

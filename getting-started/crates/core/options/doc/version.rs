use std::fmt::Write;

/// Generates a short version string of the form `gs x.y.z`.
pub(crate) fn generate_short() -> String {
    let digits = generate_digits();
    format!("gs {digits}")
}

pub(crate) fn generate_long() -> String {
    let (compile, runtime) = (compile_cpu_features(), runtime_cpu_features());

    let mut out = String::new();
    writeln!(out, "{}", generate_short()).unwrap();
    writeln!(out).unwrap();
    writeln!(out, "features:{}", features().join(",")).unwrap();
    if !compile.is_empty() {
        writeln!(out, "simd(compile):{}", compile.join(",")).unwrap();
    }
    if !runtime.is_empty() {
        writeln!(out, "simd(runtime):{}", runtime.join(",")).unwrap();
    }
    out
}

pub(crate) fn generate_digits() -> String {
    let semver = option_env!("CARGO_PKG_VERSION").unwrap_or("N/A");
    semver.to_string()
    // match option_env!("RIPGREP_BUILD_GIT_HASH") {
    //     None => semver.to_string(),
    //     Some(hash) => format!("{semver} (rev {hash})"),
    // }
}

fn compile_cpu_features() -> Vec<String> {
    #[cfg(target_arch = "x86_64")]
    {
        let mut features = vec![];

        let sse2 = cfg!(target_feature = "sse2");
        features.push(format!("{sign}SSE2", sign = sign(sse2)));

        let ssse3 = cfg!(target_feature = "ssse3");
        features.push(format!("{sign}SSSE3", sign = sign(ssse3)));

        let avx2 = cfg!(target_feature = "avx2");
        features.push(format!("{sign}AVX2", sign = sign(avx2)));

        features
    }
}

fn runtime_cpu_features() -> Vec<String> {
    #[cfg(target_arch = "x86_64")]
    {
        let mut features = vec![];

        let sse2 = is_x86_feature_detected!("sse2");
        features.push(format!("{sign}SSE2", sign = sign(sse2)));

        let ssse3 = is_x86_feature_detected!("ssse3");
        features.push(format!("{sign}SSSE3", sign = sign(ssse3)));

        let avx2 = is_x86_feature_detected!("avx2");
        features.push(format!("{sign}AVX2", sign = sign(avx2)));

        features
    }
}

/// Returns a list of "features" supported (or not) by this build of ripgrpe.
fn features() -> Vec<String> {
    let mut features = vec![];

    let pcre2 = cfg!(feature = "pcre2");
    features.push(format!("{sign}pcre2", sign = sign(pcre2)));

    features
}

/// Returns `+` when `enabled` is `true` and `-` otherwise.
fn sign(enabled: bool) -> &'static str {
    if enabled {
        "+"
    } else {
        "-"
    }
}
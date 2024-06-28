# Rust开发环境安装

## Rust安装

参考[安装Rust](https://www.rust-lang.org/zh-CN/tools/install#/)。

```shell
# 下载 rustup-init.sh 脚本并执行，脚本会安装 rust 工具链
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

安装过程关键信息：

```verilog
Welcome to Rust!

This will download and install the official compiler for the Rust
programming language, and its package manager, Cargo.

Rustup metadata and toolchains will be installed into the Rustup
home directory, located at:

  /Users/arvin/.rustup

This can be modified with the RUSTUP_HOME environment variable.

The Cargo home directory is located at:

  /Users/arvin/.cargo

This can be modified with the CARGO_HOME environment variable.

The cargo, rustc, rustup and other commands will be added to
Cargo's bin directory, located at:

  /Users/arvin/.cargo/bin

This path will then be added to your PATH environment variable by
modifying the profile files located at:

  /Users/arvin/.profile
  /Users/arvin/.zshenv

You can uninstall at any time with rustup self uninstall and
these changes will be reverted.

Current installation options:


   default host triple: x86_64-apple-darwin
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with standard installation (default - just press enter)
2) Customize installation
3) Cancel installation
>

info: profile set to 'default'
info: default host triple is x86_64-apple-darwin
info: syncing channel updates for 'stable-x86_64-apple-darwin'
info: latest update on 2024-06-13, rust version 1.79.0 (129f3b996 2024-06-10)
info: downloading component 'cargo'
info: downloading component 'clippy'
info: downloading component 'rust-docs'
info: downloading component 'rust-std'
info: downloading component 'rustc'
 55.5 MiB /  55.5 MiB (100 %)  39.1 MiB/s in  1s ETA:  0s
info: downloading component 'rustfmt'
info: installing component 'cargo'
info: installing component 'clippy'
info: installing component 'rust-docs'
 15.4 MiB /  15.4 MiB (100 %)   3.9 MiB/s in  3s ETA:  0s
info: installing component 'rust-std'
 23.4 MiB /  23.4 MiB (100 %)  13.9 MiB/s in  1s ETA:  0s
info: installing component 'rustc'
 55.5 MiB /  55.5 MiB (100 %)  14.8 MiB/s in  3s ETA:  0s
info: installing component 'rustfmt'
info: default toolchain set to 'stable-x86_64-apple-darwin'

  stable-x86_64-apple-darwin installed - rustc 1.79.0 (129f3b996 2024-06-10)


Rust is installed now. Great!

To get started you may need to restart your current shell.
This would reload your PATH environment variable to include
Cargo's bin directory ($HOME/.cargo/bin).

To configure your current shell, you need to source
the corresponding env file under $HOME/.cargo.

This is usually done by running one of the following (note the leading DOT):
. "$HOME/.cargo/env"            # For sh/bash/zsh/ash/dash/pdksh
source "$HOME/.cargo/env.fish"  # For fish
```

> rustup 是工具链管理器，cargo 是 rust 包管理器；还创建了两个配置文件用于将 `$HOME/.cargo/bin` 加入 $PATH。

```shell
~ tree -L 5 .rustup
.rustup
├── downloads
├── settings.toml
├── tmp
├── toolchains
│   └── stable-x86_64-apple-darwin # 这个应该就是SDK
│       ├── bin
│       │   ├── cargo
│       │   ├── cargo-clippy
│       │   ├── cargo-fmt
│       │   ├── clippy-driver
│       │   ├── rust-gdb
│       │   ├── rust-gdbgui
│       │   ├── rust-lldb
│       │   ├── rustc
│       │   ├── rustdoc
│       │   └── rustfmt
│       ├── etc
│       │   └── bash_completion.d
│       │       └── cargo
│       ├── lib
│       │   ├── librustc-stable_rt.asan.dylib
│       │   ├── librustc-stable_rt.lsan.dylib
│       │   ├── librustc-stable_rt.tsan.dylib
│       │   ├── librustc_driver-4e5aeb77ba2450e9.dylib
│       │   ├── libstd-85e77511d3e3991b.dylib
│       │   └── rustlib
│       │       ├── components
│       │       ├── etc
│       │       ├── manifest-cargo-x86_64-apple-darwin
│       │       ├── manifest-clippy-preview-x86_64-apple-darwin
│       │       ├── manifest-rust-docs-x86_64-apple-darwin
│       │       ├── manifest-rust-std-x86_64-apple-darwin
│       │       ├── manifest-rustc-x86_64-apple-darwin
│       │       ├── manifest-rustfmt-preview-x86_64-apple-darwin
│       │       ├── multirust-channel-manifest.toml
│       │       ├── multirust-config.toml
│       │       ├── rust-installer-version
│       │       └── x86_64-apple-darwin
│       ├── libexec
│       │   └── rust-analyzer-proc-macro-srv
│       └── share
│           ├── doc
│           │   ├── cargo
│           │   ├── clippy
│           │   ├── rust
│           │   └── rustfmt
│           ├── man
│           │   └── man1
│           └── zsh
│               └── site-functions
└── update-hashes
    └── stable-x86_64-apple-darwin
    
~  tree -L 3 .cargo
.cargo
├── bin		# 和 .rustup/toolchains/stable-x86_64-apple-darwin/bin 下有很多同名文件，但是文件并不相同，todo 区别？
│   ├── cargo
│   ├── cargo-clippy
│   ├── cargo-fmt
│   ├── cargo-miri
│   ├── clippy-driver
│   ├── rls
│   ├── rust-analyzer
│   ├── rust-gdb
│   ├── rust-gdbgui
│   ├── rust-lldb
│   ├── rustc
│   ├── rustdoc
│   ├── rustfmt
│   └── rustup
└── env

~ rustc --version
rustc 1.79.0 (129f3b996 2024-06-10)
~ rustup update
~ rustup self uninstall
```

## Rust IDE

### RustRover

+ 运行/调试配置

  添加程序参数（参考[Cargo run/debug configuration](https://www.jetbrains.com/help/rust/cargo-run-debug-configuration.html)）：格式`[command] [build options] [--] [program arguments]`。

  比如：`run --package getting-started --bin gs -- version`

  > 浏览器搜索竟然没有搜到结果，只能去RustRover官网查，最终查到上面方法。


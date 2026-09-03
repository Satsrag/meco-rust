# meco

[English](README.md) | [简体中文](README.zh-CN.md)

`meco` 用于在 ZVVNMOD、Delehi、MenkShape、MenkLetter 和 Z52 之间转换蒙古文文本。转换核心使用 Rust 编写，并通过 11,492 行测试语料与原始 Java 实现进行了逐字节校验。

`meco-core` crate 同时提供：

- Rust library API；
- 适用于桌面和服务器的 `meco` 命令。

UTN #57 Unicode 输出已经内置到所有平台。转换逻辑是纯 Rust、在进程内完成，因此 CLI、Rust library、Web、移动端和预编译 C 包都直接支持，不需要外部命令、解释器或安装器。

## 在浏览器里试用

**<https://www.satsrag.dev/convert/>** —— 同一个 Rust 核心编译成 WebAssembly，全部在你的浏览器本地运行，
不上传任何内容。

左栏输入，右栏选目标编码，中间那栏是 ZVVNMOD 中间码 —— 转换实际上要经过它，所以转错时看中间这一步
往往就能判断问题出在哪一半。每栏下方列出全部码点，不属于该栏编码的字符会标红，这样「`--from` 选错」
是看得见的，而不是悄悄透传过去。页面上的报问题按钮会带着这些序列直接到本仓库开 issue。

页面由 `crates/meco-wasm/web` 及其 `build.sh` 构建。

## 支持的编码

| CLI 名称 | 说明 | 可作为普通转换源 | 可作为普通转换目标 |
|---|---|---:|---:|
| `zvvnmod` | meco 使用的、偏位置字形的中间格式 | 是 | 是 |
| `delehi` | Delehi Unicode 字母约定 | 是 | 是 |
| `menk_shape` | Menk 位置字形编码 | 是 | 是 |
| `menk_letter` | Menk 字母约定 | 是 | 是 |
| `z52` | Z52/zcode 位置字形编码 | 是 | 是 |
| `utn57` | 按 reviewed UTN #57 mapping 的 Unicode | 是 | 是 |
| `oyun` | 原始 API 保留的类型 | 否 | 否 |

MenkLetter 和 Delehi 使用很多相同的 Unicode 码位，但上下文解释规则不同。`meco` 不会自动猜测源编码。应根据产生文本的应用、输入法、字体系统或数据库字段选择 `--from`。

## 安装命令行工具

### 环境要求

- Rust 1.82 或更新版本；
- `cargo` 已加入 `PATH`。

如果运行 `cargo --version` 找不到命令，可以先通过 [rustup](https://rustup.rs/) 安装 Rust。

### 安装普通 CLI

从 crates.io 安装已经发布的 `meco-core 0.4.0`：

```sh
cargo install meco-core --version 0.4.0 --locked
```

检查安装结果：

```sh
meco --version
meco --help
```

正确的版本输出是：

```text
meco 0.4.0
```

### 直接转换一段文本

```sh
meco translate --from z52 --to menk_shape 'text'
```

建议使用上表中的标准编码名。兼容别名 `menkshape` 和 `menkletter` 也可以使用。

### 从 stdin 读取

省略最后的文本参数时，`meco` 从 stdin 读取 UTF-8：

```sh
printf '%s' 'text' | meco translate --from z52 --to menk_shape
```

也可以处理文件或用于服务器任务：

```sh
meco translate --from z52 --to delehi < input.txt > output.txt
```

`meco` 只向 stdout 写入转换后的 UTF-8 字节，不会自动增加换行。错误写入 stderr，转换失败时返回非零退出状态。

在终端中人工查看时，可以在命令后补一个换行：

```sh
meco translate --from z52 --to delehi 'text'; echo
```

macOS 默认 zsh 有时会在结果末尾显示 `%`。这是 zsh 的“上一条输出没有换行”标记，不属于转换结果。

## 转换到 UTN #57

UTN #57 路径使用 `zvvnmod-utn57 0.1.0` 中 reviewed 的 ZVVNMOD → positioned written unit mapping，以及固定版本的纯 Rust 归一化库 `mongol-norm 0.1.1`。两者都已编译进 `meco`，不需要额外安装任何东西。

```sh
meco translate --from z52 --to utn57 'ᡳᡬᡦ ᢌᡭᡪᢊᡱᡱᡭᢐ ᢋᡭᡬᢎᡭᡧ'; echo
```

其他已支持的源编码也可以输出 UTN #57：

```sh
meco translate --from menk_letter --to utn57 '...'
meco translate --from delehi --to utn57 '...'
meco translate --from menk_shape --to utn57 '...'
meco translate --from zvvnmod --to utn57 '...'
```

UTN #57 现在也能反向读回，可以像其他编码一样用作 `--from`：

```sh
meco translate --from utn57 --to delehi '...'
meco translate --from utn57 --to z52 '...'
```

反向还不是严格的逆运算。在 `crates/meco-core/tests/golden/corpus_delehi.txt` 的 1,508 词语料上，
排除正向本就会丢弃的控制字符后，1,053 个词里有 1,020 个经 `zvvnmod → utn57 → zvvnmod` 原样返回，
33 个不返回，多数是多了或少了一个字形码。`tests/utn57.rs` 把这两个数字钉住了，缺口扩大会立刻失败。
往返结果要紧时请保留原文。

### UTN #57 常见问题

#### `conversion not supported for Utn57`

旧版 `meco`。当前版本的 UTN #57 可读可写，两个方向都不需要 `utn57-command` feature 或外部 backend。升级即可。

#### `UTN #57 conversion failed: ...`

进程内的 backend 拒绝了这段文本（正向是 ZVVNMOD 中间码，反向是 UTN #57 输入）。先确认 `--from` 是文本真实的源编码；如果源编码无误，请把输入和错误信息一起反馈。

#### 输出中出现 FVS、MVS 或 ZWJ

这是正常现象。UTN #57 序列化使用标准 Unicode 蒙古文字母和格式控制符来指定 written form。判断结果时应检查精确 code point，不应只依赖某一种字体的显示效果。

#### MenkLetter 和 Delehi 的结果不同

它们是不同的源编码约定，只是都使用 Unicode 蒙古文字母。先确认文本来自哪个输入法、应用或数据库字段，不要根据视觉效果随意切换 `--from`。

## 作为 Rust library 使用

添加默认的纯 Rust library：

```sh
cargo add meco-core@0.4.0
```

也可以直接修改 `Cargo.toml`：

```toml
[dependencies]
meco-core = "0.4.0"
```

调用转换 API：

```rust
use meco_core::{translate, CodeType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = "text";
    let output = translate(CodeType::Z52, CodeType::MenkShape, input)?;
    print!("{output}");
    Ok(())
}
```

library 不依赖外部 runtime，也不会启动外部命令，并且可以编译到 `wasm32-unknown-unknown`。

### 在 Rust library 中输出 UTN #57

```rust
use meco_core::{translate, CodeType};

let output = translate(CodeType::MenkLetter, CodeType::Utn57, input)?;
```

不需要任何 Cargo feature。`utn57-command` 这个 feature 名仍然保留为已废弃的空操作，以便旧的构建命令继续可用。

## 预编译 Release 包

从 [v0.4.0 Release](https://github.com/Satsrag/meco-rust/releases/tag/v0.4.0) 下载。

| 平台 | Release asset |
|---|---|
| Linux x86_64 C ABI | `meco-c-linux-x86_64.zip` |
| Linux AArch64 C ABI | `meco-c-linux-aarch64.zip` |
| macOS Apple Silicon C ABI | `meco-c-macos-arm64.zip` |
| macOS Intel C ABI | `meco-c-macos-x86_64.zip` |
| Windows x86_64 C ABI | `meco-c-windows-x86_64.zip` |
| iOS Swift | `MecoSwift.xcframework.zip` |
| Apple C ABI | `MecoC.xcframework.zip` |
| Android | `meco-android-release.aar` |
| 浏览器/WebAssembly | `meco-wasm-web-0.4.0.tgz` |
| Node.js/WebAssembly | `meco-wasm-nodejs-0.4.0.tgz` |

C 压缩包包含对应平台的 header、静态库和动态库。Go、Python、PHP、Java、Dart 等运行时可以加载 C ABI。Swift、Android、浏览器和 Node.js 使用各自的专用包。

C、C++、Go、Python、Dart、Java、Android、Swift、Objective-C、浏览器、Node.js 和 PHP 的示例见 [USAGE.md](USAGE.md)。

所有预编译包都包含 UTN #57 输出，以及 ZVVNMOD、Delehi、MenkShape、MenkLetter 和 Z52 之间的普通转换。

## 转换模型

普通转换以 ZVVNMOD 为中间层：

```text
源编码
→ 对应的 letter 或 shape decoder
→ ZVVNMOD
→ 目标编码的 letter 或 shape encoder
→ 目标文本
```

UTN #57 输出增加两个 reviewed 阶段，都链接在同一个二进制里：

```text
源编码
→ meco-core
→ ZVVNMOD positioned shapes
→ zvvnmod-utn57 0.1.0 positioned written units
→ mongol-norm 0.1.1（纯 Rust，进程内）
→ Unicode 字母和格式控制符
```

MenkLetter 和 Delehi 是字母级源约定；MenkShape 和 Z52 偏位置字形。位置字形编码不一定保存了恢复唯一音位拼写所需的信息，因此从 Z52、MenkShape 或 ZVVNMOD 得到的 UTN #57 输出是 reviewed、保留字形关系的 Unicode 序列化，不是词典或拼写恢复工具。

## 数据安全与往返转换

迁移语料时应保留原文。转换可能规范化 FVS/MVS 序列、把多个旧写法合并为一个目标写法，或丢失源编码特有的边界信息。UTN #57 两个方向都能转，但往返尚未无损 —— 具体数字见上文。

建议至少保留：

```text
raw_source      原始文本及其明确的源编码
normalized      转换得到的 Unicode/UTN #57 派生文本
search_text     拉丁转写或其他用于检索的表示
```

不能只根据 code point 范围自动区分 MenkLetter 和 Delehi。数据库应同时保存源编码类型。

## 构建与测试

克隆仓库并运行测试：

```sh
git clone https://github.com/Satsrag/meco-rust.git
cd meco-rust
cargo test --workspace --locked
```

构建 CLI：

```sh
cargo build -p meco-core --bin meco --release --locked
```

普通转换矩阵已通过 11,492 行 golden corpus 与原始 Java meco 实现逐字节比较。

## 仓库结构

```text
crates/meco-core      Rust library 和 meco CLI
crates/meco-cabi      C ABI
crates/meco-uniffi    Swift/Kotlin bindings
crates/meco-wasm      浏览器和 Node.js WebAssembly
bindings/             各平台打包配置
.github/workflows/    CI 和发布自动化
```

## 相关文档

- [English README](README.md)
- [各平台调用示例](USAGE.md)
- [分发与发布流程](DISTRIBUTION.md)
- [crates.io 上的 `meco-core`](https://crates.io/crates/meco-core)
- [`meco-core` API 文档](https://docs.rs/meco-core)
- [GitHub Releases](https://github.com/Satsrag/meco-rust/releases)

## 许可证

Apache-2.0。本项目是 Java [east-mod/meco](https://github.com/east-mod/meco) 实现的 Rust 移植版本。

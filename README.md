<div align="center">
  
  # ✨KiraFramework✨
  
  <h3 align="center">Written in Rust, an implementation of OneBot 11 based on the ECS architecture</h3>

  <a href="https://github.com/botuniverse/onebot-11"><img src="https://img.shields.io/badge/OneBot-11-black" alt="OneBot"></a>
  <a href="https://crates.io/crates/kira_framework"><img src="https://img.shields.io/crates/v/kira_framework" alt="crates.io"></a>
  <a href="https://github.com/YouZiSoftware/KiraFramework/actions"><img src="https://github.com/YouZiSoftware/KiraFramework/actions/workflows/rust.yml/badge.svg" alt="Build"/></a>

  ### English | [简体中文(中国大陆)](README_zh_CN.md) | [正體中文(台灣地區)](README_zh_TW.md) | [Русский](README_ru.md)
</div>

## 🎉Introduction🎉
### KiraFramework is a OneBot 11 implementation written in Rust language.
KiraFramework is a OneBot 11 implementation developed using the Rust language, based on the Bevy ECS architecture. You can easily write bots for various instant messaging applications with KiraFramework. A multitude of macros allows your code to be more concise and efficient.
> [!IMPORTANT]
> Please be aware that this project is currently in a very early stage of development and may contain numerous **unknown bugs**, **unresolved security vulnerabilities**, and **many features that have yet to be implemented**.
> 
> Furthermore, **there is no official release version of the project** at this time, so all beta versions will be built using **Github Actions**.
> 
> We strongly advise against using this project in a **production environment**, even if certain parts may appear to be complete, until an official release has been issued.
## 🎶Features🎶
- **Modularity**: The meticulously designed modularity of KiraFramework allows developers to easily modify the code.

- **ECS architecture**: The ECS architecture allows you to find a more efficient way to manage bots, while components between various plugins can be shared, reducing unnecessary hassles.

- **Concise and Efficient Macros**: A multitude of concise and efficient macros can reduce the number of lines in your code, while minimizing unnecessary repetitive code, making your code writing more efficient.

## 🎆Getting Started🎆
#### KiraFramework is written in Rust language, so you need to use Cargo to build it.
[Download Rust](https://www.rust-lang.org/en-US/learn/get-started)

#### Add crates to your project:
```toml
[dependencies]
kira_framework = "0.2.5"
kira_framework_proc = "0.2.5"
```

### Then enjoy!

## ✨Kira QQBot✨
### Kira QQBot is a crate developed using KiraFramework.
#### You can use it to easily create your own QQ bot.

#### Add crate to your project:
```toml
[dependencies]
kira_qqbot = "0.2.5"
```

## [📕Examples📕](https://github.com/YouZiSoftware/KiraFramework/tree/main/examples)

## 👉Feedback👈
#### We need your help in reporting any bugs or vulnerabilities that you encounter, and we also welcome any suggestions you may have.

[Issues page](https://github.com/YouZiSoftware/KiraFramework/issues)

## 📄License📄

**Copyright 2024 YouZikua, all rights reserved.**

If not otherwise specified, project content is open source under the Apache-2.0 license.



<div align="center">

  <h1>Balpan CLI</h1>
  <h5>오픈소스 생태계에 기여하고자 하는 사람들의 온보딩을 돕는 "발판"</h5>
  <h6>오픈소스 프로젝트의 가독성을 높이고, 누구나 기여할 수 있도록 하자</h6>

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
![Work In Progress](https://img.shields.io/badge/Work%20In%20Progress-orange?style=for-the-badge)

</div>

## Table of Contents

- [Introduction](#introduction)
- [Installation](#installation)
  - [Requirements](#requirements)
  - [Install using homebrew](#brew)
  - [Install using cargo](#cargo)
  - [Quickstart](#quickstart)
- [Features](#features)
  - [Supported Language](#supported-languages)
  - [`balpan init`](#balpan-init)

## Introduction<a name="introduction"></a>

**balpan**은 오픈소스 생태계에 기여하고자 하는 사람들의 온보딩을 돕는 **발판** 이라는 의미로 시작했습니다.
**balpan**은 소스코드를 트리 구조로 분석하여 treesitter 기반으로 생성된 파서를 이용합니다.

## Installation<a name="installation"></a>

### Requirements<a name="requirements"></a>

- OS: Linux/macOS
- Cargo (cargo를 이용해서 설치하는 경우)

### Install using homebrew<a name="brew"></a>

```bash
$ brew install malkoG/x/balpan
```
* ⚠️ 당장은 homebrew brew를 이용해서 설치하는 경우 `0.1.1` 릴리즈만 설치될 수 있습니다.
  * 릴리즈를 출시할때마다 homebrew에 배포하는 과정을 자동화하는 방법은 알아보고 있는 중입니다.

### Install using cargo<a name="cargo"></a>

```bash
$ cargo install --path .
```

### Quickstart<a name="quickstart"></a>

**balpan**의 모든 명령어들은 소스코드를 트리구조의 형태로 분석하기 위해 treesitter 기반으로 생성된 파서를 이용합니다.  
**balpan**의 각 명령어를 사용하기 전에 분석하고자 하는 리포지토리의 홈 디텍토리에서 아래의 명령어를 실행해주세요.

```bash
$ balpan init
```

## Features<a name="features"></a>

### Supported Languages<a name="supported-languages"></a>

- Rust 
- Python
- Ruby

### `balpan init`<a name="balpan-init"></a>

![balpan init demo animation](./assets/balpan-init-demo.gif)

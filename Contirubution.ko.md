## Contributing to Balpan CLI

Balpan CLI 기여에 관심을 가져주셔서 정말 감사합니다.

개발에 대한 밀도있는 논의는 당분간은 디스코드에서 진행되고 있습니다. 디스코드는 Pull Request가 병합된 분들을 중심으로 초대하고 있습니다. 프로젝트가 좀 더 안정화되면 추후에 디스코드 초대장 링크를 오픈할 예정입니다.

가능하다면 투명성을 위해 Issue로 남기는것을 권장합니다. 디스코드에서 반복적으로 언급이 되었거나, 장기적으로 생각해봐야 하는 문제인 경우 이슈트래커에 파일링하고 있습니다.

### Concepts

Balpan CLI는 오픈소스 프로젝트 코드분석의 접근성을 낮추기 위한 목적으로 만들어졌습니다. 그걸 가능하게 해주는 핵심적인 구성요소가 바로 [tree-sitter](https://tree-sitter.github.io/) 입니다.

tree-sitter라는 파서 제네레이터를 기반으로 다양한 언어별로 파서가 구현되어 있습니다. 수십가지의 다양한 언어별로 파서 구현체가 있으며, tree-sitter가 지원하는 언어라면 마크업 언어를 제외한 모든 언어를 지원하는 것을 목표로 합니다.

tree-sitter는 소스코드를 파싱하여 Tree 형태의 자료구조로 구성합니다. Tree를 구성하는 각각의 노드는 소스코드를 구성하는 구문이 시작하고 끝나는 범위를 나타내는 메타데이터를 포함하고 있습니다. (WIP)


### Adding language support

2023-09-02 기준으로는 Rust/Python/Ruby/C++/C 만 지원합니다. 추가적으로 지원하고 싶은 언어가 있다면, 아래에서 설명한 명령어의 동작 원리를 참고해주세요.

* `balpan analyze`

> 자세한 구현은 [analyzer.rs](https://github.com/balpan-rs/balpan/blob/main/src/analyzer.rs) 를 참고해주세요.

1. Syntax tree의 루트 노드에서부터 시작해서 너비 우선 탐색(BFS)을 합니다.  [관련 코드](https://github.com/balpan-rs/balpan/blob/main/src/analyzer.rs#L214-L281)
2. 너비 우선 탐색을 진행하면서 모듈/함수/메서드/struct 등 의미를 가지는 노드의 리스트를 구성합니다. 
3. 소스코드를 위에서부터 아래로 순차적으로 읽어들이면서, 2에서 확보한 노드의 범위에 포함된다면 노드가 나타내는 범위의 상단에 TODO 코멘트를 삽입합니다.

* `balpan init`

Balpan CLI 실행에 필요한 tree-sitter 파서를 전부 빌드한 다음, `balpan analyze` 커맨드를 실행합니다.

* `balpan toggle`

(추후 작성 예정입니다.)

---

`balpan grep` 명령어는 언어 지원 여부와는 독립적으로 동작하니 별도로 언급하지는 않겠습니다.

#### Writing tests

Balpan CLI를 구성하는 일부 명령어는 integration test가 반드시 필요합니다. 실제로 동작하는 오픈소스 프로젝트를 분석하는데 있어서 무결성이 보장되어야 하기 때문입니다.

예를 들면, `balpan grep` 명령어는 `balpan analyze` 명령어가 잘 동작한다는 전제하에 소스코드를 구성하는 일부 패턴이 잘 탐색되면 됩니다.

하지만, `balpan analyze` 명령어나 `balpan toggle` 명령어는 각각의 언어마다 tree-sitter 파서 구현체가 해석하는 방식이 다르기 때문에 각 언어마다 다른 integration test를 작성하여야 합니다. 



#### 테스트 작성하기


```sh
cargo test
```

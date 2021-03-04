# Pusher
Rust로 작성한 CLI 소코반\
[IJEMIN/Pusher](https://github.com/IJEMIN/Pusher)에서 영감받았습니다.
## 사용 방법 Usage
`pusher <STAGE_FILENAME>` \
**명령줄 인수 Arguments**:
> 기본 default
* STAGE_FILENAME: \
Path to Stage file. \
*default*: "stage.data"
> Features only: argparser 
* --default-stage:\
Shows default stage.\
`pusher --default-stage > stage.data`
* --help:\
Shows help message.\
`pusher --help`
## 조작 방법 How to Play
>빌드시 features=tui를 사용하지 않은 경우,\
>명령어를 입력하려면 반드시 엔터를 쳐야합니다.\
>Must press 'Enter/Return' to input without 'tui' features. 
* <kbd>W</kbd><kbd>A</kbd><kbd>S</kbd><kbd>D</kbd>
/
<kbd>H</kbd><kbd>J</kbd><kbd>K</kbd><kbd>L</kbd>(vi style): 이동 Move
* <kbd>Q</kbd>/<kbd>Esc</kbd>(TUI Only): 끝내기 Quit
## 스테이지 형식 Stage Format
### 타일 Tile
 * `#`:벽  Wall
 * `+`:목표 Goal \*
 * `.`:빈 통로 Empty
 * `O`:공 Ball
 * `@`:플레이어 Player
 
 \* 공이 목표에 들어가면 `$`가 됩니다.
 `$`는 스테이지 파일에 사용할 수 없습니다.\
  `$` represents 'Ball on goal'. `$` cannot be used in stage file.
### 조건 Conditions
0. 타일들과 줄바꿈 문자만 사용해야 합니다.\
Stage file uses *tiles* and *new line* only. 
1. 스테이지의 가로 세로 길이는 1 이상이어야 합니다.\
Width and height of stage cannot be 0.
2. 스테이지는 가로 길이가 일정한 직사각형이이야 합니다.\
Shape of stage must be rectangle.
3. 공의 수와 목표의 수는 같아야 합니다.\
The number of balls must always same as the number of goals.
4. 플레이어 수는 오직 하나여야 합니다.\
The number of player must be only one.
### 예시 Example
```
########
#+####+#
#O##.O.#
#..@...#
########
```
stage.data
## 빌드 방법 How to bulid
### 실행파일 Binary:
```sh
cargo build # without argument parser
cargo build --features tui # same as above with terminal ui
cargo build --bin cli --features argparser # with argument parser
cargo build --bin cli --features argparser,tui # same as above with terminal ui
```
**Features**
* argparser: 명령줄 인수 파서와 몇몇 명령줄 인수를 추가합니다.\
Add argument parser and optional arguments.
* tui: Terminal UI with [crossterm](https://github.com/crossterm-rs/crossterm)
### 문서 Docs:
```sh
cargo doc --open
``` 
## LICENSE
See [LICENSE.md](https://github.com/km19809/Pusher/blob/master/LICENSE.md)
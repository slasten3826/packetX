# Матрица Wayland -> X12

## Назначение

Этот документ проверяет гипотезу, что `Wayland compositor` можно понизить
из центра истины уровня `table` до функции уровня `manifest`.

Цель пока не в реализации.
Цель в архитектурной диагностике.

## Легенда статусов

- `clean`
  Обязанность можно переназначить в другой слой без крупного структурного конфликта.

- `boundary`
  Обязанность по природе живет на границе слоев.
  Ее не надо насильно запихивать в один чистый слой.

- `collapsed`
  Похоже, что `Wayland` держит эту обязанность внутри compositor-core потому,
  что там уже схлопнуто сразу несколько слоев в один operational center.

- `new primitive needed`
  `X12` не сможет решить это простым переносом обязанности.
  Нужна новая базовая сущность.

## Матрица

| Обязанность | Что делает | Где живет в Wayland | Куда хотим положить в X12 | Статус | Почему |
|---|---|---|---|---|---|
| display / registry / globals | публикует core objects и capabilities | compositor-core | `table` | `clean` | Это работа отношений и адресуемости, а не финальной сцены. |
| client connection lifecycle | connect, disconnect, object lifetime | compositor-core | `table` | `clean` | Естественно ложится в координацию protocol/runtime. |
| surface creation | создает клиентские drawable-объекты | compositor-core | `table` | `boundary` | Surface начинается как relation, но дальше должна стать form. |
| role assignment | toplevel/popup/cursor и т.д. | compositor + shell | `crystall` | `boundary` | Role стабилизирует form, но еще и влияет на protocol legality. |
| seat enumeration | публикует pointer/keyboard/touch capability | compositor-core | `table` | `clean` | Публикация capability относится к relation space. |
| input ingestion | получает сигналы evdev/libinput | compositor + kernel boundary | граница `chaos` | `boundary` | Это по природе слой substrate. |
| input routing | решает, кто получает pointer/keyboard/touch events | compositor-core | `table` или `crystall` | `collapsed` | Одновременно требует relation graph и знание видимой формы. |
| focus management | истина о keyboard/pointer focus | compositor-core | `crystall` | `collapsed` | Focus зависит не только от relation, но и от активной graphic form. |
| output discovery | monitor discovery, hotplug, capabilities | compositor-core | `table` | `clean` | Наблюдение topology относится к relation work. |
| output placement | раскладывает outputs в некоторую world model | compositor-core | `crystall` | `collapsed` | Wayland рано связывает outputs с global compositor space. |
| scene graph ownership | знает, что видно, в каком порядке и с какими transform | compositor-core | `manifest` или `crystall` | `collapsed` | Это один из главных центров власти в Wayland. |
| window arrangement | tiling, floating, workspace rules | shell/compositor | `crystall` | `clean` | Это слой устойчивой графической формы. |
| surface commit processing | применяет attached state, damage, pending state | compositor-core | граница `table -> crystall` | `boundary` | Commit это акт кристаллизации, а не просто bookkeeping протокола. |
| damage tracking | отслеживает изменившиеся видимые области | compositor-core | `manifest` | `boundary` | Damage имеет смысл только относительно финального показа. |
| composition | собирает финальную видимую сцену | compositor-core | `manifest` | `clean` | Это чистая работа manifestation. |
| frame scheduling | решает repaint/submit/pageflip timing | compositor-core | граница `manifest <-> chaos` | `boundary` | Timing относится к финальному показу, но ограничен ритмом железа. |
| direct scanout | обходит composition ради fullscreen/direct output | compositor-core | `manifest` | `collapsed` | Нужны одновременно form knowledge, output knowledge и hardware eligibility. |
| fullscreen path | выделенный режим финальной fullscreen-реализации | compositor-core | `manifest` | `clean` | Естественный manifestation path. |
| buffer import/export | dmabuf/shm object handoff | compositor-core | многослойная граница | `boundary` | Пересекает substrate, relation и presentation. |
| KMS atomic present | финальный вывод в hardware | compositor-core | граница `chaos` + `manifest` | `boundary` | Это финальная manifestation, упирающаяся в край substrate. |

## Первые выводы

### Чистые переназначения

Эти обязанности хорошо уезжают от compositor-centrality:

- display / registry / globals
- client connection lifecycle
- seat enumeration
- output discovery
- window arrangement
- composition
- fullscreen path

### Граничные обязанности

- surface creation
- role assignment
- input ingestion
- surface commit processing
- damage tracking
- frame scheduling
- buffer import/export
- KMS atomic present

### Схлопнутые обязанности

- input routing
- focus management
- output placement
- scene graph ownership
- direct scanout

## Промежуточный вывод

Похоже, что `Wayland` работает хорошо именно потому, что схлопывает несколько слоев в композиторе:

- relation truth
- form truth
- manifestation truth
- часть границы с substrate

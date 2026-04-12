# X112

`X112` — это прототип моста между `X11`
и будущим `X12`, которого пока не существует.

Этот репозиторий больше не следует
читать как “реализацию `X12`”.

Честное состояние проекта сейчас такое:

- написан ранний `X11`-shaped compatibility runtime
- он умеет принимать часть `X11`-wire протокола
- держит server/session discipline
- умеет собирать software manifest
- умеет сохранять visual proof в `PPM`

Но это:

- не native `X12`
- не готовый display server нового поколения
- и не завершенная новая онтология

Это именно:

**`X112` = X11-to-X12 translator prototype**

## Что здесь есть

Сейчас в репозитории есть:

- Rust-прототип server/runtime spine
- ранний `X11` wire ingress
- session / ownership / multi-client discipline
- software compositing / manifest buffer
- `PPM` visual dump path
- docs, в которых зафиксирована исходная `X12` онтология
  и последующее разделение `X112` / `X12`

## Что здесь нет

Здесь пока нет:

- настоящего native `X12` core
- реального `DRM/KMS` output backend
- input stack
- полноценной `X11` compatibility surface
- finished `X12`

## Статус

Разработка `X12` на текущем этапе остановлена.

Репозиторий сохраняется как:

- исследовательский корпус
- прототип `X112`
- документированное основание
  для возможного будущего возврата к `X12`

## Как это теперь читать

- `X112` — совместимый переводчик из мира `X11`
- `X12` — будущая native система,
  которая пока не начата по-настоящему

Ключевой документ для этого разделения:

- [docs/table/X112_AND_X12_RELATION_V0.md](docs/table/X112_AND_X12_RELATION_V0.md)

## Полезные точки входа

- [docs/table/X112_AND_X12_RELATION_V0.md](docs/table/X112_AND_X12_RELATION_V0.md)
- [docs/table/X12_X11_COMPATIBILITY_BOUNDARY_RULE_V0.md](docs/table/X12_X11_COMPATIBILITY_BOUNDARY_RULE_V0.md)
- [docs/table/X12_PHASE_ROADMAP_V0.md](docs/table/X12_PHASE_ROADMAP_V0.md)
- [docs/table/X12_TEST_BATTERY_CANON_V0.md](docs/table/X12_TEST_BATTERY_CANON_V0.md)

## Быстрый запуск

Список встроенных сценариев:

```bash
cargo run --bin x12-server -- --list-scenarios
```

Прогон сценария с `PPM`-кадрами:

```bash
cargo run --bin x12-server -- --scenario partial-overlap --dump-ppm-dir /tmp/x112-ppm
```

Прогон тестов:

```bash
cargo test
```

# Table

Слой `table` описывает отношения, адресацию и coordination.

Сюда должны ложиться документы про:

- object model
- relation model
- registry / globals / lifecycle
- bridges / translators
- capability exposure
- topology discovery
- protocol semantics

Именно здесь `X12` должен удерживать формальную связность системы,
не превращаясь ни в window manager, ни в compositor.

## Текущий порядок чтения

После большого атомарного packet-insight читать лучше так:

1. `X12_ATOMIC_PACKET_MODEL_V0.md`
2. `X12_PACKET_BIRTH_INVARIANT_V0.md`
3. `X12_OPERATOR_TOPOLOGY_V0.md`
4. `X12_ONTOLOGY_V0.md`

Потом уже старые packet-драфты как `legacy`:

5. `X12_PACKET_V0.md`
6. `X12_PACKET_SPEC_V0.md`
7. `X12_PACKET_WALKTHROUGH_MOUSE_MOVE_V0.md`

## Важная пометка

Старые packet-документы не удаляются.

Но после атомарного поворота их надо читать:

- не как каноничное определение packet
- а как legacy-слой мысли
- и как возможное описание более крупной сущности,
  чем packet-атом

## Следующий practical path

После `Milestone 0` и живого Rust spine
следующий главный документ для кода:

- `X12_X11_WIRE_SUBSET_V0.md`

Именно он теперь закрывает
пустое место между:

- headless scenarios
- и настоящим `Unix socket` ingress

# fast-transit-network-analytics

Систем за анализу графова транзитних мрежа FastTransitNetwork (FTN).

Имплементира три алгоритма за анализу графова великих размера:
- **BFS** (Breadth-First Search) - растојања од изворног чвора
- **WCC** (Weakly Connected Components) - слабо повезане компоненте
- **PageRank** - скор важности чворова

За сваки алгоритам постоји секвенцијална и паралелна имплементација.

## Захтеви

- Rust 1.75+ (препоручено: најновија стабилна верзија)
- Cargo 1.75+

## Инсталација и компајлирање

```bash
# Debug build
cargo build

# Release build (оптимизовано, препоручено за бенчмаркинг)
cargo build --release
```

## Коришћење

### Општа синтакса

```bash
./target/release/fast-transit-network-analytics <ALGORITHM> [OPTIONS]
```

### BFS - претрага у ширину

Рачуна најкраћа растојања (број грана) од изворног чвора до свих осталих.

```bash
# Секвенцијално
./target/release/fast-transit-network-analytics bfs \
    --input edges.txt \
    --source 0 \
    --mode seq \
    --out bfs_result.txt

# Паралелно са 8 нити
./target/release/fast-transit-network-analytics bfs \
    --input edges.txt \
    --source 0 \
    --mode par \
    --threads 8 \
    --out bfs_result.txt
```

**Параметри:**
| Параметар | Опис | Подразумевано |
|-----------|------|---------------|
| `--input` | Путања до улазне датотеке (edge list) | (обавезно) |
| `--source` | ID изворног чвора | (обавезно) |
| `--mode` | Режим извршавања: `seq` или `par` | `seq` |
| `--threads` | Број нити (само за `par` режим) | сви доступни |
| `--out` | Путања до излазне датотеке | (обавезно) |

**Излаз:** Датотека са `n` линија, где `i`-та линија садржи `dist[i]` - растојање од `source` до чвора `i`, или `-1` ако чвор није достижан.

### WCC - слабо повезане компоненте

Проналази слабо повезане компоненте третирајући граф као неусмерени.

```bash
# Секвенцијално
./target/release/fast-transit-network-analytics wcc \
    --input edges.txt \
    --mode seq \
    --out wcc_result.txt

# Паралелно са 8 нити
./target/release/fast-transit-network-analytics wcc \
    --input edges.txt \
    --mode par \
    --threads 8 \
    --out wcc_result.txt
```

**Параметри:**
| Параметар | Опис | Подразумевано |
|-----------|------|---------------|
| `--input` | Путања до улазне датотеке (edge list) | (обавезно) |
| `--mode` | Режим извршавања: `seq` или `par` | `seq` |
| `--threads` | Број нити (само за `par` режим) | сви доступни |
| `--out` | Путања до излазне датотеке | (обавезно) |

**Излаз:** Датотека са `n` линија, где `i`-та линија садржи ID компоненте којој чвор `i` припада.

### PageRank - рангирање чворова

Итеративно рачуна скор важности за сваки чвор.

```bash
# Секвенцијално
./target/release/fast-transit-network-analytics pagerank \
    --input edges.txt \
    --mode seq \
    --alpha 0.85 \
    --iters 50 \
    --eps 1e-10 \
    --out pagerank_result.txt

# Паралелно са 8 нити
./target/release/fast-transit-network-analytics pagerank \
    --input edges.txt \
    --mode par \
    --threads 8 \
    --alpha 0.85 \
    --iters 50 \
    --eps 1e-10 \
    --out pagerank_result.txt
```

**Параметри:**
| Параметар | Опис | Подразумевано |
|-----------|------|---------------|
| `--input` | Путања до улазне датотеке (edge list) | (обавезно) |
| `--mode` | Режим извршавања: `seq` или `par` | `seq` |
| `--threads` | Број нити (само за `par` режим) | сви доступни |
| `--alpha` | Damping factor (фактор пригушења) | `0.85` |
| `--iters` | Максималан број итерација | `50` |
| `--eps` | Праг конвергенције (L1 норма) | `1e-10` |
| `--out` | Путања до излазне датотеке | (обавезно) |

**Излаз:** Датотека са `n` линија, где `i`-та линија садржи `rank[i]` - PageRank скор чвора `i` (реалан број).

## Формат улазних података

Текстуална датотека у формату edge list. Свака линија садржи једну грану:

```
<src> <dst>
```

где су `src` и `dst` ненегативни цели бројеви (ID-јеви чворова). Граф је усмерен. Број чворова се одређује као `max(ID) + 1`.

**Пример:**
```
0 1
1 2
2 0
0 3
```

## Генератор синтетичких графова

Генерише случајни граф по Erdős–Rényi моделу:

```bash
./target/release/gen \
    --nodes 1000 \
    --edges 5000 \
    --seed 42 \
    --model erdos-renyi \
    --out graph.txt
```

**Параметри:**
| Параметар | Опис |
|-----------|------|
| `--nodes` | Број чворова |
| `--edges` | Број грана |
| `--seed` | Seed за генератор случајних бројева |
| `--model` | Модел графа: `erdos-renyi` |
| `--out` | Путања до излазне датотеке |

## Тестирање

```bash
# Покретање свих тестова
cargo test

# Покретање тестова са детаљним излазом
cargo test -- --nocapture
```

Тестови укључују:
- Јединичне тестове за сваки алгоритам
- Поређење секвенцијалне и паралелне верзије
- CLI интеграционе тестове

## Бенчмаркинг

### Генерисање бенчмарк графова и покретање мерења

```bash
# Покретање комплетног бенчмарка
bash bench/run_bench.sh

# Регенерисање графова и покретање бенчмарка
REGENERATE=1 bash bench/run_bench.sh
```

### Параметри бенчмарка

Бенчмарк користи Graph500 конвенцију:
- **Мали граф**: SCALE=18 → 262,144 чворова, 4,194,304 грана
- **Велики граф**: SCALE=20 → 1,048,576 чворова, 16,777,216 грана

### Генерисање графикона

```bash
python3 bench/collect_results.py
```

Резултати се чувају у:
- `bench/results.csv` - сирови подаци
- `bench/figures/` - графикони убрзања и времена извршавања

## Структура пројекта

```
├── src/
│   ├── main.rs              # CLI entrypoint
│   ├── lib.rs               # Библиотека
│   ├── graph/
│   │   └── csr.rs           # CSR репрезентација графа
│   ├── algorithms/
│   │   ├── bfs_seq.rs       # BFS секвенцијално
│   │   ├── bfs_par.rs       # BFS паралелно
│   │   ├── wcc_seq.rs       # WCC секвенцијално
│   │   ├── wcc_par.rs       # WCC паралелно
│   │   ├── pagerank_seq.rs  # PageRank секвенцијално
│   │   └── pagerank_par.rs  # PageRank паралелно
│   ├── generator/
│   │   └── er.rs            # Erdős–Rényi генератор
│   └── bin/
│       └── gen.rs           # CLI за генератор
├── tests/
│   ├── cli.rs               # CLI интеграциони тестови
│   └── compare_seq_par.rs   # Поређење seq/par
├── bench/
│   ├── run_bench.sh         # Скрипта за бенчмаркинг
│   ├── collect_results.py   # Генерисање графикона
│   └── figures/             # Излазни графикони
└── report.tex               # LaTeX извештај
```

## Лиценца

MIT

## Аутор

Марко Кубурић, E2 62/2025

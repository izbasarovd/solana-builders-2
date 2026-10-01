# sol-tools — неделя 3, Rust mini-project

CLI-утилита на Rust с мини-инструментами для Solana: конвертация SOL ↔ lamports, проверка публичного ключа, расчёт базовой комиссии, разбор типа инструкции System Program. Работает локально, без сети. Примеры используют данные транзакции недели 1 (devnet): отправитель `Fo9Q…B3fS`, получатель `3Zrz…gvKJi`, комиссия 5 000 lamports, остаток 4.899995 SOL.

## Запуск

```bash
cargo build --release
./target/release/sol-tools --help
# или без сборки release:
cargo run -- lamports 1.5
```

Проверки: `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (22 теста), `cargo build --release` — всё проходит.

## Команды

| Команда | Что делает |
|---|---|
| `lamports <SOL>` | SOL → lamports, точно, без float (до 9 знаков после точки) |
| `sol <lamports>` | lamports → SOL |
| `validate <pubkey>` | проверка base58 и длины 32 байта |
| `fee <signatures> [--balance <SOL>] [--payer <pubkey>]` | базовая комиссия 5000 lamports × подписи; с `--balance` — остаток после оплаты |
| `parse-ix <тип>` | тип инструкции System Program → enum, индекс, accounts |

## Структура

```
src/
  main.rs         точка входа: разбор аргументов, вывод, exit code
  cli.rs          enum Command (clap derive)
  commands.rs     обработчики команд -> Result<String, ToolError>
  errors.rs       enum ToolError + impl Display
  units.rs        SOL <-> lamports, базовая комиссия (u64, checked-арифметика)
  pubkey.rs       struct Pubkey([u8; 32]), base58
  instruction.rs  enum InstructionType + FromStr
  wallet.rs       struct Wallet: deposit / withdraw / pay_fee
```

## Концепция Rust → где в коде → зачем в Solana

| Концепция Rust | Где в коде | Зачем в Solana |
|---|---|---|
| Типы: u64 для lamports, f64 только для вывода, &str / String | units.rs (sol_to_lamports, lamports_to_sol, format_sol) | Суммы в Solana — целые u64 lamports; float даёт ошибки округления |
| struct и impl | pubkey.rs (Pubkey([u8; 32])), wallet.rs (Wallet) | Аккаунты и состояние программ — структуры с фиксированным размером |
| enum с данными | cli.rs (Command), instruction.rs (InstructionType), errors.rs (ToolError) | Инструкции и ошибки программы — enum (как в Anchor/SPL) |
| Ownership и borrowing | wallet.rs: new(String), into_label(self), &self / &mut self | Изменять состояние можно только через &mut; нет скрытых копий и гонок |
| Result, Option, оператор ? | commands.rs, units.rs, pubkey.rs | Программа возвращает ошибку, а не падает; ошибка откатывает транзакцию |
| checked-арифметика | units.rs: checked_mul/checked_add; wallet.rs: checked_add/checked_sub | Переполнение u64 — баг и риск потери средств, проверять обязательно |
| Трейты Display, FromStr, Error | errors.rs, instruction.rs, pubkey.rs | Привычный способ разбора и вывода типов; те же трейты в Solana SDK |
| Тесты (#[cfg(test)]) | по модулю tests в каждом файле, 22 теста | Программы нельзя «починить» после деплоя; логику проверяют заранее |

## Примеры

```
$ sol-tools lamports 1.5
1.5 SOL = 1 500 000 000 lamports
$ sol-tools sol 4899995000
4 899 995 000 lamports = 4.899995 SOL (f64: 4.899995)
$ sol-tools validate 3ZrzPYSzSm3ARXCq3xtYQtFcfpk4vB4rMvQUsqpgvKJi
OK: корректный публичный ключ, длина 32 байта
3ZrzPYSzSm3ARXCq3xtYQtFcfpk4vB4rMvQUsqpgvKJi
$ sol-tools validate 11111111111111111111111111111111
OK: корректный публичный ключ, длина 32 байта
11111111111111111111111111111111
это System Program
$ sol-tools fee 1
подписей: 1
базовая комиссия: 5 000 lamports = 0.000005 SOL
$ sol-tools fee 3 --balance 4.899995 --payer Fo9Qwfn1DybRm6sPKY5SdwSPKSqA5xDKwFuU7CQpB3fS
подписей: 3
базовая комиссия: 15 000 lamports = 0.000015 SOL
fee payer Fo9Qwfn1DybRm6sPKY5SdwSPKSqA5xDKwFuU7CQpB3fS: 4.899995 SOL -> после комиссии 4.89998 SOL
$ sol-tools parse-ix transfer
Transfer (индекс 2 в System Program)
перевести lamports с одного аккаунта на другой
accounts: from (signer, writable); to (writable)
$ sol-tools lamports 1.0000000001
ошибка: слишком точная сумма: 10 знаков после точки, максимум 9 (1 lamport = 0.000000001 SOL)
$ sol-tools validate abc
ошибка: неверная длина публичного ключа: получено байт — 3, нужно 32
$ sol-tools fee 1 --balance 0.000001
ошибка: недостаточно средств: нужно 5000 lamports, доступно 1000
$ sol-tools parse-ix swap
ошибка: неизвестный тип инструкции 'swap'. Доступно: CreateAccount, Assign, Transfer, CreateAccountWithSeed, AdvanceNonceAccount, Allocate
```

## Результат cargo test

```
test result: ok. 22 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
Все 22 теста: конвертация и ловушка float, переполнение u64, неверный base58 и длина ключа (в том числе 64 байта signature недели 1), нехватка средств, неизвестная инструкция, сценарий недели 1 (5 SOL − 0.1 SOL − 5 000 lamports = 4 899 995 000 lamports).

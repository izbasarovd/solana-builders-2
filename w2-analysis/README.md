# Неделя 2. Как на самом деле работает Solana: анализ транзакций

Скрипт `analyze.py` разбирает транзакцию Solana (только чтение, JSON-RPC `getTransaction`, `jsonParsed`, `maxSupportedTransactionVersion 0`): accounts и их роли, инструкции (включая inner), программы, комиссия (базовая + priority), compute units, изменения SOL- и токен-балансов.

## Запуск
```bash
python3 analyze.py <signature> [mainnet|devnet]
python3 analyze.py --file tx3.json mainnet
python3 make_report_short.py     # пересобрать report_week2.docx
```

## Транзакции (signatures.txt)
| № | Сеть | Что это | Комиссия | CU |
|---|------|---------|----------|----|
| tx0 | devnet | мой перевод 0.1 SOL из недели 1 | 5 000 | 150 |
| tx1 | mainnet | перевод SOL (1000 lamports) | 5 000 | 150 |
| tx2 | mainnet | перевод 11.765859 USDC (Memo, ATA, transferChecked) | 5 294 | 14 662 |
| tx3 | mainnet | swap через Jupiter v6 (SOL → TTWO → banked, Raydium CLMM и LaunchLab) | 11 478 | 181 419 |

Ответы RPC лежат в `tx0.json`–`tx3.json`, разборы в `tx*.analysis.txt`. Mainnet-транзакции перепроверены, ответ RPC совпадает с сохранённым.

## Бонус: tx0 (моя транзакция, devnet)
- Отправитель (fee payer, signer): `Fo9Qwfn1DybRm6sPKY5SdwSPKSqA5xDKwFuU7CQpB3fS`
- Получатель: `3ZrzPYSzSm3ARXCq3xtYQtFcfpk4vB4rMvQUsqpgvKJi`
- Сумма 0.1 SOL (100 000 000 lamports), комиссия 5 000 lamports (одна подпись × 5 000, priority fee нет)
- Баланс отправителя 5 000 000 000 → 4 899 995 000; получателя 0 → 100 000 000
- Slot 506330834, 2026-10-01 16:16:42 UTC
- Signature: `bN4kbDxprwPvZA9HpS4HZ24ip9pUsZAf6tDVpaYhnFCtHh7yZX1tpKzGuzoiAZ1N8LQeKyxtwYSqwp3s12kxVrK`
- https://explorer.solana.com/tx/bN4kbDxprwPvZA9HpS4HZ24ip9pUsZAf6tDVpaYhnFCtHh7yZX1tpKzGuzoiAZ1N8LQeKyxtwYSqwp3s12kxVrK?cluster=devnet

## Выводы
- Транзакция — список инструкций и заранее объявленных аккаунтов с правами signer/writable.
- Комиссия = 5 000 lamports × подписи + priority fee (µlamports/CU × лимит CU).
- Swap тяжелее перевода примерно в 1209 раз по CU (CPI в несколько программ), а комиссия выше лишь в 2.3 раза.
- Реальное движение средств видно по pre/post балансам.

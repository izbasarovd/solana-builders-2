# Неделя 1. Вход в Web3: первая транзакция в Solana

## Что сделано
- Создан новый кошелёк (`keys/main.json`), сеть Devnet.
- Получен тестовый SOL через кран (5 SOL).
- Создан второй keypair-получатель (`keys/second.json`).
- Отправлена первая транзакция: 0.1 SOL с main на second.
- Транзакция разобрана через `solana confirm -v` и JSON-RPC `getTransaction` (ответ в `tx.json`).

## Адреса кошельков (только публичные)
- Отправитель (main): `Fo9Qwfn1DybRm6sPKY5SdwSPKSqA5xDKwFuU7CQpB3fS`
- Получатель (second): `3ZrzPYSzSm3ARXCq3xtYQtFcfpk4vB4rMvQUsqpgvKJi`

## Использованные команды
```bash
solana --version
solana config set --url devnet --keypair ~/solana-builders-2/w1-wallet/keys/main.json
solana config get
solana address
solana balance
solana airdrop 1                      # лимит крана, пополнение через faucet.solana.com
solana-keygen new --no-bip39-passphrase --silent --outfile keys/second.json
solana transfer 3ZrzPYSzSm3ARXCq3xtYQtFcfpk4vB4rMvQUsqpgvKJi 0.1 --allow-unfunded-recipient
solana confirm -v <SIGNATURE>
curl https://api.devnet.solana.com -d '{"jsonrpc":"2.0","id":1,"method":"getTransaction","params":["<SIGNATURE>",{"encoding":"jsonParsed","maxSupportedTransactionVersion":0}]}'
```

## Signature и ссылки
Signature:
`bN4kbDxprwPvZA9HpS4HZ24ip9pUsZAf6tDVpaYhnFCtHh7yZX1tpKzGuzoiAZ1N8LQeKyxtwYSqwp3s12kxVrK`

- Explorer: https://explorer.solana.com/tx/bN4kbDxprwPvZA9HpS4HZ24ip9pUsZAf6tDVpaYhnFCtHh7yZX1tpKzGuzoiAZ1N8LQeKyxtwYSqwp3s12kxVrK?cluster=devnet
- Solscan: https://solscan.io/tx/bN4kbDxprwPvZA9HpS4HZ24ip9pUsZAf6tDVpaYhnFCtHh7yZX1tpKzGuzoiAZ1N8LQeKyxtwYSqwp3s12kxVrK?cluster=devnet

## Разбор транзакции
- **Fee payer и signer:** один подписант, он же плательщик комиссии, это main (`Fo9Q…B3fS`). Подпись одна.
- **Инструкция:** одна, `transfer` программы System Program (`11111111111111111111111111111111`).
- **Отправитель → получатель:** `Fo9Q…B3fS` → `3Zrz…gvKJi`.
- **Сумма:** 0.1 SOL = 100 000 000 lamports (1 SOL = 10⁹ lamports).
- **Комиссия:** 5 000 lamports = 0.000005 SOL. Базовая комиссия Solana — 5 000 lamports за каждую подпись; подпись одна, значит ровно 5 000 (приоритетной комиссии нет).
- **Balances до → после (lamports):**
  - main: 5 000 000 000 → 4 899 995 000 (минус 100 000 000 и 5 000)
  - second: 0 → 100 000 000
- **Slot и время:** slot 506330834, 2026-10-01 16:16:42 UTC (21:16:42 +05:00). Статус Ok, Finalized, 150 compute units.
- **Почему адрес виден на devnet:** блокчейн публичный. Любой может найти аккаунт и транзакцию по адресу или signature в Explorer. Видны адреса, суммы и балансы, личность владельца не видна. Devnet — тестовая сеть, SOL там не имеет стоимости.

## Что я понял
Кошелёк — это пара ключей: публичный ключ (адрес, его можно показывать) и приватный ключ (им подписываются транзакции, его нельзя никому отдавать). Транзакция — подписанный набор инструкций, который сеть исполняет и записывает в блок за небольшую комиссию. Devnet — тестовая сеть с бесплатным SOL, где можно безопасно учиться. Все транзакции публичны и проверяемы любым человеком.

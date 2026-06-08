# Desafio: Sistema de Verificação de Saldo

## Descrição

Você foi contratado como desenvolvedor júnior para um banco digital inovador chamado **ByteBank**. Sua primeira tarefa é criar uma funcionalidade simples, porém essencial: o sistema de verificação de saldo.

Os clientes do ByteBank frequentemente consultam seus saldos para decidir se podem realizar uma compra. Para tornar o processo mais eficiente, o banco deseja um programa que, ao receber o saldo atual da conta e o valor de uma compra, informe se a transação pode ser realizada ou não.

Sua solução será integrada ao aplicativo do banco, ajudando milhares de usuários a tomar decisões rápidas e seguras.

## Tarefa

Implemente um programa que receba dois números inteiros positivos: o **saldo disponível** na conta e o **valor da compra** desejada.

O programa deve verificar se o saldo é suficiente para cobrir a compra:

- Caso seja, exiba a mensagem `"Compra aprovada"`
- Caso contrário, exiba `"Saldo insuficiente"`

**Observações:**

- Não há taxas ou descontos
- O valor da compra nunca será negativo
- O saldo pode ser zero

## Entrada

Dois números inteiros positivos separados por espaço, representando respectivamente o saldo disponível e o valor da compra.

## Saída

Uma string indicando o resultado da verificação:

- `"Compra aprovada"` se o saldo for suficiente
- `"Saldo insuficiente"` caso contrário

## Exemplos

A tabela abaixo apresenta exemplos de entrada e saída:

| Entrada | Saída              |
| ------- | ------------------ |
| 100 50  | Compra aprovada    |
| 30 40   | Saldo insuficiente |
| 0 0     | Compra aprovada    |
| 75 75   | Compra aprovada    |

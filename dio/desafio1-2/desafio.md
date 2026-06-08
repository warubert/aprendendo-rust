# Desafio: Interface de Boas-Vindas Bancária

## Descrição

Você foi contratado como desenvolvedor júnior para a equipe de um banco digital inovador. Seu primeiro desafio é criar uma interface simples para o aplicativo bancário, responsável por exibir mensagens personalizadas de boas-vindas aos clientes.

O objetivo é garantir que cada usuário se sinta valorizado ao acessar o app, mostrando seu **nome** e o **tipo de conta** que possui. Para isso, você deverá processar uma string de entrada contendo o nome do cliente e o tipo de conta, separados por um espaço, e gerar uma mensagem de saudação padronizada.

Este exercício é fundamental para praticar manipulação de strings, leitura de entrada e saída, além de reforçar a importância da experiência do usuário em aplicações financeiras digitais.

## Tarefa

Implemente um programa que leia uma linha contendo o **nome do cliente** e o **tipo de conta** (ambos sem espaços internos), e produza uma mensagem no formato:

```
Welcome, [nome]! Your account type is [tipo].
```

**Requisitos:**

- A mensagem deve seguir exatamente esse padrão, substituindo `[nome]` e `[tipo]` pelos valores fornecidos
- Não utilize bibliotecas externas, apenas recursos padrão da linguagem
- O programa deve funcionar corretamente para qualquer nome e tipo de conta válidos
- Lidar com casos em que a entrada esteja incompleta, exibindo `"Invalid input."` nesses casos

## Entrada

Uma única linha contendo dois valores separados por um espaço: o nome do cliente e o tipo de conta.

Se a entrada não contiver exatamente dois valores, considere-a inválida.

## Saída

Uma única linha com:

- A mensagem de saudação no formato especificado, ou
- `"Invalid input."` caso a entrada seja inválida

## Exemplos

A tabela abaixo apresenta exemplos de entrada e saída:

| Entrada             | Saída                                          |
| ------------------- | ---------------------------------------------- |
| Ana savings         | Welcome, Ana! Your account type is savings.    |
| Lucas checking      | Welcome, Lucas! Your account type is checking. |
| Maria               | Invalid input.                                 |
| Pedro premium extra | Invalid input.                                 |

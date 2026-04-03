# official.ssh

Integracao de controle remoto via SSH.

## Visao geral

Habilita execucao remota de comandos via SSH para workflows do Ralph Engine. Este plugin fornece um provider de controle remoto tipado que permite aos agentes executar comandos em hosts remotos, transferir arquivos e gerenciar ambientes remotos — tudo dentro da seguranca do sistema de plugins do Ralph Engine.

## Como funciona

O plugin SSH expoe um provider de controle remoto (`official.ssh.remote`) que os agentes podem usar para:

- Executar comandos em hosts remotos via SSH
- Transferir arquivos entre maquinas local e remota
- Verificar conectividade e saude do ambiente remoto
- Gerenciar servicos remotos (iniciar/parar/reiniciar)

## Casos de uso

### Integracao com DevContainer

Rode testes, migrations e servidores de desenvolvimento dentro de um DevContainer enquanto o agente edita arquivos localmente. O plugin SSH faz a ponte entre o ambiente local do agente e os servicos no container.

### Desenvolvimento multi-maquina

Para projetos que abrangem varias maquinas (ex.: frontend local, backend em servidor remoto), o plugin SSH permite que o agente opere entre diferentes hosts de forma transparente.

### Workflows de deploy

Execute scripts de deploy em servidores de producao ou staging como parte de um workflow automatizado do Ralph Engine.

## Requisitos

- Cliente SSH instalado na maquina local
- Autenticacao por chave SSH configurada para os hosts de destino
- Um projeto Ralph Engine valido

## Seguranca

Todas as operacoes SSH passam pelo sistema de plugins do Ralph Engine, o que significa que sao auditaveis e podem ser controladas por plugins de politica. O plugin SSH nunca armazena credenciais — ele depende do SSH agent e da configuracao de chaves do seu sistema.

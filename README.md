# Hexvim

**Hexvim** é um editor hexadecimal inspirado no **Vim**, desenvolvido para **engenharia reversa**, análise binária e manipulação direta de bytes, tudo direto do terminal.

Nada de firula, nada de mouse, nada de IDE pesada.  
Aqui é **teclado, buffer e liberdade**.

## 🧠 Filosofia

- Interface modal inspirada no Vim
- Fluxo rápido para análise de arquivos binários
- Feito para quem entende o que está mexendo
- Terminal-first, minimalista e eficiente
- Zero dependência de ambientes gráficos

Se você gosta de Vim, C, ASM e engenharia reversa, você está em casa.

## 🚀 Funcionalidades

- Visualização hexadecimal e ASCII lado a lado
- Navegação por offsets
- Movimentação estilo Vim (`h j k l`)
- Leitura direta de arquivos binários
- Interface 100% em terminal
- Foco em desempenho e simplicidade

> ⚠️ O projeto está em desenvolvimento ativo. Algumas funcionalidades ainda estão sendo expandidas.

## 📦 Requisitos

### Sistema Operacional
- Linux (Arch, Debian, Fedora, Gentoo, Void, etc)

### Ambiente
- Terminal compatível:
  - `kitty`
  - `alacritty`
  - `foot`
  - `xterm`
- Fonte monoespaçada (recomendado):
  - JetBrains Mono
  - Fira Code
  - Iosevka
  - Terminus

## ❌ Não suportado

- ❌ **Windows**
  - Console limitado
  - APIs ruins
  - Experiência inconsistente
- ❌ **Termux / Android**
  - Ambiente capado
  - Problemas de TTY
  - Não é foco do projeto

Hexvim é feito para **Linux**

## 🛠️ Build / Compilação

(ajuste conforme o projeto evoluir)

```sh
git clone https://github.com/seu-usuario/hexvim.git
cd hexvim
make

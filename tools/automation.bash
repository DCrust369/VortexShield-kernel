#!/bin/bash
# direitos autorais DCrust 16/04/2026

echo "Selecione o modo de execução:"

select opcao in "emprestimo_zig" "build_zig" "cargo_run" "zig_run" "sair"; do
    case $opcao in
        "emprestimo_zig") zig run src/main.zig  ;;
        "build_zig")      zig build              ;;
        "cargo_run")      cargo run              ;;
        "zig_run")        zig run src/main.zig  ;;
        "sair")           break                  ;;
        *) echo "Opção inválida"                 ;;
    esac
done

.section .data
    msg:    .ascii "Olá, Kernel!\n"
    len = . - msg

.section .text
.globl _start

_start:
    # syscall write(1, msg, len)
    movq $1, %rax        # número da syscall (write)
    movq $1, %rdi        # fd (stdout)
    leaq msg(%rip), %rsi # ponteiro para mensagem
    movq $len, %rdx      # tamanho
    syscall              # chama o kernel

    # syscall exit(0)
    movq $60, %rax       # número da syscall (exit)
    xorq %rdi, %rdi      # código de retorno 0
    syscall

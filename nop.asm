; nasm -fwin32 nop.asm / nasm -fwin64 nop.asm
; link nop.obj msvcrt.lib ; nop.exe
; nasm -felf32 nop.asm -o nop.o && gcc -m32 nop.o
; nasm -felf64 nop.asm -o nop.o && gcc -m64 nop.o
global main

section .text
main:
    db 0x0f, 0x1f, 0x44, 0x00, 0x00
    db 0x0f, 0x1f, 0x80, 0x00, 0x00, 0x00, 0x00
    db 0x66, 0x90
    db 0x0f, 0x1f, 0x00
    nop
    db 0x66, 0x0f, 0x1f, 0x44, 0x00, 0x00
    db 0x0f, 0x1f, 0x40, 0x00
    db 0xc2, 0x00, 0x00

section     .data
filename:   db      'input.txt', 0h ;input file
fileSize:   dw      10408           ;input file size

section     .bss
fileCont:   resb 10408           ;Reserved bytes for file

; ---------------------------------
; Code Section
section     .text
global      _start
_start:
;-------------------------
; Str openFile(String fileName)
; Puts file contents into reserved memory
; (Get byte size from wc -c)
openFile:
    mov     ecx, 2              ;Read Write access mode
    mov     ebx, filename       ;filename to open
    mov     eax, 5              ;Kernel opcode 5 -> SYS_OPEN
    int     80h                 ;Kernel call

    mov     edx, fileSize       ;number of bytes to be read
    mov     ecx, fileCont       ;Memory address for contents to be read
    mov     ebx, eax
    mov     eax, 3              ;Kernel opcode 3 -> SYS_READ
    int     80h                 ;Kernel call

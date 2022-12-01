section     .data
    ;; File vars
filename:   db      'input.txt', 0h ;input file
fileSize:   dw      10408

    ;; Integer vars
max:        db      0           ;max calories
currentSum: db      0           ;current sum of calories

    ;; String vars
lineSize:   db      0           ;size of line
filePtr:    db      0           ;pointer for file
linePtr:    db      0           ;pointer of current line

section     .bss
lineCont:   resb 10            ;Reserved bytes for lines
fileCont:   resb 10408          ;Reserved bytes for file

; ---------------------------------
;
; Code Section
section     .text
global      _start
_start:
    call    openFile            ;Get file contents into reserved memory
    mov     ecx, fileCont       ;put pointer for fileCont into ecx
    xor     eax, eax                ;inbetween iterations

getLength:
    jmp    .buildInt

.endLineCheck:
    add     ecx, 1              ;iterate forward one char
    mov     edx, [linePtr]      ;VV
    add     edx, 1              ;linePtr += 1
    mov     [linePtr], edx      ;^^
    cmp     [ecx], byte 0Ah     ;if 0 (newLine) is the character
    jnz     getLength           ;jump to getLength

    mov     [filePtr], ecx      ;set ptr to end of current line


.buildInt:
    mov     bl, byte [ecx]     ;put current byte into bl
    sub     bl, '0'            ;convert ASCII to int
    imul    eax, 10
    add     eax, ebx
    jmp .endLineCheck
;--------------------------------
;Exits gracefully
quit:
    mov     rbx, 0              ;Sys_call to end program
    mov     rax, 1
    int     80h



;-------------------------
; Str openFile(String fileName)
; Puts file contents into reserved memory
; (Get byte size from wc -c)
openFile:
    mov     ecx, 2              ;Readonly access mode
    mov     ebx, filename       ;filename for open
    mov     eax, 5              ;Kernel opcode 5 -> SYS_OPEN
    int     80h                 ;Kernel call

    mov     edx, fileSize       ;number of bytes to be read
    mov     ecx, fileCont       ;Memory address for contents to be read
    mov     ebx, eax
    mov     eax, 3              ;Kernel opcode 3 -> SYS_READ
    int     80h                 ;Kernel call

    mov     eax, fileCont
    ret

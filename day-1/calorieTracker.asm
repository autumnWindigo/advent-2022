section     .data
filename:   db      'input.txt', 0h ;input file
fileSize:   dw      10408           ;input file size

section     .bss
fileCont:   resb 10408           ;Reserved bytes for file
max:        resb 32              ;Reserved bytes for max

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

    ;; Set up for parseFile
    mov     ecx, fileCont       ;put pointer for fileCont into ecx
    xor     eax, eax            ;Clear out regs we need
    xor     esi, esi            ;clear esi
    xor     edi, edi            ;clear edi
    mov     [max], eax          ;clear out max

parseFile:
.buildInt:
    mov     bl, byte [ecx]      ;put current byte into bl
    sub     bl, '0'             ;convert ASCII to int
    imul    eax, 10             ;shift left (but in decimal)
    add     eax, ebx            ;add the new int to the end
    jmp     .endLineCheck       ;jump to endLineCheck

.endLineCheck:
    inc     ecx                 ;iterate forward one char
    inc     edx                 ;linePtr += 1
    cmp     [ecx], byte 0h      ;if null
    jz      quit                ;quit
    cmp     [ecx], byte 0Ah     ;if 0 (newLine) is not the character
    jnz     parseFile           ;Iterate more
    ;; ---------------------
    ;; If it is end of line
    ;; ---------------------
    push    rax                 ;push first number to stack
    xor     rax, rax            ;clear rax
    inc     esi                 ;counting how many nums we have
    inc     ecx                 ;Move ecx to point to next line

.newLineCheck:
    cmp     [ecx], byte 0Ah     ;are there two new lines in a row?
    jnz     parseFile           ;Iterate more if not blank

.stackPopLoop:                  ;Here is where we add them up
    pop     rax                 ;pop from stack into rax
    add     edi, eax            ;add popped int to sum
    dec     esi                 ;Decrement esi
    cmp     esi, 0              ;Check if esi is 0
    jne     .stackPopLoop       ;if not, iterate

    xor     rax, rax            ;clear eax
    cmp     edi, [max]          ;comp edi and max
    jle     .findNextNum        ;jump if less than max
    mov     [max], edi          ;set max to edi

.findNextNum:
    inc     ecx                 ;move ecx forwards
    cmp     [ecx], byte 0Ah     ;if ecx points to \n
    jz      .findNextNum        ;if yes, iterate
    xor     edi, edi            ;clear edi
    jmp     parseFile           ;if no, go back to getLength



;--------------------------------
;Exits gracefully
quit:
    mov     eax, [max]
    mov     rbx, 0              ;Sys_call to end program
    mov     rax, 1
    int     80h

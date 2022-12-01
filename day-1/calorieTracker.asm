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

section     .bss
lineCont:   resb 255            ;Reserved bytes for lines
fileCont:   resb 10408          ;Reserved bytes for file

; ---------------------------------
;
; Code Section
section     .text
global      _start
_start:
    call    openFile            ;Get file contents into reserved memory
    mov     ecx, fileCont       ;put pointer for fileCont into ecx

getLength:
.iterateChar:
    add     ecx, 1              ;iterate forward one char

.endLineCheck:
    cmp     [ecx], byte 0Ah     ;if 0 (newLine) is the character
    jnz     .iterateChar        ;if they aren't equal, iterate more

    mov     [filePtr], ecx      ;set ptr to end of current line
    sub     ecx, fileCont       ;get length of first String

    ;; ecx is length
    ;; eax is start of line
.iterateLine:
    mov     eax, fileCont
    mov     esi, [eax+4]


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

;;; Functions -----------------------------
;------------------------------------------
; int slen(String message)
; String length calculation function
slen:
    push    rbx
    mov     rbx, rax

nextchar:
    cmp     byte [rax], 0
    jz      finished
    inc     rax
    jmp     nextchar

finished:
    sub     rax, rbx
    pop     rbx
    ret


;------------------------------------------
; void sprint(String message)
; String printing function
; Takes input in rax
sprint:
    push    rdx
    push    rcx
    push    rbx
    push    rax
    call    slen

    mov     rdx, rax
    pop     rax

    mov     rcx, rax
    mov     rbx, 1
    mov     rax, 4
    int     80h

    pop     rbx
    pop     rcx
    pop     rdx
    ret

;------------------------------------------
; void sprintLF(String message)
; String printing with line feed function
sprintLF:
    call    sprint

    push    rax         ; push eax onto the stack to preserve it while we use the eax register in this function
    mov     rax, 0Ah    ; move 0Ah into eax - 0Ah is the ascii character for a linefeed
    push    rax         ; push the linefeed onto the stack so we can get the address
    mov     rax, rsp    ; move the address of the current stack pointer into eax for sprint
    call    sprint      ; call our sprint function
    pop     rax         ; remove our linefeed character from the stack
    pop     rax         ; restore the original value of eax before our function was called
    ret                 ; return to our program

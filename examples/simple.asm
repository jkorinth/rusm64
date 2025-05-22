; Simple C64 assembly example
; This program writes "HELLO" to the screen

.org $c000    ; Start at $c000

; Constants
SCREEN_BASE = $0400   ; Base address of screen memory
COLOR_BASE  = $d800   ; Base address of color memory
CHAR_COLOR  = $01     ; White text

start:
    ldx #0            ; Initialize X register to 0
  
print_loop:
    lda hello,x       ; Load next character from message
    beq exit          ; If it's 0 (end of string), exit
    sta SCREEN_BASE,x ; Store on screen
    lda #CHAR_COLOR   ; Load color
    sta COLOR_BASE,x  ; Set color for character
    inx               ; Increment X
    jmp print_loop    ; Repeat

exit:
    rts               ; Return from subroutine

hello:
    .byte "H"  ; Simple bytes instead of a string with null terminator
    .byte "E"
    .byte "L"
    .byte "L"
    .byte "O"
    .byte 0

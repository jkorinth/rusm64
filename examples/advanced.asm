; Advanced C64 assembly example with various addressing modes
; This is a more comprehensive example to test the assembler

.org $c000    ; Start at $c000

; Constants
SCREEN_BASE = $0400   ; Base address of screen memory
COLOR_BASE  = $d800   ; Base address of color memory
CHAR_COLOR  = $01     ; White text
CURSOR_X    = $10     ; Zero page cursor X position
CURSOR_Y    = $11     ; Zero page cursor Y position
MAX_X       = 40      ; Screen width
MAX_Y       = 25      ; Screen height

start:
    ; Initialize
    lda #0
    sta CURSOR_X      ; Set cursor X to 0
    sta CURSOR_Y      ; Set cursor Y to 0
    
    jsr clear_screen  ; Clear the screen
    
    ; Print message
    ldx #0
print_loop:
    lda message,x     ; Load character from message
    beq input_loop    ; If zero (end of string), go to input loop
    jsr print_char    ; Print the character
    inx
    jmp print_loop
    
input_loop:
    ; Wait for keypress - this is just a placeholder
    lda #0
    beq input_loop    ; Always branch (infinite loop)

; Subroutines

; Clear the screen
clear_screen:
    ldx #0
    lda #' '          ; Space character
clear_loop:
    sta SCREEN_BASE,x
    sta SCREEN_BASE+$100,x
    sta SCREEN_BASE+$200,x
    sta SCREEN_BASE+$300,x
    lda #CHAR_COLOR   ; Set color
    sta COLOR_BASE,x
    sta COLOR_BASE+$100,x
    sta COLOR_BASE+$200,x
    sta COLOR_BASE+$300,x
    lda #' '          ; Restore space character
    inx
    bne clear_loop
    rts

; Print character in A register at current cursor position
print_char:
    pha               ; Save A
    ldy CURSOR_Y
    lda screen_lo,y   ; Get low byte of screen row address
    sta screen_ptr
    lda screen_hi,y   ; Get high byte of screen row address
    sta screen_ptr+1
    
    ldy CURSOR_X
    pla               ; Restore A
    sta (screen_ptr),y ; Write character to screen
    
    ; Set color
    pha               ; Save A again
    lda color_lo,y    ; Get low byte of color row address
    sta color_ptr
    lda color_hi,y    ; Get high byte of color row address
    sta color_ptr+1
    
    lda #CHAR_COLOR
    sta (color_ptr),y ; Write color to color memory
    
    ; Update cursor
    inc CURSOR_X
    lda CURSOR_X
    cmp #MAX_X
    bcc print_done    ; If X < MAX_X we're done
    lda #0
    sta CURSOR_X      ; Reset X to 0
    inc CURSOR_Y      ; Move to next line
    lda CURSOR_Y
    cmp #MAX_Y
    bcc print_done    ; If Y < MAX_Y we're done
    lda #0
    sta CURSOR_Y      ; Reset Y to 0 (wrap)
    
print_done:
    pla               ; Restore A
    rts

; Lookup tables for screen and color row addresses
screen_lo:
    .byte <SCREEN_BASE, <(SCREEN_BASE+40), <(SCREEN_BASE+80)
    .byte <(SCREEN_BASE+120), <(SCREEN_BASE+160), <(SCREEN_BASE+200)
    .byte <(SCREEN_BASE+240), <(SCREEN_BASE+280), <(SCREEN_BASE+320)
    .byte <(SCREEN_BASE+360), <(SCREEN_BASE+400), <(SCREEN_BASE+440)
    .byte <(SCREEN_BASE+480), <(SCREEN_BASE+520), <(SCREEN_BASE+560)
    .byte <(SCREEN_BASE+600), <(SCREEN_BASE+640), <(SCREEN_BASE+680)
    .byte <(SCREEN_BASE+720), <(SCREEN_BASE+760), <(SCREEN_BASE+800)
    .byte <(SCREEN_BASE+840), <(SCREEN_BASE+880), <(SCREEN_BASE+920)
    .byte <(SCREEN_BASE+960)

screen_hi:
    .byte >SCREEN_BASE, >(SCREEN_BASE+40), >(SCREEN_BASE+80)
    .byte >(SCREEN_BASE+120), >(SCREEN_BASE+160), >(SCREEN_BASE+200)
    .byte >(SCREEN_BASE+240), >(SCREEN_BASE+280), >(SCREEN_BASE+320)
    .byte >(SCREEN_BASE+360), >(SCREEN_BASE+400), >(SCREEN_BASE+440)
    .byte >(SCREEN_BASE+480), >(SCREEN_BASE+520), >(SCREEN_BASE+560)
    .byte >(SCREEN_BASE+600), >(SCREEN_BASE+640), >(SCREEN_BASE+680)
    .byte >(SCREEN_BASE+720), >(SCREEN_BASE+760), >(SCREEN_BASE+800)
    .byte >(SCREEN_BASE+840), >(SCREEN_BASE+880), >(SCREEN_BASE+920)
    .byte >(SCREEN_BASE+960)

color_lo:
    .byte <COLOR_BASE, <(COLOR_BASE+40), <(COLOR_BASE+80)
    .byte <(COLOR_BASE+120), <(COLOR_BASE+160), <(COLOR_BASE+200)
    .byte <(COLOR_BASE+240), <(COLOR_BASE+280), <(COLOR_BASE+320)
    .byte <(COLOR_BASE+360), <(COLOR_BASE+400), <(COLOR_BASE+440)
    .byte <(COLOR_BASE+480), <(COLOR_BASE+520), <(COLOR_BASE+560)
    .byte <(COLOR_BASE+600), <(COLOR_BASE+640), <(COLOR_BASE+680)
    .byte <(COLOR_BASE+720), <(COLOR_BASE+760), <(COLOR_BASE+800)
    .byte <(COLOR_BASE+840), <(COLOR_BASE+880), <(COLOR_BASE+920)
    .byte <(COLOR_BASE+960)

color_hi:
    .byte >COLOR_BASE, >(COLOR_BASE+40), >(COLOR_BASE+80)
    .byte >(COLOR_BASE+120), >(COLOR_BASE+160), >(COLOR_BASE+200)
    .byte >(COLOR_BASE+240), >(COLOR_BASE+280), >(COLOR_BASE+320)
    .byte >(COLOR_BASE+360), >(COLOR_BASE+400), >(COLOR_BASE+440)
    .byte >(COLOR_BASE+480), >(COLOR_BASE+520), >(COLOR_BASE+560)
    .byte >(COLOR_BASE+600), >(COLOR_BASE+640), >(COLOR_BASE+680)
    .byte >(COLOR_BASE+720), >(COLOR_BASE+760), >(COLOR_BASE+800)
    .byte >(COLOR_BASE+840), >(COLOR_BASE+880), >(COLOR_BASE+920)
    .byte >(COLOR_BASE+960)

; Zero page pointers
screen_ptr = $fb      ; 2 bytes
color_ptr  = $fd      ; 2 bytes

; Message to display
message:
    .text "RUSM - RUST 6502 ASSEMBLER FOR C64"
    .byte 0           ; Null terminator

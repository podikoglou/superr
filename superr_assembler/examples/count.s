; Superr Assembly Example: Count
;
; Counts from 0 to 255 and prints each number

LOAD 0, 0       ; Initialize counter at state[0]
LOAD 1, 255     ; Set the upper limit at state[1]

loop:
    PUT 0       ; Print the counter (state[0])
    INC 0       ; Increment counter
    SUB 1, 0    ; Subtract the counter value from max.
    JZ end      ; If the result is zero, jump to end
    JNZ end     ; Jump to loop if not zero

end:
    PUT 0       ; Print the counter one last time

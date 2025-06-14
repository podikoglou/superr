LOAD 10     ; a = 10 at address 0
SWAP 0 1    ; Move to address 1
LOAD 20     ; b = 20 at address 0

PUT 0       ; Should print 20 (b)
PUT 1       ; Should print 10 (a)

XOR 0 1     ; a = a XOR b (at addr 0)
XOR 1 0     ; b = b XOR a (at addr 1)
XOR 0 1     ; a = a XOR b (at addr 0)

PUT 0       ; Should print 10 (original b)
PUT 1       ; Should print 20 (original a)

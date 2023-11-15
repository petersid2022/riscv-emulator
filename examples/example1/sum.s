.data
    num1:    .word 5
    num2:    .word 7
    result:  .word 0

.text
    la t0, num1
    lw t0, 0(t0)
    la t1, num2
    lw t1, 0(t1)
    add t2, t0, t1
    la t3, result
    sw t2, 0(t3)
    li a0, 10
    li a7, 93
    ecall

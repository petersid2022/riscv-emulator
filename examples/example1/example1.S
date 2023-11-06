# RV32I Assembly Code
.data
    # Data section (optional, for variables)
    num1:   .word 5
    num2:   .word 7
    result: .word 0

.text
    # Text section (instructions)
    la      a0, num1        # Load address of num1 into a0
    lw      a1, 0(a0)       # Load the value of num1 into a1

    la      a0, num2        # Load address of num2 into a0
    lw      a2, 0(a0)       # Load the value of num2 into a2

    add     a3, a1, a2      # Add a1 and a2, result in a3

    la      a0, result      # Load address of result into a0
    sw      a3, 0(a0)       # Store the result in the result variable

    # Exit the program (optional)
    li      a7, 10          # System call code for program exit
    ecall


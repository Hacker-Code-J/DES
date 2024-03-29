def get_bit_value(num, position):
    bitmask = 1 << position
    bit_value = (num & bitmask) != 0
    return int(bit_value)

S = [
    0x2, 0xe, 0xc, 0xb, 0x4, 0x2, 0x1, 0xc, 0x7, 0x4, 0xa, 0x7, 0xb, 0xd, 0x6, 0x1,
    0x8, 0x5, 0x5, 0x0, 0x3, 0xf, 0xf, 0xa, 0xd, 0x3, 0x0, 0x9, 0xe, 0x8, 0x9, 0x6,
    0x4, 0xb, 0x2, 0x8, 0x1, 0xc, 0xb, 0x7, 0xa, 0x1, 0xd, 0xe, 0x7, 0x2, 0x8, 0xd,
    0xf, 0x6, 0x9, 0xf, 0xc, 0x0, 0x5, 0x9, 0x6, 0xa, 0x3, 0x4, 0x0, 0x5, 0xe, 0x3
]

A = []
B = []
C = []
D = []

for i in range(63):
    if get_bit_value(i, 4) == get_bit_value(S[i], 3) ^ get_bit_value(S[i], 2) ^ get_bit_value(S[i], 1) ^ get_bit_value(S[i], 0):
        A.append(i)
    else:
        B.append(i)
    if get_bit_value(i, 4) == get_bit_value(S[i], 3) ^ get_bit_value(S[i], 2) ^ get_bit_value(S[i], 1) ^ get_bit_value(S[i], 0) ^ 1:
        C.append(i)
    else:
        D.append(i)
        
hex_A = [hex(num) for num in A]
hex_B = [hex(num) for num in B]
hex_C = [hex(num) for num in C]
hex_D = [hex(num) for num in D]

print(hex_A)
print(hex_B)
print(hex_C)
print(hex_D)

X = [ 0x2, 0x7, 0xa, 0xe ]
Y = [ 0x0, 0x1, 0x3, 0x4, 0x5, 0x6, 0x8, 0x9, 0xb, 0xc, 0xd]
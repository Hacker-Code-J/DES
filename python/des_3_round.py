import itertools

# DES key schedule constants
PC1 = [
    57, 49, 41, 33, 25, 17,  9,  1, 58, 50, 42, 34, 26, 18, 10,  2,
    59, 51, 43, 35, 27, 19, 11,  3, 60, 52, 44, 36, 63, 55, 47, 39,
    31, 23, 15,  7, 62, 54, 46, 38, 30, 22, 14,  6, 61, 53, 45, 37,
    29, 21, 13, 5, 28, 20, 12, 4
]

PC2 = [14, 17, 11, 24, 1, 5, 3, 28, 15, 6, 21, 10, 23, 19, 12, 4, 26, 8, 16, 7, 27, 20, 13, 2, 41, 52, 31, 37, 47, 55, 30, 40, 51, 45, 33, 48, 44, 49, 39, 56, 34, 53, 46, 42, 50, 36, 29, 32]

# Initial and Final Permutation Tables
IP = [58, 50, 42, 34, 26, 18, 10, 2, 60, 52, 44, 36, 28, 20, 12, 4, 62, 54, 46, 38, 30, 22, 14, 6, 64, 56, 48, 40, 32, 24, 16, 8, 57, 49, 41, 33, 25, 17, 9, 1, 59, 51, 43, 35, 27, 19, 11, 3, 61, 53, 45, 37, 29, 21, 13, 5, 63, 55, 47, 39, 31, 23, 15, 7]
FP = [40, 8, 48, 16, 56, 24, 64, 32, 39, 7, 47, 15, 55, 23, 63, 31, 38, 6, 46, 14, 54, 22, 62, 30, 37, 5, 45, 13, 53, 21, 61, 29, 36, 4, 44, 12, 52, 20, 60, 28, 35, 3, 43, 11, 51, 19, 59, 27, 34, 2, 42, 10, 50, 18, 58, 26, 33, 1, 41, 9, 49, 17, 57, 25]

# Expansion table
E = [32, 1, 2, 3, 4, 5, 4, 5, 6, 7, 8, 9, 8, 9, 10, 11, 12, 13, 12, 13, 14, 15, 16, 17, 16, 17, 18, 19, 20, 21, 20, 21, 22, 23, 24, 25, 24, 25, 26, 27, 28, 29, 28, 29, 30, 31, 32, 1]

# S-box Table (Simplified; real DES has 8 unique S-boxes)
S_BOX = [
# S1
[14, 4, 13, 1, 2, 15, 11, 8, 3, 10, 6, 12, 5, 9, 0, 7,
0, 15, 7, 4, 14, 2, 13, 1, 10, 6, 12, 11, 9, 5, 3, 8,
4, 1, 14, 8, 13, 6, 2, 11, 15, 12, 9, 7, 3, 10, 5, 0,
15, 12, 8, 2, 4, 9, 1, 7, 5, 11, 3, 14, 10, 0, 6, 13],
# Placeholder for S2 to S8 for brevity
]

P = [16, 7, 20, 21, 29, 12, 28, 17, 1, 15, 23, 26, 5, 18, 31, 10, 2, 8, 24, 14, 32, 27, 3, 9, 19, 13, 30, 6, 22, 11, 4, 25]

def apply_table(input_bits, table):
    return ''.join(input_bits[i-1] for i in table)

def initial_permutation(block):
    return apply_table(block, IP)

def final_permutation(input_bits):
    # Simplified for demonstration; real DES has a more complex permutation
    return input_bits

def expand(input_bits):
    # Expansion function for demonstration; real DES expands 32 bits to 48 bits
    return input_bits

def substitute(input_bits):
    # Simplified substitution for demonstration; real DES uses 8 S-boxes
    # This is a placeholder; in real DES, it's more complex
    return input_bits

def permute(input_bits):
    # Simplified permutation function; real DES has a specific permutation table
    return input_bits

def xor(bits1, bits2):
    # Simple XOR function for two bit strings
    return ''.join('1' if b1 != b2 else '0' for b1, b2 in zip(bits1, bits2))

def key_schedule(round_number):
    # Simplified key schedule; real DES uses a complex key schedule
    # This is a placeholder to return a round key; in practice, it's derived from the main key
    return '11001010'

def round_function(input_bits, round_key):
    expanded_bits = expand(input_bits)
    xored = xor(expanded_bits, round_key)
    substituted = substitute(xored)
    permuted = permute(substituted)
    return permuted

def des_3_rounds_encrypt(plaintext, keys):
    ip = initial_permutation(plaintext)
    l, r = ip[:len(ip)//2], ip[len(ip)//2:]
    
    for i in range(3):
        round_key = keys[i]
        l, r = r, xor(l, round_function(r, round_key))
    
    pre_output = r + l  # Note the reversal of L and R here
    ciphertext = final_permutation(pre_output)
    return ciphertext

def linear_attack(ciphertexts, plaintexts):
    # Example linear attack on 3-round DES
    # This is highly simplified and would require statistical analysis in a real scenario
    for key_guess in itertools.product('01', repeat=8):
        print("loop ", key_guess);
        key_guess_str = ''.join(key_guess)
        correct_guesses = 0
        for plaintext, ciphertext in zip(plaintexts, ciphertexts):
            if des_3_rounds_encrypt(plaintext, [key_guess_str] * 3) == ciphertext:
                correct_guesses += 1
        if correct_guesses / len(plaintexts) > 0.9:  # Arbitrary threshold
            print(f"Possible key: {key_guess_str}")
            break

# Example usage
plaintexts = ['0101010101010101', '1010101010101010']  # Simplified examples
ciphertexts = [des_3_rounds_encrypt(pt, ['01010101', '10101010', '11110000']) for pt in plaintexts]

linear_attack(ciphertexts, plaintexts)

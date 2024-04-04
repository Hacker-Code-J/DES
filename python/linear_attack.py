import numpy as np

# Adjusted KEY_SIZE to match BLOCK_SIZE for simplicity
KEY_SIZE = 8  # Adjusted key size to match block size
BLOCK_SIZE = 8  # Simplified block size
SAMPLE_SIZE = 1000  # Number of plaintext-ciphertext pairs

def round_function(input_bits, key):
    # Ensure key is the correct shape, adjusted to match input_bits if necessary
    return np.bitwise_xor(input_bits, key[:len(input_bits)])  # Adjust key size if necessary

def encrypt(plaintext, key):
    # Simplify to direct operation without additional conversions
    return round_function(plaintext, key)

def generate_plaintexts(sample_size):
    # Generate a SAMPLE_SIZE x BLOCK_SIZE array of random bits
    return np.random.randint(0, 2, (sample_size, BLOCK_SIZE), dtype=np.uint8)

def linear_attack(ciphertexts, plaintexts):
    # Simplified linear attack example
    correct_key_bits = np.zeros(KEY_SIZE, dtype=np.uint8)
    key_guesses = np.arange(2)
    for bit in range(1):  # Adjusted for simplicity
        max_correlation = 0
        for guess in key_guesses:
            hypothesis = (plaintexts[:, bit] ^ ciphertexts[:, bit]) == guess
            correlation = np.mean(hypothesis)
            if abs(correlation - 0.5) > max_correlation:
                max_correlation = abs(correlation - 0.5)
                correct_key_bits[bit] = guess
    print(f"Recovered key bits: {correct_key_bits}")

# Key and plaintext generation for the attack
true_key = np.random.randint(0, 2, KEY_SIZE, dtype=np.uint8)
plaintexts = generate_plaintexts(SAMPLE_SIZE)
ciphertexts = np.array([encrypt(pt, true_key) for pt in plaintexts])

linear_attack(ciphertexts, plaintexts)
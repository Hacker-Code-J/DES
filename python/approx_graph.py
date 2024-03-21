import matplotlib.pyplot as plt
import numpy as np

# Given data
X = [0x2, 0x7, 0xa, 0xe]
Y = [0x0, 0x1, 0x3, 0x4, 0x5, 0x6, 0x8, 0x9, 0xb, 0xc, 0xd]

# Function to calculate bit statistics for '0's and '1's
def bit_statistics(numbers, bits=4):
    bit_counts_1 = [0] * bits  # Count of '1' bits
    bit_counts_0 = [0] * bits  # Count of '0' bits
    for number in numbers:
        for i in range(bits):
            if number & (1 << i):
                bit_counts_1[i] += 1
            else:
                bit_counts_0[i] += 1
    return bit_counts_0, bit_counts_1

# Calculate statistics for each group
X_stats_0, X_stats_1 = bit_statistics(X)
Y_stats_0, Y_stats_1 = bit_statistics(Y)

# Number of bits
N = 4

# Set up the bar chart
fig, axs = plt.subplots(2, 1, figsize=(10, 12), sharey=True)

# Bar width
bar_width = 0.35
index = np.arange(N)

# Group X
axs[0].bar(index, X_stats_0, bar_width, label='0 bits')
axs[0].bar(index, X_stats_1, bar_width, bottom=X_stats_0, label='1 bits')
axs[0].set_title('Group X')
axs[0].set_xlabel('Bit Position')
axs[0].set_ylabel('Counts')
axs[0].set_xticks(index)
axs[0].set_xticklabels(['Bit 0 (LSB)', 'Bit 1', 'Bit 2', 'Bit 3 (MSB)'])
axs[0].legend()

# Group Y
axs[1].bar(index, Y_stats_0, bar_width, label='0 bits')
axs[1].bar(index, Y_stats_1, bar_width, bottom=Y_stats_0, label='1 bits')
axs[1].set_title('Group Y')
axs[1].set_xlabel('Bit Position')
axs[1].set_ylabel('Counts')
axs[1].set_xticks(index)
axs[1].set_xticklabels(['Bit 0 (LSB)', 'Bit 1', 'Bit 2', 'Bit 3 (MSB)'])
axs[1].legend()

# Display the plots
plt.tight_layout()
plt.show()
import matplotlib.pyplot as plt
import numpy as np

# Given data
X = [2, 7, 10, 14, 16, 24, 28, 29, 37, 40, 59, 62]
Y = [0, 1, 3, 4, 5, 6, 8, 9, 11, 12, 13, 15, 17, 18, 19, 20, 21, 22, 23, 25, 26, 27, 30, 31, 32, 33, 34, 35, 36, 38, 39, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 60, 61]

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
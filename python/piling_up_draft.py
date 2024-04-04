# pip install matplotlib numpy ipywidgets

import numpy as np
import matplotlib.pyplot as plt
from ipywidgets import interact, IntSlider
import ipywidgets as widgets

def piling_up_lemma(n, p_x_0):
    """
    Calculates the probability P(Z = 0) using the Piling-Up Lemma.
    n: number of variables
    p_x_0: Probability that X_i = 0
    """
    return 0.5 + (2**(n-1)) * np.prod([p_x_0 - 0.5 for _ in range(n)])

def plot_piling_up(n=2):
    # Range of probabilities for P(X_i = 0)
    p_x_0_range = np.linspace(0.5, 1, 100)
    probabilities = [piling_up_lemma(n, p) for p in p_x_0_range]
    
    plt.figure(figsize=(10, 6))
    plt.plot(p_x_0_range, probabilities, label=f'n = {n}')
    plt.xlabel('Probability P(X_i = 0)')
    plt.ylabel('Probability P(Z = 0)')
    plt.title('Piling-Up Lemma Visualization')
    plt.legend()
    plt.grid(True)
    plt.show()

# Interactive widget to adjust the number of variables n
interact(plot_piling_up, n=IntSlider(value=2, min=2, max=10, step=1, description='Number of Variables (n):'))

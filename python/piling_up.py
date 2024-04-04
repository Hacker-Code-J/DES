import matplotlib.pyplot as plt
import numpy as np
from matplotlib.widgets import Slider

# Setting up the figure and axis for the plot
fig, ax = plt.subplots(2, 1, figsize=(10, 8))

# Adjust space for the slider
plt.subplots_adjust(bottom=0.3)

# Function to calculate combined probability using Piling-up Lemma
def piling_up_lemma(p_list):
    product_term = np.prod([2*(p-0.5) for p in p_list])
    return 0.5 + (product_term / 2)

# Plotting the initial state
def update(val):
    p1, p2, p3 = s_p1.val, s_p2.val, s_p3.val
    combined_p = piling_up_lemma([p1, p2, p3])
    
    # Clear previous lines
    ax[0].clear()
    ax[1].clear()
    
    # Individual Probabilities
    ax[0].bar(['P1', 'P2', 'P3'], [p1, p2, p3], color=['red', 'green', 'blue'])
    ax[0].set_ylim(0, 1)
    ax[0].set_title('Individual Linear Approximations')
    
    # Combined Probability
    ax[1].bar('Combined P', combined_p, color='purple')
    ax[1].set_ylim(0, 1)
    ax[1].set_title('Combined Probability Using Piling-up Lemma')
    
    fig.canvas.draw_idle()

# Sliders for adjusting probabilities
axcolor = 'lightgoldenrodyellow'
ax_p1 = plt.axes([0.2, 0.2, 0.65, 0.03], facecolor=axcolor)
ax_p2 = plt.axes([0.2, 0.15, 0.65, 0.03], facecolor=axcolor)
ax_p3 = plt.axes([0.2, 0.1, 0.65, 0.03], facecolor=axcolor)

s_p1 = Slider(ax_p1, 'P1', 0.5, 1.0, valinit=0.6)
s_p2 = Slider(ax_p2, 'P2', 0.5, 1.0, valinit=0.7)
s_p3 = Slider(ax_p3, 'P3', 0.5, 1.0, valinit=0.75)

# Update the plot upon changing the slider's value
s_p1.on_changed(update)
s_p2.on_changed(update)
s_p3.on_changed(update)

# Initial call to update the plots
update(None)

plt.show()
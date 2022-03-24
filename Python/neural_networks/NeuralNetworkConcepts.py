import numpy as np
import nnfs
from nnfs.datasets import spiral_data
# import matplotlib.pyplot as plt
nnfs.init()

# 1 neuron using plain python:
# inputs = [1, 2, 3, 2.5]
# weights1 = [ 0.2, 0.8, -0.5, 1.0 ]
# weights2 = [ 0.5, -0.91, 0.26, -0.5 ]
# weights3 = [ -0.26, -0.27, 0.17, 0.87 ]
# bias1 = 2.0
# bias2 = 3.0
# bias3 = 0.5
# outputs = [
# Neuron 1
# inputs[0] * weights1[0] + inputs[1] * weights1[1] + inputs[2] * weights1[2] + inputs[3] * weights1[3] + bias1,
# Neuron 2
# inputs[0] * weights2[0] + inputs[1] * weights2[1] + inputs[2] * weights2[2] + inputs[3] * weights2[3] + bias2,
# Neuron 3
# inputs[0] * weights3[0] + inputs[1] * weights3[1] + inputs[2] * weights3[2] + inputs[3] * weights3[3] + bias3
# ]
# print(outputs[0], outputs[1], outputs[2])

# 3 neurons using plain python:
# inputs = [1, 2, 3, 2.5]
# weights = [[0.2, 0.8, -0.5, 1],
#            [0.5, -0.91, 0.26, -0.5],
#            [-0.26, -0.27, 0.17, 0.87]]
# biases = [2, 3, 0.5]
# Output of current layer
# layer_outputs = []
# For each neuron
# for neuron_weights, neuron_bias in zip(weights, biases):
# Zeroed output of given neuron
#    neuron_output = 0
# For each input and weight to the neuron
#    for n_input, weight in zip(inputs, neuron_weights):
# Multiply this input by associated weight
# and add to the neuron's output variable
#        neuron_output += n_input*weight
# Add bias
#    neuron_output += neuron_bias
# Put neuron's result to the layer's output list
#    layer_outputs.append(neuron_output)
# print(layer_outputs)

# 1 neuron using numpy's dot product:
# inputs = [1.0, 2.0, 3.0, 2.5]
# weights = [0.2, 0.8, -0.5, 1.0]
# bias = 2.0
# outputs = np.dot(weights, inputs) + bias
# print(outputs)

# 3 neurons using numpy's dot product:
# inputs = [1.0, 2.0, 3.0, 2.5]
# weights = [[0.2, 0.8, -0.5, 1.0],
#            [0.5, -0.91, 0.26, -0.5],
#            [-0.26, -0.27, 0.17, 0.87]]
# biases = [2.0, 3.0, 0.5]
# outputs = np.dot(weights, inputs) + biases
# print(outputs)

# 3 neurons using numpy's matrix product and input as a batch(2D array)
#inputs = [[1, 2, 3, 2.5], [2.0, 5.0, -1.0, 2], [-1.5, 2.7, 3.3, -0.8]]
#weights = [[0.2, 0.8, -0.5, 1], [0.5, -0.91, 0.26, -0.5], [-0.26, -0.27, 0.17, 0.87]]
#biases = [2, 3, 0.5]
#weights2 = [[0.1, -0.14, 0.5], [-0.5, 0.12, -0.33], [-0.44, 0.73, -0.13]]
#biases2 = [-1, 2, -0.5]
#layer1_outputs = np.dot(inputs, np.array(weights).T) + biases
#layer2_outputs = np.dot(layer1_outputs, np.array(weights2).T) + biases2
#print(layer2_outputs)

# Dense Neural Network with "randomly" generated data
class Layer_Dense:
    # Layer initialization
    def __init__(self, n_inputs, n_neurons):
        # Initialize weights and biases
        self.weights = 0.01 * np.random.randn(n_inputs, n_neurons)
        self.biases = np.zeros((1, n_neurons))
        pass # using pass statement as a placeholder
    # Forward pass
    def forward(self, inputs):
        # Calculate output values from inputs, weights and biases
        self.output = np.dot(inputs, self.weights) + self.biases
        pass # using pass statement as a placeholder


class Activatior_ReLU:
    def forward(self, inputs):
        self.output = np.maximum(0, inputs)


class Activatior_Softmax:
    def forward(self, inputs):
        exp_values = np.exp(inputs - np.max(inputs, axis=1, keepdims=True))
        probabilities = exp_values / np.sum(exp_values, axis=1, keepdims=True)
        self.output = probabilities


# Create dataset
X, y = spiral_data(samples=100, classes=3)
# Create Dense layer with 2 input features and 3 output values
dense1 = Layer_Dense(2, 3)
# Create ReLU activation (to be used with Dense layer):
activation1 = Activatior_ReLU()
# Create second Dense layer with 3 input features (as we take output
# of previous layer here) and 3 output values
dense2 = Layer_Dense(3, 3)
# Create Softmax activation (to be used with Dense layer):
activation2 = Activatior_Softmax()
# Perform a forward pass of our training data through this layer
dense1.forward(X)
activation1.forward(dense1.output)
dense2.forward(activation1.output)
activation2.forward(dense2.output)
# Let's see output of the first few samples:
print(activation1.output[:5])

# Values from the previous output when we described
# what a neural network is
layer_outputs = [4.8, 1.21, 2.385]
# e - mathematical constant, we use E here to match a common coding
# style where constants are uppercased
E = 2.71828182846 # you can also use math.e
# For each value in a vector, calculate the exponential value
exp_values = []
for output in layer_outputs:
    exp_values.append(E ** output) # ** - power operator in Python
print('exponentiated values:')
print(exp_values)
# Now normalize values
norm_base = sum(exp_values) # We sum all values
norm_values = []
for value in exp_values:
    norm_values.append(value / norm_base)
print('Normalized exponentiated values:')
print(norm_values)
print('Sum of normalized values:', sum(norm_values))

# Get unnormalized probabilities
exp_values = np.exp(layer_outputs)
print(exp_values)
# Normalize them for each sample
probabilities = exp_values / np.sum(exp_values, axis=0, keepdims=True)
print(probabilities)



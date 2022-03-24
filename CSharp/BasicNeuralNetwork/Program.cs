// See https://aka.ms/new-console-template for more information
using BasicNeuralNetwork;

float[] inputs = { 1.0f, 2.0f, 3.0f, 2.5f };
float[] weights1 = { 0.2f, 0.8f, -0.5f, 1.0f };
float[] weights2 = { 0.5f, -0.91f, 0.26f, -0.5f };
float[] weights3 = { -0.26f, -0.27f, 0.17f, 0.87f };
float bias1 = 2.0f;
float bias2 = 3.0f;
float bias3 = 0.5f;
float[] outputs = {
        //Neuron 1
        MoreMaths.Dot(inputs, weights1) + bias1,
        //inputs[0] * weights1[0] + inputs[1] * weights1[1] + inputs[2] * weights1[2] + inputs[3] * weights1[3] + bias1,
        //Neuron 2
        MoreMaths.Dot(inputs, weights2) + bias2,
        //inputs[0] * weights2[0] + inputs[1] * weights2[1] + inputs[2] * weights2[2] + inputs[3] * weights2[3] + bias2,
        //Neuron 3
        inputs[0] * weights3[0] + inputs[1] * weights3[1] + inputs[2] * weights3[2] + inputs[3] * weights3[3] + bias3
        };
Console.WriteLine($"{outputs[0]} {outputs[1]} {outputs[2]}");


float[,] weights = { { 0.2f, 0.8f, -0.5f, 1.0f }, { 0.5f, -0.91f, 0.26f, -0.5f }, { -0.26f, -0.27f, 0.17f, 0.87f } };
float[] biases = { 2.0f, 3.0f, 0.5f };


// for(int i=0; i<weights.length && i<biases.length; ++i)
// {
//     neuron_weights[i] = weights[i];
//     neuron_bias[i] = biases[i];
//     byte neuron_output = 0;
//     for (int j=0; j<inputs.length && j<neuron_weights.length; ++j)
//     {


//     }
// }






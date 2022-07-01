#include <iostream>
#include <vector>

float DotProduct(std::vector<float> a, std::vector<float> b)
{
  if (a.size() == b.size())
  {
    float dotProduct;
    for (size_t i = 0; i < a.size(); i++)
    {
      dotProduct += a[i] * b[i];
    }
    return dotProduct;
  }
  return 0;
}

// std::vector<float> VectorAdd(std::vector<float> a, std::vector<float> b)
// {
//   if (a.size() == b.size())
//   {
//     /* code */
//   }
// }

int main() 
{
  std::vector<float> inputs = {1, 2, 3, 2.5};
  std::vector<std::vector<float>> weights = {{0.2, 0.8, -0.5, 1},
		{0.5, -0.91, 0.26, -0.5},
		{-0.26, -0.27, 0.17, 0.87}};
  std::vector<float> biases = {2, 3, 0.5};
  std::vector<float> layerOutputs;
  for (size_t i = 0; i < weights.size(); i++)
  {
    float neuronOutput = 0;
    neuronOutput = DotProduct(weights[i], inputs);
    // for (size_t j = 0; j < inputs.size(); j++)
    // {
    //   neuronOutput += inputs[j] * weights[i][j];
    // }
    neuronOutput += biases[i];
    layerOutputs.push_back(neuronOutput);
  }
  std::cout << layerOutputs[0] << layerOutputs[1] << layerOutputs[2] << std::endl;
  return 0;
}
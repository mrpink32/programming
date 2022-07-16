// See https://aka.ms/new-console-template for more information
double DotProduct(double[] a, double[] b)
{
	double result = 0;
	for (int i = 0; i < a.Length; i++)
	{
		result += a[i] * b[i];
	}
    return result;
}

double[][] MatrixProduct(double[][] a, double[][] b)
{
	int height = a.GetLength(0);
	int width = b.GetLength(0);
	double[][] result = new double[height][width];
    for (int i = 0; i < height; i++)
	{
		for (int j = 0; j < width; j++)
		{
			result[i][j] += DotProduct(a[i], b[j]);
		}
	}
	return result;
}

// double[,] AddBias(double[,] a, double[] b)
// {
//     return null;
// }

double[][] inputs = new double[][]{
	new double[]{1.0, 2.0, 3.0, 2.5},
	new double[]{2.0, 5.0, -1.0, 2.0},
	new double[]{-1.5, 2.7, 3.3, -0.8}};
double[][] weights = new double[][]{
	new double[]{0.2, 0.8, -0.5, 1},
	new double[]{0.5, -0.91, 0.26, -0.5},
	new double[]{-0.26, -0.27, 0.17, 0.87}};
double[] biases = new double[]{2.0, 3.0, 0.5};
double[][] weights2 = new double[][]{
	new double[]{0.1, -0.14, 0.5},
	new double[]{-0.5, 0.12, -0.33},
	new double[]{-0.44, 0.73, -0.13}};
double[] biases2 = new double[]{-1.0, 2.0, -0.5};
double[][] layer1outputs = MatrixProduct(inputs, weights);

for (int i = 0; i < layer1outputs.Length; i++)
{
	Console.WriteLine(layer1outputs[i]);
}

Console.WriteLine("Hello, World!");

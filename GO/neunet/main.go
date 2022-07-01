package main

import (
	"fmt"
)

type vector []float64
type matrix []vector

func (a vector) Add(b vector) vector {
	if len(a) == len(b) {
		var result vector
		for i := range a {
			var c float64 = a[i] + b[i]
			result = append(result, c)
		}
		return result
	}
	panic("Calculation of vector addition failed!")
}

func (a matrix) AddBias(b vector) matrix {
	for i := 0; i < len(a); i++ {
		for j := 0; j < len(a[i]); j++ {
			a[i][j] += b[j]
		}
	}
	return a
}

func (a vector) Dot(b vector) float64 {
	if len(a) == len(b) {
		var dotProduct float64
		for i := range a {
			dotProduct += a[i] * b[i]
		}
		return dotProduct
	}
	panic("Calculation of dot product failed!")
}

func (a matrix) T() matrix {
	var width int = len(a)
	var height int = len(a[0])
	var transposedMatrix matrix = make(matrix, height)
	for i := 0; i < height; i++ {
		transposedMatrix[i] = make(vector, width)
	}
	for i := 0; i < width; i++ {
		for j := 0; j < height; j++ {
			// fmt.Println(a[i][j])
			transposedMatrix[j][i] = a[i][j]
			// fmt.Println(transposedMatrix[j][i])
			// fmt.Println(transposedMatrix)
		}
	}
	return transposedMatrix
}

func (a matrix) MatrixProduct(b matrix) matrix {
	var height int = len(a)
	var width int = len(b)
	// println(len(a), height)
	var result matrix = make(matrix, height)
	// fmt.Println("debug 1", width, height, result)
	for i := 0; i < height; i++ {
		result[i] = make(vector, width)
	}
	// fmt.Println("debug 2", width, height, result)
	// b = b.T()
	// fmt.Println("debug 3", width, height, result, len(a[0]), len(b[0]))
	for i := 0; i < height; i++ {
		for j := 0; j < width; j++ {
			// fmt.Println("debug 4", width, height, result)
			result[i][j] = a[i].Dot(b[j])
		}
	}
	// fmt.Println("debug 5", width, height, result)
	return result
}

func TestStuff() {
	fmt.Println("~~~~~~~~~~~~~~")
	// Test of vector addition and dot product
	fmt.Println("Vector addition and Dot product test:")
	var a = vector{1, 2, 3}
	var b = vector{3, 2, 1}
	fmt.Println("Vector a:", a)
	fmt.Println("Vector b:", b)
	fmt.Println("Dot product:", a.Dot(b))
	fmt.Println("Vector addition:", a.Add(b))
	fmt.Println("~~~~~~~~~~~~~~")

	// Test of matrix transposition
	fmt.Println("Matrix transposition test:")
	var c = matrix{{1, 2, 3, 4, 5}, {2, 3, 4, 5, 6}, {3, 4, 5, 6, 7}}
	for i := 0; i < len(c); i++ {
		fmt.Println(c[i])
	}
	fmt.Println("--------------")
	c = c.T()
	for i := 0; i < len(c); i++ {
		fmt.Println(c[i])
	}
	fmt.Println("~~~~~~~~~~~~~~")

	// Test of matrix product
	fmt.Println("Matrix product test:")
	var d matrix = matrix{{1.0, 2.0, 3.0, 2.5},
		{2.0, 5.0, -1.0, 2.0},
		{-1.5, 2.7, 3.3, -0.8}}
	var e matrix = matrix{{0.2, 0.8, -0.5, 1.0},
		{0.5, -0.91, 0.26, -0.5},
		{-0.26, -0.27, 0.17, 0.87}}
	var result matrix = d.MatrixProduct(e)
	for i := 0; i < len(result); i++ {
		fmt.Println(result[i])
	}
	fmt.Println("~~~~~~~~~~~~~~")
}

func main() {
	var inputs matrix = matrix{{1.0, 2.0, 3.0, 2.5},
		{2.0, 5.0, -1.0, 2.0},
		{-1.5, 2.7, 3.3, -0.8}}
	var weights matrix = matrix{{0.2, 0.8, -0.5, 1},
		{0.5, -0.91, 0.26, -0.5},
		{-0.26, -0.27, 0.17, 0.87}}
	var biases = vector{2.0, 3.0, 0.5}
	var weights2 matrix = matrix{{0.1, -0.14, 0.5},
		{-0.5, 0.12, -0.33},
		{-0.44, 0.73, -0.13}}
	var biases2 = vector{-1.0, 2.0, -0.5}
	// var layerOutputs vector
	// for i := range weights {
	// 	var neuronOutput float64 = 0
	// 	neuronOutput = weights[i].Dot(inputs)
	// 	// for j := range inputs {
	// 	// 	neuronOutput += inputs[j] * weights[i][j]
	// 	// }
	// 	neuronOutput += biases[i]
	// 	layerOutputs = append(layerOutputs, neuronOutput)
	// }
	var layer1Outputs matrix = inputs.MatrixProduct(weights).AddBias(biases)
	// layer1Outputs.AddBias(biases)
	var layer2Outputs matrix = layer1Outputs.MatrixProduct(weights2).AddBias(biases2)
	// layer2Outputs.AddBias(biases2)
	for i := 0; i < len(layer2Outputs); i++ {
		fmt.Println(layer2Outputs[i])
	}

	// TestStuff()
}

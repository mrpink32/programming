using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;


namespace BasicNeuralNetwork
{
    public static class MoreMaths
    {
        public static float Dot(float[] vector1, float[] vector2)
        {
            if (vector1.Length == vector2.Length)
            {
                float[] output = new float[vector1.Length];
                for (int i = 0; i < vector1.Length && i < vector2.Length; i++)
                {
                    output[i] = vector1[i] * vector2[i];
                }
                return output.Sum();
            }
            else
            {
                throw new ArgumentException("You can only perform the dot product if the input vectors are homologous");
            }
        }
    }
}

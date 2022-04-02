import numpy as np
import matplotlib as plt
import math

t = float(input("Enter temprature in celseus: "))
Vsound = 331 * math.sqrt((t + 273.15) / 273)
print(Vsound)

time_from_heard_in_seconds = float(input("Enter the amount of seconds that passed since you heard the sound: "))
distance_from_sound = Vsound * time_from_heard_in_seconds
print("Your distance from the sound is: ", distance_from_sound)

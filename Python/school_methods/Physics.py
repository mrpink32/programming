import math

##Sound region
def	speed_of_sound():
	temprature = float(input("Enter temprature in celsius: "))
	Vsound = 331 * math.sqrt((temprature + 273.15) / 273)
	print(Vsound, "m/s")


def distance_from_sound_origin():
	temprature = float(input("Enter temprature in celsius: "))
	Vsound = 331 * math.sqrt((temprature + 273.15) / 273)
	time_From_Heard_In_Seconds = float(input("Enter the amount of seconds that passed since you heard the sound: "))
	distance_From_Sound_Origin = Vsound * time_From_Heard_In_Seconds
	if distance_From_Sound_Origin < 1000:
		print("Your distance from the sound origin is:", distance_From_Sound_Origin, "Meters")
	elif distance_From_Sound_Origin > 1000:
		print("Your distance from the sound origin is:", distance_From_Sound_Origin / 1000, "Kilometers")


def frequency_infront_of_object():
	wave_Velocity = float(input("Enter velocity of the sound wave: "))
	wave_Length_Infront = float(input("Enter length between wave tops in front of the object: "))
	object_Velocity = float(input("Enter velocity of the object in m/s: "))
	frequency = wave_Velocity / wave_Length_Infront
	frequency_Infront = wave_Velocity / (wave_Velocity - object_Velocity) * frequency
	print("The frequency infront of the object is:", frequency_Infront, "Hz")


def frequency_behind_object():
	wave_Velocity = float(input("Enter velocity of the sound wave in m/s: "))
	wave_Length_Behind = float(input("Enter length between wave tops in front of the object in meters: "))
	object_Velocity = float(input("Enter velocity of the object in m/s: "))
	frequency = wave_Velocity / wave_Length_Behind
	frequency_Behind = wave_Velocity / (wave_Velocity + object_Velocity) * frequency
	print("The frequency infront of the object is:", frequency_Behind, "Hz")
##Sound endregion

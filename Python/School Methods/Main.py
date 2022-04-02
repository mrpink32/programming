import Physics
import Math

method_names = []
method_names.extend(["speed_of_sound", "distance_from_sound_origin", "frequency_infront_of_object", "frequency_behind_object"])
print(method_names)
formel = str(input("Choose your desired method: "))
if formel == "speed_of_sound":
	Sound.speed_of_sound()
elif formel == "distance_from_sound_origin":
	Sound.distance_from_sound_origin()
elif formel == "frequency_infront_of_object":
	Sound.frequency_infront_of_object()
elif formel == "frequency_behind_object":
	Sound.frequency_behind_object()
import math

##Lyd region
def	Lydens_Hastighed():
	tempratur = float(input("Indtast tempratur som celsius: "))
	lydens_Hastighed = 331 * math.sqrt((tempratur + 273.15) / 273)
	print(lydens_Hastighed, "m/s")


def Distance_Fra_Lydkilde():
	tempratur = float(input("Indtast tempratur som celsius: "))
	lydens_Hastighed = 331 * math.sqrt((tempratur + 273.15) / 273)
	tid_Fra_Sidst_Hørt = float(input("Indtast den mængde sekunder der er passeret siden du hørte lyden: "))
	distance_Fra_Lydkilde = lydens_Hastighed * tid_Fra_Sidst_Hørt
	if distance_Fra_Lydkilde < 1000:
		print("Din distance fra lydkilden er:", distance_Fra_Lydkilde, "Meter")
	elif distance_Fra_Lydkilde > 1000:
		print("Din distance fra lydkilden er:", distance_Fra_Lydkilde / 1000, "Kilometer")


def Frekvens_Foran_Object():
	bølge_Hastighed = float(input("Indtast hastigheden af lydbølgen: "))
	bølge_Længde_Foran = float(input("Enter length between wave tops in front of the object: "))
	object_Hastighed = float(input("Enter velocity of the object in m/s: "))
	frekvens = bølge_Hastighed / bølge_Længde_Foran
	frekvens_Foran_Object = bølge_Hastighed / (bølge_Hastighed - object_Hastighed) * frekvens
	print("The frequency infront of the object is:", frekvens_Foran_Object, "Hz")


def Frekvens_Bagved_Object():
	bølge_Hastighed = float(input("Enter velocity of the sound wave in m/s: "))
	bølge_Længde_Bag = float(input("Enter length between wave tops in front of the object in meters: "))
	object_Hastighed = float(input("Enter velocity of the object in m/s: "))
	frekvens = bølge_Hastighed / bølge_Længde_Bag
	frekvens_Bagved_Object = bølge_Hastighed / (bølge_Hastighed + object_Hastighed) * frekvens
	print("The frequency infront of the object is:", frekvens_Bagved_Object, "Hz")
##Lyd endregion
##Lys region
def Find_Bølgelængde():
	hastighed = float(input("hastighed i m/s: "))
	Thertz = float(input("hertz i THz: "))
	bølgeLængde = float(hastighed / (Thertz / 1000000000000))
	print(round(bølgeLængde), 5)


def Find_Bølgelængde_Med_Gitter():
	x = float(input("x: "))
	d = float(input("d: "))
	L = float(input("L: "))
	Lambda = float(((x * 1000000) * (1/d)) / L)
	print(round(Lambda, 5))


def Find_TerraHertz():
	hastighed_I_Materiale = float(input("Indtast hatighed af lyset i km/s: "))
	bølgeLængde = float(input("Indtast λ i nm: "))
	terraHertz = float((hastighed_I_Materiale * 1000)/(bølgeLængde / 1000000000))
	print(round(terraHertz, 5))
##Lys endregion
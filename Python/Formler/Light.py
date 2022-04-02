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


Find_TerraHertz()

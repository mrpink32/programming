import tkinter
import Fysik
import Matematik

formel_Samling_Navne = []
formel_Samling_Navne.extend(["Fysik", "Matematik"])
print(formel_Samling_Navne)
formel_Samling = str(input("Vælg din ønskede formel samling "))
if formel_Samling == "Fysik":
	methode_Navne = []
	methode_Navne.extend(["lydens_hastighed", "distance_fra_lydkilde", "frequency_infront_of_object", "frequency_behind_object"])
	print(methode_Navne)
	formel = str(input("Vælg din ønskede formel: "))
	if formel == "lydens_hastighed":
		Fysik.Lydens_Hastighed()
	elif formel == "distance_fra_lydkilde":
		Fysik.Distance_Fra_Lydkilde()
	elif formel == "frekvens_foran_object":
		Fysik.Frekvens_Foran_Object()
	elif formel == "frekvens_bagved_object":
		Fysik.Frekvens_Bagved_Object()
elif formel_Samling == "Matematik":
	formel_Sæt = []
	formel_Sæt.extend(["vækst", "annuitetslån", "annuitetsopsparing"])
	print(formel_Sæt)
	valgt_Formel_Sæt = str(input("Vælg dit ønskede formel sæt: "))
	if valgt_Formel_Sæt == "vækst":
		formler = []
		formler.extend(["beregn slutkapital", "beregn startkapital", "beregn renten", "beregn terminer"])
		print(formler)
		valgt_Formel = str(input("Vælg din ønskede formel: "))
		if valgt_Formel == "beregn slutkapital":
			Matematik.Vækst_Find_Slutkapital()
		elif valgt_Formel == "beregn startkapital":
			Matematik.Vækst_Find_Startkapital()
		elif valgt_Formel == "beregn renten":
			Matematik.Vækst_Find_Rente()
		elif valgt_Formel == "beregn terminer":
			Matematik.Vækst_Find_Terminer()
	elif valgt_Formel_Sæt == "annuitetslån":
		formler = []
		formler.extend(["beregn hovedstol", "beregn ydelse", "beregn terminer"])
		valgt_Formel = str(input("Vælg din ønskede formel: "))
		if valgt_Formel == "beregn slutkapital":
			Matematik.Annuitetslån_Find_Slutkapital()
		elif valgt_Formel == "beregn startkapital":
			Matematik.Annuitetslån_Find_Startkapital()
		elif valgt_Formel == "beregn terminer":
			Matematik.Annuitetslån_Find_Terminer()
	elif valgt_Formel_Sæt == "annuitetsopsparing":
		pass
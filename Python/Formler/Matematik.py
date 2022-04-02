import math


##Geometri region

##Geometri endregion


##Vækst region
def Vækst_Find_Slutkapital():
	startkapital = float(input("Indtast startkapital: "))
	rente = float(input("Indtast rente i decimaltal: "))
	termin = float(input("Indtast termin: "))
	slutkapital = float(round(startkapital * (1 + rente) ** termin, 5))
	print("din slutkapital er:", slutkapital)


def Vækst_Find_Startkapital():
	slutkapital = float(input("Indtast slutkapital: "))
	rente = float(input("Indtast rente i decimaltal: "))
	termin = float(input("Indtast terminer: "))
	startkapital = float(round(slutkapital / (1 + rente) ** termin, 5))
	print("Din startkapital er:", startkapital)


def Vækst_Find_Rente():
	termin = float(input("Indtast terminer: "))
	slutkapital = float(input("Indtast slutkapital: "))
	startkapital = float(input("Indtast startkapital: "))
	rente = float(round((slutkapital / startkapital) ** (1 / termin) - 1, 5))
	print("Din rente er:", rente)


def Vækst_Find_Terminer():
	slutkapital = float(input("Indtast slutkapital: "))
	startkapital = float(input("Indtast startkapital: "))
	rente = float(input("Indtast rente i decimaltal: "))
	termin = float(round(math.log(slutkapital / startkapital) / math.log(1 + rente), 5))
	print("Dine terminer er:", termin)
##Vækst endregion

##annuitetsopsparing
def Annuitetslån_Find_Ydelse():
	slutkapital = float(input("Indtast slutkapital: "))
	rente = float(input("Indtast rente i decimaltal: "))
	termin = float(input("Indtast terminer: "))
	ydelse = float(round(slutkapital * (rente / (1-(1+rente)^termin))), 5)
	print(ydelse)


def Annuitetslån_Find_Slutkapital():
	ydelse = float(input("Indtast ydelse: "))
	



def Annuitetslån_Find_Terminer():
	slutkapital = float(input("Indtast slutkapital: "))
	ydelse = float(input("Indtast ydelse: "))
	rente = float(input("Indtast rente i decimaltal: "))
	termin = float(round(log((slutkapital/startkapital)/ydelse)/log(1 + rente)), 5)
	print(termin)


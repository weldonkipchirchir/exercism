package annalyn

// CanFastAttack can be executed only when the knight is sleeping.
func CanFastAttack(knightIsAwake bool) bool {
	// const canAttack =  knightIsAwake == true ? false : true
	var canAttack bool
	if knightIsAwake{
		canAttack = false
	} else{
		canAttack = true
	}
	return canAttack
}

// CanSpy can be executed if at least one of the characters is awake.
func CanSpy(knightIsAwake, archerIsAwake, prisonerIsAwake bool) bool {
	var canSpy bool
	if knightIsAwake || archerIsAwake || prisonerIsAwake{
		canSpy = true
	} else{
		canSpy = false
	}
	return canSpy
}

// CanSignalPrisoner can be executed if the prisoner is awake and the archer is sleeping.
func CanSignalPrisoner(archerIsAwake, prisonerIsAwake bool) bool {
	var canSignal bool
	if prisonerIsAwake && !archerIsAwake{
		canSignal = true
	} else{
		canSignal = false
	}
	return canSignal
}

// CanFreePrisoner can be executed if the prisoner is awake and the other 2 characters are asleep
// or if Annalyn's pet dog is with her and the archer is sleeping.
func CanFreePrisoner(knightIsAwake, archerIsAwake, prisonerIsAwake, petDogIsPresent bool) bool {
	var canFree bool
	if (prisonerIsAwake && !knightIsAwake && !archerIsAwake) || (petDogIsPresent && !archerIsAwake){
		canFree = true
	} else{
		canFree = false
	}
	return canFree
}

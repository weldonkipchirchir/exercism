static class QuestLogic
{
    public static bool CanFastAttack(bool knightIsAwake)
    {
        bool canAttack = knightIsAwake == true ? false : true;
        return canAttack;
    }

    public static bool CanSpy(bool knightIsAwake, bool archerIsAwake, bool prisonerIsAwake)
    {
        if (knightIsAwake == true || archerIsAwake == true || prisonerIsAwake == true)
        {
            return true;
        }
        else
        {
            return false;
        }
    }

    public static bool CanSignalPrisoner(bool archerIsAwake, bool prisonerIsAwake)
    {
        if (archerIsAwake != true && prisonerIsAwake == true)
        {
            return true;
        }
        return false;
    }

    public static bool CanFreePrisoner(bool knightIsAwake, bool archerIsAwake, bool prisonerIsAwake, bool petDogIsPresent)
    {
        if (petDogIsPresent == true && archerIsAwake != true)
        {
            return true;
        }
        else if (petDogIsPresent != true && prisonerIsAwake == true && knightIsAwake == false && archerIsAwake == false)
        {
            return true;
        }
        else
        {
            return false;
        }
    }
}

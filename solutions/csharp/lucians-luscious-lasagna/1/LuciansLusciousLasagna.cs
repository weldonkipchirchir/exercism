class Lasagna
{
    int lasagna = 40;

    // TODO: define the 'ExpectedMinutesInOven()' method
    public int ExpectedMinutesInOven()
    {
        return lasagna;
    }

    // TODO: define the 'RemainingMinutesInOven()' method
    public int RemainingMinutesInOven(int actualMinutes)
    {
        var remainingMinutes = lasagna - actualMinutes;
        return remainingMinutes;
    }

    // TODO: define the 'PreparationTimeInMinutes()' method
    public int PreparationTimeInMinutes(int numberOfLayers)
    {
        var minutesSpent = numberOfLayers * 2;
        return minutesSpent;
    }

    // TODO: define the 'ElapsedTimeInMinutes()' method
    public int ElapsedTimeInMinutes(int numberOfLayers, int minutes)
    {
        var numberOfMinutes = 2 * numberOfLayers + minutes;
        return numberOfMinutes;
    }
}

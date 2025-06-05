#[derive(Debug, PartialEq, Clone)]
pub enum Unit {
    Length(Length),
    Mass(Mass),
    Area(Area),
    Volume(Volume),
    Force(Force),
    Energy(Energy),
    Temperature(Temperature),
    Time(Time),
    Speed(Speed),
    Pressure(Pressure),
    Frequency(Frequency),
    ElectricCurrent(ElectricCurrent),
    Voltage(Voltage),
    Capacitance(Capacitance),
    Luminosity(Luminosity),
    Radiation(Radiation),
    MagneticFlux(MagneticFlux),
}

#[derive(Debug, PartialEq)]
pub enum Dimension {
    Length, Mass, Area, Volume, Force, Energy, Temperature, Time, Speed, Pressure, Frequency,
    ElectricCurrent, Voltage, Capacitance, Luminosity, Radiation, MagneticFlux,
    Unknown, // Should ideally not be reached if `Unit` enum is exhaustive
}

#[derive(Debug, PartialEq, Clone)]
pub enum Length {
    // Metric Length
    Kilometer,
    Hectometer,
    Decameter,
    Meter,
    Decimeter,
    Centimeter,
    Millimeter,

    // Astronomical Length
    LightYear,
    AstronomicalUnit,
    Parsec,
    Angstrom,
    Micron,          // Also known as Micrometer
    Nanometer,
    Picometer,
    Femtometer,
    Attometer,

    // Imperial/US Customary Length
    Inch,
    Foot,
    Yard,
    NauticalMile,
    League,
    Furlong,
    Rod,
    Chain,
    Mile,

    // Colloquial/Reference Lengths
    FootballFieldLength, // Clarified from just FootballField
    WhaleLength,         // Clarified from just Whale
}

#[derive(Debug, PartialEq, Clone)]
pub enum Mass {
    // Metric Mass
    Kilogram,
    Hectogram,
    Decagram,
    Gram,
    Decigram,
    Centigram,
    Milligram,

    // Smaller Metric Mass
    Microgram,
    Nanogram,
    Picogram,
    Femtogram,
    Attogram,

    // Larger Metric Mass
    Ton,         // Often implies Metric Ton, but good to be explicit for the other "Ton"
    MetricTon,   // Synonymous with Tonne
    Quintal,     // Also known as Centner

    // Imperial/US Customary Mass
    Pounds,      // Avoirdupois Pounds
    Ounce,       // Avoirdupois Ounce
    Stone,       // Used in the UK and Ireland
    Carat,       // Used for gemstones and pearls
}

#[derive(Debug, PartialEq, Clone)]
pub enum Area {
    SquareMeter,
    SquareKilometer,
    SquareCentimeter,
    SquareMillimeter,
    SquareDecimeter,
    Hectare,
    Acre,
    SquareFoot,
    SquareInch,
    SquareYard,
    SquareMile,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Volume {
    CubicMeter,
    CubicDecimeter,
    CubicCentimeter,
    CubicMillimeter,

    Liter,
    Milliliter,
    Deciliter,
    Kiloliter,
    Megaliter,
    Gigaliter,
    Teraliter,

    CubicInch,
    CubicFoot,
    CubicYard,
    CubicMile,
    TeracubicFeet,

    FluidOunce,
    Pint,
    Quart,
    Gallon,
    BarrelLiquid,

    Bushel,
    Peck,
    Cord,

    Teaspoon,
    Tablespoon,
    Cup,

    AcreFoot,
    StandardCubicFoot,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Force {
    // SI Units
    Newton,
    Millinewton,
    Kilonewton,
    Meganewton,

    // CGS Unit
    Dyne,

    // Gravitational Units
    KilogramForce, // Also known as Kilopond (kp)
    GramForce,
    PoundForce,
    OunceForce,
    TonForce,      // Typically Metric Ton-force or Short Ton-force, specify if needed

    // Imperial/US Customary Dynamic Unit
    Poundal,

    // Other specific or less common units
    Kip,           // Kilo-pound force
    Sthene,        // Obsolete unit in the meter-ton-second system
    JoulePerMeter, // Energy per unit length, dimensionally equivalent to force
}

#[derive(Debug, PartialEq, Clone)]
pub enum Energy {
    // SI Units
    Joule,

    // Calories
    Calorie,           // Thermochemical calorie (often ~4.184 J)
    Kilocalorie,       // Also known as "food calorie" (kcal)

    // Electrical Energy
    KilowattHour,      // kWh
    WattHour,          // Wh

    // Atomic/Particle Physics
    Electronvolt,      // eV
    Hartree,           // Eh, unit of energy in the atomic units system

    // Imperial/US Customary Energy
    BritishThermalUnit, // BTU
    FootPound,          // ft·lbf, sometimes written as FootPoundForce

    // CGS Unit
    Erg,

    // Large Scale / Fuel Energy
    Therm,             // Used in natural gas industry
    TonOfTntEquivalent, // tTNT
    HorsepowerHour,    // hp·h
    BarrelOfOilEquivalent, // BOE
}

#[derive(Debug, PartialEq, Clone)]
pub enum Temperature {
    Celsius,
    Fahrenheit,
    Kelvin,
    Rankine,
    Reaumur,
    Delisle,
    NewtonScale, // Renamed to avoid conflict with Force::Newton
    Romer,
    Leiden,
    PlanckTemperature,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Time {
    // Standard Units
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year,
    Decade,
    Century,
    Millennium,

    // Sub-second Units
    Microsecond,
    Millisecond,
    Nanosecond,
    Picosecond,

    // Specific/Less Common Units
    Fortnight,      // Two weeks
    Shake,          // 10 nanoseconds, used in nuclear physics
    JulianYear,     // Exactly 365.25 days
    LeapYear,       // 366 days
    SiderealDay,    // Time it takes for the Earth to complete one rotation relative to the fixed stars
    PlanckTime,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Speed {
    MeterPerSecond,
    KilometerPerHour,
    MilePerHour,
    Knot,
    FootPerSecond,
    Mach,                 // Often specified with a number (e.g., Mach 1, Mach 2)
    SpeedOfLight,         // c
    InchPerSecond,
    FurlongPerFortnight,
    BeaufortScale,        // This is a scale of wind speed, not a unit of speed directly.
                          // Consider if you want a scale or a direct unit.
    CosmicVelocity,       // Often refers to escape velocity or orbital velocity benchmarks
    GalileoUnit,          // An old unit, 1 galileo = 1 cm/s
    Benz,                 // An old unit, 1 benz = 1 meter per second
    YardPerSecond,
    EarthsRotationSpeed,  // Specific value, not a generic unit
    ParsecPerYear,
    SpeedOfLightC,
    MilePerMinute,
    KilometerPerSecond,
}

// New enum for Pressure units
#[derive(Debug, PartialEq, Clone)]
pub enum Pressure {
    // SI Units and derivatives
    Pascal,
    Decipascal,
    Hectopascal,
    Kilopascal,
    Megapascal,
    PlanckPressure,

    // Common Non-SI Units
    Bar,
    AtmosphereStandard, // Standard Atmosphere (atm)
    Torr,               // Millimeter of Mercury at 0°C
    PoundPerSquareInch, // PSI
    KipPerSquareInch,   // KSI (kips per square inch)

    // Hydrostatic Units
    MillimeterOfMercury, // mm Hg
    InchOfMercury,       // in Hg
    InchOfWater,         // in H2O
    FootOfWater,         // ft H2O

    // CGS Units
    DynePerSquareCentimeter,
    Barye,              // CGS unit of pressure, 1 dyn/cm²

    // Gravitational/Technical Units
    TechnicalAtmosphere, // at, kilogram-force per square centimeter (kgf/cm²)
    PoundalPerSquareFoot,

    // Obsolete/Less common
    SthenePerSquareMeter, // Obsolete, from MTS system
}

#[derive(Debug, PartialEq, Clone)]
pub enum Frequency {
    // SI Units and prefixes
    Hertz,
    Millihertz,
    Microhertz,
    Nanohertz,
    Kilohertz,
    Megahertz,
    Gigahertz,
    Terahertz,
    PlanckFrequency,

    // Rotational/Cyclical Frequency
    RevolutionsPerMinute, // RPM
    RadianPerSecond,      // Angular frequency
    CyclesPerSecond,      // Equivalent to Hertz
    FramesPerSecond,      // FPS, used for video/animation
    BeatsPerMinute,       // BPM, used for music tempo, heart rate
    CountsPerMinute,      // CPM, used for radioactivity

    // Other related units
    Fresnel,              // Obsolete unit of frequency (10^12 Hz)
    ReciprocalSecond,     // s^-1, general unit for rate, equivalent to Hertz
    DegreePerSecond,      // Angular speed
    GradianPerSecond,     // Angular speed
}

#[derive(Debug, PartialEq, Clone)]
pub enum ElectricCurrent {
    // SI Units and prefixes
    Ampere,
    Picoampere,
    Nanoampere,
    Microampere,
    Milliampere,
    Kiloampere,
    Megaampere,
    Teraampere,
    PlanckCurrent,

    // CGS Units
    Statampere,  // Electrostatic unit (ESU) of current
    Abampere,    // Electromagnetic unit (EMU) of current
    Biot,        // Equivalent to Abampere
    FranklinPerSecond, // Equivalent to Statampere

    // Derived units/equivalents
    CoulombPerSecond, // Equivalent to Ampere
    VoltPerOhm,       // Equivalent to Ampere (Ohm's Law: I = V/R)
    WattPerVolt,      // Equivalent to Ampere (Power Law: P = V*I => I = P/V)
    SiemensVolt,      // Siemens * Volt = (Ampere/Volt) * Volt = Ampere
    EsuPerSecond,     // Equivalent to Statampere
    EmuOfCurrent,     // Equivalent to Abampere
    Gilbert,          // Unit of magnetomotive force, not strictly current, but related in some systems.
                      // Consider if this fits your "current" domain or needs its own category (e.g., Magnetism).
}

#[derive(Debug, PartialEq, Clone)]
pub enum Voltage {
    // SI Units and prefixes
    Volt,
    Picovolt,
    Nanovolt,
    Microvolt,
    Millivolt,
    Decivolt,
    Centivolt,
    Hectovolt,
    Kilovolt,
    Megavolt,
    Gigavolt,
    Teravolt,
    PlanckVoltage,

    // CGS Units
    Statvolt,       // Electrostatic unit (ESU) of potential
    Abvolt,         // Electromagnetic unit (EMU) of potential
    EsuOfPotential, // Equivalent to Statvolt
    EmuOfPotential, // Equivalent to Abvolt

    // Derived units/equivalents
    WattPerAmpere,  // Equivalent to Volt (Power Law: P = V*I => V = P/I)
    JoulePerCoulomb, // Equivalent to Volt (Definition of Volt: V = J/C)
    ElectronvoltPerElementaryCharge, // eV/e, common in physics for potential
}

#[derive(Debug, PartialEq, Clone)]
pub enum Capacitance {
    // SI Units and prefixes
    Farad,
    Attofarad,
    Femtofarad,
    Picofarad,
    Nanofarad,
    Microfarad,
    Millifarad,
    Decafarad,
    Hectofarad,
    Kilofarad,
    Megafarad,
    Terafarad,
    Gigafarad,
    PlanckCapacitance,

    // CGS Units
    Statfarad,      // Electrostatic unit (ESU) of capacitance
    Abfarad,        // Electromagnetic unit (EMU) of capacitance
    EsuOfCapacitance, // Equivalent to Statfarad
    EmuOfCapacitance, // Equivalent to Abfarad

    // Derived units/equivalents
    CoulombPerVolt, // Equivalent to Farad (Definition of Farad: F = C/V)
    SecondPerOhm,   // Equivalent to Farad (Time constant: RC, C = T/R)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Luminosity {
    // SI Units
    Candela,        // Base SI unit of luminous intensity
    Lumen,          // SI unit of luminous flux
    Lux,            // SI unit of illuminance (lumen per square meter)
    CandelaPerSquareMeter, // SI unit of luminance (nit)
    LumenPerSquareMeter,   // Equivalent to Lux

    // Non-SI Units
    Nit,            // Equivalent to candela per square meter
    Stilb,          // CGS unit of luminance (candela per square centimeter)
    Apostilb,       // Unit of luminance, 1/pi candela per square meter
    Lambert,        // CGS unit of luminance
    FootLambert,    // Imperial unit of luminance
    Phot,           // CGS unit of illuminance
    Bril,           // Obsolete unit of luminance
    Skot,           // Obsolete unit of luminance

    // Derived/Related Units
    WattPerSteradian,          // Radiance or luminous intensity if related to radiometric flux
    LumenPerWatt,              // Luminous efficacy
    LumenSecond,               // Luminous energy (Talbot)
    Talbot,                    // Equivalent to Lumen second
    WattPerSteradianSquareMeter, // Radiance
    Candlepower,               // Obsolete unit of luminous intensity, historically related to candela
    PlanckLuminance,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Radiation {
    // Absorbed Dose (SI and non-SI)
    Gray,           // SI unit of absorbed dose
    Decigray,
    Centigray,
    Milligray,
    Microgray,
    Nanogray,
    Picogray,
    Rad,            // Non-SI unit of absorbed dose

    // Dose Equivalent (SI and non-SI)
    Sievert,        // SI unit of dose equivalent
    Rem,            // Non-SI unit of dose equivalent

    // Activity (SI and non-SI)
    Becquerel,      // SI unit of activity (disintegrations per second)
    Curie,          // Non-SI unit of activity
    Rutherford,     // Non-SI unit of activity

    // Exposure
    Roentgen,       // Unit of exposure
    CoulombPerKilogram, // SI unit of exposure

    // Other related units
    ElectronvoltPerKilogram, // Energy absorbed per unit mass
    IonPairPerKilogram,      // Measure of exposure (number of ion pairs produced)
    Rep,                     // Roentgen Equivalent Physical, older unit
    WattPerKilogram,         // Absorbed dose rate (J/s/kg)
    PlanckRadiation,         // Planck unit for spectral radiance or related radiation quantities
}

#[derive(Debug, PartialEq, Clone)]
pub enum MagneticFlux {
    // SI Units
    Weber,          // SI unit of magnetic flux
    Microweber,
    Nanoweber,
    Picoweber,
    Femtoweber,
    Attoweber,

    // CGS Units
    Maxwell,        // CGS unit of magnetic flux
    Megaline,       // One megaline = 10^6 maxwells
    Kiloline,       // One kiloline = 10^3 maxwells
    Milliline,      // One milliline = 10^-3 maxwells
    GammaSquareCentimeter, // Gamma (nT) * cm^2, implies flux density * area

    // Derived units/equivalents
    TeslaSquareMeter, // Equivalent to Weber (Tesla * Area)
    GaussSquareCentimeter, // Equivalent to Maxwell (Gauss * Area)
    VoltSecond,     // Equivalent to Weber (Faraday's Law)
    CoulombOhm,     // Equivalent to Weber (C * V/A = C * J/C / (J/Vs) = C * J/C * Vs/J = Vs)
    JoulePerAmpere, // Equivalent to Weber (Energy / Current = J/A = (V*A*s)/A = V*s)
    HenryAmpere,    // Equivalent to Weber (Inductance * Current)
    MagneticFluxQuantum, // Fundamental constant in superconductivity
    UnitPole,       // Obsolete, from older magnetic field definitions
    PlanckFlux,
}

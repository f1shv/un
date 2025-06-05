use crate::enums::*;

pub mod enums;

fn get_unit_dimension(unit: &Unit) -> Dimension {
    match unit {
        Unit::Length(_) => Dimension::Length,
        Unit::Mass(_) => Dimension::Mass,
        Unit::Area(_) => Dimension::Area,
        Unit::Volume(_) => Dimension::Volume,
        Unit::Force(_) => Dimension::Force,
        Unit::Energy(_) => Dimension::Energy,
        Unit::Temperature(_) => Dimension::Temperature,
        Unit::Time(_) => Dimension::Time,
        Unit::Speed(_) => Dimension::Speed,
        Unit::Pressure(_) => Dimension::Pressure,
        Unit::Frequency(_) => Dimension::Frequency,
        Unit::ElectricCurrent(_) => Dimension::ElectricCurrent,
        Unit::Voltage(_) => Dimension::Voltage,
        Unit::Capacitance(_) => Dimension::Capacitance,
        Unit::Luminosity(_) => Dimension::Luminosity,
        Unit::Radiation(_) => Dimension::Radiation,
        Unit::MagneticFlux(_) => Dimension::MagneticFlux,
    }
}

fn get_conversion_factor_to_base(unit: &Unit) -> f64 {
    match unit {
        // Length (Base: Meter)
        Unit::Length(Length::Kilometer) => 1000.0,
        Unit::Length(Length::Hectometer) => 100.0,
        Unit::Length(Length::Decameter) => 10.0,
        Unit::Length(Length::Meter) => 1.0,
        Unit::Length(Length::Decimeter) => 0.1,
        Unit::Length(Length::Centimeter) => 0.01,
        Unit::Length(Length::Millimeter) => 0.001,
        Unit::Length(Length::Micron) => 1e-6,
        Unit::Length(Length::Nanometer) => 1e-9,
        Unit::Length(Length::Picometer) => 1e-12,
        Unit::Length(Length::Femtometer) => 1e-15,
        Unit::Length(Length::Attometer) => 1e-18,
        Unit::Length(Length::LightYear) => 9.461e15,
        Unit::Length(Length::AstronomicalUnit) => 1.496e11,
        Unit::Length(Length::Parsec) => 3.086e16,
        Unit::Length(Length::Angstrom) => 1e-10,
        Unit::Length(Length::Inch) => 0.0254,
        Unit::Length(Length::Foot) => 0.3048,
        Unit::Length(Length::Yard) => 0.9144,
        Unit::Length(Length::Mile) => 1609.344,
        Unit::Length(Length::NauticalMile) => 1852.0,
        Unit::Length(Length::League) => 4828.032, // 3 miles
        Unit::Length(Length::Furlong) => 201.168,
        Unit::Length(Length::Rod) => 5.0292,
        Unit::Length(Length::Chain) => 20.1168,
        Unit::Length(Length::FootballFieldLength) => 91.44, // Approx 100 yards
        Unit::Length(Length::WhaleLength) => 25.0, // Approx avg length of a blue whale in meters

        // Mass (Base: Kilogram)
        Unit::Mass(Mass::Kilogram) => 1.0,
        Unit::Mass(Mass::Hectogram) => 0.1,
        Unit::Mass(Mass::Decagram) => 0.01,
        Unit::Mass(Mass::Gram) => 0.001,
        Unit::Mass(Mass::Decigram) => 0.0001,
        Unit::Mass(Mass::Centigram) => 1e-5,
        Unit::Mass(Mass::Milligram) => 1e-6,
        Unit::Mass(Mass::Microgram) => 1e-9,
        Unit::Mass(Mass::Nanogram) => 1e-12,
        Unit::Mass(Mass::Picogram) => 1e-15,
        Unit::Mass(Mass::Femtogram) => 1e-18,
        Unit::Mass(Mass::Attogram) => 1e-21,
        Unit::Mass(Mass::Ton) => 907.185, // Short ton (US)
        Unit::Mass(Mass::MetricTon) => 1000.0, // Tonne
        Unit::Mass(Mass::Quintal) => 100.0,
        Unit::Mass(Mass::Pounds) => 0.45359237,
        Unit::Mass(Mass::Ounce) => 0.028349523125,
        Unit::Mass(Mass::Stone) => 6.35029,
        Unit::Mass(Mass::Carat) => 0.0002,

        // Area (Base: Square Meter)
        Unit::Area(Area::SquareMeter) => 1.0,
        Unit::Area(Area::SquareKilometer) => 1_000_000.0,
        Unit::Area(Area::SquareCentimeter) => 0.0001,
        Unit::Area(Area::SquareMillimeter) => 0.000001,
        Unit::Area(Area::SquareDecimeter) => 0.01,
        Unit::Area(Area::Hectare) => 10_000.0,
        Unit::Area(Area::Acre) => 4046.8564224,
        Unit::Area(Area::SquareFoot) => 0.09290304,
        Unit::Area(Area::SquareInch) => 0.00064516,
        Unit::Area(Area::SquareYard) => 0.83612736,
        Unit::Area(Area::SquareMile) => 2_589_988.110336,

        // Volume (Base: Cubic Meter)
        Unit::Volume(Volume::CubicMeter) => 1.0,
        Unit::Volume(Volume::CubicDecimeter) => 0.001,
        Unit::Volume(Volume::CubicCentimeter) => 1e-6,
        Unit::Volume(Volume::CubicMillimeter) => 1e-9,
        Unit::Volume(Volume::Liter) => 0.001,
        Unit::Volume(Volume::Milliliter) => 1e-6,
        Unit::Volume(Volume::Deciliter) => 0.0001,
        Unit::Volume(Volume::Kiloliter) => 1.0,
        Unit::Volume(Volume::Megaliter) => 1000.0,
        Unit::Volume(Volume::Gigaliter) => 1_000_000.0,
        Unit::Volume(Volume::Teraliter) => 1_000_000_000.0,
        Unit::Volume(Volume::CubicInch) => 1.63871e-5,
        Unit::Volume(Volume::CubicFoot) => 0.0283168,
        Unit::Volume(Volume::CubicYard) => 0.764555,
        Unit::Volume(Volume::CubicMile) => 4.16818e9,
        Unit::Volume(Volume::TeracubicFeet) => 2.8316846592e10, // 1 trillion cubic feet
        Unit::Volume(Volume::FluidOunce) => 2.95735e-5, // US fluid ounce
        Unit::Volume(Volume::Pint) => 0.000473176, // US liquid pint
        Unit::Volume(Volume::Quart) => 0.000946353, // US liquid quart
        Unit::Volume(Volume::Gallon) => 0.00378541, // US liquid gallon
        Unit::Volume(Volume::BarrelLiquid) => 0.11924, // US liquid barrel
        Unit::Volume(Volume::Bushel) => 0.0352391, // US bushel
        Unit::Volume(Volume::Peck) => 0.00880977, // US peck
        Unit::Volume(Volume::Cord) => 3.624556, // 128 cubic feet
        Unit::Volume(Volume::Teaspoon) => 4.92892e-6, // US teaspoon
        Unit::Volume(Volume::Tablespoon) => 1.47868e-5, // US tablespoon
        Unit::Volume(Volume::Cup) => 0.000236588, // US cup
        Unit::Volume(Volume::AcreFoot) => 1233.48,
        Unit::Volume(Volume::StandardCubicFoot) => 0.028316846592, // same as cubic foot

        // Force (Base: Newton)
        Unit::Force(Force::Newton) => 1.0,
        Unit::Force(Force::Millinewton) => 0.001,
        Unit::Force(Force::Kilonewton) => 1000.0,
        Unit::Force(Force::Meganewton) => 1_000_000.0,
        Unit::Force(Force::Dyne) => 1e-5,
        Unit::Force(Force::KilogramForce) => 9.80665,
        Unit::Force(Force::GramForce) => 0.00980665,
        Unit::Force(Force::PoundForce) => 4.44822,
        Unit::Force(Force::OunceForce) => 0.278014,
        Unit::Force(Force::TonForce) => 9806.65, // Metric ton-force
        Unit::Force(Force::Poundal) => 0.138255,
        Unit::Force(Force::Kip) => 4448.22,
        Unit::Force(Force::Sthene) => 1000.0,
        Unit::Force(Force::JoulePerMeter) => 1.0, // 1 J/m = 1 N

        // Energy (Base: Joule)
        Unit::Energy(Energy::Joule) => 1.0,
        Unit::Energy(Energy::Calorie) => 4.184, // Thermochemical calorie
        Unit::Energy(Energy::Kilocalorie) => 4184.0, // Food calorie
        Unit::Energy(Energy::KilowattHour) => 3.6e6,
        Unit::Energy(Energy::WattHour) => 3600.0,
        Unit::Energy(Energy::Electronvolt) => 1.60218e-19,
        Unit::Energy(Energy::Hartree) => 4.359744e-18,
        Unit::Energy(Energy::BritishThermalUnit) => 1055.06,
        Unit::Energy(Energy::FootPound) => 1.35582,
        Unit::Energy(Energy::Erg) => 1e-7,
        Unit::Energy(Energy::Therm) => 1.05506e8, // US therm
        Unit::Energy(Energy::TonOfTntEquivalent) => 4.184e9,
        Unit::Energy(Energy::HorsepowerHour) => 2.68452e6,
        Unit::Energy(Energy::BarrelOfOilEquivalent) => 6.1178632e9, // approx

        // Temperature (Base: Kelvin)
        Unit::Temperature(Temperature::Kelvin) => 1.0,
        Unit::Temperature(Temperature::Celsius) => 1.0, // Note: Conversion needs offset for absolute values
        Unit::Temperature(Temperature::Fahrenheit) => 5.0/9.0, // Note: Conversion needs offset for absolute values
        Unit::Temperature(Temperature::Rankine) => 5.0/9.0,
        Unit::Temperature(Temperature::Reaumur) => 1.25, // 1 K = 1.25 °Ré
        Unit::Temperature(Temperature::Delisle) => -2.0/3.0, // 1 K = -2/3 °De (relative change)
        Unit::Temperature(Temperature::NewtonScale) => 100.0/33.0, // 1 K = 100/33 °N (relative change)
        Unit::Temperature(Temperature::Romer) => 40.0/21.0, // 1 K = 40/21 °Ro (relative change)
        Unit::Temperature(Temperature::Leiden) => 1.0, // Used for cryogenic temperatures, 1 K = 1 L
        Unit::Temperature(Temperature::PlanckTemperature) => 1.416785e32,

        // Time (Base: Second)
        Unit::Time(Time::Second) => 1.0,
        Unit::Time(Time::Minute) => 60.0,
        Unit::Time(Time::Hour) => 3600.0,
        Unit::Time(Time::Day) => 86400.0,
        Unit::Time(Time::Week) => 604800.0,
        Unit::Time(Time::Month) => 2.629746e6, // Average Gregorian month
        Unit::Time(Time::Year) => 3.15569e7, // Average Gregorian year
        Unit::Time(Time::Decade) => 3.15569e8,
        Unit::Time(Time::Century) => 3.15569e9,
        Unit::Time(Time::Millennium) => 3.15569e10,
        Unit::Time(Time::Microsecond) => 1e-6,
        Unit::Time(Time::Millisecond) => 1e-3,
        Unit::Time(Time::Nanosecond) => 1e-9,
        Unit::Time(Time::Picosecond) => 1e-12,
        Unit::Time(Time::Fortnight) => 1_209_600.0, // 14 days
        Unit::Time(Time::Shake) => 1e-8,
        Unit::Time(Time::JulianYear) => 3.15576e7,
        Unit::Time(Time::LeapYear) => 3.16224e7,
        Unit::Time(Time::SiderealDay) => 86164.09053,
        Unit::Time(Time::PlanckTime) => 5.391247e-44,

        // Speed (Base: Meter per Second)
        Unit::Speed(Speed::MeterPerSecond) => 1.0,
        Unit::Speed(Speed::KilometerPerHour) => 1000.0 / 3600.0, // km/h to m/s
        Unit::Speed(Speed::MilePerHour) => 1609.344 / 3600.0, // mph to m/s
        Unit::Speed(Speed::Knot) => 0.514444, // nautical miles per hour to m/s
        Unit::Speed(Speed::FootPerSecond) => 0.3048,
        Unit::Speed(Speed::Mach) => 343.0, // Approx speed of sound in dry air at 20°C
        Unit::Speed(Speed::SpeedOfLight) => 299792458.0, // exact
        Unit::Speed(Speed::InchPerSecond) => 0.0254,
        Unit::Speed(Speed::FurlongPerFortnight) => 201.168 / 1_209_600.0, // furlongs/fortnight to m/s
        Unit::Speed(Speed::BeaufortScale) => 0.5, // Approx 1 Beaufort = 0.5 m/s
        Unit::Speed(Speed::CosmicVelocity) => 11200.0, // Earth's escape velocity (approx)
        Unit::Speed(Speed::GalileoUnit) => 0.00001, // 1 cm/s
        Unit::Speed(Speed::Benz) => 1.0, // 1 Benz = 1 m/s
        Unit::Speed(Speed::YardPerSecond) => 0.9144,
        Unit::Speed(Speed::EarthsRotationSpeed) => 465.1, // at equator in m/s
        Unit::Speed(Speed::ParsecPerYear) => 3.086e16 / 3.15569e7, // pc/year to m/s
        Unit::Speed(Speed::SpeedOfLightC) => 299792458.0, // exact
        Unit::Speed(Speed::MilePerMinute) => 1609.344 / 60.0,
        Unit::Speed(Speed::KilometerPerSecond) => 1000.0,

        // Pressure (Base: Pascal)
        Unit::Pressure(Pressure::Pascal) => 1.0,
        Unit::Pressure(Pressure::Decipascal) => 0.1,
        Unit::Pressure(Pressure::Hectopascal) => 100.0,
        Unit::Pressure(Pressure::Kilopascal) => 1000.0,
        Unit::Pressure(Pressure::Megapascal) => 1_000_000.0,
        Unit::Pressure(Pressure::PlanckPressure) => 4.63309e113,
        Unit::Pressure(Pressure::Bar) => 100_000.0,
        Unit::Pressure(Pressure::AtmosphereStandard) => 101325.0,
        Unit::Pressure(Pressure::Torr) => 133.322,
        Unit::Pressure(Pressure::PoundPerSquareInch) => 6894.76,
        Unit::Pressure(Pressure::KipPerSquareInch) => 6.89476e6,
        Unit::Pressure(Pressure::MillimeterOfMercury) => 133.322,
        Unit::Pressure(Pressure::InchOfMercury) => 3386.39,
        Unit::Pressure(Pressure::InchOfWater) => 249.089, // at 4°C
        Unit::Pressure(Pressure::FootOfWater) => 2989.07, // at 4°C
        Unit::Pressure(Pressure::DynePerSquareCentimeter) => 0.1,
        Unit::Pressure(Pressure::Barye) => 0.1,
        Unit::Pressure(Pressure::TechnicalAtmosphere) => 98066.5,
        Unit::Pressure(Pressure::PoundalPerSquareFoot) => 1.48816,
        Unit::Pressure(Pressure::SthenePerSquareMeter) => 1000.0,

        // Frequency (Base: Hertz)
        Unit::Frequency(Frequency::Hertz) => 1.0,
        Unit::Frequency(Frequency::Millihertz) => 0.001,
        Unit::Frequency(Frequency::Microhertz) => 1e-6,
        Unit::Frequency(Frequency::Nanohertz) => 1e-9,
        Unit::Frequency(Frequency::Kilohertz) => 1000.0,
        Unit::Frequency(Frequency::Megahertz) => 1_000_000.0,
        Unit::Frequency(Frequency::Gigahertz) => 1_000_000_000.0,
        Unit::Frequency(Frequency::Terahertz) => 1_000_000_000_000.0,
        Unit::Frequency(Frequency::PlanckFrequency) => 1.8549e43,
        Unit::Frequency(Frequency::RevolutionsPerMinute) => 1.0 / 60.0,
        Unit::Frequency(Frequency::RadianPerSecond) => 1.0 / (2.0 * std::f64::consts::PI),
        Unit::Frequency(Frequency::CyclesPerSecond) => 1.0,
        Unit::Frequency(Frequency::FramesPerSecond) => 1.0,
        Unit::Frequency(Frequency::BeatsPerMinute) => 1.0 / 60.0,
        Unit::Frequency(Frequency::CountsPerMinute) => 1.0 / 60.0,
        Unit::Frequency(Frequency::Fresnel) => 1e12,
        Unit::Frequency(Frequency::ReciprocalSecond) => 1.0,
        Unit::Frequency(Frequency::DegreePerSecond) => 1.0 / 360.0,
        Unit::Frequency(Frequency::GradianPerSecond) => 1.0 / 400.0,

        // Electric Current (Base: Ampere)
        Unit::ElectricCurrent(ElectricCurrent::Ampere) => 1.0,
        Unit::ElectricCurrent(ElectricCurrent::Picoampere) => 1e-12,
        Unit::ElectricCurrent(ElectricCurrent::Nanoampere) => 1e-9,
        Unit::ElectricCurrent(ElectricCurrent::Microampere) => 1e-6,
        Unit::ElectricCurrent(ElectricCurrent::Milliampere) => 1e-3,
        Unit::ElectricCurrent(ElectricCurrent::Kiloampere) => 1000.0,
        Unit::ElectricCurrent(ElectricCurrent::Megaampere) => 1_000_000.0,
        Unit::ElectricCurrent(ElectricCurrent::Teraampere) => 1_000_000_000_000.0,
        Unit::ElectricCurrent(ElectricCurrent::PlanckCurrent) => 3.4789e25,
        Unit::ElectricCurrent(ElectricCurrent::Statampere) => 3.33564e-10, // 1/c_light * 10^-9
        Unit::ElectricCurrent(ElectricCurrent::Abampere) => 10.0,
        Unit::ElectricCurrent(ElectricCurrent::Biot) => 10.0,
        Unit::ElectricCurrent(ElectricCurrent::FranklinPerSecond) => 3.33564e-10,
        Unit::ElectricCurrent(ElectricCurrent::CoulombPerSecond) => 1.0,
        Unit::ElectricCurrent(ElectricCurrent::VoltPerOhm) => 1.0,
        Unit::ElectricCurrent(ElectricCurrent::WattPerVolt) => 1.0,
        Unit::ElectricCurrent(ElectricCurrent::SiemensVolt) => 1.0,
        Unit::ElectricCurrent(ElectricCurrent::EsuPerSecond) => 3.33564e-10,
        Unit::ElectricCurrent(ElectricCurrent::EmuOfCurrent) => 10.0,
        Unit::ElectricCurrent(ElectricCurrent::Gilbert) => 0.795775, // 1 Gilbert = 0.795775 Ampere-turns

        // Voltage (Base: Volt)
        Unit::Voltage(Voltage::Volt) => 1.0,
        Unit::Voltage(Voltage::Picovolt) => 1e-12,
        Unit::Voltage(Voltage::Nanovolt) => 1e-9,
        Unit::Voltage(Voltage::Microvolt) => 1e-6,
        Unit::Voltage(Voltage::Millivolt) => 1e-3,
        Unit::Voltage(Voltage::Decivolt) => 0.1,
        Unit::Voltage(Voltage::Centivolt) => 0.01,
        Unit::Voltage(Voltage::Hectovolt) => 100.0,
        Unit::Voltage(Voltage::Kilovolt) => 1000.0,
        Unit::Voltage(Voltage::Megavolt) => 1_000_000.0,
        Unit::Voltage(Voltage::Gigavolt) => 1_000_000_000.0,
        Unit::Voltage(Voltage::Teravolt) => 1_000_000_000_000.0,
        Unit::Voltage(Voltage::PlanckVoltage) => 1.04295e-1,
        Unit::Voltage(Voltage::Statvolt) => 299.792458,
        Unit::Voltage(Voltage::Abvolt) => 1e-8,
        Unit::Voltage(Voltage::EsuOfPotential) => 299.792458,
        Unit::Voltage(Voltage::EmuOfPotential) => 1e-8,
        Unit::Voltage(Voltage::WattPerAmpere) => 1.0,
        Unit::Voltage(Voltage::JoulePerCoulomb) => 1.0,
        Unit::Voltage(Voltage::ElectronvoltPerElementaryCharge) => 1.0, // 1 eV/e = 1 V

        // Capacitance (Base: Farad)
        Unit::Capacitance(Capacitance::Farad) => 1.0,
        Unit::Capacitance(Capacitance::Attofarad) => 1e-18,
        Unit::Capacitance(Capacitance::Femtofarad) => 1e-15,
        Unit::Capacitance(Capacitance::Picofarad) => 1e-12,
        Unit::Capacitance(Capacitance::Nanofarad) => 1e-9,
        Unit::Capacitance(Capacitance::Microfarad) => 1e-6,
        Unit::Capacitance(Capacitance::Millifarad) => 1e-3,
        Unit::Capacitance(Capacitance::Decafarad) => 10.0,
        Unit::Capacitance(Capacitance::Hectofarad) => 100.0,
        Unit::Capacitance(Capacitance::Kilofarad) => 1000.0,
        Unit::Capacitance(Capacitance::Megafarad) => 1_000_000.0,
        Unit::Capacitance(Capacitance::Terafarad) => 1_000_000_000_000.0,
        Unit::Capacitance(Capacitance::Gigafarad) => 1_000_000_000.0,
        Unit::Capacitance(Capacitance::PlanckCapacitance) => 1.0878e-32,
        Unit::Capacitance(Capacitance::Statfarad) => 1.11265e-12, // 1 / (c^2 * 10^-9)
        Unit::Capacitance(Capacitance::Abfarad) => 1e9,
        Unit::Capacitance(Capacitance::EsuOfCapacitance) => 1.11265e-12,
        Unit::Capacitance(Capacitance::EmuOfCapacitance) => 1e9,
        Unit::Capacitance(Capacitance::CoulombPerVolt) => 1.0,
        Unit::Capacitance(Capacitance::SecondPerOhm) => 1.0,

        // Luminosity (Base: Candela)
        Unit::Luminosity(Luminosity::Candela) => 1.0,
        Unit::Luminosity(Luminosity::Lumen) => 1.0, // Lumen is luminous flux, Candela is luminous intensity.
                                                    // This conversion is highly context-dependent (solid angle).
                                                    // For simplicity, assuming 1 cd = 1 lm/sr, and 1 lm = 1 cd * sr.
                                                    // This will be simplified to a direct conversion for now.
        Unit::Luminosity(Luminosity::Lux) => 1.0, // Lux is illuminance (lm/m^2). Not directly comparable to Candela.
                                                  // This conversion is simplified.
        Unit::Luminosity(Luminosity::CandelaPerSquareMeter) => 1.0, // Base unit for luminance
        Unit::Luminosity(Luminosity::LumenPerSquareMeter) => 1.0, // Same as Lux
        Unit::Luminosity(Luminosity::Nit) => 1.0, // Same as Candela per Square Meter
        Unit::Luminosity(Luminosity::Stilb) => 10000.0, // cd/cm^2 to cd/m^2
        Unit::Luminosity(Luminosity::Apostilb) => 1.0 / std::f64::consts::PI, // asb to cd/m^2
        Unit::Luminosity(Luminosity::Lambert) => 10000.0 / std::f64::consts::PI, // L to cd/m^2
        Unit::Luminosity(Luminosity::FootLambert) => 3.426259, // fL to cd/m^2
        Unit::Luminosity(Luminosity::Phot) => 10000.0, // lm/cm^2 to lm/m^2 (lux)
        Unit::Luminosity(Luminosity::Bril) => 1e-8, // 1 bril = 10^-8 lux
        Unit::Luminosity(Luminosity::Skot) => 1e-3, // 1 skot = 10^-3 lux
        Unit::Luminosity(Luminosity::WattPerSteradian) => 1.0, // Radiometric unit, not directly comparable to photometric.
                                                                // Placeholder for now.
        Unit::Luminosity(Luminosity::LumenPerWatt) => 1.0, // Luminous efficacy. Placeholder.
        Unit::Luminosity(Luminosity::LumenSecond) => 1.0, // Luminous energy (Talbot). Placeholder.
        Unit::Luminosity(Luminosity::Talbot) => 1.0, // Luminous energy. Placeholder.
        Unit::Luminosity(Luminosity::WattPerSteradianSquareMeter) => 1.0, // Radiance. Placeholder.
        Unit::Luminosity(Luminosity::Candlepower) => 1.0, // Historically 1 cp = 1 cd
        Unit::Luminosity(Luminosity::PlanckLuminance) => 1.2227e59, // Planck luminance in cd/m^2

        // Radiation (Base: Gray)
        Unit::Radiation(Radiation::Gray) => 1.0,
        Unit::Radiation(Radiation::Decigray) => 0.1,
        Unit::Radiation(Radiation::Centigray) => 0.01,
        Unit::Radiation(Radiation::Milligray) => 0.001,
        Unit::Radiation(Radiation::Microgray) => 1e-6,
        Unit::Radiation(Radiation::Nanogray) => 1e-9,
        Unit::Radiation(Radiation::Picogray) => 1e-12,
        Unit::Radiation(Radiation::Rad) => 0.01,
        Unit::Radiation(Radiation::Sievert) => 1.0, // Equivalent dose, not absorbed dose. Simplified.
        Unit::Radiation(Radiation::Rem) => 0.01, // Equivalent dose, not absorbed dose. Simplified.
        Unit::Radiation(Radiation::Becquerel) => 1.0, // Activity. Not absorbed dose. Placeholder.
        Unit::Radiation(Radiation::Curie) => 3.7e10, // Activity. Not absorbed dose. Placeholder.
        Unit::Radiation(Radiation::Rutherford) => 1e6, // Activity. Not absorbed dose. Placeholder.
        Unit::Radiation(Radiation::Roentgen) => 0.00877, // Exposure to absorbed dose in air. Placeholder.
        Unit::Radiation(Radiation::CoulombPerKilogram) => 38.75, // Exposure to absorbed dose in air. Placeholder.
        Unit::Radiation(Radiation::ElectronvoltPerKilogram) => 1.60218e-19, // Energy per mass. Placeholder.
        Unit::Radiation(Radiation::IonPairPerKilogram) => 1.0, // Placeholder.
        Unit::Radiation(Radiation::Rep) => 0.0093, // Roentgen equivalent physical. Placeholder.
        Unit::Radiation(Radiation::WattPerKilogram) => 1.0, // Absorbed dose rate. Placeholder.
        Unit::Radiation(Radiation::PlanckRadiation) => 1.0, // Placeholder.

        // Magnetic Flux (Base: Weber)
        Unit::MagneticFlux(MagneticFlux::Weber) => 1.0,
        Unit::MagneticFlux(MagneticFlux::Microweber) => 1e-6,
        Unit::MagneticFlux(MagneticFlux::Nanoweber) => 1e-9,
        Unit::MagneticFlux(MagneticFlux::Picoweber) => 1e-12,
        Unit::MagneticFlux(MagneticFlux::Femtoweber) => 1e-15,
        Unit::MagneticFlux(MagneticFlux::Attoweber) => 1e-18,
        Unit::MagneticFlux(MagneticFlux::Maxwell) => 1e-8,
        Unit::MagneticFlux(MagneticFlux::Megaline) => 1e-2, // 1 Megaline = 10^6 Maxwell = 0.01 Weber
        Unit::MagneticFlux(MagneticFlux::Kiloline) => 1e-5, // 1 Kiloline = 10^3 Maxwell = 10^-5 Weber
        Unit::MagneticFlux(MagneticFlux::Milliline) => 1e-8, // 1 Milliline = 10^-3 Maxwell = 10^-8 Weber
        Unit::MagneticFlux(MagneticFlux::GammaSquareCentimeter) => 1e-12, // 1 Gamma = 1 nT. 1 nT*cm^2 = 10^-12 Weber
        Unit::MagneticFlux(MagneticFlux::TeslaSquareMeter) => 1.0, // 1 T*m^2 = 1 Weber
        Unit::MagneticFlux(MagneticFlux::GaussSquareCentimeter) => 1e-8, // 1 G*cm^2 = 1 Maxwell = 10^-8 Weber
        Unit::MagneticFlux(MagneticFlux::VoltSecond) => 1.0, // 1 V*s = 1 Weber
        Unit::MagneticFlux(MagneticFlux::CoulombOhm) => 1.0, // 1 C*Ohm = 1 V*s = 1 Weber
        Unit::MagneticFlux(MagneticFlux::JoulePerAmpere) => 1.0, // 1 J/A = 1 V*s = 1 Weber
        Unit::MagneticFlux(MagneticFlux::HenryAmpere) => 1.0, // 1 H*A = 1 V*s = 1 Weber
        Unit::MagneticFlux(MagneticFlux::MagneticFluxQuantum) => 2.06783384e-15,
        Unit::MagneticFlux(MagneticFlux::UnitPole) => 1.256637e-7, // 1 unit pole = 4pi * 10^-7 Weber
        Unit::MagneticFlux(MagneticFlux::PlanckFlux) => 2.06783384e-15, // Same as magnetic flux quantum
    }
}

pub fn convert(quantity: f64, from_unit: &Unit, to_unit: &Unit) -> Option<f64> {
    // 1. Check if dimensions match
    let from_dimension = get_unit_dimension(from_unit);
    let to_dimension = get_unit_dimension(to_unit);

    if from_dimension != to_dimension {
        eprintln!("Error: Cannot convert from {:?} to {:?} (different physical dimensions)", from_unit, to_unit);
        return None;
    }

    // Special handling for temperature conversions (require offset)
    if from_dimension == Dimension::Temperature {
        return convert_temperature(quantity, from_unit, to_unit);
    }

    // 2. Convert 'from_unit' quantity to base unit quantity
    let from_factor = get_conversion_factor_to_base(from_unit);
    if from_factor == 0.0 { // This indicates a missing or invalid conversion factor
        eprintln!("Error: Missing or invalid conversion factor for {:?}", from_unit);
        return None;
    }
    let base_quantity = quantity * from_factor;

    // 3. Convert base unit quantity to 'to_unit' quantity
    let to_factor = get_conversion_factor_to_base(to_unit);
    if to_factor == 0.0 { // This indicates a missing or invalid conversion factor
        eprintln!("Error: Missing or invalid conversion factor for {:?}", to_unit);
        return None;
    }
    let converted_quantity = base_quantity / to_factor;

    Some(converted_quantity)
}

fn convert_temperature(quantity: f64, from_unit: &Unit, to_unit: &Unit) -> Option<f64> {
    let kelvin_val = match from_unit {
        Unit::Temperature(Temperature::Celsius) => quantity + 273.15,
        Unit::Temperature(Temperature::Fahrenheit) => (quantity - 32.0) * 5.0/9.0 + 273.15,
        Unit::Temperature(Temperature::Kelvin) => quantity,
        Unit::Temperature(Temperature::Rankine) => quantity * 5.0/9.0,
        Unit::Temperature(Temperature::Reaumur) => quantity * 5.0/4.0 + 273.15,
        Unit::Temperature(Temperature::Delisle) => 373.15 - quantity * 2.0/3.0,
        Unit::Temperature(Temperature::NewtonScale) => quantity * 100.0/33.0 + 273.15,
        Unit::Temperature(Temperature::Romer) => (quantity - 7.5) * 40.0/21.0 + 273.15,
        Unit::Temperature(Temperature::Leiden) => quantity, // Assuming Leiden is equivalent to Kelvin for practical purposes here
        Unit::Temperature(Temperature::PlanckTemperature) => quantity * 1.416785e32, // Direct conversion, no offset
        _ => {
            eprintln!("Error: Unsupported temperature unit for conversion: {:?}", from_unit);
            return None;
        }
    };

    let converted_val = match to_unit {
        Unit::Temperature(Temperature::Celsius) => kelvin_val - 273.15,
        Unit::Temperature(Temperature::Fahrenheit) => (kelvin_val - 273.15) * 9.0/5.0 + 32.0,
        Unit::Temperature(Temperature::Kelvin) => kelvin_val,
        Unit::Temperature(Temperature::Rankine) => kelvin_val * 9.0/5.0,
        Unit::Temperature(Temperature::Reaumur) => (kelvin_val - 273.15) * 4.0/5.0,
        Unit::Temperature(Temperature::Delisle) => (373.15 - kelvin_val) * 3.0/2.0,
        Unit::Temperature(Temperature::NewtonScale) => (kelvin_val - 273.15) * 33.0/100.0,
        Unit::Temperature(Temperature::Romer) => (kelvin_val - 273.15) * 21.0/40.0 + 7.5,
        Unit::Temperature(Temperature::Leiden) => kelvin_val,
        Unit::Temperature(Temperature::PlanckTemperature) => kelvin_val / 1.416785e32,
        _ => {
            eprintln!("Error: Unsupported temperature unit for conversion: {:?}", to_unit);
            return None;
        }
    };
    Some(converted_val)
}


pub fn string_to_target(unit: String) -> Option<Unit> {
    let lower_unit = unit.to_lowercase();
    match lower_unit.as_str() {
        // --- Length Units ---
        "kilometer" | "kilometers" | "km" => Some(Unit::Length(Length::Kilometer)),
        "hectometer" | "hectometers" | "hm" => Some(Unit::Length(Length::Hectometer)),
        "decameter" | "decameters" | "decam" | "dekameter" | "dekameters" | "dam" => Some(Unit::Length(Length::Decameter)),
        "meter" | "meters" | "m" => Some(Unit::Length(Length::Meter)),
        "decimeter" | "decimeters" | "decim" | "dm" => Some(Unit::Length(Length::Decimeter)), // Added dm
        "centimeter" | "centimeters" | "cm" => Some(Unit::Length(Length::Centimeter)),
        "millimeter" | "millimeters" | "mm" => Some(Unit::Length(Length::Millimeter)),
        "micrometer" | "micrometers" | "micron" | "microns" | "µm" => Some(Unit::Length(Length::Micron)), // Added µm
        "nanometer" | "nanometers" | "nm" => Some(Unit::Length(Length::Nanometer)),
        "picometer" | "picometers" | "pm" => Some(Unit::Length(Length::Picometer)),
        "femtometer" | "femtometers" | "fm" => Some(Unit::Length(Length::Femtometer)),
        "attometer" | "attometers" | "am" => Some(Unit::Length(Length::Attometer)),
        "light_year" | "light_years" | "lightyear" | "lightyears" | "ly" => Some(Unit::Length(Length::LightYear)),
        "astronomical_unit" | "astronomical_units" | "astronomicalunit" | "astronomicalunits" | "au" => Some(Unit::Length(Length::AstronomicalUnit)),
        "parsec" | "parsecs" | "pc" => Some(Unit::Length(Length::Parsec)),
        "angstrom" | "angstroms" | "å" | "a" => Some(Unit::Length(Length::Angstrom)), // Added a
        "inch" | "inches" | "in" => Some(Unit::Length(Length::Inch)),
        "foot" | "feet" | "ft" => Some(Unit::Length(Length::Foot)),
        "yard" | "yards" | "yd" => Some(Unit::Length(Length::Yard)),
        "mile" | "miles" | "mi" => Some(Unit::Length(Length::Mile)),
        "nautical_mile" | "nautical_miles" | "nauticalmile" | "nauticalmiles" | "nmi" => Some(Unit::Length(Length::NauticalMile)), // Added nmi
        "league" | "leagues" => Some(Unit::Length(Length::League)),
        "furlong" | "furlongs" => Some(Unit::Length(Length::Furlong)),
        "rod" | "rods" => Some(Unit::Length(Length::Rod)),
        "chain" | "chains" => Some(Unit::Length(Length::Chain)),
        "football_field" | "football_fields" | "football_field_length" | "football_field_lengths" => Some(Unit::Length(Length::FootballFieldLength)),
        "whale" | "whales" | "whale_length" | "whale_lengths" => Some(Unit::Length(Length::WhaleLength)),
        
        // --- Mass Units ---
        "kilogram" | "kilograms" | "kg" => Some(Unit::Mass(Mass::Kilogram)),
        "hectogram" | "hectograms" | "hg" => Some(Unit::Mass(Mass::Hectogram)),
        "decagram" | "decagrams" | "dag" => Some(Unit::Mass(Mass::Decagram)),
        "gram" | "grams" | "g" => Some(Unit::Mass(Mass::Gram)),
        "decigram" | "decigrams" | "dg" => Some(Unit::Mass(Mass::Decigram)),
        "centigram" | "centigrams" | "cg" => Some(Unit::Mass(Mass::Centigram)),
        "milligram" | "milligrams" | "mg" => Some(Unit::Mass(Mass::Milligram)),
        "microgram" | " micrograms" | "mcg" | "µg" => Some(Unit::Mass(Mass::Microgram)),
        "nanogram" | "nanograms" | "ng" => Some(Unit::Mass(Mass::Nanogram)),
        "picogram" | "picograms" | "pg" => Some(Unit::Mass(Mass::Picogram)),
        "femtogram" | "femtograms" | "fg" => Some(Unit::Mass(Mass::Femtogram)),
        "attogram" | "attograms" | "ag" => Some(Unit::Mass(Mass::Attogram)),
        "ton" | "tons" => Some(Unit::Mass(Mass::Ton)), // Assumed short ton
        "metric_ton" | "metric_tons" | "tonne" | "tonnes" => Some(Unit::Mass(Mass::MetricTon)),
        "quintal" | "quintals" | "centner" | "centners" => Some(Unit::Mass(Mass::Quintal)),
        "pound" | "pounds" | "lb" | "lbs" => Some(Unit::Mass(Mass::Pounds)),
        "ounce" | "ounces" | "oz" => Some(Unit::Mass(Mass::Ounce)),
        "stone" | "stones" => Some(Unit::Mass(Mass::Stone)),
        "carat" | "carats" | "ct" => Some(Unit::Mass(Mass::Carat)),

        // --- Area Units ---
        "square_meter" | "square_meters" | "sqm" | "m2" => Some(Unit::Area(Area::SquareMeter)),
        "square_kilometer" | "square_kilometers" | "sqkm" | "km2" => Some(Unit::Area(Area::SquareKilometer)),
        "square_centimeter" | "square_centimeters" | "sqcm" | "cm2" => Some(Unit::Area(Area::SquareCentimeter)),
        "square_millimeter" | "square_millimeters" | "sqmm" | "mm2" => Some(Unit::Area(Area::SquareMillimeter)),
        "square_decimeter" | "square_decimeters" | "sqdm" | "dm2" => Some(Unit::Area(Area::SquareDecimeter)),
        "hectare" | "hectares" | "ha" => Some(Unit::Area(Area::Hectare)),
        "acre" | "acres" => Some(Unit::Area(Area::Acre)),
        "square_foot" | "square_feet" | "sqft" | "ft2" => Some(Unit::Area(Area::SquareFoot)),
        "square_inch" | "square_inches" | "sqin" | "in2" => Some(Unit::Area(Area::SquareInch)),
        "square_yard" | "square_yards" | "sqyd" | "yd2" => Some(Unit::Area(Area::SquareYard)),
        "square_mile" | "square_miles" | "sqmi" | "mi2" => Some(Unit::Area(Area::SquareMile)),

        // --- Volume Units ---
        "cubic_meter" | "cubic_meters" | "cum" | "m3" => Some(Unit::Volume(Volume::CubicMeter)),
        "cubic_decimeter" | "cubic_decimeters" | "cudm" | "dm3" => Some(Unit::Volume(Volume::CubicDecimeter)),
        "cubic_centimeter" | "cubic_centimeters" | "cucc" | "cc" | "cm3" => Some(Unit::Volume(Volume::CubicCentimeter)),
        "cubic_millimeter" | "cubic_millimeters" | "cumm" | "mm3" => Some(Unit::Volume(Volume::CubicMillimeter)),
        "liter" | "liters" | "l" => Some(Unit::Volume(Volume::Liter)),
        "milliliter" | "milliliters" | "ml" | "mL" => Some(Unit::Volume(Volume::Milliliter)),
        "deciliter" | "deciliters" | "dl" => Some(Unit::Volume(Volume::Deciliter)),
        "kiloliter" | "kiloliters" | "kl" => Some(Unit::Volume(Volume::Kiloliter)),
        "megaliter" | "megaliters" => Some(Unit::Volume(Volume::Megaliter)), // Removed duplicate "ml"
        "gigaliter" | "gigaliters" | "gl" => Some(Unit::Volume(Volume::Gigaliter)),
        "teraliter" | "teraliters" | "tl" => Some(Unit::Volume(Volume::Teraliter)),
        "cubic_inch" | "cubic_inches" | "cuin" | "in3" => Some(Unit::Volume(Volume::CubicInch)),
        "cubic_foot" | "cubic_feet" | "cuft" | "ft3" => Some(Unit::Volume(Volume::CubicFoot)),
        "cubic_yard" | "cubic_yards" | "cuyd" | "yd3" => Some(Unit::Volume(Volume::CubicYard)),
        "cubic_mile" | "cubic_miles" | "cumi" | "mi3" => Some(Unit::Volume(Volume::CubicMile)),
        "teracubic_feet" | "tcf" => Some(Unit::Volume(Volume::TeracubicFeet)),
        "fluid_ounce" | "fluid_ounces" | "floz" => Some(Unit::Volume(Volume::FluidOunce)),
        "pint" | "pints" | "pt" => Some(Unit::Volume(Volume::Pint)),
        "quart" | "quarts" | "qt" => Some(Unit::Volume(Volume::Quart)),
        "gallon" | "gallons" | "gal" => Some(Unit::Volume(Volume::Gallon)),
        "barrel_liquid" | "barrel_liquids" | "bbl" => Some(Unit::Volume(Volume::BarrelLiquid)),
        "bushel" | "bushels" | "bu" => Some(Unit::Volume(Volume::Bushel)),
        "peck" | "pecks" => Some(Unit::Volume(Volume::Peck)),
        "cord" | "cords" => Some(Unit::Volume(Volume::Cord)),
        "teaspoon" | "teaspoons" | "tsp" => Some(Unit::Volume(Volume::Teaspoon)),
        "tablespoon" | "tablespoons" | "tbsp" => Some(Unit::Volume(Volume::Tablespoon)),
        "cup" | "cups" => Some(Unit::Volume(Volume::Cup)),
        "acre_foot" | "acre_feet" | "acft" => Some(Unit::Volume(Volume::AcreFoot)),
        "standard_cubic_foot" | "standard_cubic_feet" | "scf" => Some(Unit::Volume(Volume::StandardCubicFoot)),

        // --- Force Units ---
        "newton" | "newtons" | "n" => Some(Unit::Force(Force::Newton)),
        "millinewton" | "millinewtons" | "mn" => Some(Unit::Force(Force::Millinewton)),
        "kilonewton" | "kilonewtons" | "kn" => Some(Unit::Force(Force::Kilonewton)),
        "meganewton" | "meganewtons" => Some(Unit::Force(Force::Meganewton)), // Removed duplicate "mn"
        "dyne" | "dynes" | "dyn" => Some(Unit::Force(Force::Dyne)),
        "kilogram_force" | "kilogram_forces" | "kilopond" | "kiloponds" | "kgf" | "kp" => Some(Unit::Force(Force::KilogramForce)),
        "gram_force" | "gram_forces" | "gf" => Some(Unit::Force(Force::GramForce)),
        "pound_force" | "pound_forces" | "lbf" => Some(Unit::Force(Force::PoundForce)),
        "ounce_force" | "ounce_forces" | "ozf" => Some(Unit::Force(Force::OunceForce)),
        "ton_force" | "ton_forces" => Some(Unit::Force(Force::TonForce)),
        "poundal" | "poundals" | "pdl" => Some(Unit::Force(Force::Poundal)),
        "kip" | "kips" => Some(Unit::Force(Force::Kip)),
        "sthene" | "sthenes" => Some(Unit::Force(Force::Sthene)),
        "joule_per_meter" | "joules_per_meter" | "j/m" => Some(Unit::Force(Force::JoulePerMeter)),

        // --- Energy Units ---
        "joule" | "joules" | "j" => Some(Unit::Energy(Energy::Joule)),
        "calorie" | "calories" | "cal" => Some(Unit::Energy(Energy::Calorie)),
        "kilocalorie" | "kilocalories" | "kcal" | "cal_(nutrition)" => Some(Unit::Energy(Energy::Kilocalorie)),
        "kilowatt_hour" | "kilowatt_hours" | "kwh" => Some(Unit::Energy(Energy::KilowattHour)),
        "watt_hour" | "watt_hours" | "wh" => Some(Unit::Energy(Energy::WattHour)),
        "electronvolt" | "electronvolts" | "ev" => Some(Unit::Energy(Energy::Electronvolt)),
        "hartree" | "hartrees" => Some(Unit::Energy(Energy::Hartree)),
        "british_thermal_unit" | "british_thermal_units" | "btu" => Some(Unit::Energy(Energy::BritishThermalUnit)),
        "foot_pound" | "foot_pounds" | "foot_pound_force" | "foot_pound_forces" => Some(Unit::Energy(Energy::FootPound)),
        "erg" | "ergs" => Some(Unit::Energy(Energy::Erg)),
        "therm" | "therms" => Some(Unit::Energy(Energy::Therm)),
        "ton_of_tnt_equivalent" | "tons_of_tnt_equivalent" => Some(Unit::Energy(Energy::TonOfTntEquivalent)),
        "horsepower_hour" | "horsepower_hours" => Some(Unit::Energy(Energy::HorsepowerHour)),
        "barrel_of_oil_equivalent" | "barrels_of_oil_equivalent" | "boe" => Some(Unit::Energy(Energy::BarrelOfOilEquivalent)),

        // --- Temperature Units ---
        "celsius" | "°c" => Some(Unit::Temperature(Temperature::Celsius)),
        "fahrenheit" | "°f" => Some(Unit::Temperature(Temperature::Fahrenheit)),
        "kelvin" | "k" => Some(Unit::Temperature(Temperature::Kelvin)),
        "rankine" | "rankines" | "°r" => Some(Unit::Temperature(Temperature::Rankine)),
        "reaumur" | "reaumurs" | "re" | "°re" => Some(Unit::Temperature(Temperature::Reaumur)),
        "delisle" | "delisles" | "de" => Some(Unit::Temperature(Temperature::Delisle)),
        "newton_scale" | "newton_scales" | "°n" => Some(Unit::Temperature(Temperature::NewtonScale)),
        "romer" | "romers" | "ro" | "°ro" => Some(Unit::Temperature(Temperature::Romer)),
        "leiden" | "leidens" => Some(Unit::Temperature(Temperature::Leiden)),
        "planck_temperature" | "planck_temperatures" => Some(Unit::Temperature(Temperature::PlanckTemperature)),

        // --- Time Units ---
        "second" | "seconds" | "s" | "sec" => Some(Unit::Time(Time::Second)),
        "minute" | "minutes" | "min" => Some(Unit::Time(Time::Minute)),
        "hour" | "hours" | "hr" => Some(Unit::Time(Time::Hour)),
        "day" | "days" => Some(Unit::Time(Time::Day)),
        "week" | "weeks" | "wk" => Some(Unit::Time(Time::Week)),
        "month" | "months" => Some(Unit::Time(Time::Month)),
        "year" | "years" | "yr" => Some(Unit::Time(Time::Year)),
        "decade" | "decades" => Some(Unit::Time(Time::Decade)),
        "century" | "centuries" => Some(Unit::Time(Time::Century)),
        "millennium" | "millennia" => Some(Unit::Time(Time::Millennium)),
        "microsecond" | "microseconds" | "µs" => Some(Unit::Time(Time::Microsecond)),
        "millisecond" | "milliseconds" | "ms" => Some(Unit::Time(Time::Millisecond)),
        "nanosecond" | "nanoseconds" | "ns" => Some(Unit::Time(Time::Nanosecond)),
        "picosecond" | "picoseconds" | "ps" => Some(Unit::Time(Time::Picosecond)),
        "fortnight" | "fortnights" => Some(Unit::Time(Time::Fortnight)),
        "shake" | "shakes" => Some(Unit::Time(Time::Shake)),
        "julian_year" | "julian_years" => Some(Unit::Time(Time::JulianYear)),
        "leap_year" | "leap_years" => Some(Unit::Time(Time::LeapYear)),
        "sidereal_day" | "sidereal_days" => Some(Unit::Time(Time::SiderealDay)),
        "planck_time" | "planck_times" => Some(Unit::Time(Time::PlanckTime)),

        // --- Speed Units ---
        "meter_per_second" | "meters_per_second" | "m/s" => Some(Unit::Speed(Speed::MeterPerSecond)),
        "kilometer_per_hour" | "kilometers_per_hour" | "km/h" | "kph" => Some(Unit::Speed(Speed::KilometerPerHour)),
        "mile_per_hour" | "miles_per_hour" | "mph" => Some(Unit::Speed(Speed::MilePerHour)),
        "knot" | "knots" | "kt" => Some(Unit::Speed(Speed::Knot)),
        "foot_per_second" | "feet_per_second" | "ft/s" => Some(Unit::Speed(Speed::FootPerSecond)),
        "mach" | "machs" => Some(Unit::Speed(Speed::Mach)),
        "speed_of_light" | "c" => Some(Unit::Speed(Speed::SpeedOfLight)),
        "inch_per_second" | "inches_per_second" | "in/s" => Some(Unit::Speed(Speed::InchPerSecond)),
        "furlong_per_fortnight" | "furlongs_per_fortnight" => Some(Unit::Speed(Speed::FurlongPerFortnight)),
        "beaufort_scale" | "beaufort_scales" => Some(Unit::Speed(Speed::BeaufortScale)),
        "cosmic_velocity" | "cosmic_velocities" => Some(Unit::Speed(Speed::CosmicVelocity)),
        "galileo_unit" | "galileo_units" => Some(Unit::Speed(Speed::GalileoUnit)),
        "benz" | "benzs" => Some(Unit::Speed(Speed::Benz)),
        "yard_per_second" | "yards_per_second" | "yd/s" => Some(Unit::Speed(Speed::YardPerSecond)),
        "earth's_rotation_speed" => Some(Unit::Speed(Speed::EarthsRotationSpeed)),
        "parsec_per_year" | "parsecs_per_year" => Some(Unit::Speed(Speed::ParsecPerYear)),
        "speed_of_light_c" => Some(Unit::Speed(Speed::SpeedOfLightC)),
        "mile_per_minute" | "miles_per_minute" | "mpm" => Some(Unit::Speed(Speed::MilePerMinute)),
        "kilometer_per_second" | "kilometers_per_second" | "km/s" => Some(Unit::Speed(Speed::KilometerPerSecond)),

        // --- Pressure Units ---
        "pascal" | "pascals" | "pa" => Some(Unit::Pressure(Pressure::Pascal)),
        "decipascal" | "decipascals" | "dpa" => Some(Unit::Pressure(Pressure::Decipascal)),
        "hectopascal" | "hectopascals" | "hpa" => Some(Unit::Pressure(Pressure::Hectopascal)),
        "kilopascal" | "kilopascals" | "kpa" => Some(Unit::Pressure(Pressure::Kilopascal)),
        "megapascal" | "megapascals" | "mpa" => Some(Unit::Pressure(Pressure::Megapascal)),
        "planck_pressure" | "planck_pressures" => Some(Unit::Pressure(Pressure::PlanckPressure)),
        "bar" | "bars" => Some(Unit::Pressure(Pressure::Bar)),
        "atmosphere_standard" | "atmosphere_standards" | "atm" => Some(Unit::Pressure(Pressure::AtmosphereStandard)),
        "torr" | "torrs" => Some(Unit::Pressure(Pressure::Torr)),
        "psi" | "pound_per_square_inch" | "pounds_per_square_inch" => Some(Unit::Pressure(Pressure::PoundPerSquareInch)),
        "ksi" | "kip_per_square_inch" | "kips_per_square_inch" => Some(Unit::Pressure(Pressure::KipPerSquareInch)),
        "millimeter_of_mercury" | "millimeters_of_mercury" | "mmhg" => Some(Unit::Pressure(Pressure::MillimeterOfMercury)),
        "inch_of_mercury" | "inches_of_mercury" | "inhg" => Some(Unit::Pressure(Pressure::InchOfMercury)),
        "inch_of_water" | "inches_of_water" | "inwc" => Some(Unit::Pressure(Pressure::InchOfWater)),
        "foot_of_water" | "feet_of_water" | "ftwc" => Some(Unit::Pressure(Pressure::FootOfWater)),
        "dyne_per_square_centimeter" | "dynes_per_square_centimeter" => Some(Unit::Pressure(Pressure::DynePerSquareCentimeter)),
        "barye" | "baryes" | "ba" => Some(Unit::Pressure(Pressure::Barye)),
        "technical_atmosphere" | "technical_atmospheres" | "at" => Some(Unit::Pressure(Pressure::TechnicalAtmosphere)),
        "poundal_per_square_foot" | "poundals_per_square_foot" => Some(Unit::Pressure(Pressure::PoundalPerSquareFoot)),
        "sthene_per_square_meter" | "sthenes_per_square_meter" => Some(Unit::Pressure(Pressure::SthenePerSquareMeter)),

        // --- Frequency Units ---
        "hertz" | "hertzs" | "hz" => Some(Unit::Frequency(Frequency::Hertz)),
        "millihertz" | "millihertzs" | "mhz" => Some(Unit::Frequency(Frequency::Millihertz)),
        "microhertz" | "microhertzs" | "µhz" => Some(Unit::Frequency(Frequency::Microhertz)),
        "nanohertz" | "nanohertzs" | "nhz" => Some(Unit::Frequency(Frequency::Nanohertz)),
        "kilohertz" | "kilohertzs" | "khz" => Some(Unit::Frequency(Frequency::Kilohertz)),
        "megahertz" | "megahertzs" => Some(Unit::Frequency(Frequency::Megahertz)), // Removed duplicate "mhz"
        "gigahertz" | "gigahertzs" | "ghz" => Some(Unit::Frequency(Frequency::Gigahertz)),
        "terahertz" | "terahertzs" | "thz" => Some(Unit::Frequency(Frequency::Terahertz)),
        "planck_frequency" | "planck_frequencies" => Some(Unit::Frequency(Frequency::PlanckFrequency)),
        "revolutions_per_minute" | "rpm" => Some(Unit::Frequency(Frequency::RevolutionsPerMinute)),
        "radian_per_second" | "radians_per_second" | "rad/s" => Some(Unit::Frequency(Frequency::RadianPerSecond)),
        "cycles_per_second" | "cps" => Some(Unit::Frequency(Frequency::CyclesPerSecond)),
        "frames_per_second" | "fps" => Some(Unit::Frequency(Frequency::FramesPerSecond)),
        "beats_per_minute" | "bpm" => Some(Unit::Frequency(Frequency::BeatsPerMinute)),
        "counts_per_minute" | "cpm" => Some(Unit::Frequency(Frequency::CountsPerMinute)),
        "fresnel" | "fresnels" => Some(Unit::Frequency(Frequency::Fresnel)),
        "reciprocal_second" | "reciprocal_seconds" | "1/s" => Some(Unit::Frequency(Frequency::ReciprocalSecond)),
        "degree_per_second" | "degrees_per_second" | "deg/s" => Some(Unit::Frequency(Frequency::DegreePerSecond)),
        "gradian_per_second" | "gradians_per_second" | "grad/s" => Some(Unit::Frequency(Frequency::GradianPerSecond)),

        // --- Electric Current Units ---
        "ampere" | "amperes" | "amp" | "amps" => Some(Unit::ElectricCurrent(ElectricCurrent::Ampere)),
        "picoampere" | "picoamperes" => Some(Unit::ElectricCurrent(ElectricCurrent::Picoampere)),
        "nanoampere" | "nanoamperes" | "na" => Some(Unit::ElectricCurrent(ElectricCurrent::Nanoampere)),
        "microampere" | "microamperes" | "µa" => Some(Unit::ElectricCurrent(ElectricCurrent::Microampere)),
        "milliampere" | "milliamperes" | "ma" => Some(Unit::ElectricCurrent(ElectricCurrent::Milliampere)),
        "kiloampere" | "kiloamperes" | "ka" => Some(Unit::ElectricCurrent(ElectricCurrent::Kiloampere)),
        "megaampere" | "megaamperes" => Some(Unit::ElectricCurrent(ElectricCurrent::Megaampere)), // Removed duplicate "ma"
        "teraampere" | "teraamperes" | "ta" => Some(Unit::ElectricCurrent(ElectricCurrent::Teraampere)),
        "planck_current" | "planck_currents" => Some(Unit::ElectricCurrent(ElectricCurrent::PlanckCurrent)),
        "statampere" | "statamperes" | "s_amp" => Some(Unit::ElectricCurrent(ElectricCurrent::Statampere)),
        "abampere" | "abamperes" | "abamp" => Some(Unit::ElectricCurrent(ElectricCurrent::Abampere)),
        "biot" | "biots" | "bi" => Some(Unit::ElectricCurrent(ElectricCurrent::Biot)),
        "franklin_per_second" | "franklins_per_second" => Some(Unit::ElectricCurrent(ElectricCurrent::FranklinPerSecond)),
        "coulomb_per_second" | "coulombs_per_second" | "c/s" => Some(Unit::ElectricCurrent(ElectricCurrent::CoulombPerSecond)),
        "volt_per_ohm" | "volts_per_ohm" | "v/ω" => Some(Unit::ElectricCurrent(ElectricCurrent::VoltPerOhm)),
        "watt_per_volt" | "watts_per_volt" | "w/v" => Some(Unit::ElectricCurrent(ElectricCurrent::WattPerVolt)),
        "siemens_volt" | "siemens_volts" => Some(Unit::ElectricCurrent(ElectricCurrent::SiemensVolt)),
        "esu_per_second" | "esu_per_seconds" => Some(Unit::ElectricCurrent(ElectricCurrent::EsuPerSecond)),
        "emu_of_current" | "emu_of_currents" => Some(Unit::ElectricCurrent(ElectricCurrent::EmuOfCurrent)),
        "gilbert" | "gilberts" | "gb" => Some(Unit::ElectricCurrent(ElectricCurrent::Gilbert)),

        // --- Voltage Units ---
        "volt" | "volts" | "v" => Some(Unit::Voltage(Voltage::Volt)),
        "picovolt" | "picovolts" | "pv" => Some(Unit::Voltage(Voltage::Picovolt)),
        "nanovolt" | "nanovolts" | "nv" => Some(Unit::Voltage(Voltage::Nanovolt)),
        "microvolt" | "microvolts" | "µv" => Some(Unit::Voltage(Voltage::Microvolt)),
        "millivolt" | "millivolts" | "mv" => Some(Unit::Voltage(Voltage::Millivolt)),
        "decivolt" | "decivolts" | "dv" => Some(Unit::Voltage(Voltage::Decivolt)),
        "centivolt" | "centivolts" | "cv" => Some(Unit::Voltage(Voltage::Centivolt)),
        "hectovolt" | "hectovolts" | "hv" => Some(Unit::Voltage(Voltage::Hectovolt)),
        "kilovolt" | "kilovolts" | "kv" => Some(Unit::Voltage(Voltage::Kilovolt)),
        "megavolt" | "megavolts" => Some(Unit::Voltage(Voltage::Megavolt)), // Removed duplicate "mv"
        "gigavolt" | "gigavolts" | "gv" => Some(Unit::Voltage(Voltage::Gigavolt)),
        "teravolt" | "teravolts" | "tv" => Some(Unit::Voltage(Voltage::Teravolt)),
        "planck_voltage" | "planck_voltages" => Some(Unit::Voltage(Voltage::PlanckVoltage)),
        "statvolt" | "statvolts" | "s_volt" => Some(Unit::Voltage(Voltage::Statvolt)),
        "abvolt" | "abvolts" | "ab_v" => Some(Unit::Voltage(Voltage::Abvolt)),
        "esu_of_potential" | "esu_of_potentials" => Some(Unit::Voltage(Voltage::EsuOfPotential)),
        "emu_of_potential" | "emu_of_potentials" => Some(Unit::Voltage(Voltage::EmuOfPotential)),
        "watt_per_ampere" | "watts_per_ampere" | "w/a" => Some(Unit::Voltage(Voltage::WattPerAmpere)),
        "joule_per_coulomb" | "joules_per_coulomb" | "j/c" => Some(Unit::Voltage(Voltage::JoulePerCoulomb)),
        "electronvolt_per_elementary_charge" | "electronvolts_per_elementary_charge" | "ev/e" => Some(Unit::Voltage(Voltage::ElectronvoltPerElementaryCharge)),

        // --- Capacitance Units ---
        "farad" | "farads" | "f" => Some(Unit::Capacitance(Capacitance::Farad)),
        "attofarad" | "attofarads" | "af" => Some(Unit::Capacitance(Capacitance::Attofarad)),
        "femtofarad" | "femtofarads" | "ff" => Some(Unit::Capacitance(Capacitance::Femtofarad)),
        "picofarad" | "picofarads" | "pf" => Some(Unit::Capacitance(Capacitance::Picofarad)),
        "nanofarad" | "nanofarads" | "nf" => Some(Unit::Capacitance(Capacitance::Nanofarad)),
        "microfarad" | "microfarads" | "µf" => Some(Unit::Capacitance(Capacitance::Microfarad)),
        "millifarad" | "millifarads" | "mf" => Some(Unit::Capacitance(Capacitance::Millifarad)),
        "decafarad" | "decafarads" | "daf" => Some(Unit::Capacitance(Capacitance::Decafarad)),
        "hectofarad" | "hectofarads" | "hf" => Some(Unit::Capacitance(Capacitance::Hectofarad)),
        "kilofarad" | "kilofarads" | "kf" => Some(Unit::Capacitance(Capacitance::Kilofarad)),
        "megafarad" | "megafarads" => Some(Unit::Capacitance(Capacitance::Megafarad)), // Removed duplicate "mf"
        "terafarad" | "terafarads" | "tf" => Some(Unit::Capacitance(Capacitance::Terafarad)),
        "gigafarad" | "gigafarads" => Some(Unit::Capacitance(Capacitance::Gigafarad)),
        "planck_capacitance" | "planck_capacitances" => Some(Unit::Capacitance(Capacitance::PlanckCapacitance)),
        "statfarad" | "statfarads" | "s_farad" => Some(Unit::Capacitance(Capacitance::Statfarad)),
        "abfarad" | "abfarads" | "abf" => Some(Unit::Capacitance(Capacitance::Abfarad)),
        "esu_of_capacitance" | "esu_of_capacitances" => Some(Unit::Capacitance(Capacitance::EsuOfCapacitance)),
        "emu_of_capacitance" | "emu_of_capacitances" => Some(Unit::Capacitance(Capacitance::EmuOfCapacitance)),
        "coulomb_per_volt" | "coulombs_per_volt" | "c/v" => Some(Unit::Capacitance(Capacitance::CoulombPerVolt)),
        "second_per_ohm" | "seconds_per_ohm" | "s/ω" => Some(Unit::Capacitance(Capacitance::SecondPerOhm)),

        // --- Luminosity Units ---
        "candela" | "candelas" | "cd" => Some(Unit::Luminosity(Luminosity::Candela)),
        "lumen" | "lumens" | "lm" => Some(Unit::Luminosity(Luminosity::Lumen)),
        "lux" | "luxes" | "lx" => Some(Unit::Luminosity(Luminosity::Lux)),
        "candela_per_square_meter" | "candelas_per_square_meter" | "cd/m2" => Some(Unit::Luminosity(Luminosity::CandelaPerSquareMeter)),
        "lumen_per_square_meter" | "lumens_per_square_meter" | "lm/m2" => Some(Unit::Luminosity(Luminosity::LumenPerSquareMeter)),
        "nit" | "nits" => Some(Unit::Luminosity(Luminosity::Nit)),
        "stilb" | "stilbs" | "sb" => Some(Unit::Luminosity(Luminosity::Stilb)),
        "apostilb" | "apostilbs" | "asb" => Some(Unit::Luminosity(Luminosity::Apostilb)),
        "lambert" | "lamberts" => Some(Unit::Luminosity(Luminosity::Lambert)),
        "foot_lambert" | "foot_lamberts" | "fl" => Some(Unit::Luminosity(Luminosity::FootLambert)),
        "phot" | "phots" | "ph" => Some(Unit::Luminosity(Luminosity::Phot)),
        "bril" | "brils" => Some(Unit::Luminosity(Luminosity::Bril)),
        "skot" | "skots" => Some(Unit::Luminosity(Luminosity::Skot)),
        "watt_per_steradian" | "watts_per_steradian" | "w/sr" => Some(Unit::Luminosity(Luminosity::WattPerSteradian)),
        "lumen_per_watt" | "lumens_per_watt" | "lm/w" => Some(Unit::Luminosity(Luminosity::LumenPerWatt)),
        "lumen_second" | "lumen_seconds" | "lm_s" => Some(Unit::Luminosity(Luminosity::LumenSecond)),
        "talbot" | "talbots" => Some(Unit::Luminosity(Luminosity::Talbot)),
        "watt_per_steradian_square_meter" | "watts_per_steradian_square_meter" | "w/(sr·m²)" => Some(Unit::Luminosity(Luminosity::WattPerSteradianSquareMeter)),
        "candlepower" | "candlepowers" | "cp" => Some(Unit::Luminosity(Luminosity::Candlepower)),
        "planck_luminance" | "planck_luminances" => Some(Unit::Luminosity(Luminosity::PlanckLuminance)),

        // --- Radiation Units ---
        "gray" | "grays" | "gy" => Some(Unit::Radiation(Radiation::Gray)),
        "decigray" | "decigrays" | "dgy" => Some(Unit::Radiation(Radiation::Decigray)),
        "centigray" | "centigrays" | "cgy" => Some(Unit::Radiation(Radiation::Centigray)),
        "milligray" | "milligrays" | "mgy" => Some(Unit::Radiation(Radiation::Milligray)),
        "microgray" | "micrograys" | "µgy" => Some(Unit::Radiation(Radiation::Microgray)),
        "nanogray" | "nanograys" | "ngy" => Some(Unit::Radiation(Radiation::Nanogray)),
        "picogray" | "picograys" | "pgy" => Some(Unit::Radiation(Radiation::Picogray)),
        "rad" | "rads" => Some(Unit::Radiation(Radiation::Rad)),
        "sievert" | "sieverts" | "sv" => Some(Unit::Radiation(Radiation::Sievert)),
        "rem" | "rems" => Some(Unit::Radiation(Radiation::Rem)),
        "becquerel" | "becquerels" | "bq" => Some(Unit::Radiation(Radiation::Becquerel)),
        "curie" | "curies" | "ci" => Some(Unit::Radiation(Radiation::Curie)),
        "rutherford" | "rutherfords" | "rd" => Some(Unit::Radiation(Radiation::Rutherford)),
        "roentgen" | "roentgens" | "r" => Some(Unit::Radiation(Radiation::Roentgen)),
        "coulomb_per_kilogram" | "coulombs_per_kilogram" | "c/kg" => Some(Unit::Radiation(Radiation::CoulombPerKilogram)),
        "electronvolt_per_kilogram" | "electronvolts_per_kilogram" | "ev/kg" => Some(Unit::Radiation(Radiation::ElectronvoltPerKilogram)),
        "ion_pair_per_kilogram" | "ion_pairs_per_kilogram" => Some(Unit::Radiation(Radiation::IonPairPerKilogram)),
        "rep" | "reps" => Some(Unit::Radiation(Radiation::Rep)),
        "watt_per_kilogram" | "watts_per_kilogram" | "w/kg" => Some(Unit::Radiation(Radiation::WattPerKilogram)),
        "planck_radiation" | "planck_radiations" => Some(Unit::Radiation(Radiation::PlanckRadiation)),

        // --- Magnetic Flux Units ---
        "weber" | "webers" | "wb" => Some(Unit::MagneticFlux(MagneticFlux::Weber)),
        "microweber" | "microwebers" | "µwb" => Some(Unit::MagneticFlux(MagneticFlux::Microweber)),
        "nanoweber" | "nanowebers" | "nwb" => Some(Unit::MagneticFlux(MagneticFlux::Nanoweber)),
        "picoweber" | "picowebers" | "pwb" => Some(Unit::MagneticFlux(MagneticFlux::Picoweber)),
        "femtoweber" | "femtowebers" | "fwb" => Some(Unit::MagneticFlux(MagneticFlux::Femtoweber)),
        "attoweber" | "attowebers" | "awb" => Some(Unit::MagneticFlux(MagneticFlux::Attoweber)),
        "maxwell" | "maxwells" | "mx" => Some(Unit::MagneticFlux(MagneticFlux::Maxwell)),
        "megaline" | "megalines" => Some(Unit::MagneticFlux(MagneticFlux::Megaline)),
        "kiloline" | "kilolines" => Some(Unit::MagneticFlux(MagneticFlux::Kiloline)),
        "milliline" | "millilines" => Some(Unit::MagneticFlux(MagneticFlux::Milliline)),
        "gamma_square_centimeter" | "gamma_square_centimeters" => Some(Unit::MagneticFlux(MagneticFlux::GammaSquareCentimeter)),
        "tesla_square_meter" | "tesla_square_meters" | "t_m2" => Some(Unit::MagneticFlux(MagneticFlux::TeslaSquareMeter)),
        "gauss_square_centimeter" | "gauss_square_centimeters" | "g_cm2" => Some(Unit::MagneticFlux(MagneticFlux::GaussSquareCentimeter)),
        "volt_second" | "volt_seconds" | "v_s" => Some(Unit::MagneticFlux(MagneticFlux::VoltSecond)),
        "coulomb_ohm" | "coulomb_ohms" | "c_ω" => Some(Unit::MagneticFlux(MagneticFlux::CoulombOhm)),
        "joule_per_ampere" | "joules_per_ampere" | "j/a" => Some(Unit::MagneticFlux(MagneticFlux::JoulePerAmpere)),
        "henry_ampere" | "henry_amperes" | "h_a" => Some(Unit::MagneticFlux(MagneticFlux::HenryAmpere)),
        "magnetic_flux_quantum" | "magnetic_flux_quantums" | "φ0" => Some(Unit::MagneticFlux(MagneticFlux::MagneticFluxQuantum)),
        "unit_pole" | "unit_poles" => Some(Unit::MagneticFlux(MagneticFlux::UnitPole)),
        "planck_flux" | "planck_fluxes" => Some(Unit::MagneticFlux(MagneticFlux::PlanckFlux)),

        // --- Default Case ---
        _ => None, // If no match is found for the input string, return None
    }
}
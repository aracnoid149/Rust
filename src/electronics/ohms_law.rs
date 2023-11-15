// https://en.wikipedia.org/wiki/Ohm%27s_law

fn onms_law(voltage: f64, current: f64, resistance: f64) -> Result<f64, &str>
    {
        /*
        Apply Ohm's Law, on any two given electrical values, which can be voltage, current,
        and resistance, and then in a Python dict return name/value pair of the zero value.

        >>> ohms_law(voltage=0, current=0, resistance=10) -> ValueError: One and only one argument must be 0
        >>> ohms_law(voltage=0, current=1, resistance=-2) -> ValueError: Resistance cannot be negative
        >>> ohms_law(voltage=1, current=2, resistance=3) -> ValueError: Exactly one arguement must be 0
        >>> ohms_law(resistance=0, voltage=-10, current=1) -> {'resistance': -10.0}
        >>> ohms_law(voltage=0, current=-1.5, resistance=2) -> {'voltage': -3.0}
        */
        if (voltage == 0 && current == 0) || (voltage == 0 && resistance == 0) || (current == 0 && resistance == 0)
        {
            //return an error for more than one variable being zero
        }
        if resistance < 0 
        {
            //return an error indicating that the resistance cannot be less than zero
        }
        if voltage == 0
        {
            Ok(current * resistance)
        }
        if current == 0
        {
            Ok(voltage / resistance)
        }
        if resistance == 0
        {
            Ok(voltage / current)
        }
        else
        {
            //return an error that exactly one value must be 0
        }
    }



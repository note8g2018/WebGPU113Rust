
pub struct color
{
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
}

impl color
{
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32,) -> Reuslt<Self>
    {
        if red < 0 & red > 1
        {
            return none;
        }
        else if green < 0 & green > 1
        {
            return none;
        }
        else if blue < 0 & blue > 1
        {
            return none;
        }
        else if alpha < 0 & alpha > 1
        {
            return none;
        }
        Ok(Self{
            red: red,
            green: green,
            blue: blue,
            alpha: alpha,
        })
    }

    pub fn color_to_string(&self) -> String
    {
        format!("vec4f({}, {}, {}, {})", self.red, self.green, self.blue, self.alpha)
    }
}
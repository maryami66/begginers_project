pub enum EyeColor{
    Blue,
    Green,
    Brown,
    Black
}

pub enum Gym{
    FitStar(String),
    McFit(String),
    Soulplus,
    Others
}

pub struct GymInfo{
    pub name: String,
    pub country: String,
    pub with: f64,
    pub height: f64
}

pub struct User{
    pub name: String,
    country: Option<String>,
    eye_color: Option<EyeColor>,
    pub weight: f64,
    pub height: f64,
    gym: Gym
}

impl User{
    pub fn new(name: &str, country: Option<String>,  eye_color: Option<EyeColor>, w: f64, h: f64, gym: Gym) -> Self{
        User{
            name: name.to_owned(),
            country: country,
            eye_color: eye_color,
            weight: w,
            height: h,
            gym: gym
        }
    }
}




pub trait Group{
    fn what_group(&self) -> String;
}

impl Group for User{
    fn what_group(&self) -> String{
        let bmi = self.weight / (self.height * self.height);
        if bmi >= 30.{
            "Obese".to_owned()
        }
        else if bmi >= 25.{
            "Overweight".to_owned()
        }
        else if bmi >= 18.5{
            "Normal".to_owned()
        }
        else {
            "Underweight".to_owned()
        }
    }
}

impl Group for GymInfo{
    fn what_group(&self) -> String{
        let area = self.with * self.height;
        if area >= 1000.{
            "Big".to_owned()
        }
        else if area >= 500.{
            "Normal".to_owned()
        }
        else {
            "Small".to_owned()
        }
    }
}
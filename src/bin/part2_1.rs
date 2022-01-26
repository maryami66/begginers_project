enum EyeColor{
        Blue,
        Green,
        Brown,
        Black
    }
    
enum Gym{
    FitStar(String),
    McFit(String),
    Soulplus,
    Others
}

struct GymInfo{
    name: String,
    country: String,
    with: f64,
    height: f64
}


pub struct User{
    name: String,
    country: Option<String>,
    eye_color: Option<EyeColor>,
    weight: f64,
    height: f64,
    gym: Gym
}

impl User{
    fn new(name: &str, country: Option<String>,  eye_color: Option<EyeColor>, w: f64, h: f64, gym: Gym) -> Self{
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


trait Group{
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


mod utils {
    use crate::*;
    fn average_weight(users: &Vec<User>) -> f64{
        users.iter().map(|u| u.weight).sum::<f64>() as f64 / users.len() as f64
    }

    fn average_height(users: &Vec<User>) -> f64{
        users.iter().map(|u| u.height).sum::<f64>() as f64 / users.len() as f64
    }

    pub fn average_bmi(users: &Vec<User>) -> f64{
        users.iter().map(|u| u.weight / (u.height * u.height)).sum::<f64>() as f64 / users.len() as f64
    }
}


fn main(){
    let users = vec![
        User::new("Sara", None, None, 57.3, 1.71, Gym::Others),
        User::new("Alex", None, None, 106.2, 1.84, Gym::Others),
        User::new("Sam", None, None, 87.1, 1.81, Gym::Others),
        User::new("Tom", None, None, 132.5, 1.75, Gym::Others)];

    let gyms = GymInfo{
        name: "Fitstar".to_owned(),
        country: "Germany".to_owned(),
        with: 10.,
        height: 55.};
    
    for u in &users{
        println!("{:?} is {:?}", u.name, u.what_group());
    }
        
    println!("{:?} is {:?}", gyms.name, gyms.what_group());
    
    println!("Mean of the users BMI is {:?}", utils::average_bmi(&users));
    }  
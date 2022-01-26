enum EyeColor{
    Blue,
    Green,
    Brown,
    Black
}

struct User{
    name: String,
    country: String,
    eye_color: EyeColor,
    weight: f64,
    height: f64
}

impl User{
    fn new(name: &str, country: &str,  eye_color: EyeColor, w: f64, h: f64) -> Self{
        User{
            name: name.to_owned(),
            country: country.to_owned(),
            eye_color: eye_color,
            weight: w,
            height: h
        }
    }
    
    fn bmi(u: &Self) -> f64{
        u.weight / (u.height * u.height)
    }
}

fn main(){
    let users = vec![User::new("Sara", "Germany", EyeColor::Blue, 60.1, 1.71),
                    User::new("Alex", "Spain", EyeColor::Blue, 62.2, 1.68),
                    User::new("Sam", "Iran", EyeColor::Brown, 56.7, 1.62),
                    User::new("Tom", "US", EyeColor::Brown, 93.5, 1.84)];

    let mut bmi_sum = 0.;
    let num_users = users.len() as f64;
    for u in users{
        bmi_sum = bmi_sum + User::bmi(&u);
    }
    let mean = bmi_sum / num_users;
    
    println!("Mean of the users BMI is {:?}", mean);

}
use lib::modules::user;
use lib::modules::user::Group;
use lib::modules::utils;



fn main(){
    let users = vec![
        user::User::new("Sara", None, None, 57.3, 1.71, user::Gym::Others),
        user::User::new("Alex", None, None, 106.2, 1.84, user::Gym::Others),
        user::User::new("Sam", None, None, 87.1, 1.81, user::Gym::Others),
        user::User::new("Tom", None, None, 132.5, 1.75, user::Gym::Others)];

    let gyms = user::GymInfo{
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
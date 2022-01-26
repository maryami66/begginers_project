use crate::modules::user;

pub fn average_weight(users: &Vec<user::User>) -> f64{
    users.iter().map(|u| u.weight).sum::<f64>() as f64 / users.len() as f64
}

pub fn average_height(users: &Vec<user::User>) -> f64{
    users.iter().map(|u| u.height).sum::<f64>() as f64 / users.len() as f64
}

pub fn average_bmi(users: &Vec<user::User>) -> f64{
    users.iter().map(|u| u.weight / (u.height * u.height)).sum::<f64>() as f64 / users.len() as f64
}


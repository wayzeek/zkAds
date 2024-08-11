pub struct User {
    name: String,
    x: f32,
    y: f32,
}

pub fn calculate_distance(user1: &User, user2: &User) -> f32 {
    let dx = user2.x - user1.x;
    let dy = user2.y - user1.y;
    (dx * dx + dy * dy).sqrt()
}

#[no_mangle]
pub fn give_me_floats(x: f32, y: f32) -> f32 {
    println!("x: {}, y: {}", x, y);
    let sum = x + y;
    println!("sum: {}", sum);
    sum
}

#[no_mangle]
pub fn give_me_ints(x: i32, y: i32) -> i32 {
    println!("x: {}, y: {}", x, y);
    let sum = x + y;
    println!("sum: {}", sum);
    sum
}

pub fn find_nearest_company(user: User, mut companies: Vec<User>) -> User {
    if companies.len() < 2 {
        companies.remove(0) // remove returns the element, taking ownership
    } else {
        let mut smallest_distance = calculate_distance(&user, &companies[0]);
        let mut result_index = 0;

        for (i, company) in companies.iter().enumerate() {
            let distance = calculate_distance(&user, company);
            if distance < smallest_distance {
                smallest_distance = distance;
                result_index = i;
            }
        }

        companies.remove(result_index) // remove returns the element, taking ownership
    }
}

fn main() {
    let user1 = User {
        name: String::from("Valentin"),
        x: 42.42,
        y: 56.30,
    };

    let company1 = User {
        name: String::from("Ubisoft"),
        x: 40.40,
        y: 56.30,
    };
    let company2 = User {
        name: String::from("Google"),
        x: -42.42,
        y: 56.30,
    };

    let companies = vec![company1, company2];

    let nearest_company = find_nearest_company(user1, companies);
    println!(
        "Nearest company is {} at: ({}, {})",
        nearest_company.name, nearest_company.x, nearest_company.y
    );
}

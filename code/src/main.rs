struct Coordinate {
    x: f32,
    y: f32,
}

struct Company {
    name : String,
    location : Coordinate,
    distance_threshold : f32,
}

fn calculate_distance(user: &Coordinate, company: &Coordinate) -> f32 {
    let dx = company.x - user.x;
    let dy = company.y - user.y;
    (dx * dx + dy * dy).sqrt()
}

#[no_mangle]
fn is_user_close_enough(user_x : f32, user_y : f32) -> Vec<bool> {
    
    // Define all companies
    let company1 = Company {
        name : String::from("Ubisoft"),
        location : Coordinate {x : 180.15, y : 130.12},
        distance_threshold : 5.0
    };
    let company2 = Company {
        name : String::from("Google"),
        location : Coordinate {x : 0.13, y : 0.13},
        distance_threshold : 5.0
    };
    let company3 = Company {
        name : String::from("Microsoft"),
        location : Coordinate {x : 500.12, y : 98.45},
        distance_threshold : 5.0
    };
    let company4 = Company {
        name : String::from("Amazon"),
        location : Coordinate {x : -123.4, y : -130.12},
        distance_threshold : 5.0
    };
    let company5 = Company {
        name : String::from("Apple"),
        location : Coordinate {x : 200.15, y : -130.12},
        distance_threshold : 5.0
    };
    // Put all the companies into a vector
    let companies = vec![company1, company2, company3, company4, company5];

    // Create the result vector
    let mut result = Vec::new();

    //Iterate over all companies
    for company in &companies {
        // Calculate distance between user and current company
        let distance = calculate_distance(&Coordinate {x : user_x, y : user_y}, &company.location);

        // Check if the user is close enough from the company
        if distance <= company.distance_threshold {
            result.push(true);
        }
        else {
            result.push(false);
        }
    }
    result

}

fn main() {
    let test = is_user_close_enough(1.0, 1.0);

    for line in &test {
        println!("{line}");
    }
}

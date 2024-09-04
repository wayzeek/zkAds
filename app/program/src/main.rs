use serde::Deserialize;
use std::fs;
use rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsBuilder;
use rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsModelType;
use kd_tree::{KdPoint, KdTree};
use typenum::U384;

#[derive(Deserialize)]
struct Coordinate {
    latitude: f32,
    longitude: f32,
}

#[derive(Deserialize)]
struct Address {
    street: String,
    city: String,
    state: String,
    zip_code: String,
    country: String,
}

#[derive(Deserialize)]
struct CryptoWallet {
    balance: f32,
    currency: String,
}

#[derive(Deserialize)]
struct User {
    first_name: String,
    last_name: String,
    phone_number: String,
    email: String,
    location: Coordinate,
    address: Address,
    age: u32,
    crypto_wallet: CryptoWallet,
    interests: Vec<String>,
}

#[derive(Deserialize)]
struct Company {
    name: String,
    location: Coordinate,
    distance_threshold: f32,
    description: String,
    target_age_range: [u32; 2],
    crypto_interest: bool,
    wallet_balance_minimum: f32,
    user_interests: Vec<String>,
    address: Address,
    contact_number: String,
}

#[derive(Clone)]
struct EmbeddedCompany {
    company: Company,
    embedding: Vec<f32>,
}

impl KdPoint for EmbeddedCompany {
    type Scalar = ordered_float::OrderedFloat<f32>;
    type Dim = U384;

    fn at(&self, k: usize) -> Self::Scalar {
        ordered_float::OrderedFloat(self.embedding[k])
    }
}

fn calculate_distance(user: &Coordinate, company: &Coordinate) -> f32 {
    let dx = company.latitude - user.latitude;
    let dy = company.longitude - user.longitude;
    (dx * dx + dy * dy).sqrt()
}

fn encode_user(user: &User, model: &SentenceEmbeddingsBuilder) -> Vec<f32> {
    model.encode(&[user.interests.join(" ")]).unwrap().to_vec()
}

fn create_kd_tree(embedded_companies: Vec<EmbeddedCompany>) -> KdTree<EmbeddedCompany> {
    KdTree::build_by_ordered_float(embedded_companies)
}

fn is_user_a_good_fit(user: &User, kd_tree: &KdTree<EmbeddedCompany>, model: &SentenceEmbeddingsBuilder) -> Vec<String> {
    let user_embedding = encode_user(user, model);

    // Find the nearest companies in the KD-Tree
    let nearest_companies = kd_tree.nearests(&user_embedding, 10);

    // Filter companies based on additional criteria
    nearest_companies.into_iter().filter_map(|(company, _)| {
        // Check if the user's age is within the company's target age range
        if user.age >= company.company.target_age_range[0] && user.age <= company.company.target_age_range[1] &&
           (!company.company.crypto_interest || user.crypto_wallet.balance >= company.company.wallet_balance_minimum) &&
           calculate_distance(&user.location, &company.company.location) <= company.company.distance_threshold {
            Some(company.company.name.clone())
        } else {
            None
        }
    }).collect()
}

fn main() {
    // Load user data from JSON
    let user_data = fs::read_to_string("user.json").expect("Unable to read file");
    let user: User = serde_json::from_str(&user_data).expect("JSON was not well-formatted");

    // Load companies data from JSON
    let companies_data = fs::read_to_string("companies.json").expect("Unable to read file");
    let companies: Vec<Company> = serde_json::from_str(&companies_data).expect("JSON was not well-formatted");

    // Initialize the sentence transformer model
    let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2)
        .create_model()
        .expect("Failed to create model");

    // Encode each company's user interests into embeddings
    let embedded_companies: Vec<EmbeddedCompany> = companies.into_iter()
        .map(|company| {
            let embedding = model.encode(&[company.user_interests.join(" ")]).unwrap().to_vec();
            EmbeddedCompany { company, embedding }
        })
        .collect();

    // Create a KD-Tree from the embedded companies
    let kd_tree = create_kd_tree(embedded_companies);

    // Check which companies the user is a good fit for
    let matching_companies = is_user_a_good_fit(&user, &kd_tree, &model);

    // Print the matching companies
    for company in matching_companies {
        println!("User is a good fit for the company: {}", company);
    }
}

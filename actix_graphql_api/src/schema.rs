use juniper::{FieldResult};

#[derive(juniper::GraphQLEnum)]
enum Season {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
}

#[derive(juniper::GraphQLObject)]
struct Episode {
    name: String,
    original_air_date: String,
    number_overall: i32,
    number_in_season: i32,
}

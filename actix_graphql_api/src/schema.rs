use juniper::{FieldResult, RootNode};
use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};

#[derive(GraphQLEnum)]
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

#[derive(GraphQLObject)]
struct Episode {
    name: String,
    original_air_date: String,
    season: Season,
    number_overall: i32,
    number_in_season: i32,
}

#[derive(GraphQLInputObject)]
struct NewEpisode {
    name: String,
    original_air_date: String,
    season: Season,
    number_overall: i32,
    number_in_season: i32,
}

pub struct Query;

#[juniper::object]
impl Query {
    pub async fn api_version() -> &'static str {
        "0.1"
    }

    pub async fn episode(name: String) -> FieldResult<Episode> {
        Ok(Episode {
            name: "Eegah!".to_owned(),
            original_air_date: "10.10.2010".to_string(),
            season: Season::One,
            number_overall: 1,
            number_in_season: 1,
        })
    }
}

pub struct Mutation;

#[juniper::object]
impl Mutation {
    async fn create_episode(new_episode: NewEpisode) -> FieldResult<Episode> {
        Ok(Episode {
            name: new_episode.name,
            original_air_date: new_episode.original_air_date,
            season: new_episode.season,
            number_overall: new_episode.number_overall,
            number_in_season: new_episode.number_in_season,
        })
    }
}

pub type Schema = RootNode<'static, Query, Mutation>;

pub async fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}

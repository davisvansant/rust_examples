use chrono::prelude::*;

struct Movie {
    episode: String,
    title: String
}

impl Movie {
    fn new(episode: &str, title: &str) -> Movie {
        Movie { episode: episode.to_string(), title: title.to_string() }
    }
}

fn do_get_time() -> String {
    let time: DateTime<Utc> = Utc::now();
    time.to_string()
}

fn do_set_redis_key(episode: &str, title: &str) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://0.0.0.0:6379/")?;
    let mut connect = client.get_connection()?;
    let key : String = redis::cmd("SET").arg(episode).arg(title).query(&mut connect).unwrap();
    println!("{} | Set {} {}", do_get_time(), episode, title);
    println!("{} | {}", do_get_time(), key);
    Ok(())
}

fn main() {
    let key_one = Movie::new("Episode I", "The Phantom Menance");
    let key_two = Movie::new("Episode II", "Attack of the Clones");
    let key_three = Movie::new("Episode III", "Revenge of the Sith");
    let key_four = Movie::new("Episode IV", "A New Hope");
    let key_five = Movie::new("Episode V", "The Empire Strikes Back");
    let key_six = Movie::new("Episode VI", "Return of the Jedi");
    let key_seven = Movie::new("Episode VII", "The Force Awakens");
    let key_eight = Movie::new("Episode VIII", "The Last Jedi");
    let key_nine = Movie::new ("Episode IX", "The Rise of Skywalker");
    let saga = vec![
        &key_one,
        &key_two,
        &key_three,
        &key_four,
        &key_five,
        &key_six,
        &key_seven,
        &key_eight,
        &key_nine
    ];

    for m in &saga {
        match do_set_redis_key(&m.episode, &m.title) {
            Err(err) => {
                println!("Could not execute example:");
                println!("  {}: {}", err.category(), err);
            }
            Ok(()) => {}
        };
    }
}

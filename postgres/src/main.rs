use postgres::{Client, NoTls};

fn create_table(client: &mut Client) -> Result<(), std::io::Error> {
    println!("Creating Table...");

    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS person (
            id      SERIAL PRIMARY KEY,
            name    TEXT NOT NULL UNIQUE,
            data    BYTEA
        )"
    ).unwrap();

    Ok(())
}

fn insert_data(client: &mut Client) -> Result<(), std::io::Error> {
    println!("Inserting data...");

    let name = "Ferris";
    let data = None::<&[u8]>;

    client.execute(
        "INSERT INTO person (name, data) VALUES ($1, $2) ON CONFLICT (name) DO NOTHING",
        &[&name, &data],
    ).unwrap();

    Ok(())
}

fn query_data(client: &mut Client) -> Result<(), std::io::Error> {
    for row in client.query("SELECT id, name, data FROM person", &[]).unwrap() {
        let id: i32 = row.get(0);
        let name: String = row.get(1);
        let data: Option<Vec<u8>> = row.get(2);

        println!("id : {:?}", id);
        println!("name : {:?}", name);
        println!("data : {:?}", data);
    };

    Ok(())
}

fn main() -> Result<(), std::io::Error>  {
    let mut client = Client::connect("host=localhost user=postgres password=password", NoTls).unwrap();

    create_table(&mut client).unwrap();
    insert_data(&mut client).unwrap();
    query_data(&mut client).unwrap();

    Ok(())
}

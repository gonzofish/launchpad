use rocket::local::blocking::Client;

#[test]
fn test_create() {
    blast_off();
    // let client = blast_off();
    // let response = client.post("/api/project").body().dispatch();
}

fn blast_off() -> Client {
    let test_instance = launchpad::blast_off("DATABASE_URL");

    Client::tracked(test_instance).expect("Rocket instance is invalid")
}

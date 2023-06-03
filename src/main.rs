// use paho_mqtt as mqtt;
// mod mqtt_sync_subscriber;

// fn main() {
//     let broker_url = "mqtt://broker.emqx.io:1883";
//     let client_id = "mqtt-pub";
//     let topic = "jack/";
//     let payload = "Hello from Rust!";
//     let qos = 1;

//     let create_opts = mqtt::CreateOptionsBuilder::new()
//         .server_uri(broker_url)
//         .client_id(client_id)
//         .finalize();

//     let client = mqtt::Client::new(create_opts).unwrap_or_else(|err| {
//         println!("Error creating the client: {:?}", err);
//         std::process::exit(1);
//     });

//     let conn_opts = mqtt::ConnectOptionsBuilder::new()
//         .keep_alive_interval(std::time::Duration::from_secs(30))
//         .clean_session(true)
//         .finalize();

//     if let Err(e) = client.connect(conn_opts) {
//         println!("Unable to connect: {:?}", e);
//         std::process::exit(1);
//     }

//     let msg = mqtt::MessageBuilder::new()
//         .topic(topic)
//         .payload(payload)
//         .qos(qos)
//         .finalize();

//     if let Err(e) = client.publish(msg) {
//         println!("Error sending message: {:?}", e);
//         std::process::exit(1);
//     }

//     if let Err(e) = client.disconnect(None) {
//         println!("Error disconnecting: {:?}", e);
//         std::process::exit(1);
//     }
//     print!("Message published.\n")
// }

mod mqtt_sync_subscriber;
use mqtt_sync_subscriber::MqttClient;

fn main() {
    let broker_url = "mqtt://broker.emqx.io:1883";
    let client_id = "mqtt-pub";
    let topic = "jack/";
    let payload = "Hello from Rust! this is cool code ";
    let qos = 1;

    let mqtt_sync_subscriber = MqttClient::new(
        broker_url.to_string(),
        client_id.to_string(),
        topic.to_string(),
        qos,
        payload.to_string(),
    );
    // let client = mqtt_sync_subscriber.connect();

    let client = match mqtt_sync_subscriber.connect() {
        Ok(client) => client,
        Err(e) => {
            println!("Error creating the client: {:?}", e);
            std::process::exit(1);
        }
    };

    match mqtt_sync_subscriber.publish(&client) {
        Ok(_) => print!("Message published.\n"),
        Err(e) => {
            println!("Error sending message: {:?}", e);
            std::process::exit(1);
        }
    }
}

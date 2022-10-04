mod subscription;

use subscription::{LiveSubscription,SubscriptionData, Subscription};
use url::Url;
use tungstenite::{connect, Message};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct SocketResponse{
    success: bool,
    event:Subscription
}

fn main() {
    let mut live_subscription: SubscriptionData = LiveSubscription::new();

    let (mut socket, _response) = 
    connect(Url::parse("ws://localhost:4545").unwrap()).expect("Can't connect");

    // let wrapped_socket = Arc::new(Mutex::new(socket));
    // let wrapped_socket_2nd_handle = wrapped_socket.clone();

    socket.write_message(Message::Text("1".into())).unwrap();

    //Create a thread that loops over the socket and prints the messages
    loop {
        let msg = socket.read_message().unwrap();
        let msg = msg.to_text().unwrap();
        let parsed: SocketResponse= serde_json::from_str(&msg).expect("Can't parse to JSON");

        if parsed.success {
            live_subscription.add_subscription(parsed.event);
        }
        println!("{:?}", live_subscription);
    }


}

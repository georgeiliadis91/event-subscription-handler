use serde::{Serialize, Deserialize};
 
#[derive(Serialize, Deserialize, Debug)]
pub struct Outcome{
    pub team_id: i16,
    pub odd: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Market{
    pub id:i16,
    pub name:String,
    pub outcomes: Vec<Outcome>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Teams{
    pub team_id: i16,
    pub team: String,
    pub is_home: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subscription{
    pub id: i16,
    pub name: String,
    pub sport_id : i16,
    pub teams: [Teams;2],
    pub markets: Vec<Market>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscriptionData {
    pub subscriptions: Vec<Subscription>,
    pub is_busy: bool,
}

pub trait LiveSubscription {
    fn new () -> Self;
    fn add_subscription(&mut self, subscription: Subscription);
    fn add_subscriptions(&mut self, subscriptions: Vec<Subscription>);
    fn remove_subscription(&mut self, subscriptions: Subscription);
    fn remove_subscriptions(&mut self, subscriptions: Vec<Subscription>);
}

impl PartialEq for Subscription{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl LiveSubscription for SubscriptionData {
    fn new() -> SubscriptionData {
        return SubscriptionData { subscriptions: Vec::new(), is_busy: false }
    }

    fn add_subscription(&mut self, subscription: Subscription) {
        self.subscriptions.push(subscription);
        self.is_busy = true;
    }

    fn add_subscriptions(&mut self, subscriptions: Vec<Subscription>) {
        self.subscriptions.extend(subscriptions);
        self.is_busy = true;
    }

    fn remove_subscription(&mut self, subscription: Subscription) {
        self.subscriptions.retain(|x| x.id != subscription.id);
        self.is_busy = false;
    }

    fn remove_subscriptions(&mut self, subscriptions: Vec<Subscription>){
        for subscription in subscriptions {
            if self.subscriptions.contains(&subscription){
                self.subscriptions.retain(|x| x.id != subscription.id);
            }
            self.remove_subscription(subscription);
        }
        self.is_busy = false;
    }
}

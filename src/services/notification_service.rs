use actix_web_lab::sse::{self, ChannelStream, Sender, Sse};
use serde::Serialize;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

use crate::{
    constants::{known_roles, sse_events, SSE_DATA_CHUNK_SIZE},
    models::{
        GroupedNotification, InternalInventoryItem, LocateModel, LocateRequestModel,
        ProviderSymbolLocatesInfoWithDiscountedPrice,
    },
};

pub struct NotificationsService {
    client_groups: Arc<Mutex<HashMap<&'static str, HashMap<Uuid, Arc<Sender>>>>>,
}

impl NotificationsService {
    pub fn new() -> Self {
        Self {
            client_groups: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub async fn send_external_provider_quote_response(
        &self,
        data: ProviderSymbolLocatesInfoWithDiscountedPrice,
    ) {
        self.send_item_to_clients(sse_events::EXTERNAL_PROVIDER, &data)
            .await;
    }

    pub async fn send_notifications(&self, data: Vec<GroupedNotification>) {
        self.send_vec_to_clients(sse_events::NOTIFICATION, &data)
            .await;
    }

    pub async fn send_locate_request(&self, data: LocateRequestModel) {
        self.send_item_to_clients(sse_events::LOCATE_REQUEST, &data)
            .await;
    }
    pub async fn send_locate(&self, data: LocateModel) {
        self.send_item_to_clients(sse_events::LOCATE, &data).await;
    }

    pub async fn send_internal_inventory_item(&self, message: InternalInventoryItem) {
        tracing::info!("send_internal_inventory_item: {:?}", message);
        self.send_item_to_clients(sse_events::INTERNAL_INVENTORY, &message)
            .await;
    }

    pub async fn send_locates_history_to_clients(&self, history_items: &[LocateModel]) {
        self.send_vec_to_clients(sse_events::LOCATE_HISTORY, history_items)
            .await;
    }

    pub async fn send_locates_history_to_client(
        &self,
        sender_id: Uuid,
        sender: Arc<Sender>,
        history_items: &[LocateModel],
    ) {
        self.send_vec_to_client(sender_id, sender, sse_events::LOCATE_HISTORY, history_items)
            .await;
    }

    pub async fn send_locate_requests_history_to_clients(
        &self,
        history_items: &[LocateRequestModel],
    ) {
        self.send_vec_to_clients(sse_events::LOCATE_REQUEST_HISTORY, history_items)
            .await;
    }

    pub async fn send_locate_requests_history_to_client(
        &self,
        sender_id: Uuid,
        sender: Arc<Sender>,
        history_items: &[LocateRequestModel],
    ) {
        self.send_vec_to_client(
            sender_id,
            sender,
            sse_events::LOCATE_REQUEST_HISTORY,
            history_items,
        )
        .await;
    }

    async fn send_vec_to_clients<T>(&self, event_name: &str, items: &[T])
    where
        T: Serialize,
    {
        let admins = self.get_senders(known_roles::ADMIN);

        self.send_vec_to_group(admins, event_name, items).await;

        // TODO:
        /*
        var method = Constants.SignalRMethods.LocateRequestHistory;

        List<Task> tasks = new()
        {
            SendToGroupAsync(Constants.KnownRoles.Admin, method, SerializeObject(historyItems)),
            SendToGroupAsync(Constants.KnownRoles.Viewer, method, SerializeObject(historyItems))
        };

        var providerIds = historyItems.SelectMany(item => item.SourceDetails.Select(x => x.Source)).Distinct();
        foreach (var providerId in providerIds)
        {
            var items = historyItems.Where(item => item.SourceDetails.Any(x => x.Source == providerId));
            tasks.Add(SendToGroupAsync($"{Constants.KnownRoles.Provider}_{providerId}", method, SerializeObject(items)));
        }
        Task.WhenAll(tasks).GetAwaiter().GetResult();
         */
    }

    async fn send_item_to_clients<T>(&self, event_name: &str, item: &T)
    where
        T: Serialize,
    {
        let admins = self.get_senders(known_roles::ADMIN);

        self.send_item_to_group(admins, event_name, item).await;
    }

    fn get_senders(&self, group_name: &str) -> HashMap<Uuid, Arc<Sender>> {
        let client_groups = self.client_groups.lock().unwrap();
        let admins = client_groups.get(group_name);
        match admins {
            Some(value) => value.clone(),
            None => HashMap::new(),
        }
    }

    pub fn add_user_to_provider_group(&self) -> (Uuid, Arc<Sender>, Sse<ChannelStream>) {
        let (sender, sse): (sse::Sender, sse::Sse<sse::ChannelStream>) = sse::channel(2);

        //let sender = Arc::new(sender);
        let mut client_groups = self.client_groups.lock().unwrap();
        let group_name = known_roles::ADMIN; // depend on a user's role
                                             // if (User.HasRole(Constants.KnownRoles.Admin))
                                             // {
                                             // await Groups.AddToGroupAsync(connectionId, groupName);
                                             // }
                                             // else if (User.HasRole(Constants.KnownRoles.Viewer))
                                             // {
                                             //     groupName = Constants.KnownRoles.Viewer;
                                             //     await Groups.AddToGroupAsync(connectionId, groupName);
                                             // }
                                             // else if (User.HasRole(Constants.KnownRoles.Provider))
                                             // {
                                             //     groupName = $"{Constants.KnownRoles.Provider}_{User.GetProviderId()}";
                                             //     await Groups.AddToGroupAsync(connectionId, groupName);
                                             // }

        let sender = Arc::new(sender);
        let sender_id = Uuid::new_v4();
        let group = client_groups.get_mut(group_name);
        if let Some(group) = group {
            group.insert(sender_id, sender.clone());
        } else {
            let mut new_group: HashMap<Uuid, Arc<Sender>> = HashMap::new();
            new_group.insert(sender_id, sender.clone());
            client_groups.insert(group_name, new_group);
        }
        // if group.is_none() {
        //     group = Some(&new_group);
        // }

        // let cloned_sender = sender.clone();
        (sender_id, sender, sse)
    }

    pub fn delete_user_from_provider_group(&self, sender_id: Uuid) {
        let mut client_groups = self.client_groups.lock().unwrap();
        let group_name = known_roles::ADMIN;

        let group = client_groups.get_mut(group_name);
        if let Some(group) = group {
            group.remove(&sender_id);
        }
    }

    async fn send_vec_to_group<T>(
        &self,
        admins: HashMap<Uuid, Arc<Sender>>,
        event_name: &str,
        items: &[T],
    ) where
        T: Serialize,
    {
        for sender in admins {
            self.send_vec_to_client(sender.0, sender.1, event_name, items)
                .await;
        }
    }

    async fn send_item_to_group<T>(
        &self,
        admins: HashMap<Uuid, Arc<Sender>>,
        event_name: &str,
        item: &T,
    ) where
        T: Serialize,
    {
        for sender in admins {
            self.send_item_to_client(sender.0, sender.1, event_name, item)
                .await;
        }
    }

    async fn send_vec_to_client<T>(
        &self,
        sender_id: Uuid,
        sender: Arc<Sender>,
        event_name: &str,
        items: &[T],
    ) where
        T: Serialize,
    {
        for item in items.chunks(SSE_DATA_CHUNK_SIZE) {
            let msg = sse::Data::new_json(item).unwrap().event(event_name);

            if !self.send(sender_id, sender.clone(), msg).await {
                break;
            }
        }
    }

    async fn send_item_to_client<T>(
        &self,
        sender_id: Uuid,
        sender: Arc<Sender>,
        event_name: &str,
        item: &T,
    ) where
        T: Serialize,
    {
        let msg = sse::Data::new_json(item).unwrap().event(event_name);

        self.send(sender_id, sender, msg).await;
    }

    async fn send(&self, sender_id: Uuid, sender: Arc<Sender>, msg: sse::Data) -> bool {
        if sender.send(msg).await.is_err() {
            tracing::warn!("client disconnected; could not send SSE message");
            self.delete_user_from_provider_group(sender_id);
            return false;
        }
        true
    }
}

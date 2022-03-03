use crate::routes::balances::models::Balances;
use crate::routes::safe_apps::models::SafeApp;
use crate::routes::safes::models::SafeState;
use crate::routes::transactions::models::summary::TransactionListItem;
use serde::Serialize;
use serde_json::value::RawValue;

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum DashboardUiComponent {
    HistoryTxs {
        transactions: Vec<TransactionListItem>,
    },
    PendingTxs {
        transactions: Vec<TransactionListItem>,
    },
    Balances(Balances),
    SafeApps {
        safe_apps: Vec<SafeApp>,
    },
    Safe(SafeState),
    Collectibles {
        json: String, // unfortunately we don't have a type yet for Collectibles
    },
    ErrorLoadingComponent,
    SectionTitle {
        value: String,
    },
}

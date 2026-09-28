use super::models::Transaction;
use crate::customers::services::CustomerService;
use crate::infrastructure::ApiClient;
use crate::shared::services::SupervisorServices;

#[derive(Clone)]
pub struct WalletService {
    api: ApiClient,
    customer_service: CustomerService,
}

impl WalletService {
    pub fn new(demo: &SupervisorServices) -> Self {
        Self {
            api: demo.api.clone(),
            customer_service: CustomerService::new(demo),
        }
    }

    pub fn get_balance(&self, customer_id: &str) -> Option<f32> {
        self.customer_service
            .get_customers()
            .into_iter()
            .find(|c| c.id == customer_id)
            .map(|c| c.balance)
    }

    pub fn get_transactions(&self) -> Vec<Transaction> {
        [
            ("TX-2841", "Ahmed Ben Ali", "Top Up", 20.0, "Credit Card"),
            ("TX-2840", "Maya Cherif", "Payment", -5.5, "Wallet Balance"),
            ("TX-2839", "Yassine Trabelsi", "Top Up", 15.0, "Cash"),
            (
                "TX-2838",
                "PC-03 Session",
                "Payment",
                -6.0,
                "Wallet Balance",
            ),
            ("TX-2837", "Omar Khaled", "Top Up", 10.0, "Mobile Payment"),
        ]
        .into_iter()
        .map(|(id, customer, kind, amount, method)| Transaction {
            id: id.into(),
            customer: customer.into(),
            kind: kind.into(),
            amount,
            method: method.into(),
        })
        .collect()
    }

    pub fn api(&self) -> &ApiClient {
        &self.api
    }
}

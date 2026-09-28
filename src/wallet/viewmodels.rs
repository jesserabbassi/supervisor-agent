use super::models::Transaction;
use super::services::WalletService;

pub struct WalletViewModel {
    pub transactions: Vec<Transaction>,
    pub service: WalletService,
}

impl WalletViewModel {
    pub fn load_wallet(service: WalletService) -> Self {
        let transactions = service.get_transactions();
        Self {
            transactions,
            service,
        }
    }
    pub fn load_transactions(&mut self) {
        self.transactions = self.service.get_transactions();
    }
}

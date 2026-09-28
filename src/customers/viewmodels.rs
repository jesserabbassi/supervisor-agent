use super::models::Customer;
use super::services::CustomerService;

pub struct CustomerViewModel {
    pub selected: usize,
    pub customers: Vec<Customer>,
    pub service: CustomerService,
}

impl CustomerViewModel {
    pub fn load_customers(service: CustomerService) -> Self {
        let customers = service.get_customers();
        Self {
            selected: 0,
            customers,
            service,
        }
    }
    pub fn search_customer(&self, query: &str) -> Vec<Customer> {
        self.service.search(query)
    }
}

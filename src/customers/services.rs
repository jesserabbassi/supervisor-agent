use super::models::Customer;
use crate::infrastructure::ApiClient;
use crate::shared::services::SupervisorServices;

#[derive(Clone)]
pub struct CustomerService {
    api: ApiClient,
    demo_mode: bool,
}

impl CustomerService {
    pub fn new(demo: &SupervisorServices) -> Self {
        Self {
            api: demo.api.clone(),
            demo_mode: demo.demo_mode,
        }
    }

    pub fn get_customers(&self) -> Vec<Customer> {
        [
            (
                "C-1042",
                "Ahmed Ben Ali",
                "ahmed.benali@email.com",
                "VIP Gold",
                45.5,
                true,
            ),
            (
                "C-1041",
                "Yassine Trabelsi",
                "yassine.t@email.com",
                "Regular",
                12.0,
                true,
            ),
            (
                "C-1040",
                "Sara Mohamed",
                "sara.m@email.com",
                "Student",
                8.5,
                false,
            ),
            (
                "C-1039",
                "Maya Cherif",
                "maya.c@email.com",
                "VIP Gold",
                32.0,
                true,
            ),
            (
                "C-1038",
                "Omar Khaled",
                "omar.k@email.com",
                "Regular",
                5.0,
                false,
            ),
            (
                "C-1037",
                "Karim Ben Salem",
                "karim.b@email.com",
                "VIP Silver",
                18.0,
                true,
            ),
            (
                "C-1036",
                "Amine Haddad",
                "amine.h@email.com",
                "Regular",
                0.0,
                false,
            ),
            (
                "C-1035",
                "Leila Ghorbel",
                "leila.g@email.com",
                "Student",
                6.5,
                true,
            ),
        ]
        .into_iter()
        .map(|(id, name, email, membership, balance, online)| Customer {
            id: id.into(),
            name: name.into(),
            email: email.into(),
            membership: membership.into(),
            balance,
            online,
        })
        .collect()
    }

    pub fn search(&self, query: &str) -> Vec<Customer> {
        self.get_customers()
            .into_iter()
            .filter(|c| c.name.to_lowercase().contains(&query.to_lowercase()))
            .collect()
    }

    pub fn api(&self) -> &ApiClient {
        &self.api
    }
}

//Codigo do contrato inteligente (codigo adaptado do codigo da primeira avaliacao de gerenciamento de estoque)
//Testes unitarios e de integracao

#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod stock_management {
    use ink::prelude::string::String;
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct StockManagement {
        products: Mapping<u32, Product>,  // Usando um mapeamento para armazenar produtos por ID
        next_id: u32,
    }

    #[derive(Debug, Clone, scale::Encode, scale::Decode, Default)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Product {
        pub name: String,
        pub quantity: i32,
    }

    impl StockManagement {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                products: Mapping::new(),
                next_id: 1,
            }
        }

        #[ink(message)]
        pub fn add_product(&mut self, name: String, quantity: i32) -> u32 { //Adiciona um produto e retorna o ID gerado
            let product_id = self.next_id;
            self.next_id += 1;

            let product = Product { name, quantity };
            self.products.insert(product_id, &product);

            product_id
        }

        #[ink(message)]
        pub fn update_quantity(&mut self, product_id: u32, quantity: i32) -> bool { //Atualiza a quantidade do produto se ele existir
            match self.products.get(product_id) { //caso onde o produto não é encontrado
                Some(mut product) => {
                    product.quantity = quantity;
                    self.products.insert(product_id, &product);
                    true
                }
                None => false,
            }
        }

        #[ink(message)]
        pub fn get_product(&self, product_id: u32) -> Option<Product> {
            self.products.get(product_id)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn test_add_product() {
            let mut contract = StockManagement::new();
            let product_id = contract.add_product("arroz".to_string(), 20);

            let product = contract.get_product(product_id);
            assert_eq!(product.unwrap().name, "arroz");
            assert_eq!(product.unwrap().quantity, 20);
        }

        #[ink::test]
        fn test_update_quantity() {
            let mut contract = StockManagement::new();
            let product_id = contract.add_product("arroz".to_string(), 20);

            contract.update_quantity(product_id, 30);
            let product = contract.get_product(product_id);
            assert_eq!(product.unwrap().quantity, 30);
        }
    }

    
    #[cfg(all(test, feature = "e2e-tests"))]
mod e2e_tests {
    use super::*;
    use ink_e2e::{Client, ContractsBackend};

    type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[ink_e2e::test]
    async fn test_add_product_e2e(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        // Given
        let constructor = StockManagement::new();
        let contract = client
            .instantiate("stock_management", &ink_e2e::alice(), &constructor)
            .submit()
            .await
            .expect("instantiate failed");

        let call_builder = contract.call_builder::<StockManagement>();

        // Test adding a product
        let product_id = client
            .call(&ink_e2e::alice(), &call_builder.add_product("arroz".to_string(), 20))
            .submit()
            .await
            .expect("add product failed");

        // Test retrieving the product
        let get_result = client
            .call(&ink_e2e::alice(), &call_builder.get_product(product_id))
            .dry_run()
            .await?;

        assert!(get_result.is_ok());
        assert_eq!(get_result.return_value().name, "arroz");
        assert_eq!(get_result.return_value().quantity, 20);

        Ok(())
    }
}

    

}

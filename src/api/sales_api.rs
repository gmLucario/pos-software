//! Sales API
//!
//! Business logic for processing sales transactions.

use crate::models::{Operation, Sale, SaleInput};
use crate::repo::{ProductRepository, SaleRepository};
use rust_decimal::Decimal;
use std::sync::Arc;

#[derive(Clone)]
pub struct SalesApi {
    sale_repo: Arc<dyn SaleRepository>,
    product_repo: Arc<dyn ProductRepository>,
}

impl std::fmt::Debug for SalesApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SalesApi").finish()
    }
}

impl PartialEq for SalesApi {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.sale_repo, &other.sale_repo)
            && Arc::ptr_eq(&self.product_repo, &other.product_repo)
    }
}

impl SalesApi {
    pub fn new(
        sale_repo: Arc<dyn SaleRepository>,
        product_repo: Arc<dyn ProductRepository>,
    ) -> Self {
        Self {
            sale_repo,
            product_repo,
        }
    }

    /// Process a new sale with validation
    pub async fn process_sale(&self, input: SaleInput) -> Result<Sale, String> {
        // Validate sale has items
        if input.items.is_empty() {
            return Err("Sale must have at least one item".to_string());
        }

        // Validate all products exist and have sufficient stock
        for item in &input.items {
            let product = self
                .product_repo
                .get_by_id(&item.product_id)
                .await?
                .ok_or_else(|| format!("Product not found: {}", item.product_id))?;

            if item.quantity <= 0.0 {
                return Err(format!(
                    "Invalid quantity for product '{}': must be positive",
                    product.full_name
                ));
            }

            if product.current_amount < item.quantity {
                return Err(format!(
                    "Insufficient stock for '{}': available {}, requested {}",
                    product.full_name, product.current_amount, item.quantity
                ));
            }

            // Validate price matches (security check)
            if item.unit_price != product.user_price {
                return Err(format!(
                    "Price mismatch for '{}': expected ${}, got ${}",
                    product.full_name, product.user_price, item.unit_price
                ));
            }
        }

        // Validate payment amounts
        let total = input.total_amount();

        if input.paid_amount < Decimal::ZERO {
            return Err("Paid amount cannot be negative".to_string());
        }

        // Validate payment based on whether it's a loan
        if !input.is_loan() {
            // Cash sales must be paid in full
            if input.paid_amount < total {
                return Err("Cash sales must be paid in full".to_string());
            }
        }
        // Loans can have partial or zero payment - no validation needed

        // Create the sale (repository handles stock deduction)
        self.sale_repo.create(input).await
    }

    /// Get sale by ID with operations
    pub async fn get_sale(&self, id: &str) -> Result<SaleWithOperations, String> {
        let sale = self
            .sale_repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| format!("Sale not found: {}", id))?;

        let operations = self.sale_repo.get_operations(id).await?;

        Ok(SaleWithOperations { sale, operations })
    }

    /// List all sales
    pub async fn list_sales(&self) -> Result<Vec<Sale>, String> {
        self.sale_repo.list_all().await
    }

    /// Get sales within date range
    pub async fn get_sales_by_date(&self, start: &str, end: &str) -> Result<Vec<Sale>, String> {
        self.sale_repo.list_by_date_range(start, end).await
    }

    /// Get sales for a customer
    pub async fn get_customer_sales(&self, customer_name: &str) -> Result<Vec<Sale>, String> {
        self.sale_repo.get_by_customer(customer_name).await
    }

    /// Get sales statistics
    pub async fn get_sales_stats(&self) -> Result<SalesStats, String> {
        let sales = self.sale_repo.list_all().await?;

        let total_sales = sales.len();
        let total_revenue = sales.iter().map(|s| s.total_amount).sum();

        let cash_sales = sales.iter().filter(|s| !s.is_loan).count();

        let loan_sales = sales.iter().filter(|s| s.is_loan).count();

        let total_cash_received = sales.iter().map(|s| s.paid_amount).sum();

        Ok(SalesStats {
            total_sales,
            total_revenue,
            total_cash_received,
            cash_sales,
            loan_sales,
        })
    }

    /// Get today's sales
    pub async fn get_today_sales(&self) -> Result<Vec<Sale>, String> {
        let today = chrono::Utc::now().date_naive();
        let start = format!("{}T00:00:00Z", today);
        let end = format!("{}T23:59:59Z", today);

        self.sale_repo.list_by_date_range(&start, &end).await
    }

    /// Get sales statistics for today
    pub async fn get_today_stats(&self) -> Result<SalesStats, String> {
        let sales = self.get_today_sales().await?;

        let total_sales = sales.len();
        let total_revenue = sales.iter().map(|s| s.total_amount).sum();

        let cash_sales = sales.iter().filter(|s| !s.is_loan).count();

        let loan_sales = sales.iter().filter(|s| s.is_loan).count();

        let total_cash_received = sales.iter().map(|s| s.paid_amount).sum();

        Ok(SalesStats {
            total_sales,
            total_revenue,
            total_cash_received,
            cash_sales,
            loan_sales,
        })
    }
}

/// Sale with operations
#[derive(Debug, Clone)]
pub struct SaleWithOperations {
    pub sale: Sale,
    pub operations: Vec<Operation>,
}

/// Sales statistics
#[derive(Debug, Clone)]
pub struct SalesStats {
    pub total_sales: usize,
    pub total_revenue: Decimal,
    pub total_cash_received: Decimal,
    pub cash_sales: usize,
    pub loan_sales: usize,
}

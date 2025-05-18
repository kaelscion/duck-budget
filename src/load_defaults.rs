use account::Account;
use category::Category;

pub fn load_default_categories() -> Vec<Category> {
    vec![
        Category::new("Food".to_string(), "Food and Dining".to_string(), "🍔".to_string()),
        Category::new("Transport".to_string(), "Transportation".to_string(), "🚗".to_string()),
        Category::new("Entertainment".to_string(), "Entertainment and Leisure".to_string(), "🎉".to_string()),
        Category::new("Utilities".to_string(), "Utilities and Bills".to_string(), "💡".to_string()),
        Category::new("Health".to_string(), "Health and Fitness".to_string(), "🏥".to_string()),
    ]
}

pub fn load_default_accounts() -> Vec<Account> {
    vec![
        Account::new("Cash".to_string(), 0.0),
        Account::new("Personal Checking".to_string(), 0.0),
        Account::new("Personal Savings".to_string(), 0.0),
        Account::new("Credit Card".to_string(), 0.0),
    ]
}

pub fn load_defaults() -> (Vec<Category>, Vec<Account>) {
    let categories = load_default_categories();
    let accounts = load_default_accounts();
    (categories, accounts)
}
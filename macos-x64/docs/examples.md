# Ovie Examples

This comprehensive collection of examples demonstrates all major features of the Ovie Programming Language. Each example includes detailed explanations and is designed to be both educational and practical.

## Table of Contents

1. [Basic Examples](#basic-examples)
2. [Data Structures](#data-structures)
3. [Control Flow](#control-flow)
4. [Functions](#functions)
5. [Error Handling](#error-handling)
6. [Real-World Applications](#real-world-applications)
7. [AI-Friendly Patterns](#ai-friendly-patterns)
8. [Advanced Examples](#advanced-examples)

## Basic Examples

### Hello World

The classic first program in Ovie:

```ovie
// examples/hello.ov
fn main() {
    seeAm "Hello, Ovie World!"
}
```

**Key Features Demonstrated:**
- Function definition with `fn`
- Main function as entry point
- String output with `seeAm`

### Variables and Mutability

```ovie
// examples/variables.ov
fn main() {
    // Immutable variables (default)
    name = "Alice"
    age = 25
    
    // Mutable variables
    mut counter = 0
    mut balance = 100.50
    
    seeAm "Name: " + name
    seeAm "Age: " + age
    seeAm "Counter: " + counter
    seeAm "Balance: $" + balance
    
    // Modify mutable variables
    counter = counter + 1
    balance = balance - 10.25
    
    seeAm "Updated counter: " + counter
    seeAm "Updated balance: $" + balance
    
    // This would cause an error:
    // name = "Bob"  // Error: cannot assign to immutable variable
}
```

**Key Features Demonstrated:**
- Immutable variables by default
- Mutable variables with `mut`
- Type inference
- String concatenation

### Basic Math Operations

```ovie
// examples/math.ov
fn main() {
    mut a = 10
    mut b = 3
    
    seeAm "Numbers: a = " + a + ", b = " + b
    
    // Basic arithmetic
    seeAm "Addition: " + a + " + " + b + " = " + (a + b)
    seeAm "Subtraction: " + a + " - " + b + " = " + (a - b)
    seeAm "Multiplication: " + a + " * " + b + " = " + (a * b)
    seeAm "Division: " + a + " / " + b + " = " + (a / b)
    seeAm "Remainder: " + a + " % " + b + " = " + (a % b)
    
    // Floating point math
    mut x = 3.14159
    mut y = 2.71828
    
    seeAm "Float addition: " + x + " + " + y + " = " + (x + y)
    seeAm "Float multiplication: " + x + " * " + y + " = " + (x * y)
}
```

## Data Structures

### Structs

```ovie
// examples/struct.ov
struct Person {
    name: string,
    age: u32,
    email: string,
    is_active: bool,
}

struct Address {
    street: string,
    city: string,
    state: string,
    zip_code: string,
}

struct Employee {
    person: Person,
    employee_id: u32,
    department: string,
    salary: f64,
    address: Address,
}

fn main() {
    // Create a simple struct
    mut alice = Person {
        name: "Alice Johnson",
        age: 28,
        email: "alice@example.com",
        is_active: true,
    }
    
    seeAm "Employee: " + alice.name
    seeAm "Age: " + alice.age
    seeAm "Email: " + alice.email
    seeAm "Active: " + alice.is_active
    
    // Create nested structs
    mut employee = Employee {
        person: Person {
            name: "Bob Smith",
            age: 35,
            email: "bob@company.com",
            is_active: true,
        },
        employee_id: 12345,
        department: "Engineering",
        salary: 85000.0,
        address: Address {
            street: "123 Main St",
            city: "Tech City",
            state: "CA",
            zip_code: "94105",
        },
    }
    
    seeAm "Employee: " + employee.person.name
    seeAm "ID: " + employee.employee_id
    seeAm "Department: " + employee.department
    seeAm "Salary: $" + employee.salary
    seeAm "Address: " + employee.address.street + ", " + employee.address.city
    
    // Modify struct fields
    employee.salary = 90000.0
    employee.person.age = 36
    
    seeAm "Updated salary: $" + employee.salary
    seeAm "Updated age: " + employee.person.age
}
```

### Enums

```ovie
// examples/enums.ov
enum Color {
    Red,
    Green,
    Blue,
    Custom(u8, u8, u8),  // RGB values
}

enum OrderStatus {
    Pending,
    Processing,
    Shipped(string),     // Tracking number
    Delivered,
    Cancelled(string),   // Reason
}

enum Result<T, E> {
    Ok(T),
    Error(E),
}

fn main() {
    // Simple enum usage
    mut primary_color = Color.Red
    mut secondary_color = Color.Custom(255, 128, 0)  // Orange
    
    seeAm "Primary color set"
    seeAm "Secondary color: custom RGB"
    
    // Enum with data
    mut order_status = OrderStatus.Shipped("1Z999AA1234567890")
    
    match order_status {
        OrderStatus.Pending => seeAm "Order is pending",
        OrderStatus.Processing => seeAm "Order is being processed",
        OrderStatus.Shipped(tracking) => seeAm "Order shipped with tracking: " + tracking,
        OrderStatus.Delivered => seeAm "Order has been delivered",
        OrderStatus.Cancelled(reason) => seeAm "Order cancelled: " + reason,
    }
    
    // Result type usage
    mut division_result = safe_divide(10.0, 2.0)
    
    match division_result {
        Result.Ok(value) => seeAm "Division result: " + value,
        Result.Error(message) => seeAm "Error: " + message,
    }
}

fn safe_divide(a: f64, b: f64) -> Result<f64, string> {
    if b == 0.0 {
        return Result.Error("Division by zero")
    }
    return Result.Ok(a / b)
}
```

## Control Flow

### Conditionals

```ovie
// examples/conditionals.ov
fn main() {
    mut age = 25
    mut has_license = true
    mut years_experience = 3
    
    // Simple if statement
    if age >= 18 {
        seeAm "You are an adult"
    }
    
    // If-else
    if age >= 21 {
        seeAm "You can drink alcohol in the US"
    } else {
        seeAm "You cannot drink alcohol in the US yet"
    }
    
    // Multiple conditions
    if age >= 16 && has_license {
        seeAm "You can drive"
    } else if age >= 16 {
        seeAm "You can get a license"
    } else {
        seeAm "You're too young to drive"
    }
    
    // Complex conditions
    if years_experience >= 5 {
        seeAm "Senior level"
    } else if years_experience >= 2 {
        seeAm "Mid level"
    } else if years_experience >= 1 {
        seeAm "Junior level"
    } else {
        seeAm "Entry level"
    }
    
    // Nested conditions
    if age >= 18 {
        if has_license {
            if years_experience > 0 {
                seeAm "Experienced adult driver"
            } else {
                seeAm "New adult driver"
            }
        } else {
            seeAm "Adult without license"
        }
    } else {
        seeAm "Minor"
    }
}
```

### Loops

```ovie
// examples/loops.ov
fn main() {
    seeAm "=== While Loop Example ==="
    mut counter = 0
    while counter < 5 {
        seeAm "Counter: " + counter
        counter = counter + 1
    }
    
    seeAm "=== For Loop Example ==="
    for i in 0..10 {
        if i % 2 == 0 {
            seeAm "Even number: " + i
        } else {
            seeAm "Odd number: " + i
        }
    }
    
    seeAm "=== Nested Loops Example ==="
    for row in 1..4 {
        mut line = ""
        for col in 1..4 {
            line = line + "[" + row + "," + col + "] "
        }
        seeAm line
    }
    
    seeAm "=== Loop with Break Example ==="
    mut search_value = 7
    mut found = false
    
    for num in 1..20 {
        if num == search_value {
            seeAm "Found " + search_value + " at position " + num
            found = true
            break
        }
    }
    
    if !found {
        seeAm "Value " + search_value + " not found"
    }
    
    seeAm "=== Loop with Continue Example ==="
    for num in 1..11 {
        if num % 3 == 0 {
            continue  // Skip multiples of 3
        }
        seeAm "Processing: " + num
    }
}
```

## Functions

### Function Examples

```ovie
// examples/functions.ov

// Simple function with no parameters
fn greet() {
    seeAm "Hello from Ovie!"
}

// Function with parameters
fn greet_person(name: string) {
    seeAm "Hello, " + name + "!"
}

// Function with return value
fn add(a: u32, b: u32) -> u32 {
    return a + b
}

// Function with multiple parameters and return
fn calculate_rectangle_area(width: f64, height: f64) -> f64 {
    mut area = width * height
    seeAm "Calculating area: " + width + " × " + height + " = " + area
    return area
}

// Function with early return
fn check_password_strength(password: string) -> string {
    if password.length() < 8 {
        return "Weak: Too short"
    }
    
    if !password.contains_digit() {
        return "Weak: No numbers"
    }
    
    if !password.contains_uppercase() {
        return "Medium: No uppercase letters"
    }
    
    if !password.contains_special_char() {
        return "Strong: All requirements met"
    }
    
    return "Very Strong: Excellent password"
}

// Recursive function
fn factorial(n: u32) -> u32 {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

// Function that calls other functions
fn demonstrate_math() {
    mut sum = add(5, 3)
    mut area = calculate_rectangle_area(4.5, 6.2)
    mut fact = factorial(5)
    
    seeAm "Sum: " + sum
    seeAm "Area: " + area
    seeAm "Factorial of 5: " + fact
}

fn main() {
    greet()
    greet_person("Alice")
    greet_person("Bob")
    
    demonstrate_math()
    
    mut password_result = check_password_strength("MyP@ssw0rd123")
    seeAm "Password strength: " + password_result
    
    // Test factorial
    for i in 1..6 {
        seeAm "Factorial of " + i + " = " + factorial(i)
    }
}
```

## Error Handling

### Safe Error Handling

```ovie
// examples/errors.ov

enum Result<T, E> {
    Ok(T),
    Error(E),
}

enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
    Overflow,
}

// Safe division function
fn safe_divide(numerator: f64, denominator: f64) -> Result<f64, MathError> {
    if denominator == 0.0 {
        return Result.Error(MathError.DivisionByZero)
    }
    return Result.Ok(numerator / denominator)
}

// Safe square root function
fn safe_sqrt(value: f64) -> Result<f64, MathError> {
    if value < 0.0 {
        return Result.Error(MathError.NegativeSquareRoot)
    }
    return Result.Ok(value.sqrt())
}

// Function that handles multiple error types
fn calculate_quadratic_formula(a: f64, b: f64, c: f64) -> Result<(f64, f64), string> {
    // Calculate discriminant
    mut discriminant = b * b - 4.0 * a * c
    
    // Check for negative discriminant
    if discriminant < 0.0 {
        return Result.Error("No real solutions: negative discriminant")
    }
    
    // Check for division by zero
    if a == 0.0 {
        return Result.Error("Not a quadratic equation: coefficient 'a' is zero")
    }
    
    mut sqrt_discriminant = discriminant.sqrt()
    mut solution1 = (-b + sqrt_discriminant) / (2.0 * a)
    mut solution2 = (-b - sqrt_discriminant) / (2.0 * a)
    
    return Result.Ok((solution1, solution2))
}

fn main() {
    seeAm "=== Safe Division Examples ==="
    
    // Test safe division
    mut division_tests = [
        (10.0, 2.0),
        (15.0, 3.0),
        (7.0, 0.0),    // This will cause an error
        (100.0, 4.0),
    ]
    
    for test in division_tests {
        mut result = safe_divide(test.0, test.1)
        match result {
            Result.Ok(value) => seeAm test.0 + " ÷ " + test.1 + " = " + value,
            Result.Error(MathError.DivisionByZero) => seeAm "Error: Cannot divide " + test.0 + " by zero",
            Result.Error(_) => seeAm "Unexpected error in division",
        }
    }
    
    seeAm "=== Safe Square Root Examples ==="
    
    mut sqrt_tests = [9.0, 16.0, -4.0, 25.0]
    
    for test in sqrt_tests {
        mut result = safe_sqrt(test)
        match result {
            Result.Ok(value) => seeAm "√" + test + " = " + value,
            Result.Error(MathError.NegativeSquareRoot) => seeAm "Error: Cannot take square root of negative number " + test,
            Result.Error(_) => seeAm "Unexpected error in square root",
        }
    }
    
    seeAm "=== Quadratic Formula Examples ==="
    
    // Test quadratic formula: ax² + bx + c = 0
    mut quadratic_tests = [
        (1.0, -5.0, 6.0),   // x² - 5x + 6 = 0 (solutions: 2, 3)
        (1.0, -2.0, 1.0),   // x² - 2x + 1 = 0 (solution: 1)
        (1.0, 0.0, -4.0),   // x² - 4 = 0 (solutions: 2, -2)
        (1.0, 2.0, 5.0),    // x² + 2x + 5 = 0 (no real solutions)
        (0.0, 2.0, 3.0),    // Not quadratic
    ]
    
    for test in quadratic_tests {
        seeAm "Solving: " + test.0 + "x² + " + test.1 + "x + " + test.2 + " = 0"
        
        mut result = calculate_quadratic_formula(test.0, test.1, test.2)
        match result {
            Result.Ok(solutions) => {
                if solutions.0 == solutions.1 {
                    seeAm "One solution: x = " + solutions.0
                } else {
                    seeAm "Two solutions: x₁ = " + solutions.0 + ", x₂ = " + solutions.1
                }
            },
            Result.Error(message) => seeAm "Error: " + message,
        }
        seeAm ""
    }
}
```

## Real-World Applications

### Simple Calculator

```ovie
// examples/calculator.ov

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

enum CalculatorError {
    DivisionByZero,
    InvalidOperation,
}

struct Calculator {
    history: mut [string],
}

fn create_calculator() -> Calculator {
    return Calculator {
        history: [],
    }
}

fn calculate(calc: mut Calculator, a: f64, b: f64, op: Operation) -> Result<f64, CalculatorError> {
    mut result: f64
    mut operation_str: string
    
    match op {
        Operation.Add => {
            result = a + b
            operation_str = " + "
        },
        Operation.Subtract => {
            result = a - b
            operation_str = " - "
        },
        Operation.Multiply => {
            result = a * b
            operation_str = " × "
        },
        Operation.Divide => {
            if b == 0.0 {
                return Result.Error(CalculatorError.DivisionByZero)
            }
            result = a / b
            operation_str = " ÷ "
        },
    }
    
    // Add to history
    mut history_entry = a + operation_str + b + " = " + result
    calc.history.push(history_entry)
    
    return Result.Ok(result)
}

fn print_history(calc: Calculator) {
    seeAm "=== Calculator History ==="
    if calc.history.is_empty() {
        seeAm "No calculations performed yet"
        return
    }
    
    for entry in calc.history {
        seeAm entry
    }
}

fn main() {
    mut calc = create_calculator()
    
    seeAm "Simple Calculator Demo"
    seeAm "====================="
    
    // Perform various calculations
    mut calculations = [
        (10.0, 5.0, Operation.Add),
        (15.0, 3.0, Operation.Subtract),
        (7.0, 8.0, Operation.Multiply),
        (20.0, 4.0, Operation.Divide),
        (10.0, 0.0, Operation.Divide),  // This will cause an error
        (100.0, 25.0, Operation.Divide),
    ]
    
    for calc_data in calculations {
        mut result = calculate(calc, calc_data.0, calc_data.1, calc_data.2)
        
        match result {
            Result.Ok(value) => seeAm "Result: " + value,
            Result.Error(CalculatorError.DivisionByZero) => seeAm "Error: Cannot divide by zero!",
            Result.Error(CalculatorError.InvalidOperation) => seeAm "Error: Invalid operation!",
        }
    }
    
    print_history(calc)
}
```

### Bank Account System

```ovie
// examples/bank_account.ov

enum TransactionType {
    Deposit,
    Withdrawal,
    Transfer,
}

struct Transaction {
    transaction_id: string,
    transaction_type: TransactionType,
    amount: f64,
    timestamp: string,
    description: string,
}

struct BankAccount {
    account_number: string,
    owner_name: string,
    balance: mut f64,
    is_active: mut bool,
    transactions: mut [Transaction],
}

enum BankError {
    InsufficientFunds,
    AccountClosed,
    InvalidAmount,
    AccountNotFound,
}

fn create_account(account_number: string, owner_name: string) -> BankAccount {
    return BankAccount {
        account_number: account_number,
        owner_name: owner_name,
        balance: 0.0,
        is_active: true,
        transactions: [],
    }
}

fn deposit(account: mut BankAccount, amount: f64, description: string) -> Result<f64, BankError> {
    if !account.is_active {
        return Result.Error(BankError.AccountClosed)
    }
    
    if amount <= 0.0 {
        return Result.Error(BankError.InvalidAmount)
    }
    
    account.balance = account.balance + amount
    
    mut transaction = Transaction {
        transaction_id: generate_transaction_id(),
        transaction_type: TransactionType.Deposit,
        amount: amount,
        timestamp: get_current_timestamp(),
        description: description,
    }
    
    account.transactions.push(transaction)
    
    seeAm "Deposited $" + amount + " to account " + account.account_number
    seeAm "New balance: $" + account.balance
    
    return Result.Ok(account.balance)
}

fn withdraw(account: mut BankAccount, amount: f64, description: string) -> Result<f64, BankError> {
    if !account.is_active {
        return Result.Error(BankError.AccountClosed)
    }
    
    if amount <= 0.0 {
        return Result.Error(BankError.InvalidAmount)
    }
    
    if account.balance < amount {
        return Result.Error(BankError.InsufficientFunds)
    }
    
    account.balance = account.balance - amount
    
    mut transaction = Transaction {
        transaction_id: generate_transaction_id(),
        transaction_type: TransactionType.Withdrawal,
        amount: amount,
        timestamp: get_current_timestamp(),
        description: description,
    }
    
    account.transactions.push(transaction)
    
    seeAm "Withdrew $" + amount + " from account " + account.account_number
    seeAm "New balance: $" + account.balance
    
    return Result.Ok(account.balance)
}

fn print_account_summary(account: BankAccount) {
    seeAm "=== Account Summary ==="
    seeAm "Account Number: " + account.account_number
    seeAm "Owner: " + account.owner_name
    seeAm "Balance: $" + account.balance
    seeAm "Status: " + (if account.is_active { "Active" } else { "Closed" })
    seeAm "Total Transactions: " + account.transactions.length()
}

fn print_transaction_history(account: BankAccount) {
    seeAm "=== Transaction History ==="
    
    if account.transactions.is_empty() {
        seeAm "No transactions found"
        return
    }
    
    for transaction in account.transactions {
        mut type_str = match transaction.transaction_type {
            TransactionType.Deposit => "DEPOSIT",
            TransactionType.Withdrawal => "WITHDRAWAL",
            TransactionType.Transfer => "TRANSFER",
        }
        
        seeAm transaction.timestamp + " | " + type_str + " | $" + transaction.amount + " | " + transaction.description
    }
}

// Helper functions (simplified implementations)
fn generate_transaction_id() -> string {
    return "TXN" + get_random_number()
}

fn get_current_timestamp() -> string {
    return "2026-01-28 10:30:00"  // Simplified
}

fn get_random_number() -> string {
    return "123456"  // Simplified
}

fn main() {
    seeAm "Bank Account System Demo"
    seeAm "========================"
    
    // Create account
    mut account = create_account("ACC001", "Alice Johnson")
    print_account_summary(account)
    
    seeAm ""
    
    // Perform transactions
    mut deposit_result = deposit(account, 1000.0, "Initial deposit")
    match deposit_result {
        Result.Ok(_) => seeAm "Deposit successful",
        Result.Error(error) => seeAm "Deposit failed: " + error,
    }
    
    mut withdraw_result = withdraw(account, 250.0, "ATM withdrawal")
    match withdraw_result {
        Result.Ok(_) => seeAm "Withdrawal successful",
        Result.Error(error) => seeAm "Withdrawal failed: " + error,
    }
    
    // Try to withdraw more than balance
    mut overdraft_result = withdraw(account, 1000.0, "Large withdrawal")
    match overdraft_result {
        Result.Ok(_) => seeAm "Withdrawal successful",
        Result.Error(BankError.InsufficientFunds) => seeAm "Withdrawal failed: Insufficient funds",
        Result.Error(error) => seeAm "Withdrawal failed: " + error,
    }
    
    seeAm ""
    print_account_summary(account)
    seeAm ""
    print_transaction_history(account)
}
```

## AI-Friendly Patterns

### Natural Language Code

```ovie
// examples/ai_friendly.ov

// AI-friendly function names and structure
fn calculate_monthly_loan_payment(loan_amount: f64, annual_interest_rate: f64, loan_term_months: u32) -> f64 {
    // Clear variable names that explain their purpose
    mut monthly_interest_rate = annual_interest_rate / 12.0
    mut number_of_payments = loan_term_months
    
    // Handle special case: no interest
    if monthly_interest_rate == 0.0 {
        return loan_amount / number_of_payments
    }
    
    // Standard loan payment formula
    mut factor = (1.0 + monthly_interest_rate).pow(number_of_payments)
    mut monthly_payment = loan_amount * (monthly_interest_rate * factor) / (factor - 1.0)
    
    // Clear output showing what was calculated
    seeAm "Loan amount: $" + loan_amount
    seeAm "Annual interest rate: " + (annual_interest_rate * 100.0) + "%"
    seeAm "Loan term: " + loan_term_months + " months"
    seeAm "Monthly payment: $" + monthly_payment
    
    return monthly_payment
}

// Function that processes a list of items (common AI pattern)
fn process_student_grades(student_names: [string], grades: [f64]) -> f64 {
    mut total_score = 0.0
    mut student_count = student_names.length()
    
    seeAm "Processing grades for " + student_count + " students:"
    
    // Process each student
    for i in 0..student_count {
        mut student_name = student_names[i]
        mut grade = grades[i]
        
        // Determine letter grade
        mut letter_grade = if grade >= 90.0 {
            "A"
        } else if grade >= 80.0 {
            "B"
        } else if grade >= 70.0 {
            "C"
        } else if grade >= 60.0 {
            "D"
        } else {
            "F"
        }
        
        seeAm student_name + ": " + grade + "% (" + letter_grade + ")"
        total_score = total_score + grade
    }
    
    // Calculate and display average
    mut class_average = total_score / student_count
    seeAm "Class average: " + class_average + "%"
    
    return class_average
}

// Function with clear error handling (AI can easily understand)
fn validate_email_address(email: string) -> Result<bool, string> {
    // Check if email is empty
    if email.is_empty() {
        return Result.Error("Email address cannot be empty")
    }
    
    // Check for @ symbol
    if !email.contains("@") {
        return Result.Error("Email address must contain @ symbol")
    }
    
    // Check for domain part
    mut parts = email.split("@")
    if parts.length() != 2 {
        return Result.Error("Email address must have exactly one @ symbol")
    }
    
    mut local_part = parts[0]
    mut domain_part = parts[1]
    
    // Check local part
    if local_part.is_empty() {
        return Result.Error("Email address must have a local part before @")
    }
    
    // Check domain part
    if domain_part.is_empty() {
        return Result.Error("Email address must have a domain part after @")
    }
    
    if !domain_part.contains(".") {
        return Result.Error("Domain must contain at least one dot")
    }
    
    seeAm "Email address " + email + " is valid"
    return Result.Ok(true)
}

fn main() {
    seeAm "AI-Friendly Code Patterns Demo"
    seeAm "=============================="
    
    // Loan calculation example
    seeAm "=== Loan Payment Calculation ==="
    mut monthly_payment = calculate_monthly_loan_payment(250000.0, 0.045, 360)
    seeAm ""
    
    // Grade processing example
    seeAm "=== Student Grade Processing ==="
    mut students = ["Alice", "Bob", "Charlie", "Diana", "Eve"]
    mut grades = [92.5, 87.0, 78.5, 95.0, 82.0]
    mut average = process_student_grades(students, grades)
    seeAm ""
    
    // Email validation example
    seeAm "=== Email Validation ==="
    mut test_emails = [
        "alice@example.com",
        "bob.smith@company.org",
        "invalid-email",
        "@missing-local.com",
        "missing-domain@",
        "",
    ]
    
    for email in test_emails {
        mut validation_result = validate_email_address(email)
        match validation_result {
            Result.Ok(_) => seeAm "✓ Valid: " + email,
            Result.Error(message) => seeAm "✗ Invalid: " + email + " - " + message,
        }
    }
}
```

## Advanced Examples

### Generic Data Structures (Future Feature)

```ovie
// examples/generics.ov (Future Stage 1/2 feature)

// Generic Option type
enum Option<T> {
    Some(T),
    None,
}

// Generic Result type
enum Result<T, E> {
    Ok(T),
    Error(E),
}

// Generic List structure
struct List<T> {
    items: mut [T],
    capacity: u32,
}

// Generic functions
fn create_list<T>() -> List<T> {
    return List<T> {
        items: [],
        capacity: 0,
    }
}

fn add_item<T>(list: mut List<T>, item: T) {
    list.items.push(item)
    list.capacity = list.capacity + 1
}

fn find_item<T>(list: List<T>, predicate: fn(T) -> bool) -> Option<T> {
    for item in list.items {
        if predicate(item) {
            return Option.Some(item)
        }
    }
    return Option.None
}

fn main() {
    // Create lists of different types
    mut string_list = create_list<string>()
    mut number_list = create_list<u32>()
    
    // Add items
    add_item(string_list, "Hello")
    add_item(string_list, "World")
    add_item(number_list, 42)
    add_item(number_list, 100)
    
    // Find items
    mut found_string = find_item(string_list, fn(s: string) -> bool { return s == "Hello" })
    mut found_number = find_item(number_list, fn(n: u32) -> bool { return n > 50 })
    
    match found_string {
        Option.Some(value) => seeAm "Found string: " + value,
        Option.None => seeAm "String not found",
    }
    
    match found_number {
        Option.Some(value) => seeAm "Found number: " + value,
        Option.None => seeAm "Number not found",
    }
}
```

### Concurrent Programming (Future Feature)

```ovie
// examples/concurrency.ov (Future Stage 2 feature)

// Async function example
async fn fetch_data(url: string) -> Result<string, string> {
    seeAm "Fetching data from: " + url
    
    // Simulate network delay
    await sleep(1000)  // 1 second
    
    if url.starts_with("https://") {
        return Result.Ok("Data from " + url)
    } else {
        return Result.Error("Invalid URL: must use HTTPS")
    }
}

// Channel-based communication
fn producer(channel: Channel<u32>) {
    for i in 1..11 {
        channel.send(i)
        seeAm "Produced: " + i
    }
    channel.close()
}

fn consumer(channel: Channel<u32>) {
    while true {
        match channel.receive() {
            Some(value) => {
                seeAm "Consumed: " + value
                // Process the value
            },
            None => {
                seeAm "Channel closed, consumer stopping"
                break
            },
        }
    }
}

async fn main() {
    seeAm "Concurrency Examples"
    seeAm "==================="
    
    // Async/await example
    mut urls = [
        "https://api.example.com/data",
        "https://api.example.com/users",
        "http://insecure.example.com/data",  // This will fail
    ]
    
    for url in urls {
        mut result = await fetch_data(url)
        match result {
            Result.Ok(data) => seeAm "Success: " + data,
            Result.Error(error) => seeAm "Error: " + error,
        }
    }
    
    // Channel example
    mut channel = Channel.new<u32>()
    
    // Spawn producer and consumer
    spawn producer(channel.clone())
    spawn consumer(channel)
    
    // Wait for completion
    await sleep(5000)  // 5 seconds
    
    seeAm "Concurrency demo complete"
}
```

## Running the Examples

### Prerequisites

Make sure you have Ovie installed:

```bash
ovie --version
```

### Running Individual Examples

```bash
# Create a new project
ovie new ovie-examples
cd ovie-examples

# Copy any example into src/main.ov
# For example, copy the hello world example:
echo 'fn main() { seeAm "Hello, Ovie World!" }' > src/main.ov

# Build and run
ovie build
ovie run
```

### Running All Examples

```bash
# Clone the examples repository
git clone https://github.com/ovie-lang/examples.git
cd examples

# Run specific example
ovie run examples/hello.ov
ovie run examples/calculator.ov
ovie run examples/bank_account.ov

# Run with different backends
ovie run --backend=wasm examples/math.ov
ovie run --backend=llvm examples/functions.ov
```

### Testing Examples

```bash
# Run tests for examples
ovie test examples/

# Run with Aproko analysis
ovie aproko examples/
```

## Next Steps

After exploring these examples:

1. **[Language Guide](language-guide.md)**: Deep dive into Ovie syntax and features
2. **[CLI Reference](cli.md)**: Master the command-line tools
3. **[Testing](testing.md)**: Learn to write tests for your programs
4. **[AI Integration](ai-integration.md)**: Use Ovie with AI systems

## Contributing Examples

Have a great example to share? We'd love to include it!

1. Fork the [examples repository](https://github.com/ovie-lang/examples)
2. Add your example with clear comments and documentation
3. Include tests demonstrating the example works
4. Submit a pull request

---

*These examples demonstrate Ovie Stage 0 features. Additional examples for Stage 1 and Stage 2 features will be added as they become available.*
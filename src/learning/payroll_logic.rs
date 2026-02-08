struct Employee {
    id: u64,
    salary: u64,
    is_active: bool,
}

impl Employee {
    fn pay_bonus(&mut self, amount: u64) {
        self.salary += amount;
        println!("Bonus paid: {}", amount);
    }

    fn is_active(&self) -> bool {
        self.is_active
    }

    fn set_active(&mut self, is_active: bool) {
        self.is_active = is_active;
    }

    fn raise_salary(&mut self, percentage: u64) {
        self.salary += self.salary * percentage / 100;
    }
}

fn main() {
    let mut employee = Employee {
        id: 1,
        salary: 1000,
        is_active: true,
    };
    employee.pay_bonus(100);
    println!("Employee salary: {}", employee.salary);
    employee.raise_salary(10);
    println!("Employee salary: {}", employee.salary);
}

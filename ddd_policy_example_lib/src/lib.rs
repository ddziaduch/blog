struct Bill {
    total: f64
}

struct Customer {
    is_using_application: bool,
    is_pensioner: bool
}

trait CustomerSpecification {
    fn is_satisfied(&self) -> bool;
}

struct CustomerUsingApplicationSpecification<'a> {
    customer: &'a Customer
}

impl CustomerSpecification for CustomerUsingApplicationSpecification<'_> {
    fn is_satisfied(&self) -> bool {
        self.customer.is_using_application
    }
}

struct PensionerCustomerSpecification<'a> {
    customer: &'a Customer
}

impl CustomerSpecification for PensionerCustomerSpecification<'_> {
    fn is_satisfied(&self) -> bool {
        self.customer.is_pensioner
    }
}

trait DiscountPolicy {
    fn apply(&self, bill: &Bill) -> Bill;
}

struct CustomerUsingApplicationDiscountPolicy<'a> {
    specification: &'a CustomerUsingApplicationSpecification<'a>
}

impl DiscountPolicy for CustomerUsingApplicationDiscountPolicy<'_> {
    fn apply(&self, bill: &Bill) -> Bill {
        if self.specification.is_satisfied() {
            Bill {
                total: bill.total * 0.9
            }
        } else {
            Bill {
                total: bill.total
            }
        }
    }
}

struct PensionerDiscountPolicy<'a> {
    specification: &'a PensionerCustomerSpecification<'a>
}

impl DiscountPolicy for PensionerDiscountPolicy<'_> {
    fn apply(&self, bill: &Bill) -> Bill {
        if self.specification.is_satisfied() {
            Bill {
                total: bill.total * 0.85
            }
        } else {
            Bill {
                total: bill.total
            }
        }
    }
}

#[test]
fn customer_using_application_discount_policy_should_reduce_the_bill_total_by_ten_percents() {
    let initial_bill = Bill {
        total: 100.0
    };
    let customer = Customer {
        is_using_application: true,
        is_pensioner: false
    };
    let specification = CustomerUsingApplicationSpecification {
        customer: &customer
    };
    let policy = CustomerUsingApplicationDiscountPolicy {
        specification: &specification
    };
    let discounted_bill = policy.apply(&initial_bill);
    assert_eq!(discounted_bill.total, 90.0);
}

#[test]
fn pensioner_discount_policy_should_reduce_the_bill_total_by_fifteen_percents() {
    let initial_bill = Bill {
        total: 100.0
    };
    let customer = Customer {
        is_using_application: false,
        is_pensioner: true
    };
    let specification = PensionerCustomerSpecification {
        customer: &customer
    };
    let policy = PensionerDiscountPolicy {
        specification: &specification
    };
    let discounted_bill = policy.apply(&initial_bill);
    assert_eq!(discounted_bill.total, 85.0);
}

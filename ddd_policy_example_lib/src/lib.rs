struct Bill {
    total: f64
}

trait DiscountPolicy {
    fn apply(bill: &Bill) -> Bill;
}

struct ApplicationUserDiscountPolicy {}

impl DiscountPolicy for ApplicationUserDiscountPolicy {
    fn apply(bill: &Bill) -> Bill {
        Bill {
            total: bill.total * 0.9
        }
    }
}

struct PensionerDiscountPolicy {}

impl DiscountPolicy for PensionerDiscountPolicy {
    fn apply(bill: &Bill) -> Bill {
        Bill {
            total: bill.total * 0.85
        }
    }
}

#[test]
fn application_user_discount_policy_should_reduce_the_bill_total_by_ten_percents() {
    let initial_bill = Bill {
        total: 100.0
    };
    let discounted_bill = ApplicationUserDiscountPolicy::apply(&initial_bill);
    assert_eq!(discounted_bill.total, 90.0);
}

#[test]
fn pensioner_discount_policy_should_reduce_the_bill_total_by_fifteen_percents() {
    let initial_bill = Bill {
        total: 100.0
    };
    let discounted_bill = PensionerDiscountPolicy::apply(&initial_bill);
    assert_eq!(discounted_bill.total, 85.0);
}

struct Bill<'a> {
    total: &'a f64
}

trait DiscountPolicy {
    fn apply(bill: &mut Bill);
}

struct ApplicationUserDiscountPolicy {}

impl DiscountPolicy for ApplicationUserDiscountPolicy {
    fn apply(bill: &mut Bill) {
        bill.total = &(bill.total * 0.9);
    }
}

struct PensionerDiscountPolicy {}

impl DiscountPolicy for PensionerDiscountPolicy {
    fn apply(bill: &mut Bill) {
        bill.total = &(bill.total * 0.85);
    }
}

#[test]
fn application_user_discount_policy_should_reduce_the_bill_total_by_ten_percents() {
    let mut bill = Bill {
        total: &100.0
    };
    ApplicationUserDiscountPolicy::apply(&mut bill);
    assert_eq!(bill.total, &(90.0 as f64));
}

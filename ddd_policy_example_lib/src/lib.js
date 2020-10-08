'use strict';

class Bill {
    total;

    /**
     * @param {number} total
     */
    constructor(total) {
        this.total = total;
    }
}

class DiscountPolicy {
    /**
     * @param {Bill} bill
     * @returns {Bill}
     */
    apply(bill) {
        return bill;
    }
}

class ApplicationUserDiscount extends DiscountPolicy {
    /**
     * @param {Bill} bill
     * @returns {Bill}
     */
    apply(bill) {
        return new Bill(bill.total * 0.9);
    }
}

class PensionerDiscountPolicy {
    /**
     * @param {Bill} bill
     * @returns {Bill}
     */
    apply(bill) {
        return new Bill(bill.total * 0.85);
    }
}

function applicationUserDiscountPolicyShouldReduceTheBillTotalByTenPercents() {
    const initialBill = new Bill(100);
    const discountedBill = new ApplicationUserDiscount().apply(initialBill);
    console.assert(discountedBill.total === 90.0);
}

function pensionerDiscountPolicyShouldReduceTheBillTotalByFifteenPercents() {
    const initialBill = new Bill(100);
    const discountedBill = new PensionerDiscountPolicy().apply(initialBill);
    console.assert(discountedBill.total === 85.0);
}

applicationUserDiscountPolicyShouldReduceTheBillTotalByTenPercents();
pensionerDiscountPolicyShouldReduceTheBillTotalByFifteenPercents();

'use strict';

class Bill {
    _total;

    /**
     * @param {number} total
     */
    constructor(total) {
        this._total = total;
    }

    /**
     * @returns {number}
     */
    get total() {
        return this._total;
    }
}

class Customer {
    _isUsingApplication;
    _isPensioner;

    /**
     * @param {boolean} isUsingApplication
     * @param {boolean} isPensioner
     */
    constructor(isUsingApplication, isPensioner) {
        this._isUsingApplication = isUsingApplication;
        this._isPensioner = isPensioner;
    }

    /**
     * @returns {boolean}
     */
    get isUsingApplication() {
        return this._isUsingApplication;
    }

    /**
     * @returns {boolean}
     */
    get isPensioner() {
        return this._isPensioner;
    }
}

class CustomerSpecification {
    _customer;

    /**
     * @param {Customer} customer
     */
    constructor(customer) {
        this._customer = customer;
    }

    /**
     * @abstract
     * @returns {boolean}
     */
    isSatisfied() {}
}

class CustomerUsingApplicationSpecification extends CustomerSpecification {
    /**
     * @returns {boolean}
     */
    isSatisfied() {
        return this._customer.isUsingApplication;
    }
}

class PensionerCustomerSpecification extends CustomerSpecification {
    /**
     * @returns {boolean}
     */
    isSatisfied() {
        return this._customer.isPensioner;
    }
}

class DiscountPolicy {
    _specification;

    /**
     * @param {CustomerSpecification} specification
     */
    constructor(specification) {
        this._specification = specification;
    }

    /**
     * @abstract
     * @param {Bill} bill
     * @returns {Bill}
     */
    apply(bill) {}
}

class CustomerUsingApplicationDiscountPolicy extends DiscountPolicy {
    /**
     * @param {CustomerUsingApplicationSpecification} specification
     */
    constructor(specification) {
        super(specification);
    }

    /**
     * @param {Bill} bill
     * @returns {Bill}
     */
    apply(bill) {
        if (this._specification.isSatisfied()) {
            return new Bill(bill.total * 0.9);
        }

        return new Bill(bill.total);
    }
}

class PensionerDiscountPolicy extends DiscountPolicy {
    /**
     * @param {PensionerCustomerSpecification} specification
     */
    constructor(specification) {
        super(specification);
    }

    /**
     * @param {Bill} bill
     * @returns {Bill}
     */
    apply(bill) {
        if (this._specification.isSatisfied()) {
            return new Bill(bill.total * 0.85);
        }

        return new Bill(bill.total);
    }
}

function applicationUserDiscountPolicyShouldReduceTheBillTotalByTenPercents() {
    const initialBill = new Bill(100);
    const customer = new Customer(true, false);
    const specification = new CustomerUsingApplicationSpecification(customer);
    const policy = new CustomerUsingApplicationDiscountPolicy(specification);
    const discountedBill = policy.apply(initialBill);
    console.assert(discountedBill.total === 90.0);
}

function pensionerDiscountPolicyShouldReduceTheBillTotalByFifteenPercents() {
    const initialBill = new Bill(100);
    const customer = new Customer(false, true);
    const specification = new PensionerCustomerSpecification(customer);
    const policy = new PensionerDiscountPolicy(specification);
    const discountedBill = policy.apply(initialBill);
    console.assert(discountedBill.total === 85.0);
}

applicationUserDiscountPolicyShouldReduceTheBillTotalByTenPercents();
pensionerDiscountPolicyShouldReduceTheBillTotalByFifteenPercents();

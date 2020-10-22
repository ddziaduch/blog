<?php

declare(strict_types=1);

class Bill
{
    private float $total;

    public function __construct(float $total)
    {
        $this->total = $total;
    }

    public function getTotal(): float
    {
        return $this->total;
    }
}

class Customer
{
    private bool $isUsingApplication;
    private bool $isPensioner;

    public function __construct(bool $isUsingApplication, bool $isPensioner)
    {
        $this->isUsingApplication = $isUsingApplication;
        $this->isPensioner = $isPensioner;
    }

    public function isUsingApplication(): bool
    {
        return $this->isUsingApplication;
    }

    public function isPensioner(): bool
    {
        return $this->isPensioner;
    }
}

interface CustomerSpecification
{
    public function isSatisfied(): bool;
}

class CustomerUsingApplicationSpecification implements CustomerSpecification
{
    private Customer $customer;

    public function __construct(Customer $customer)
    {
        $this->customer = $customer;
    }

    public function isSatisfied(): bool
    {
        return $this->customer->isUsingApplication();
    }
}

class PensionerCustomerSpecification implements CustomerSpecification
{
    private Customer $customer;

    public function __construct(Customer $customer)
    {
        $this->customer = $customer;
    }

    public function isSatisfied(): bool
    {
        return $this->customer->isPensioner();
    }
}

interface DiscountPolicy
{
    public function apply(Bill $bill): Bill;
}

class CustomerUsingApplicationDiscountPolicy implements DiscountPolicy
{
    private CustomerUsingApplicationSpecification $specification;

    public function __construct(CustomerUsingApplicationSpecification $specification)
    {
        $this->specification = $specification;
    }

    public function apply(Bill $bill): Bill
    {
        if ($this->specification->isSatisfied()) {
            return new Bill($bill->getTotal() * 0.9);
        }

        return new Bill($bill->getTotal());
    }
}

class PensionerDiscountPolicy implements DiscountPolicy
{
    private PensionerCustomerSpecification $specification;

    public function __construct(PensionerCustomerSpecification $specification)
    {
        $this->specification = $specification;
    }

    public function apply(Bill $bill): Bill
    {
        if ($this->specification->isSatisfied()) {
            return new Bill($bill->getTotal() * 0.85);
        }

        return new Bill($bill->getTotal());
    }
}

function applicationUserDiscountPolicyShouldReduceTheBillTotalByTenPercents()
{
    $initialBill = new Bill(100);
    $customer = new Customer(true, false);
    $specification = new CustomerUsingApplicationSpecification($customer);
    $policy = new CustomerUsingApplicationDiscountPolicy($specification);
    $discountedBill = $policy->apply($initialBill);
    assert($discountedBill->getTotal() === 90.0);
}

function pensionerDiscountPolicyShouldReduceTheBillTotalByFifteenPercents()
{
    $initialBill = new Bill(100);
    $customer = new Customer(false, true);
    $specification = new PensionerCustomerSpecification($customer);
    $policy = new PensionerDiscountPolicy($specification);
    $discountedBill = $policy->apply($initialBill);
    assert($discountedBill->getTotal() === 85.0);
}

applicationUserDiscountPolicyShouldReduceTheBillTotalByTenPercents();
pensionerDiscountPolicyShouldReduceTheBillTotalByFifteenPercents();

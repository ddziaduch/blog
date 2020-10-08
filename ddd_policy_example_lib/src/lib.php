<?php

declare(strict_types=1);

class Bill
{
    public float $total;

    public function __construct(float $total)
    {
        $this->total = $total;
    }
}

interface DiscountPolicy
{
    public function apply(Bill $bill): Bill;
}

class ApplicationUserDiscountPolicy implements DiscountPolicy
{
    public function apply(Bill $bill): Bill
    {
        return new Bill($bill->total * 0.9);
    }
}

class PensionerDiscountPolicy implements DiscountPolicy
{
    public function apply(Bill $bill): Bill
    {
        return new Bill($bill->total * 0.85);
    }
}

function applicationUserDiscountPolicyShouldReduceTheBillTotalByTenPercents()
{
    $initialBill = new Bill(100);
    $discountedBill = (new ApplicationUserDiscountPolicy())->apply($initialBill);
    assert($discountedBill->total === 90.0);
}

function pensionerDiscountPolicyShouldReduceTheBillTotalByFifteenPercents()
{
    $initialBill = new Bill(100);
    $discountedBill = (new PensionerDiscountPolicy())->apply($initialBill);
    assert($discountedBill->total === 85.0);
}

applicationUserDiscountPolicyShouldReduceTheBillTotalByTenPercents();
pensionerDiscountPolicyShouldReduceTheBillTotalByFifteenPercents();

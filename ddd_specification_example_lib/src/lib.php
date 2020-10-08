<?php

declare(strict_types=1);

class Package
{
    public string $colour;
    public string $shape;

    public function __construct(string $colour, string $shape)
    {
        $this->colour = $colour;
        $this->shape = $shape;
    }
}

interface PackageSpecification
{
    public function isSatisfied(Package $package): bool;
}

class PackageColourSpecification implements PackageSpecification
{
    public string $expectedColour;

    public function __construct(string $expectedColour)
    {
        $this->expectedColour = $expectedColour;
    }

    public function isSatisfied(Package $package): bool
    {
        return $package->colour === $this->expectedColour;
    }
}

class PackageShapeSpecification implements PackageSpecification
{
    public string $expectedShape;

    public function __construct(string $expectedShape)
    {
        $this->expectedShape = $expectedShape;
    }

    public function isSatisfied(Package $package): bool
    {
        return $package->shape === $this->expectedShape;
    }
}

class AndPackageSpecification implements PackageSpecification
{
    public PackageSpecification $left;
    public PackageSpecification $right;

    public function __construct(
        PackageSpecification $left,
        PackageSpecification $right
    ) {
        $this->left = $left;
        $this->right = $right;
    }

    public function isSatisfied(Package $package): bool
    {
        return $this->left->isSatisfied($package)
            && $this->right->isSatisfied($package);
    }
}

function shouldBePossibleToCreateSpecificationForRedOvalPackage()
{
    $redOvalPackage = new Package('red', 'oval');
    $redSquarePackage = new Package('red', 'square');
    $greenOvalPackage = new Package('green', 'oval');
    $redOvalPackageSpecification = new AndPackageSpecification(
        new PackageColourSpecification('red'),
        new PackageShapeSpecification('oval'),
    );

    assert($redOvalPackageSpecification->isSatisfied($redOvalPackage) === true);
    assert($redOvalPackageSpecification->isSatisfied($redSquarePackage) === false);
    assert($redOvalPackageSpecification->isSatisfied($greenOvalPackage) === false);
}

shouldBePossibleToCreateSpecificationForRedOvalPackage();

struct Package<'a> {
    colour: &'a str,
    shape: &'a str
}

trait PackageSpecification {
    fn is_satisfied(&self, package: &Package) -> bool;
}

struct PackageColourSpecification<'a> {
    expected_colour: &'a str
}

impl PackageSpecification for PackageColourSpecification<'_> {
    fn is_satisfied(&self, package: &Package) -> bool {
        package.colour == self.expected_colour
    }
}

struct PackageShapeSpecification<'a> {
    expected_shape: &'a str
}

impl PackageSpecification for PackageShapeSpecification<'_> {
    fn is_satisfied(&self, package: &Package) -> bool {
        package.shape == self.expected_shape
    }
}

struct AndPackageSpecification<'a> {
    left: &'a dyn PackageSpecification,
    right: &'a dyn PackageSpecification
}

impl PackageSpecification for AndPackageSpecification<'_> {
    fn is_satisfied(&self, package: &Package) -> bool {
        self.left.is_satisfied(package) && self.right.is_satisfied(package)
    }
}

#[test]
fn should_be_possible_to_create_specification_for_red_oval_package() {
    let red_oval_package = Package {
        colour: "red",
        shape: "oval"
    };
    let red_square_package = Package {
        colour: "red",
        shape: "square"
    };
    let green_oval_package = Package {
        colour: "green",
        shape: "oval"
    };
    let red_oval_package_specification = AndPackageSpecification {
        left: &PackageColourSpecification { expected_colour: "red" },
        right: &PackageShapeSpecification { expected_shape: "oval" }
    };
    assert_eq!(
        red_oval_package_specification.is_satisfied(&red_oval_package),
        true,
    );
    assert_eq!(
        red_oval_package_specification.is_satisfied(&red_square_package),
        false,
    );
    assert_eq!(
        red_oval_package_specification.is_satisfied(&green_oval_package),
        false,
    );
}

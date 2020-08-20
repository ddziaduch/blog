'use strict';

class Box {
    colour;
    shape;

    /**
     * @param {string} colour
     * @param {string} shape
     */
    constructor(colour, shape) {
        this.colour = colour;
        this.shape = shape;
    }
}

class BoxSpecification {
    /**
     * @param {Box} box
     * @returns {boolean}
     */
    isSatisfiedBy(box) {
        return box instanceof Box;
    }
}

class BoxColourSpecification extends BoxSpecification {
    _expectedColour;

    /**
     * @param {string} expectedColour
     */
    constructor(expectedColour) {
        super();
        this._expectedColour = expectedColour;
    }

    /**
     * @param {Box} box
     * @returns {boolean}
     */
    isSatisfiedBy(box) {
        return super.isSatisfiedBy(box) && box.colour === this._expectedColour;
    }
}

class BoxShapeSpecification extends BoxSpecification {
    _expectedShape;

    /**
     * @param {string} expectedShape
     */
    constructor(expectedShape) {
        super();
        this._expectedShape = expectedShape;
    }

    /**
     * @param {Box} box
     * @returns {boolean}
     */
    isSatisfiedBy(box) {
        return super.isSatisfiedBy(box) && box.shape === this._expectedShape;
    }
}

class AndBoxSpecification extends BoxSpecification {
    _left;
    _right;

    /**
     * @param {BoxSpecification} left
     * @param {BoxSpecification} right
     */
    constructor(left, right) {
        super();
        this._left = left;
        this._right = right;
    }

    /**
     * @param {Box} box
     * @returns {boolean}
     */
    isSatisfiedBy(box) {
        return super.isSatisfiedBy(box)
            && this._left.isSatisfiedBy(box)
            && this._right.isSatisfiedBy(box);
    }
}

const redOvalBox = new Box('red', 'oval');
const redSquareBox = new Box('red', 'square');
const greenOvalBox = new Box('green', 'oval');
const redOvalBoxSpecification = new AndBoxSpecification(
    new BoxColourSpecification('red'),
    new BoxShapeSpecification('oval'),
)

console.assert(
    redOvalBoxSpecification.isSatisfiedBy(redOvalBox) === true,
    'should be possible to create specification for red oval package',
);
console.assert(
    redOvalBoxSpecification.isSatisfiedBy(redSquareBox) === false,
    'should be possible to create specification for red oval package',
);
console.assert(
    redOvalBoxSpecification.isSatisfiedBy(greenOvalBox) === false,
    'should be possible to create specification for red oval package',
);

const isEven = require('is-even');
const isOdd = require('is-odd');

const isEvenOrOdd = (x) => isEven(x) || isOdd(x);

console.log(isEvenOrOdd(5));
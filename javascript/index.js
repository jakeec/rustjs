var aNumber = 10;
var aString = 'jake';
var anArray = [10, 20, 30, 40];
var anObject = {
  propA: 20.2,
  propB: 'blue',
  propC: {
    propA: ['test'],
  },
};

var a = aNumber + aString;
var b = aString + aNumber;
var c = anArray + aNumber;
var d = anArray + aString;
var e = aNumber + anArray;
var f = aString + anArray;
var g = anArray + anArray;
var h = aNumber + aNumber;
var i = aNumber + null;
var j = aNumber + undefined;
var k = aNumber + [];

console.log(`${a} - ${typeof a}`);
console.log(`${b} - ${typeof b}`);
console.log(`${c} - ${typeof c}`);
console.log(`${d} - ${typeof d}`);
console.log(`${e} - ${typeof e}`);
console.log(`${f} - ${typeof f}`);
console.log(`${g} - ${typeof g}`);
console.log(`${h} - ${typeof h}`);
console.log(`${i} - ${typeof i}`);
console.log(`${j} - ${typeof j}`);
console.log(`${k} - ${typeof k}`);

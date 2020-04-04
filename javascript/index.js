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
var l = null + aNumber;
var m = undefined + aNumber;
var n = [] + aNumber;
var o = 10 + NaN;
var p = NaN + 10;
var q = true + true;
var r = true + false;
var s = false + true;
var t = false + false;

console.log(`a ${a} - ${typeof a}`);
console.log(`b ${b} - ${typeof b}`);
console.log(`c ${c} - ${typeof c}`);
console.log(`d ${d} - ${typeof d}`);
console.log(`e ${e} - ${typeof e}`);
console.log(`f ${f} - ${typeof f}`);
console.log(`g ${g} - ${typeof g}`);
console.log(`h ${h} - ${typeof h}`);
console.log(`i ${i} - ${typeof i}`);
console.log(`j ${j} - ${typeof j}`);
console.log(`k ${k} - ${typeof k}`);
console.log(`l ${l} - ${typeof l}`);
console.log(`m ${m} - ${typeof m}`);
console.log(`n ${n} - ${typeof n}`);
console.log(`o ${o} - ${typeof o}`);
console.log(`p ${p} - ${typeof p}`);
console.log(`q ${q} - ${typeof q}`);
console.log(`r ${r} - ${typeof r}`);
console.log(`s ${s} - ${typeof s}`);
console.log(`t ${t} - ${typeof t}`);

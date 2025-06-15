function böl(str, delim) {
    return str.split(delim);
}

function cəm(arr) {
    return arr.reduce((a, b) => a + b, 0);
}

for (let i = 0; i < 1_0000000; i++) {
    const b = böl("S a l a m", " ");
    const x = b[0];
    const s = cəm([1, 2, 3]);
}

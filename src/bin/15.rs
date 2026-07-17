// # 15
// Starting in the top left corner of a 2 x 2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.
// <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAOwAAACcCAYAAAB88UfVAAAALnRFWHRDcmVhdGlvbiBUaW1lAFNhdCAxMSBKdWwgMjAyNiAwMzowNjowOCBQTSBDRVNU/O4ShgAAABl0RVh0U29mdHdhcmUAZ25vbWUtc2NyZWVuc2hvdO8Dvz4AAARiSURBVHic7d3tctowFATQ0On7v3JaOqUhaQBfsGStdM7vTFMWra/jz5/vv70BEX68ATEUFoIoLARRWAiisBDk5xtdnU6ntx5mOfgvr89eKmyvMHuZ6QzX+btxxm67lLxM2Im13KDOuDFoPYD2yExhr8w2lVp9ltn2rC5afvd7ZeagEwR5ecK2nkitp94R08Lkq5HXBxMWgigsBFFYCKKwEERhIYjCQhCFhSAKC0EUFoIoLARRWAiisBBEYSGIwkIQhYUgCgtBFBaCKCwEUVgIorAQRGEhiOcST2zWpyiuzISFIC9PWFvxOpnVyOvD8A8SP/PSrW165nT5XT2+/155JayzmL9hPf2d1hLeMOGg00QuC67l601s4I7loBMEUVgIorAQRGEhyBIHnSoHSh797ExvaL9FXuNaYsJaNDXyGpfTOgUWco289rfM37AWT428xmTCbmQB18irjaWOEltENfIajwm7gYVbI692ljsPazHVyGssJuwDFmyNvNpa8koni6pGXuMwYe+wUGvk1Z7Cwh09n6yxxbIX/z/6AkyLz1bP61zcEW7eN2Gh4OiJu/TtdbdCN12/J68PR01cExZe0HviLn8D+9egTdf75PW9XhPXhIUdtZ64CnvFtKiZLa89J2Sr4sa8qmOm5+H2+CzyGsPe//fTu7HC4nptEPaoml1i6GCvuaiwLO9emV6dvsP9DQv8z1FiCND6kJDCwg56HbtVWHhB75MsCgtPOOpsqMJCwdGXLSgsbDDK9UUKC3eMdiGg98NCEIWFIAoLQRQWgigsBFFYCKKwEERhIYjCQhCFhSAKC0EUFoIoLARRWAiisBBEYSHIUzewz/TelovWNyrPlpm86oZ5VUfLL+/8xc24OFp9JnnV9Mjr8nv2YJcYgigsBFFYCKKwEERhIYjCQhCFhSAKC0EUFoIoLARRWAiisBBEYSGIwkIQhYUgCgtBFBaCKCwEUVgIorAQRGEhiMJCEIWFILs8l7j1c2pnfKh0y890+bd7PG+XvnYpLLQ240b7GQr714xTaaZF3vrtEikUdmItX28xm5TXmzxV2MuH6/lekvTF1yMzu43zc5QYgigsBFFYCKKwEGSJo8SVgzGPftbFCBxpiQmrZMzCedgCxedoy/wNq2zMwITdSOEZwVJHiZWOdCbsBorOKJY7D6t8JDNhH1BwRrLklU5KSCoT9g7FZjQKuxDPeqobLbNlL/5/9AXMvKjPi9DN7jWjZGbCLszErTs6s6Vvr7sV+moL2MStOyozE5Z/TNy63pktfwP716AtVhP3Gb0yM2G5ycSta52Zwl6ZbWHutcUfobhpr4NpldnLhe216zTTLlrqZ7GbXLd3Zi8V1q5SXa8HrzMnu8SU2VDX7ZWZwrJZz6Jev9ok2d6Znd5tLpfx7OJfeYmMlpkJy12253UtM1NYvqWodT0yU1g+UdS6npkpLH8oat0RmSksyvqEozJT2IUpat3RmSnsghS1bpTMnIeFIL8ArsrES4GDq/4AAAAASUVORK5CYII=" alt="" width="236" height="156" />
// How many such routes are there through a 20 x 20 grid?
//
// Answer: 137846528820

fn count_routes(x: usize, y: usize, memo: &mut [[u64; 21]; 21]) -> u64 {
    if memo[x][y] != 0 {
        return memo[x][y];
    }

    if x == 20 || y == 20 {
        return 1;
    }

    memo[x][y] = count_routes(x + 1, y, memo) + count_routes(x, y + 1, memo);

    memo[x][y]
}

fn main() {
    // To arrive at the bottom-right corner, you must move 20 times down and 20 times right.
    // So, the result is 40 C 20 = 137846528820.
    // Let's calculate it empirically

    let mut memo = [[0; 21]; 21]; // a 21x21 grid storing the number of routes from each cell
    let result = count_routes(0, 0, &mut memo);
    println!("Total routes: {}", result); // 137846528820
}

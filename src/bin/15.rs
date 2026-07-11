// # 15
// Starting in the top left corner of a 2 x 2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.
// <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAOwAAACcCAYAAAB88UfVAAAALnRFWHRDcmVhdGlvbiBUaW1lAFNhdCAxMSBKdWwgMjAyNiAwMzowNjowOCBQTSBDRVNU/O4ShgAAABl0RVh0U29mdHdhcmUAZ25vbWUtc2NyZWVuc2hvdO8Dvz4AAARiSURBVHic7d3tctowFATQ0On7v3JaOqUhaQBfsGStdM7vTFMWra/jz5/vv70BEX68ATEUFoIoLARRWAiisBDk5xtdnU6ntx5mOfgvr89eKmyvMHuZ6QzX+btxxm67lLxM2Im13KDOuDFoPYD2yExhr8w2lVp9ltn2rC5afvd7ZeagEwR5ecK2nkitp94R08Lkq5HXBxMWgigsBFFYCKKwEERhIYjCQhCFhSAKC0EUFoIoLARRWAiisBBEYSGIwkIQhYUgCgtBFBaCKCwEUVgIorAQRGEhiOcST2zWpyiuzISFIC9PWFvxOpnVyOvD8A8SP/PSrW165nT5XT2+/155JayzmL9hPf2d1hLeMOGg00QuC67l601s4I7loBMEUVgIorAQRGEhyBIHnSoHSh797ExvaL9FXuNaYsJaNDXyGpfTOgUWco289rfM37AWT428xmTCbmQB18irjaWOEltENfIajwm7gYVbI692ljsPazHVyGssJuwDFmyNvNpa8koni6pGXuMwYe+wUGvk1Z7Cwh09n6yxxbIX/z/6AkyLz1bP61zcEW7eN2Gh4OiJu/TtdbdCN12/J68PR01cExZe0HviLn8D+9egTdf75PW9XhPXhIUdtZ64CnvFtKiZLa89J2Sr4sa8qmOm5+H2+CzyGsPe//fTu7HC4nptEPaoml1i6GCvuaiwLO9emV6dvsP9DQv8z1FiCND6kJDCwg56HbtVWHhB75MsCgtPOOpsqMJCwdGXLSgsbDDK9UUKC3eMdiGg98NCEIWFIAoLQRQWgigsBFFYCKKwEERhIYjCQhCFhSAKC0EUFoIoLARRWAiisBBEYSHIUzewz/TelovWNyrPlpm86oZ5VUfLL+/8xc24OFp9JnnV9Mjr8nv2YJcYgigsBFFYCKKwEERhIYjCQhCFhSAKC0EUFoIoLARRWAiisBBEYSGIwkIQhYUgCgtBFBaCKCwEUVgIorAQRGEhiMJCEIWFILs8l7j1c2pnfKh0y890+bd7PG+XvnYpLLQ240b7GQr714xTaaZF3vrtEikUdmItX28xm5TXmzxV2MuH6/lekvTF1yMzu43zc5QYgigsBFFYCKKwEGSJo8SVgzGPftbFCBxpiQmrZMzCedgCxedoy/wNq2zMwITdSOEZwVJHiZWOdCbsBorOKJY7D6t8JDNhH1BwRrLklU5KSCoT9g7FZjQKuxDPeqobLbNlL/5/9AXMvKjPi9DN7jWjZGbCLszErTs6s6Vvr7sV+moL2MStOyozE5Z/TNy63pktfwP716AtVhP3Gb0yM2G5ycSta52Zwl6ZbWHutcUfobhpr4NpldnLhe216zTTLlrqZ7GbXLd3Zi8V1q5SXa8HrzMnu8SU2VDX7ZWZwrJZz6Jev9ok2d6Znd5tLpfx7OJfeYmMlpkJy12253UtM1NYvqWodT0yU1g+UdS6npkpLH8oat0RmSksyvqEozJT2IUpat3RmSnsghS1bpTMnIeFIL8ArsrES4GDq/4AAAAASUVORK5CYII=" alt="" width="236" height="156" />
// How many such routes are there through a 20 x 20 grid?
use std::io;
fn main() {
    let max_right = 3;
    let max_down = 3;

    // [20,20]
    // [19, 20, 1, 0]
    // [18, 20, 2, 0]
    //     ...
    // [1, 20, 19, 0]
    // [19, 19, 1, 1]
    // [18, 19, 2, 1]
    //     ...
    // [1, 19, 19, 1]
    // [19, 18, 1, 2]
    // ...
    // [19, 1, 1, 19]
    // ...
    // [1, 1, 19, 19]
    // [1, 1, 18, 19, 1, 0]
    // ...
    // [1, 1, 1, 19, 18, 0]
    // [1, 1, 18, 18, 1, 1]
    // [1, 1, 17, 18, 1, 2]
    // ...
    // [1, 1, 1, 18, 18, 1]
    // [1, 1, 18, 17, 1, 2]
    // ...
    // [1, 1, 1, 17, 18, 2]
    // ...
    // [1, 1, 18, 1, 1, 18]
    // [1, 1, 17, 1, 2, 18]
    // ...
    // [1, 1, 1, 1, 18, 18]
    //...
    // [1,1,1,...1,1]

    let mut trace: Vec<usize> = vec![max_down, max_right];
    let mut down_pos = 0;
    let mut right_pos = 1;
    let mut n_traces = 0;

    let mut down_reset_value = max_down - 1;
    while trace != vec![1; max_down + max_right] {
        if trace[down_pos] == 1 && trace[right_pos] == 1 {
            down_pos += 2;
            right_pos += 2;
            trace.push(down_reset_value);
            trace.push(down_reset_value);
            down_reset_value -= 1;
            n_traces += 1;
            continue;
        } else if trace[down_pos] == 1 && trace[right_pos] > 1 {
            trace[right_pos] -= 1;
            n_traces += 1;
            trace[down_pos] = down_reset_value;
            continue;
        }

        println!("{:?}", trace);
        let mut input = String::new();

        //DEBUG
        io::stdin().read_line(&mut input).expect("Reading error");

        trace[down_pos] -= 1;
        n_traces += 1;
    }
    println!("{}", n_traces * 2);

    // TODO
}

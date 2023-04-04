import http from "k6/http"
import { sleep } from 'k6';
import { SharedArray } from 'k6/data';

export let options = {
    insecureSkipTLSVerify: true,
    noConnectionReuse: false,
    //discard the response since we don't really use it for anything at this point in time.
    // https://github.com/grafana/k6-docs/blob/main/src/data/markdown/translated-guides/en/06%20Testing%20Guides/04%20Running%20large%20tests.md#save-memory-with-discardresponsebodies
    discardResponseBodies: true,
    stages: [
        { duration: "2m", target: 100 }, // below normal load
        { duration: "5m", target: 100 },
        { duration: "2m", target: 200 }, // normal load
        { duration: "5m", target: 200 },
        { duration: "2m", target: 250 }, // around breaking point
        { duration: "5m", target: 250 },
        { duration: "2m", target: 350 }, // beyond breaking point
        { duration: "5m", target: 350 },
        { duration: "10m", target: 0 }, // recovery stage.
    ],
}

const body = new SharedArray('some name', function() {
    return parse(open(`/${__ENV.HOME}/tests/s/${__ENV.TEST_AMOUNT}.txt`));
});

function parse(text) {
    return text.split("\n").map(Number);
}

export default () => {
    http.post(`http://142.93.107.93:8080/sort/${__ENV.TARGET}`, JSON.stringify(body), {
        headers: { 'Content-type': 'application/json' },
    });
    sleep(1);
}

// This function will only run if the test was successful
export function teardown() {
    console.log('teardown will still be called after test.abort()');
}

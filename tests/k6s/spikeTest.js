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
        { duration: "10s", target: 100 }, // below normal load
        { duration: "1m", target: 100 },
        { duration: "10s", target: 300 }, // spike to 300 users
        { duration: "3m", target: 300 }, // stay at 300 for 3 minutes
        { duration: "10s", target: 100 }, // scale back down
        { duration: "3m", target: 100 },
        { duration: "10s", target: 0 },
    ],
}

const body = new SharedArray('some name', function() {
    return parse(open(`/${__ENV.HOME}/tests/s/${__ENV.TEST_AMOUNT}.txt`));
});

function parse(text) {
    return text.split("\n").map(Number);
}

export default () => {
    http.post(`http://142.93.107.93:8080/${__ENV.TARGET}`, JSON.stringify(body), {
        headers: { 'Content-type': 'application/json' },
    });
    sleep(1);
}

// This function will only run if the test was successful
export function teardown() {
    console.log('teardown will still be called after test.abort()');
}

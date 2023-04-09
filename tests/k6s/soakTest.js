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
        { duration: "2m", target: 200 }, // ramp up to 200 users
        { duration: "3h56m", target: 200 }, // stay at 200 users for around 4 hours.
        { duration: "2m", target: 0 }, // ramp down.
    ],
}

const body = new SharedArray('some name', function() {
    return parse(open(`/${__ENV.HOME}/tests/documents/unsorted/${__ENV.TEST_AMOUNT}.txt`));
});

function parse(text) {
    return text.split("\n").map(Number);
}

console.log(`Starting soak test with ENV: ${__ENV} at: ${new Date()}`)

export default () => {
    http.post(`http://142.93.107.93:8080${__ENV.TARGET}`, JSON.stringify(body), {
        headers: { 'Content-type': 'application/json' },
    });
    sleep(1);
}

// This function will only run if the test was successful
export function teardown() {
    console.log(`Finished soak test with ENV: ${__ENV} at: ${new Date()}`)
}

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
        { duration: "5m", target: 100 }, // simulate ramp-up of traffic from 1 to 100 users over 5 minutes
        { duration: "10m", target: 100 }, // stay at 100 users for 10 minutes
        { duration: "5m", target: 0 }, // ramp-down to 0 users
    ],
}

const body = new SharedArray('some name', function() {
    return parse(open(`/${__ENV.HOME}/tests/documents/${__ENV.TEST_AMOUNT}.txt`));
});

function parse(text) {
    return text.split("\n").map(Number);
}

console.log(`Starting load test with ENV: ${JSON.stringify(__ENV, null, 2)} at: ${new Date()}`)

export default () => {
    http.post(`http://142.93.107.93:8080${__ENV.TARGET}`, JSON.stringify(body), {
        headers: { 'Content-type': 'application/json' },
    });
    sleep(1);
}

export function teardown() {
    console.log(`Finished load test with ENV: ${JSON.stringify(__ENV, null, 2)} at: ${new Date()}`)
}

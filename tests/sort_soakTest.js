import http from "k6/http"
import { sleep } from 'k6';
import { SharedArray } from 'k6/data';

export let options = {
    insecureSkipTLSVerify: true,
    noConnectionReuse: false,
    stages: [
        { duration: "2m", target: 200 },
        { duration: "3h56m", target: 200 },
        { duration: "2m", target: 0 },
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

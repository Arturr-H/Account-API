/*- Check if values assert/match if not throw -*/
const assert = require("assert");

/*- Fetching because node fetch is still experimental I think. Bump node later and delete this -*/
const fetch = require("node-fetch");

/*- Path handling -*/
const path = require("path");

/*- Read env files -*/
require("dotenv").config({ path: path.resolve("config/.env.development") });

/*- Put the variables you wanna check here -*/
const { SERVER_URL, DEBUG } = process.env;

const successMessagePadding = 20;

/*- Success message -*/
const succeed = (i) => {
    const padding = successMessagePadding - i.toString().length;
    const paddingStr = Array(padding).fill(" ").join("");

    console.log("\x1b[32m", `${i} ${paddingStr} 200 OK`);
}

/*- Unsuccessful message -*/
const fail = (i) => {
    const padding = successMessagePadding - i.toString().length;
    const paddingStr = Array(padding).fill(" ").join("");

    console.log("\x1b[31m", `${i} ${paddingStr} FAILED`);
}

/*- Check if db is up -*/
const checkDBstatus = async () => {
    await fetch(SERVER_URL).then(data => {
        try {
            assert.equal(data.status, 200);
        }catch {
            return fail("DB status")
        }
    })
    
    /*- If nothing failed -*/
    succeed("DB status");
}

/*- Check if debug is on in production which it shouldn't -*/
const checkDebug = () => {
    try {
        assert.equal(DEBUG == "true", false);
    }catch(e) {
        return fail("DEBUG")
    }

    /*- If nothing failed -*/
    succeed("DEBUG");
}

/*- MAIN -*/
(() => {   
    checkDBstatus();
    checkDebug();
})();
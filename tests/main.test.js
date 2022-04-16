/*- Check if values assert/match if not throw -*/
const assert = require("assert");

/*- Fetching because node fetch is still experimental I think. Bump node later and delete this -*/
const fetch = require("node-fetch");

/*- Path handling -*/
const path = require("path");

/*- Read env files -*/
require("dotenv").config({ path: path.resolve("config/global.env") });
const { DEBUG, APPLICATION_STATE } = process.env;

require("dotenv").config({ path: path.resolve(`config/.env.${APPLICATION_STATE}`) });

/*- Put the variables you wanna check here -*/
const { SERVER_URL, CDN_URL } = process.env;
const { checkUsername, getPrettifiedDate } = require(path.resolve("routes/Api.js"));
const successMessagePadding = 30;

/*- Success message -*/
const succeed = (i) => {
    const padding = successMessagePadding - i.toString().length;
    const paddingStr = Array(padding).fill(" ").join("");

    console.log("\x1b[32m|", `${i} ${paddingStr} | 200 OK |`);
}

/*- Unsuccessful message -*/
const fail = (i, f = "Nothing provided.") => {
    const padding = successMessagePadding - i.toString().length;
    const paddingStr = Array(padding).fill(" ").join("");

    console.log("\x1b[31m|", `${i} ${paddingStr} | FAILED | :: ${f}`);
}

const initPattern = (e) => console.log(`${e?"\n":""}\x1b[32m++++++++++++++++++${!e?" ASYNC ":"+++++++"}+++++++++++++++++++`)

/*- Check if db is up -*/
const checkDBstatus = async () => {
    await fetch(SERVER_URL).then(data => {
        try {
            assert.equal(data.status, 200);

            /*- If nothing failed -*/
            succeed("DB status");
        } catch {
            return fail("DB status", `<${SERVER_URL}> is not responding`);
        }
    })
}

/*- Check if content delivery network is up -*/
const checkCDNstatus = async () => {
    await fetch(CDN_URL).then(data => {
        try {
            assert.equal(data.status, 200);

            /*- If nothing failed -*/
            succeed("CDN status");
        } catch {
            return fail("CDN status", `<${CDN_URL}> is not responding`);
        }
    })
}

/*- Check if debug is on in production which it shouldn't -*/
const checkDebug = () => {
    try {
        assert.equal(DEBUG == "true", false);
    } catch (e) {
        return fail("DEBUG", `<DEBUG> true in production`);
    }

    /*- If nothing failed -*/
    succeed("DEBUG");
}

/*- Username check function test -*/
const checkFN__checkUsername = async () => {

    const uname = "username123";

    await checkUsername(uname, (d) => {
        assert.equal(d.success, true);

        /*- If nothing failed -*/
        succeed("checkUsername");
    }, true).catch(_ => {
        return fail("checkUsername", `<${uname}> is invalid`)
    })
}

/*- Prettify date test -*/
const checkFN__getPrettifiedDate = () => {
    try {
        assert.equal(getPrettifiedDate(1577836800000), "Thursday, January 1 - 2020");
    } catch (e) {
        return fail("getPrettifiedDate")
    }

    /*- If nothing failed -*/
    succeed("getPrettifiedDate");
}


/*- Application state -*/
const checkApplicationState = () => {
    try {
        assert.equal(APPLICATION_STATE, "production");
    }catch {
        return fail("checkApplicationState", `<APPLICATION_STATE> is not production`);
    }
    
    /*- If nothing failed -*/
    succeed("checkApplicationState");
}

/*- MAIN -*/
(() => {
    initPattern(true);

    checkApplicationState();
    checkDebug();
    checkFN__checkUsername();
    checkFN__getPrettifiedDate();
    checkDBstatus();
    checkCDNstatus();

    initPattern(false);
})();
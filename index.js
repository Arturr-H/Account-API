/*- Mongo client to easily connect to mongoDB (in docker-compose) -*/
const { MongoClient } = require("mongodb");

/*- Express as main server handler. -*/
const express = require("express");
const app = express();

/*- Path handling -*/
const path = require("path");

/*- 3party access -*/
const cors = require("cors");
app.use(cors());

/*- Make .env files readable -*/
require("dotenv").config({ path: path.resolve("config/.env.production") });

/*- Mongo connection-string. Mostly defined in docker-compose -*/
const uri = process.env.MONGO_URI_STRING;
const dbs = process.env.DBS;
const debug = process.env.DEBUG;

/*- All imported routes -*/
const api = require("./routes/Api");

/*- User templates for filtering sensitive data -*/
const { User, SafeUser } = require("./data/models/User.js");

/*- Use routes -*/
app.use("/api", api.app);

/*- Immutable variables -*/
const PORT = process.env.PORT;

/*- Main -*/
app.get("/", (_, res) => {
    MongoClient.connect(uri, (err, client) => {
        if (err) throw err;

        const db = client.db(dbs);
        db.collection("users").find().toArray((err, items) => {
            if (err && debug) return console.log(err);
            res.send(items.map(item => 
                /*- Map all items to only contain information that isn't private -*/
                new SafeUser(item)
            ));
        });
    });
});

/*- Start the server -*/
app.listen(PORT, () => {
    console.log(`Server started on ${PORT}`);
});
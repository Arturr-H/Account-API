/*- Mongo client to easily connect to mongoDB (in docker-compose) -*/
const { MongoClient } = require("mongodb");

/*- Express as main server handler. -*/
const express = require("express");
const app = express();

/*- Path handling -*/
const path = require("path");

/*- Make .env files readable -*/
require("dotenv").config({ path: path.resolve("config/.env") });

/*- Mongo connection-string. Mostly defined in docker-compose -*/
const uri = process.env.MONGO_URI_STRING;
const dbs = process.env.DBS;

/*- All imported routes -*/
const api = require("./routes/Api");

/*- Use routes -*/
app.use("/api", api);

/*- Immutable variables -*/
const PORT = process.env.PORT;

/*- Main -*/
app.get("/", (_, res) => {
    MongoClient.connect(uri, (err, client) => {
        if (err) throw err;

        const db = client.db(dbs);
        db.collection("users").find().toArray((err, items) => {
            if (err) {
                return console.log(err);
            }
            res.send(items.map(item => {

                /*- Map all items to only contain information that isn't private -*/
                return {
                    username: item.username,
                    displayname: item.displayname,
                    joined: item.joined,
                    role: item.role,
                    profile: `${process.env.SERVER_URL}/api/profile-data/image/${item.suid}`,
                }
            }));
        });
    });
});

/*- Start the server -*/
app.listen(PORT, () => {
    console.log(`Server started on ${PORT}`);
});
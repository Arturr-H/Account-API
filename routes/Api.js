/*-------- API routes ---------*/
/*- Express as server handler -*/
const express = require("express");

/*- Mongo client to easily connect to mongoDB (in docker-compose) -*/
const { MongoClient } = require("mongodb");

/*- Make .env files readable -*/
require("dotenv").config();

/*- For encrypting / hashing sensitive information -*/
const crypto = require("crypto");

/*- Mongo connection-string. Mostly defined in docker-compose -*/
const uri = process.env.MONGO_URI_STRING;
const dbs = process.env.DBS;

/*- Path handling -*/
const path = require("path");

/*- Image uploading -*/
const multer = require("multer");
const upload = multer({
    dest: path.join(__dirname, "../public/uploads/"),
    fileFilter(_, file, cb){
        if(!file.originalname.match(/\.(jpg|jpeg|png)$/)){
            return cb(new Error("File must be an image"));
        }
        cb(undefined, true);
    },
});

/*- GM for image handling -*/
const gm = require('gm').subClass({ imageMagick: true });

/*- Yaml reader and fs lib (yaml requires fs to be in project) -*/
const yaml = require("js-yaml");
const fs = require("fs");

/*- For reading cookies -*/
const cookieParser = require("cookie-parser");

/*- Immutable variables -*/
const debug = process.env.DEBUG;
const dictionary = yaml.load(fs.readFileSync(path.resolve("data/dict.yml"), "utf8")).dictionary;
const variables = yaml.load(fs.readFileSync(path.resolve("data/variables.yml"), "utf8")).variables;

/*- Move to another file -*/
const roles = {
    admin: "admin",
    moderator: "moderator",
    user: "user",
}

console.log(dictionary.reserved_usernames);

/*- Export it so that index.js or whatever main file can import this -*/
module.exports = (() => {
    const app = express.Router();
    
    /*- Make cookies readable using req.cookies -*/
    app.use(cookieParser());

    /*- api/create-account -*/
    app.get("/create-account", async (req, res) => {
        const { email, username, displayname, password } = req.headers;

        /*- Check if fields are missing -*/
        if (!email || !username || !displayname || !password) {

            /*- The missing field, sorted by most least to most important -*/
            let missing;
            if (!displayname) missing = "displayname";
            if (!username) missing    = "username";
            if (!password) missing    = "password";
            if (!email) missing       = "email";

            return res.json({
                message: `${dictionary.missing_fields} ${missing}`,
                status: 400
            });
        };

        /*- Salt & hash the password for security. -*/
        const salt = crypto.randomBytes(16).toString("hex");
        const hash = crypto.pbkdf2Sync(password, salt, 1000, 64, "sha512").toString("hex");

        /*- Check if username is already in use -*/
        await checkUsername(username, (result) => {
            if (result && !result.success) return res.json({
                message: result.message,
                status: 400
            });
            else if (result && result.success) {
                /*- Try creating the account -*/
                MongoClient.connect(uri, async (err, client) => {
        
                    /*- Get the database -*/
                    const db = client.db(dbs);
        
                    /*- Check if the email already exists -*/
                    const checkEmail = await db.collection("users").findOne({ email: email });
        
                    /*- If email exists -*/
                    if (checkEmail) return res.json({
                        message: dictionary.illegal_email,
                        status: 400,
                    });
        
                    /*- User-id -*/
                    const uid = crypto.randomUUID();
                    const suid = crypto.randomBytes(16).toString("hex");
        
                    /*- Small user info that might be used somewhere -*/
                    const userInfo = {
                        joined: new Date().toDateString(),
                        role: roles.user,
                    }
        
                    /*- Insert the new user -*/
                    const insertUser = await db.collection("users").insertOne({
                        uid,
                        suid,
                        salt,
                        email,
                        username,
                        displayname,
                        ...userInfo,
                        password: hash, 
                    });
        
                    /*- If user was inserted -*/
                    if (insertUser.insertedCount === 1) {
                        console.log("three")
                        return res.json({
                            status: 200,
                            message: dictionary.account_created,
                        });
                    }
                });
            }
            else{
                console.log("two", username, result)
            }
        });
    });

    /*- api/login -*/
    app.get("/login", (req, res) => {
        const { email, password } = req.headers;

        /*- Check if fields are missing -*/
        if (!email || !password) {
            return res.json({
                message: dictionary.error.login.missing_fields,
                status: 400
            });
        };

        /*- Try logging in -*/
        MongoClient.connect(uri, (_, client) => {

            const db = client.db(dbs);
            db.collection("users").findOne({ email }, (_, user) => {

                if (!user) {
                    return res.json({
                        message: dictionary.error.login.invalid_credentials,
                        status: 400
                    });
                }

                /*- Hash the password for security. -*/
                const hash = crypto.pbkdf2Sync(password, user.salt, 1000, 64, "sha512").toString("hex");

                /*- Check if passwords match -*/
                if (user.password !== hash) {
                    return res.json({
                        message: dictionary.error.login.invalid_credentials,
                        status: 400
                    });
                }

                /*- Respond with user-data -*/
                return res.json({
                    message: dictionary.status.success,
                    status: 200,
                    data: {
                        uid: user.uid
                    }
                });
            });
        });
    });

    /*- api/profile-upload -*/
    app.post("/profile-upload", upload.single("profile-file"), async (req, res) => {
    
        try{
            /*-
                Get the secure-user-id, all profile
                images are saved with the SUID, so
                that it easily links with all accounts
            -*/
            const suid = req.cookies["suid"];

            /*- If the image already exists, remove it -*/
            if (fs.existsSync(path.resolve(`uploads/profile/${suid}.jpg`))) {
                fs.unlinkSync(path.resolve(`uploads/profile/${suid}.jpg`));
            }

            /*- Temp path is used for finding the image and replacing / renaming it -*/
            const tempPath = req.file.path;
            const targetPath = path.resolve(`uploads/profile/${suid}.jpg`);
        
            /*- Upload it -*/
            if (path.extname(req.file.originalname).toLowerCase() === ".png" || path.extname(req.file.originalname).toLowerCase() === ".jpg" || path.extname(req.file.originalname).toLowerCase() === ".jpeg") {
                fs.rename(tempPath, targetPath, (err) => {
                    if (err) console.log(err);
                });
            }else{
                fs.unlink(tempPath, err => {
                    if (err) console.log(err);
                });
            }
        
            /*- Compress and downscale the image -*/
            gm(path.resolve(`uploads/profile/${suid}.jpg`))
                .resize(256, 256)
                .quality(50)/*- Compression -*/
                .autoOrient()/*- Rotation -*/
                .noProfile()/*- Remove EXIF data -*/
                .write(path.resolve(`uploads/profile/${suid}.jpg`), (err) => {
                    if (err) console.log(err)
                });
        
            res.sendStatus(200);
        }catch(e){
            console.log(e)
            res.sendStatus(404);
        }
    });

    /*- Simple profile data that outputs non-secret data -*/
    app.get("/profile-data", (req, res) => {
        const { suid } = req.headers;

        /*- Check if SUID was specified in headers -*/
        if (!suid) {
            return res.json({
                message: dictionary.error.missing_fields,
                status: 400
            });
        }

        /*- Search for the account using the SUID -*/
        MongoClient.connect(uri, async (_, client) => {
            try{
                const db = client.db(dbs);
                
                /*- Find user -*/
                const userData = await db.collection("users").findOne({ suid });
                
                /*- Response object, we don't want stuff like salt, password and other values -*/
                const responseData = {
                    username: userData.username,
                    displayname: userData.displayname,
                    joined: userData.joined,
                    role: userData.role,
                    profile: `${process.env.SERVER_URL}/api/profile-data/image/${suid}`,
                }
                
                /*- Send the response back -*/
                res.json({
                    ...responseData,
                    status: 200
                });
            }catch{
                res.json({
                    status: 404,
                    message: dictionary.error.user.not_found
                });
            }
        });
    });

    /*- Profile images -*/
    app.get("/profile-data/image/:img/", (req, res) => {
        /*- All profile images are named after the users SUID -*/
        const suid = req.params["img"];

        /*- Check if SUID was specified in headers -*/
        if (!suid) {
            return res.json({
                message: dictionary.error.missing_fields,
                status: 400
            });
        }

        /*- Check if the image exists -*/
        if (!fs.existsSync(path.resolve(`uploads/profile/${suid}.jpg`))) {
            return res.sendFile(
                path.resolve(`data/images/default-user.jpg`)
            );
        }

        /*- Send the image back to the client -*/
        res.sendFile(path.resolve(`uploads/profile/${suid}.jpg`));
    });

    /*- TEMP -*/
    app.get("/delete", (_, res) => {

        /*- TEMP -*/
        try {
            MongoClient.connect(uri, (_, client) => {

                const db = client.db(dbs);
                db.collection("users").deleteMany({}, () => {
                    return res.json({
                        message: dictionary.status.success,
                        status: 200,
                    });
                });
            });
        } catch {
            return res.json({
                message: dictionary.status.failure,
                status: 404,
            });
        }
    });

    /*- temporary image upload, html form -*/
    app.get("/temp", (req, res) => {
        res.send(`
            <form action="https://wss.artur.red/api/profile-upload" method="post" enctype="multipart/form-data">
                <input type="file" name="profile-file">
                <input type="submit" value="Upload">
            </form>

            <script>
                //cookie functions
                function setCookie(cname, cvalue, exdays) {
                    var d = new Date();
                    d.setTime(d.getTime() + (exdays*24*60*60*1000));
                    var expires = "expires="+ d.toUTCString();
                    document.cookie = cname + "=" + cvalue + ";" + expires + ";path=/";
                }

                function getCookie(cname) {
                    var name = cname + "=";
                    var decodedCookie = decodeURIComponent(document.cookie);
                    var ca = decodedCookie.split(';');
                    for(var i = 0; i <ca.length; i++) {
                        var c = ca[i];
                        while (c.charAt(0) == ' ') {
                            c = c.substring(1);
                        }
                        if (c.indexOf(name) == 0) {
                            return c.substring(name.length, c.length);
                        }
                    }
                    return "";
                }
            </script>
        `);
    });

    return app;
})();

const checkUsername = async (username, callback) => {

    /*- Check if username is too long -*/
    if (username.length > variables.username_len_max)     return callback({ success: false, message: dictionary.error.username.too_long });
    
    /*- Check if username is too short -*/
    if (username.length < variables.username_len_min)     return callback({ success: false, message: dictionary.error.username.too_short });
    
    /*- Check if username contains illegal characters -*/
    if (username.match(/[^a-zA-Z0-9_\.]/))                return callback({ success: false, message: dictionary.error.username.illegal });
    
    /*- Check if username is reserved -*/
    if (dictionary.reserved_usernames.includes(username)) return callback({ success: false, message: dictionary.error.username.reserved });

    /*- Check if username is already in use -*/
    try{
        MongoClient.connect(uri, async (err, client) => {
            if (err) console.log(err);
            
            const db = client.db(dbs);
            db.collection("users").findOne({ username }, (_, user) => {

                if (user) return callback({ success: false, message: dictionary.error.username.occupied });

                return callback({ success: true });
            });
        });
    }catch(err){
        return callback({ success: false, message: dictionary.error.internal });
    }
}
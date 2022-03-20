/*- The default user parameters -*/
class User {
    constructor(props) {
        this.uid = props.uid;
        this.suid = props.suid;
        this.password = props.password;
        this.email = props.email;
        this.salt = props.salt;
        this.username = props.username;
        this.displayname = props.displayname;
        this.joined = props.joined;
        this.role = props.role;
        this.profile = props.profile;
    }

    /*- Getters -*/
    get all() {
        return {
            uid: this.uid,
            suid: this.suid,
            password: this.password,
            email: this.email,
            salt: this.salt,
            username: this.username,
            displayname: this.displayname,
            joined: this.joined,
            role: this.role,
            profile: this.profile
        }
    }
}

/*- Safe user is User without sensitive information -*/
class SafeUser {
    constructor(props) {
        this.suid = props.suid;
        this.username = props.username;
        this.displayname = props.displayname;
        this.joined = props.joined;
        this.role = props.role;
        this.profile = `${process.env.SERVER_URL}/api/profile-data/image/${props.suid}`;
    }

    /*- Getters -*/
    get all() {
        return {
            suid: this.suid,
            username: this.username,
            displayname: this.displayname,
            joined: this.joined,
            role: this.role,
            profile: this.profile
        }
    }
}

/*- Export them to other files -*/
module.exports = {
    User,
    SafeUser
}
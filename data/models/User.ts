/*- The default user parameters -*/
class User {
    uid: string;
    suid: string;
    username: string;
    displayName: string;
    email: string;
    password: string;
    joined: string;
    role: string;
    salt: string;
    profile: string;
    
    constructor(uid: string, suid: string, username: string, displayName: string, email: string, password: string, joined: string, role: string, salt: string, profile: string) {
        this.uid = uid;
        this.suid = suid;
        this.username = username;
        this.displayName = displayName;
        this.email = email;
        this.password = password;
        this.joined = joined;
        this.role = role;
        this.salt = salt;
        this.profile = profile;
    }
}

/*- Safe user is User without sensitive information -*/
class SafeUser {
    suid: string;
    username: string;
    displayName: string;
    joined: string;
    role: string;
    profile: string;
}

/*- Export them to other files -*/
module.exports = {
    User,
    SafeUser
}
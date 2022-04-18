## Account API

API made using docker, mongodb, nodejs and more.
Free for everyone to use.

## Create Accounts

```javascript
/*- Change this to the URL where your API is running on -*/
const API_URL = "http://localhost:3000/";

/*- Create Account -*/
const createAccount = async (name, email, password, displayName) => {
    const response = await fetch(API_URL + "api/create-account", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            name,
            displayName,
            email,
            password,
        },
    });
    const data = await response.json();
    return data;
};
```

## Other routes

 - api/login &nbsp;&emsp;&emsp;&emsp;&emsp;&emsp;**Headers:** `email` `password`
 - api/profile-data &emsp;&emsp;**Headers:** `suid`
 - api/profile-data/image/`suid`/



## The Command Line Interface (CLI)
To begin using the CLI, you'll need to build the docker containers. The CLI depends on most of them.
run ```$ docker-compose build``` and wait - it might take some time at first.

After everything is set, run ```$ docker-compose run cli```, it will start building the CLI for you, which may also take some time, again 😪.

Some time later you'll be granted with a terminal-looking CLI. Type help for further info on all available commands!
```
==> help
```

```
==> create name:john last_name:doe email:john@doe.com
Document created!
```
```
==> get where name is john
Some(Document({"_id": ..., "name": String("john"), "last_name": Strin...
```

## Contributing
Pull requests are welcome! For major changes, please open an issue first to discuss what you'd like to change.


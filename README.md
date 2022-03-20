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
 - api/profile-data &emsp;&emsp;**Headers:** `suid`
 - api/profile-data/image/`suid`/

## Contributing
Pull requests are welcome! For major changes, please open an issue first to discuss what you'd like to change.
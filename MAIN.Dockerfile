FROM node:14

WORKDIR /usr/src/app
COPY ["package.json", "package-lock.json*", "./"]

RUN npm install

COPY . .

EXPOSE 8080

CMD ["./node_modules/nodemon/bin/nodemon.js"]
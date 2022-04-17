FROM node:14

WORKDIR /usr/src/app
COPY ["package.json", "package-lock.json*", "./"]

RUN npm install

COPY . .

EXPOSE 8081

CMD ["./node_modules/nodemon/bin/nodemon.js"]
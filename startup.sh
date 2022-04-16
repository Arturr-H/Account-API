## install dependencies
echo -e "\033[33mStarting mongo&node container...\033[0m"
docker-compose up -d
echo -e "\033[33mStarting main server...\033[0m"
(node index.js&)
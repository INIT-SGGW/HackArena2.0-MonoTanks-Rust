# Docker

<!-- TODO: NOT WORKING YET -->

```sh
# Set variables
tagName="rust_client"
memoryLimit="2g"
host="localhost"
port="5000"
joinCode=""

# Build image 
docker build -t $tagName .

# Run container with memory limit
docker run --rm --memory $memoryLimit --network="bridge" $tagName --host $host --port $port --code "$joinCode"
```

Or you can for testing do this one liner:

```sh
tagName="rust_client"; memoryLimit="2g"; url="ws://prosi100pkt.lol:5000/?joinCode="; joinCode="1234"; docker build -t $tagName .; docker run --rm --memory $memoryLimit $tagName $url$joinCode
```

How to stop and delete all containers and images with a given tag name:

```sh
# Set tag name
tagName="rust_client"

# Stop and remove all containers
docker ps -a -q --filter "ancestor=$tagName" | xargs -r docker stop | xargs -r docker rm

# Remove all images
docker images -q $tagName | xargs -r docker rmi
```

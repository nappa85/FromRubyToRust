docker run -ti -v $(pwd)/ruby:/app --workdir /app ruby /app/docker-entrypoint.sh | tail -n2
docker run -ti -v $(pwd)/rust:/app --workdir /app rust cargo bench | tail -n6

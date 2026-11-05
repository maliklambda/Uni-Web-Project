# Run this script to build and run the docker container
docker build -t query_visualizer .

# Simulate Render environment
docker run -p 10000:10000 \
  -e PORT=10000 \
  query_visualizer

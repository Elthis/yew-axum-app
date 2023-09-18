serve-frontend:
    cd frontend && trunk serve

local-build:
    cd frontend && trunk build && cd .. && cargo build

local-run:
    cd frontend && trunk build && cd .. && BACKEND_DIST_PATH="frontend/dist" cargo run --bin backend

docker-build:
    docker build --network host -t yew-axum-webapp .

docker-run:
    docker run --network host --rm -it yew-axum-webapp:latest

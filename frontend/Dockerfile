# Dockerfile.frontend
FROM node:latest

RUN npm install -g elm

WORKDIR /app/frontend

COPY ./frontend /app/frontend

CMD ["elm", "reactor", "--port=3002"]
FROM node:20-alpine

WORKDIR /app

RUN npm install -g pnpm

COPY package.json pnpm-workspace.yaml pnpm-lock.yaml ./

COPY services/auth-service/package.json ./services/auth-service/package.json

RUN pnpm install --frozen-lockfile

COPY . .

WORKDIR /app/services/auth-service

EXPOSE 4002

CMD ["pnpm", "dev"]

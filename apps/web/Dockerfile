FROM node:20-alpine

WORKDIR /app

RUN npm install -g pnpm

COPY package.json pnpm-workspace.yaml pnpm-lock.yaml ./

COPY apps/web/package.json ./apps/web/package.json
COPY services/auth-service/package.json ./services/auth-service/package.json
COPY services/payment-service/package.json ./services/payment-service/package.json
COPY services/commission-service/package.json ./services/commission-service/package.json

RUN pnpm install --frozen-lockfile

COPY . .

WORKDIR /app/apps/web

EXPOSE 5173

CMD ["pnpm", "dev", "--host"]
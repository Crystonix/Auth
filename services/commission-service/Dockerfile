FROM node:20-alpine

WORKDIR /app

# Install PNPM globally
RUN npm install -g pnpm

# Copy root package.json and lockfile
COPY package.json pnpm-lock.yaml ./

# Install dependencies
RUN pnpm install --shamefully-hoist

# Copy service source code
COPY services/commission-service ./

EXPOSE 4001
CMD ["pnpm", "dev"]

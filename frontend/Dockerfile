FROM node:20-alpine

WORKDIR /frontend

# Install pnpm
RUN npm install -g pnpm

# Copy package.json and lockfile first for caching
COPY package.json ./package.json
COPY pnpm-lock.yaml ./pnpm-lock.yaml

# Install dependencies
RUN pnpm install --frozen-lockfile

# Copy source code
COPY . .

EXPOSE 5173

# Run dev server
CMD ["pnpm", "dev", "--host"]

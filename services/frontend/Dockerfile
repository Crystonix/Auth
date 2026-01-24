FROM node:22-alpine

WORKDIR /frontend

# Install pnpm globally
RUN npm install -g pnpm

# Copy all files from build context (frontend/)
COPY . .

# Tell pnpm we are in CI to avoid TTY errors
ENV CI=true

# Install dependencies
RUN pnpm install --frozen-lockfile

# Build production
RUN pnpm build

# Expose SvelteKit preview port
EXPOSE 5173

CMD ["node", "build/index.js"]
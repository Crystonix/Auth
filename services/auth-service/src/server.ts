import Fastify from 'fastify';
import cookie from '@fastify/cookie';
import session from '@fastify/session';
import Keycloak from 'keycloak-connect';
import { createClient } from 'redis';
import {RedisStore} from 'connect-redis';
import { authRoutes } from './routes/auth.ts';

const fastify = Fastify({ logger: true });

// --- Redis client (node-redis) ---
const redisClient = createClient({
  url: `redis://${process.env.REDIS_HOST || 'redis'}:${process.env.REDIS_PORT || 6379}`
});
redisClient.on('error', (err) => fastify.log.error('Redis Client Error', err));
await redisClient.connect();

// --- Redis store for sessions ---
let redisStore = new RedisStore({
  client: redisClient,
  prefix: "commi:",
});

// --- Cookie & session ---
await fastify.register(cookie);
await fastify.register(session, {
  store: redisStore,
  secret: process.env.SESSION_SECRET || 'supersecret32charactersmin!',
  cookie: {
    httpOnly: true,
    secure: process.env.ENV_MODE === 'prod',
    sameSite: 'lax',
    maxAge: 24 * 60 * 60 * 1000
  }
});

// --- Keycloak setup ---
const keycloakConfig = {
  realm: process.env.KEYCLOAK_REALM || 'commission-realm',
  'auth-server-url': process.env.KEYCLOAK_URL || 'http://keycloak:8080',
  'ssl-required': 'external',
  resource: process.env.KEYCLOAK_CLIENT_ID || 'commi-client',
  credentials: { secret: process.env.KEYCLOAK_CLIENT_SECRET },
  'confidential-port': 0
};

const keycloak = new Keycloak({ store: redisStore }, keycloakConfig);
fastify.decorate('keycloak', keycloak);

// --- CORS ---
await fastify.register(import('@fastify/cors'), {
  origin: process.env.FRONTEND_URL || 'http://localhost:5173',
  credentials: true
});

// --- Auth routes ---
await fastify.register(authRoutes);

// --- Start server ---
const start = async () => {
  try {
    await fastify.listen({ port: 4000, host: '0.0.0.0' });
    console.log('Auth service running on http://localhost:4000');
  } catch (err) {
    fastify.log.error(err);
    process.exit(1);
  }
};

start();

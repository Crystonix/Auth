import { FastifyPluginAsync, FastifyRequest, FastifyReply } from 'fastify';
import { Keycloak } from 'keycloak-connect';
import { Request as ExpressRequest, Response as ExpressResponse } from 'express';

// Augment FastifySessionObject to include 'user'
declare module '@fastify/session' {
  interface FastifySessionObject {
    user?: any;
  }
}

// Type augmentation
declare module 'fastify' {
  interface FastifyInstance {
    keycloak: Keycloak;
  }
  interface FastifyRequest {
    kauth?: {
      grant: {
        access_token: {
          content: any;
        };
      };
    };
  }
}

export const authRoutes: FastifyPluginAsync = async (fastify) => {

  // --- Login: redirect to Keycloak ---
fastify.get('/login', async (req: FastifyRequest, reply: FastifyReply) => {
  const state = Math.random().toString(36).substring(2); // CSRF state
  const redirectUri = process.env.KEYCLOAK_REDIRECT_URI || 'http://localhost:4000/api/auth/callback';

  // Generate login URL
  let loginUrl = fastify.keycloak.loginUrl(state, redirectUri);

  // Force Discord as the IdP
  const separator = loginUrl.includes('?') ? '&' : '?';
  loginUrl += `${separator}kc_idp_hint=discord`;

  return reply.redirect(loginUrl);
});


    // --- Callback: handle Keycloak (and IdP) response ---
  fastify.get('/callback', async (req: FastifyRequest, reply: FastifyReply) => {
    try {
      // Wrap Express middleware in a Promise
      await new Promise<void>((resolve, reject) => {
        fastify.keycloak.protect()(
          req.raw as unknown as ExpressRequest,
          reply.raw as unknown as ExpressResponse,
          (err?: any) => {
            if (err) return reject(err);
            resolve();
          }
        );
      });

      const grant = req.kauth?.grant;
      if (!grant) throw new Error('No grant received');

      // Store user info in session
      req.session.user = grant.access_token.content;

      // Redirect to frontend dashboard
      return reply.redirect((process.env.FRONTEND_URL || 'http://localhost:5173') + '/dashboard');
    } catch (err) {
      fastify.log.error(err);
      return reply.status(500).send({ error: 'Login failed' });
    }
  });

  // --- Protected route: user info ---
  fastify.get('/me', {
    preHandler: async (req: FastifyRequest, reply: FastifyReply) => {
      await new Promise<void>((resolve, reject) => {
        fastify.keycloak.protect()(
          req.raw as unknown as ExpressRequest,
          reply.raw as unknown as ExpressResponse,
          (err?: any) => {
            if (err) return reject(err);
            resolve();
          }
        );
      });
    }
  }, async (req) => {
    return req.kauth?.grant.access_token.content || { message: 'Not logged in' };
  });

  // --- Logout ---
  fastify.get('/logout', {
    preHandler: async (req: FastifyRequest, reply: FastifyReply) => {
      await new Promise<void>((resolve, reject) => {
        fastify.keycloak.protect()(
          req.raw as unknown as ExpressRequest,
          reply.raw as unknown as ExpressResponse,
          (err?: any) => {
            if (err) return reject(err);
            resolve();
          }
        );
      });
    }
  }, async (req: FastifyRequest, reply: FastifyReply) => {
    req.session.destroy(() => {
      reply.clearCookie('connect.sid').redirect(process.env.FRONTEND_URL || '/');
    });
  });
};
import Fastify from "fastify";
import cors from "@fastify/cors";

const app = Fastify();
app.register(cors);

app.get("/hello", async () => ({ msg: "Hello from commission service" }));

app.listen({ port: 4000, host: "0.0.0.0" });

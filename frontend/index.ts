import { type BunRequest, serve } from "bun";
import index from "src/index.html";

const proxyPort = "3000";

const server = serve({
  port: 3001,
  routes: {
    // Serve index.html for all unmatched routes.
    "/": index,
    "/*": async (req: BunRequest) => {
      const url = new URL(req.url);
      url.port = proxyPort;

      const newReq = new Request(url, {
        method: req.method,
        headers: req.headers,
        body: req.body,
      });

      const res = await fetch(newReq);
      return new Response(res.body, {
        status: res.status,
        statusText: res.statusText,
        headers: res.headers,
      });
    },
  },

  development: process.env.NODE_ENV !== "production" && {
    // Enable browser hot reloading in development
    hmr: true,

    // Echo console logs from the browser to the server
    console: true,
  },
});

console.log(`🚀 Server running at ${server.url}`);

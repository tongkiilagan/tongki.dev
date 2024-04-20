import { Elysia } from "elysia";
import { staticPlugin } from "@elysiajs/static";
import { html } from "@elysiajs/html";

const app = new Elysia()
  .use(html())
  .use(staticPlugin())
  .get("/",  () => {
    return (
      <html lang="en">
        <head>
          <title>Tongki | Home</title>
          <meta charset="UTF-8"/>
          <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
          <link type="text/css" rel="stylesheet" href="/public/materialize.min.css"  media="screen,projection"/>
          <link href="https://fonts.googleapis.com/icon?family=Material+Icons" rel="stylesheet"/>
          <link rel="preconnect" href="https://fonts.googleapis.com"/>
          <link rel="preconnect" href="https://fonts.gstatic.com"/>
          <link href="https://fonts.googleapis.com/css2?family=IM+Fell+DW+Pica&display=swap" rel="stylesheet"/>    
          <script src="/public/htmx.min.js"></script>
        </head>
        <body>

          <script type="text/javascript" src="/public/materialize.min.js"></script>
        </body>
      </html>
    )
    
  })
  .listen(3000);

console.log(
  `Your application is running at http://${app.server?.hostname}:${app.server?.port}`
);
# Structure

## /pages

This is the directory where you put the main pages. Each directory in `/pages` would be the HTML files for the matched routes. For example, if you add `/pages/example` and put pages inside that directory, then the parsed HTML file would serve the route `/example`. You can add even deeper directories to serve the matched routes.

The parsed HTML will have the same name as the .txt files you put in this directory. For most cases, you will add the `index.txt` file for main pages, and you can also add files like `404.txt` for `Not Found` cases.

## /components

This directory is for the components that you can use throughout your main pages. You can create multiple files(components), but every file should have its name starting with `_`, not to be confused with normal HTML tags. To know how to define and use components, check out [this documentation](./txt-grammar.md).

## /public

You can store `.css`, `.js` source code, images, videos, or any files here to access from `/public/**`.

TODO: add `recommened using `/public/css`for css and `/public/js` for using js.

## /dist

This is the directory that this tool puts its final result in. You can upload this directory to deploy your project to Cloudflare, Vercel, or anywhere.

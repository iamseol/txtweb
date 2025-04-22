# Structure

## /contents

This is the directory where you put the main contents in. Each directory in `/contents` would be the HTML files for the matched routes. For example, if you add `/contents/example` and put contents inside of that directory, then the parsed HTML file would serve the route `/example`. You can add more and more directories even deeper, to serve the mached routes.

The directories that are named with a three-digit number, it is considered an error page and they will serve special cases like `/contents/404` for `Not Found`, or `/contents/500` for `Server Internal Error`. As you might have guessed, you need to name the directories with the status code.

The root directory(`/contents`, not `/`), should have the `index.txt` file. You can add any contents there like adding tags, but you should also put `#{name}` to include dynamic contents for different routes. The `#{name}` would point to the file that has the same name and exists in the same directory.

## /components

This directory is for the components that you can use throughout your main pages. You can create multiple files(components), but every file should have its name starting with `_` not to be confused with normal HTML tags. Lastly, you cannot use components inside of components.

## /public

You can store `.css`, `.js` source code, images, videos, or any files here to access from `/public/**`.

## /dist

This is the directory that this tool puts its final result in. You can upload this directory to deploy your project to Cloudflare, Vercel, or anywhere.

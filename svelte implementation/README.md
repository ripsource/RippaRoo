This demo of a flappy bird clone is not representative of best practices for either Svelte or Scrypto. The full live version can be found at https://rippyclip.com/rippyroo

I've been learning Scrypto and Svelte for only a few months and while I've been able to make more advanced dApps than this, I thought this would be a fun and easy example to do. In fact, it took less than an hour or so to pull together the Scrypto code and the javascript to make this. There are lots of improvements that could be made if you spent a little more time on the Scrypto code, as well as areas to expand or build upon what's there. I'm also a complete amateur, so defer to other official or more to up to date resources from Radix in all instances.

Saying all this, it does work (kind of), which could be helpful to someone, somewhere. I've put some basic comments throughout the svelte files, but if there are any questions, feel free to post a message to me in Discord directly or through relevant channels in the Radix Scrypto discussions.

For general knowledge of how to deploy packages and the requirements to run this - you should follow the much clearer explanations found on the full-stack gumball example on Radix's Github - https://github.com/radixdlt/scrypto-examples/tree/main/full-stack/dapp-toolkit-gumball-machine

Helpful resources for learning Scrypto can be found on the Radix Discord under the "Developers" chat groups, in the "dev-resources" channel. Beem's message on 19/04/2023 provides the best concise list of resources that I've been using to learn Scrypto.



Boiler Plate Intructions for SvelteKit 

# create-svelte

Everything you need to build a Svelte project, powered by [`create-svelte`](https://github.com/sveltejs/kit/tree/master/packages/create-svelte).

## Creating a project

If you're seeing this, you've probably already done this step. Congrats!

```bash
# create a new project in the current directory
npm create svelte@latest

# create a new project in my-app
npm create svelte@latest my-app
```

## Developing

Once you've created a project and installed dependencies with `npm install` (or `pnpm install` or `yarn`), start a development server:

```bash
npm run dev

# or start the server and open the app in a new browser tab
npm run dev -- --open
```

## Building

To create a production version of your app:

```bash
npm run build
```

You can preview the production build with `npm run preview`.

> To deploy your app, you may need to install an [adapter](https://kit.svelte.dev/docs/adapters) for your target environment.

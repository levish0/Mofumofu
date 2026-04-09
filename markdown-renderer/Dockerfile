FROM oven/bun:1 AS base
WORKDIR /app

# --- install dependencies ---
FROM base AS install

RUN mkdir -p /temp/dev
COPY package.json bun.lock /temp/dev/
RUN cd /temp/dev && bun install --frozen-lockfile

RUN mkdir -p /temp/prod
COPY package.json bun.lock /temp/prod/
RUN cd /temp/prod && bun install --frozen-lockfile --production

# --- build ---
FROM base AS build
COPY --from=install /temp/dev/node_modules node_modules
COPY . .
RUN bun run build

# --- release ---
FROM base AS release

RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*

COPY --from=install /temp/prod/node_modules node_modules
COPY --from=build /app/dist ./dist
COPY --from=build /app/package.json .

EXPOSE 6700
ENTRYPOINT ["bun", "dist/index.js"]

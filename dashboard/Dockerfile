FROM node:13

COPY package.json .
RUN yarn

COPY . .

CMD yarn build && yarn serve --host=0.0.0.0

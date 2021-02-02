# 親イメージとして公式イメージを使う
FROM node:current-slim

# 作業用ディレクトリの指定
WORKDIR /usr/src/app

# ホスト上のファイルを現在の場所にコピー
COPY package.json .

# イメージのファイルシステムでコマンドを実行
RUN npm install

# 実行時にコンテナが特定のポートをリッスンするようDockerに通知
EXPOSE 8080

# コンテナ内で指定したこまんどを実行
CMD [ "npm", "start" ]

# 残りのソースコードをホスト上からイメージ上のファイルシステムにコピー
COPY . .

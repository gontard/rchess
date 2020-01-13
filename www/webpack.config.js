const path = require("path");
const CopyWebpackPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

const appConfig = {
  entry: "./js/main.js",
  output: {
    path: dist,
    filename: "main.js"
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin([
      "index.html",
      {
        from: "node_modules/chessboardjs/www/css/chessboard.css",
        to: "css/chessboard.css"
      },
      { from: "css/rchess.css", to: "css/rchess.css" },
      { from: "node_modules/chessboardjs/www/img", to: "img" },
      { from: "img", to: "img" }
    ])
  ],
  devServer: {
    port: 8080,
    disableHostCheck: true
  }
};

const workerConfig = {
  entry: "./js/worker.js",
  target: "webworker",
  plugins: [
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "../crate-wasm")
    })
  ],
  resolve: {
    extensions: [".js", ".wasm"]
  },
  output: {
    path: dist,
    filename: "worker.js"
  }
};

module.exports = [appConfig, workerConfig];

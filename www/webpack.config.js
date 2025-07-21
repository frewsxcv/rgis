const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./bootstrap.js",
  experiments: { asyncWebAssembly: true },
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  resolve: {
    alias: {
      "rgis-pkg": path.resolve(__dirname, "../rgis/pkg"),
    },
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin({
      patterns: [{
        from: 'assets',
        to: 'assets',
        context: '../rgis'
      }, {
        from: 'index.html',
        to: 'index.html',
      }, {
        from: 'index.js',
        to: 'index.js',
      }]
    }),
  ],
};

const HtmlWebpackPlugin = require('html-webpack-plugin')
const MiniCssExtractPlugin = require('mini-css-extract-plugin')
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin")
const path = require('path')
const resolve_rel = (...a) => require('path').resolve(__dirname, ...a)

const mode = process.env.NODE_ENV || 'development'
const is_dev = mode === 'development'

module.exports = {
  mode,
  entry: './src/main.js',
  output: {
    path: resolve_rel('dist'),
  },
  resolve: {
    alias: {
      'muscad-wasm': resolve_rel('../muscad-wasm/pkg'),
    },
  },
  module: {
    rules: [
      {
        test: /\.glu/i,
        use: 'raw-loader',
      },
      {
        test: /\.s[c|a]ss/i,
        use: [
          is_dev ? 'style-loader' : MiniCssExtractPlugin.loader,
          'css-loader',
          'sass-loader',
        ]
      },
      //{ test: /\.(wasm)$/, use: 'file-loader', type: 'javascript/auto' },
    ],
  },
  plugins: [
    new MiniCssExtractPlugin({
      filename: './style/main.css'
    }),
    new HtmlWebpackPlugin({
      template: 'index.html',
      inlineSource: '.(js|css)$',
    }),
    new WasmPackPlugin({
      crateDirectory: resolve_rel('../muscad-wasm'),
      extraArgs: '--no-typescript',
      watchDirectories: [
        resolve_rel('../muscad-gluon')
      ],
    }),
  ],
}

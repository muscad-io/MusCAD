const HtmlWebpackPlugin = require('html-webpack-plugin')
const path = require('path')
const resolve_rel = (...a) => require('path').resolve(__dirname, ...a)

const mode = process.env.NODE_ENV || 'development'
const is_dev = mode === 'development'

module.exports = {
  mode,
  devtool: 'eval-source-map',
  entry: './src/main.js',
  output: {
    path: resolve_rel('dist'),
  },
  resolve: {
    fallback: { env: false },
    alias: {
      'muscad-gluon': resolve_rel('../muscad-gluon/pkg/muscad_gluon'),
    },
  },
  module: {
    rules: [
      {
        test: /\.(glu)/i,
        use: 'raw-loader',
      },
      {
        test: /\.s[c|a]ss/i,
        use: [
          'style-loader',
          'css-loader',
          'sass-loader',
        ]
      },
    ],
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: 'index.html',
      inlineSource: '.(js|css)$',
    }),
    //new (require("@wasm-tool/wasm-pack-plugin"))({
    //  crateDirectory: resolve_rel('../muscad-gluon'),
    //  extraArgs: '--no-typescript',
    //  watchDirectories: [ resolve_rel('../muscad-gluon/src') ],
    //}),
  ],
  experiments: {
    syncWebAssembly: true,
  },
}

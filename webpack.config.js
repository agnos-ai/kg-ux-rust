//
// TODO: Set up prod, dev and common versions of this file as described here:
// https://www.freecodecamp.org/news/creating-a-production-ready-webpack-4-config-from-scratch/
//
const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const HTMLWebpackPlugin = require("html-webpack-plugin");
const { CleanWebpackPlugin } = require('clean-webpack-plugin')
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const FileManagerPlugin = require('filemanager-webpack-plugin');

const isDevelopment = process.env.NODE_ENV === 'development';

module.exports = (env, argv) => {
  return {
    mode: argv.mode,
    //
    // See https://webpack.js.org/configuration/output/
    //
    output: {
      path: path.join(__dirname, './dist'),
      filename: "js/kg_ux.[fullhash].js",
//      webassemblyModuleFilename: "kg_ux.wasm"
    },
    //
    // See https://webpack.js.org/configuration/dev-server/
    //
    devServer: {
      contentBase: path.join(__dirname, './dist'),
      compress: argv.mode === 'production',
      port: 5000
    },
    entry: [
      './static/js/kg_ux.js',
      './static/css/styles.scss'
    ],
    experiments: {
      asyncWebAssembly: false,
      syncWebAssembly: true
    },
    resolve: {
      extensions: ['.js', '.jsx', '.scss']
    },
    module: {
      rules: [
        {
          test: /\.s[ac]ss$/i,
          use: [
            // Creates `style` nodes from JS strings
            "style-loader",
            // Translates CSS into CommonJS
            "css-loader",
            // Compiles Sass to CSS
            {
              loader: "sass-loader",
              options: {
                implementation: require("sass"),
                sassOptions: {
                  fiber: false,
                },
                sourceMap: isDevelopment,
              },
            },
          ],
        },
        {
          test: /\.ttf$/,
          use: [
            'url-loader',
          ],
        },
      ],
    },
    plugins: [
      new HTMLWebpackPlugin({
        hash: true,
        title: 'Nirvana KG/UX - ${argv.mode}',
        filename: 'index.html',
        template: 'static/index.html'
      }),
      new MiniCssExtractPlugin({
        // Options similar to the same options in webpackOptions.output
        // both options are optional
        filename: isDevelopment ? '[name].css' : '[name].[fullhash].css',
        chunkFilename: isDevelopment ? '[id].css' : '[id].[fullhash].css'
      }),
      new CleanWebpackPlugin(),
      new FileManagerPlugin({
        events: {
          onStart: {
            mkdir: ['./dist/js', './dist/css'],
          },
        },
      }),
      //
      // See https://www.npmjs.com/package/copy-webpack-plugin
      //
//      new CopyWebpackPlugin({
//        patterns: [
//          { from: './static/js', to: path.join(__dirname, './dist/js/') },
//          { from: './static/css', to: path.join(__dirname, './dist/css/') },
//          { from: './static/image', to: path.join(__dirname, './dist/image/') },
//        ],
//      }),
      //
      // See https://github.com/wasm-tool/wasm-pack-plugin
      //
      new WasmPackPlugin({
        crateDirectory: ".",
        outDir: path.join(__dirname, './dist/js'),
        outName: "kg_ux_wasm",
        watchDirectories: [
          path.resolve(__dirname, "./src")
        ],
        forceWatch: false,
        // Check https://rustwasm.github.io/wasm-pack/book/commands/build.html for
        // the available set of arguments.
        //
        // Optional space delimited arguments to appear before the wasm-pack
        // command. Default arguments are `--verbose`.
//        args: "--log-level warn",
        // Default arguments are `--typescript --target browser --mode normal`.
        extraArgs: "--no-typescript",
        pluginLogLevel: 'debug'
      })
    ],
    optimization: {
      splitChunks: {
        chunks: 'all',
        minSize: {
          javascript: 30000,
          webassembly: 50000,
        },
        maxSize: {
          javascript: 300000,
          webassembly: 500000,
        }
      }
    },
    performance: {
      maxAssetSize: 400000
    },
    watch: argv.mode !== 'production'
  };
};
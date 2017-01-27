// Frontend Webpack build configs

var webpack = require('webpack');

// Command line env variable to minify output
var minify = JSON.parse(process.env.MINIFY || '0');

var plugins = [
  new webpack.ProvidePlugin({
    $: 'jquery',
    jQuery: 'jquery',
  }),
];

if (minify) {
  plugins.push(new webpack.optimize.UglifyJsPlugin({
    compress: { warnings: false }
  }));
}

module.exports = {
  entry: {
    // Frontend code (no other targets for now)
    main: './main.ts',
  },
  resolve: {
    extensions: ['', '.webpack.js', '.web.js', '.ts', '.js']
  },
  output: {
    path: '../www/output',
    filename: '[name].built.js'
  },
  module: {
    loaders: [
      {
        loader: 'ts-loader',
        test: /\.ts$/
      }
    ]
  },
  plugins: plugins,
}


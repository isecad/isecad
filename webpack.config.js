const { resolve } = require('upath');

const HtmlPlugin = require('html-webpack-plugin');
const TerserPlugin = require('terser-webpack-plugin');
const { BundleAnalyzerPlugin } = require('webpack-bundle-analyzer');

const dev = process.env.NODE_ENV === 'development';

module.exports = {
    mode: dev ? 'development' : 'production',
    devtool: 'source-map',
    resolve: {
        extensions: ['.ts', '.tsx', '.js', '.jsx', '.json'],
        alias: {
            '~': resolve(__dirname, 'src'),
            '-': resolve(__dirname, 'build'),
        },
    },
    optimization: {
        minimizer: [
            new TerserPlugin({
                terserOptions: require('./terser.config.json'),
            }),
        ],
    },
    module: {
        rules: [
            {
                test: /\.[jt]sx?$/iu,
                use: 'source-map-loader',
                enforce: 'pre',
            },
            {
                test: /\.[jt]sx?$/iu,
                exclude: /node_modules/u,
                use: 'babel-loader',
            },
            {
                test: /\.(png|jpe?g|gif|svgz?|ttf|otf|eot|woff2?)$/iu,
                use: 'url-loader',
            },
            {
                test: /\.glsl$/iu,
                use: 'raw-loader',
            },
        ],
    },
    plugins: [
        new HtmlPlugin({
            template: './src/index.html',
        }),
        new BundleAnalyzerPlugin({
            analyzerMode: 'static',
            defaultSizes: 'gzip',
            openAnalyzer: false,
        }),
    ],
    devServer: {
        compress: true,
        contentBase: resolve(__dirname, 'dist'),
        port: 1488,
    },
};

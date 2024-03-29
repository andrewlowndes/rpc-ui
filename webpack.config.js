const path = require('path');

module.exports = {
    //mode: "production",
    mode: "development",
    entry: './js/index.ts',
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                use: 'ts-loader',
                exclude: /node_modules/,
            },
        ],
    },
    resolve: {
        extensions: ['.tsx', '.ts', '.js'],
    },
    output: {
        filename: 'lib.js',
        path: path.resolve(__dirname, 'public'),
    },
};

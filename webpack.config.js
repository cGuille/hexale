const path = require('path');

module.exports = {
    entry: './src/web/main.js',
    devtool: 'inline-source-map',
    output: {
        filename: 'hexale.js',
        path: path.resolve(__dirname, 'static', 'js')
    }
};

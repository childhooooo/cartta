const path = require('path')
const MiniCssExtractPlugin=require('mini-css-extract-plugin')

module.exports = {
  pages: {
    single: 'src/single/main.js',
    editor: 'src/editor/main.js',
    book: 'src/book/main.js',
    signup: 'src/signup/main.js',
    signin: 'src/signin/main.js',
    error: 'src/error/main.js'
  },

  chainWebpack: config => {
    const types = ['vue-modules', 'vue', 'normal-modules', 'normal']
    types.forEach(type => addStyleResource(config.module.rule('stylus').oneOf(type)))
    if (process.env.NODE_ENV === 'production'){
      config.optimization.delete('splitChunks')
      config.output.filename('js/[name].js')
      config.plugin('extract-css')
          .use(MiniCssExtractPlugin, [{
            filename:'css/[name].css',
            chunkFilename:''
          }])
    }
  },

  configureWebpack: {
    resolve: {
      alias: {
        config: path.resolve('./config.js'),
      }
    }
  },

}

function addStyleResource (rule) {
  rule.use('style-resource')
    .loader('style-resources-loader')
    .options({
      patterns: [
        path.resolve(__dirname, './src/styles/imports.styl'),
      ],
    })
}
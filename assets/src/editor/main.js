import Vue from 'vue'
import Vuetify from 'vuetify'
import Editor from './Editor.vue'
import 'vuetify/dist/vuetify.min.css' 
import store from '../store'

Vue.config.productionTip = false
Vue.use(Vuetify)

new Vue({
  store,
  render: h => h(Editor),
}).$mount('#editor')
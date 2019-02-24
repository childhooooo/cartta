import Vue from 'vue'
import Vuetify from 'vuetify'
import Single from './Single.vue'
import 'vuetify/dist/vuetify.min.css' 
import store from '../store'

Vue.config.productionTip = false
Vue.use(Vuetify)

Vue.config.productionTip = false

new Vue({
  store,
  render: h => h(Single),
}).$mount('#single')

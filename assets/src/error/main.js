import Vue from 'vue'
import Vuetify from 'vuetify'
import Error from './Error.vue'
import 'vuetify/dist/vuetify.min.css' 

Vue.config.productionTip = false
Vue.use(Vuetify)

new Vue({
  render: h => h(Error),
}).$mount('#error')